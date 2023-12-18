use std::{ffi::c_void, ptr};

use esp_idf_svc::sys::{
    esp, gpio_config, gpio_config_t, gpio_get_level, gpio_int_type_t_GPIO_INTR_NEGEDGE,
    gpio_isr_handler_add, gpio_mode_t_GPIO_MODE_INPUT, vQueueDelete, xQueueGenericCreate,
    xQueueGiveFromISR, xQueueReceive, QueueDefinition,
};

#[link_section = ".iram0.text"]
unsafe extern "C" fn button_interrupt(arg: *mut c_void) {
    let button = &mut *(arg as *mut Button);
    if gpio_get_level(button.gpio) == 0 {
        xQueueGiveFromISR(button.queue, ptr::null_mut());
    }
}

pub struct Button {
    gpio: i32,
    queue: *mut QueueDefinition,
}

impl Button {
    pub fn new(gpio: i32) -> Button {
        let queue = unsafe { xQueueGenericCreate(1, 0, 0) };
        Button { gpio, queue }
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        let io_conf = gpio_config_t {
            pin_bit_mask: (1 << self.gpio),
            mode: gpio_mode_t_GPIO_MODE_INPUT,
            pull_up_en: false.into(),
            pull_down_en: false.into(),
            intr_type: gpio_int_type_t_GPIO_INTR_NEGEDGE,
        };

        unsafe {
            // Writes the button configuration to the registers
            esp!(gpio_config(&io_conf))?;

            // Registers our function with the generic GPIO interrupt handler we installed earlier.
            esp!(gpio_isr_handler_add(
                self.gpio,
                Some(button_interrupt),
                self as *mut _ as *mut c_void,
            ))?;
        }

        Ok(())
    }

    pub fn wait_for_press(&self) -> bool {
        let res = unsafe { xQueueReceive(self.queue, ptr::null_mut(), 10000) };
        res == 1
    }
}

impl Drop for Button {
    fn drop(&mut self) {
        unsafe { vQueueDelete(self.queue) };
    }
}
