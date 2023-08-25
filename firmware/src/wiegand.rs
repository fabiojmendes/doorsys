use core::ffi::c_void;
use std::{
    ffi::CString,
    ptr,
    sync::mpsc::{self, Receiver, RecvTimeoutError, Sender},
    time::Duration,
};

use esp_idf_sys::{
    esp, esp_timer_create, esp_timer_create_args_t, esp_timer_delete,
    esp_timer_dispatch_t_ESP_TIMER_TASK, esp_timer_handle_t, esp_timer_start_once, esp_timer_stop,
    gpio_config, gpio_config_t, gpio_get_level, gpio_int_type_t_GPIO_INTR_DISABLE,
    gpio_int_type_t_GPIO_INTR_NEGEDGE, gpio_isr_handler_add, gpio_isr_handler_remove,
    gpio_mode_t_GPIO_MODE_INPUT, gpio_reset_pin, gpio_set_intr_type,
};

const WIEGAND_TIMEOUT: u64 = 50000; // 50ms
const BUFFER_SIZE: usize = 4;
const PIN_TIMEOUT: Duration = Duration::from_secs(10);

#[link_section = ".iram0.text"]
unsafe extern "C" fn wiegand_interrupt(arg: *mut c_void) {
    let reader = &mut *(arg as *mut Reader);
    let d0 = gpio_get_level(reader.gpio_d0) as u32;
    let d1 = gpio_get_level(reader.gpio_d1) as u32;
    if d0 == d1 {
        return;
    }
    // Overflow
    if reader.bits > reader.data.len() * 8 {
        return;
    }

    let timer = reader.timer.unwrap();

    esp_timer_stop(timer);

    let value = if d0 == 0 { 0 } else { 0x80 };
    reader.data[reader.bits / 8] |= value >> (reader.bits % 8);
    reader.bits += 1;

    esp_timer_start_once(timer, WIEGAND_TIMEOUT);
}

unsafe extern "C" fn timer_interrupt(arg: *mut c_void) {
    let reader = &mut *(arg as *mut Reader);
    reader.stop();

    let packet = Packet::new(reader.bits, reader.data);

    if let Err(e) = reader.reader_tx.send(packet) {
        log::error!("send error {}", e);
    }
    reader.reset();
}

#[derive(Debug)]
pub enum Packet {
    Key {
        key: u8,
    },
    Card {
        rfid: u32,
    },
    Unknown {
        bits: usize,
        data: [u8; BUFFER_SIZE],
    },
}

impl Packet {
    fn new(bits: usize, data: [u8; BUFFER_SIZE]) -> Self {
        log::info!("data received; bits: {}, data: {:02X?}", bits, data);
        match bits {
            4 => Self::Key { key: data[0] >> 4 },
            26 => {
                let mut rfid: u32 = (data[0] as u32) << 24
                    | (data[1] as u32) << 16
                    | (data[2] as u32) << 8
                    | (data[3] as u32);

                rfid &= !(1 << 31);
                rfid >>= 7;
                // TODO: check parity
                Self::Card { rfid }
            }
            _ => Self::Unknown { bits, data },
        }
    }
}

pub struct Reader {
    bits: usize,
    data: [u8; BUFFER_SIZE],
    gpio_d0: i32,
    gpio_d1: i32,
    timer: Option<esp_timer_handle_t>,
    reader_tx: Sender<Packet>,
    reader_rx: Receiver<Packet>,
}

impl Reader {
    pub fn new(gpio_d0: i32, gpio_d1: i32) -> Self {
        let (reader_tx, reader_rx) = mpsc::channel();
        Reader {
            gpio_d0,
            gpio_d1,
            data: [0; BUFFER_SIZE],
            bits: 0,
            timer: None,
            reader_tx,
            reader_rx,
        }
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        let reader_ptr = self as *mut _ as *mut c_void;

        let timer_config = esp_timer_create_args_t {
            name: CString::new("wiegand")?.into_raw(),
            arg: reader_ptr,
            callback: Some(timer_interrupt),
            dispatch_method: esp_timer_dispatch_t_ESP_TIMER_TASK,
            skip_unhandled_events: true,
        };

        let mut timer_handle = ptr::null_mut();
        esp!(unsafe { esp_timer_create(&timer_config, &mut timer_handle) })?;
        self.timer = Some(timer_handle);

        // Configures the button
        let io_conf = gpio_config_t {
            pin_bit_mask: (1 << self.gpio_d0 | 1 << self.gpio_d1),
            mode: gpio_mode_t_GPIO_MODE_INPUT,
            pull_up_en: true.into(),
            pull_down_en: false.into(),
            intr_type: gpio_int_type_t_GPIO_INTR_NEGEDGE,
        };

        unsafe {
            // Writes the button configuration to the registers
            esp!(gpio_config(&io_conf))?;

            // Registers our function with the generic GPIO interrupt handler we installed earlier.
            esp!(gpio_isr_handler_add(
                self.gpio_d0,
                Some(wiegand_interrupt),
                reader_ptr
            ))?;
            esp!(gpio_isr_handler_add(
                self.gpio_d1,
                Some(wiegand_interrupt),
                reader_ptr
            ))?;
        }

        Ok(())
    }

    fn stop(&mut self) {
        if let Some(timer) = self.timer {
            unsafe { esp_timer_stop(timer) };
        }
        unsafe {
            gpio_set_intr_type(self.gpio_d0, gpio_int_type_t_GPIO_INTR_DISABLE);
            gpio_set_intr_type(self.gpio_d1, gpio_int_type_t_GPIO_INTR_DISABLE);
        }
    }

    fn reset(&mut self) {
        unsafe {
            gpio_set_intr_type(self.gpio_d0, gpio_int_type_t_GPIO_INTR_NEGEDGE);
            gpio_set_intr_type(self.gpio_d1, gpio_int_type_t_GPIO_INTR_NEGEDGE);
        }
        self.data = [0; BUFFER_SIZE];
        self.bits = 0;
    }
}

impl Iterator for Reader {
    type Item = Result<Packet, RecvTimeoutError>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.reader_rx.recv_timeout(PIN_TIMEOUT))
    }
}

impl Drop for Reader {
    fn drop(&mut self) {
        unsafe {
            if let Some(timer) = self.timer {
                esp_timer_stop(timer);
                esp_timer_delete(timer);
            }
            gpio_isr_handler_remove(self.gpio_d0);
            gpio_reset_pin(self.gpio_d0);

            gpio_isr_handler_remove(self.gpio_d1);
            gpio_reset_pin(self.gpio_d1);
        }
    }
}
