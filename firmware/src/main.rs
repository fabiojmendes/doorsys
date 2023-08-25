// Reference: https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/system/freertos.html

mod buttons;
mod door;
mod network;
mod wiegand;

use embedded_svc::mqtt::client::QoS;
use esp_idf_hal as _;
use esp_idf_hal::gpio::OutputPin;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::mqtt::client::EspMqttClient;
use esp_idf_svc::nvs::{EspDefaultNvsPartition, EspNvs, NvsDefault};
use esp_idf_svc::systime::EspSystemTime;
use esp_idf_sys::{esp, esp_get_free_heap_size, gpio_install_isr_service, ESP_INTR_FLAG_IRAM};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};
use wiegand::Packet;

use crate::buttons::Button;
use crate::wiegand::Reader;

const MAX_PIN_LENGTH: usize = 8;
const HASH_KEY: u8 = 0x0B;
const GPIO_D0: i32 = 7;
const GPIO_D1: i32 = 8;

const GPIO_BUTTON: i32 = 9;

fn setup_button(door_tx: Sender<()>) {
    thread::spawn(move || {
        let button = Button::new(GPIO_BUTTON).unwrap();

        loop {
            if button.wait_for_press() {
                log::info!("button press");
                door_tx.send(()).unwrap();
            }
        }
    });
}

fn setup_door(pin: impl OutputPin, door_rx: Receiver<()>) -> anyhow::Result<()> {
    let mut door = door::Door::new(pin)?;

    thread::spawn(move || loop {
        door_rx.recv().unwrap();
        if let Err(e) = door.open() {
            log::error!("error: {}", e);
        }
        // Drain the queue while the door is open
        while door_rx.recv_timeout(Duration::from_secs(2)).is_ok() {}
        if let Err(e) = door.close() {
            log::error!("error: {}", e);
        }
    });
    Ok(())
}

fn setup_reader(door_tx: Sender<()>, nvs: Arc<Mutex<EspNvs<NvsDefault>>>) -> anyhow::Result<()> {
    thread::spawn(move || {
        let mut reader = Reader::new(GPIO_D0, GPIO_D1);
        reader.start().unwrap();

        let mut keys = Vec::with_capacity(MAX_PIN_LENGTH);

        // Reads the queue in a loop.
        for packet in reader {
            match packet {
                Ok(Packet::Key { key }) => {
                    if key == HASH_KEY {
                        log::info!("open door {:?}", keys);
                        let pin: String = keys.iter().cloned().map(|i: u8| i.to_string()).collect();
                        let contains = { nvs.lock().unwrap().get_u8(&pin) };
                        log::info!("contains pin: {:?}", contains);
                        match contains {
                            Ok(Some(_)) => door_tx.send(()).unwrap(),
                            Ok(None) => log::warn!("invalid pin: {}", &pin),
                            Err(e) => log::error!("error reading nvs {}", e),
                        }
                        keys.clear();
                    } else if keys.len() == MAX_PIN_LENGTH {
                        log::warn!("pin sequence is too big {:?}", keys);
                        keys.clear();
                    } else {
                        keys.push(key);
                    }
                }
                Ok(Packet::Card { rfid }) => {
                    log::info!("RFID: {}", rfid);

                    let rfid_str = rfid.to_string();
                    let contains = { nvs.lock().unwrap().get_u8(&rfid_str) };
                    match contains {
                        Ok(Some(_)) => door_tx.send(()).unwrap(),
                        Ok(None) => log::warn!("invalid card id: {}", &rfid_str),
                        Err(e) => log::error!("error reading nvs {}", e),
                    }
                    keys.clear();
                }
                Ok(Packet::Unknown { bits, data }) => {
                    log::warn!("pattern not recognized bits: {}, data: {:02X?}", bits, data);
                }
                Err(_e) => {
                    if !keys.is_empty() {
                        log::warn!("incomplete pin sequence {:?}", keys);
                        keys.clear();
                    }
                }
            }
        }
    });

    Ok(())
}

fn health_check(mut mqtt_client: EspMqttClient) -> anyhow::Result<()> {
    let systime = EspSystemTime {};

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(60));
        let heap_size = unsafe { esp_get_free_heap_size() };
        let status = format!("doorsys heap={heap_size} {}", systime.now().as_millis());
        log::info!("{}", status);
        if let Err(e) =
            mqtt_client.enqueue("doorsys/status", QoS::AtMostOnce, false, status.as_bytes())
        {
            log::warn!("mqtt publish error: {}", e);
        }
    });

    Ok(())
}

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // Installs the generic GPIO interrupt handler
    esp!(unsafe { gpio_install_isr_service(ESP_INTR_FLAG_IRAM as i32) })?;

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;
    let nvs_part = EspDefaultNvsPartition::take()?;

    let doorsys_nvs = Arc::new(Mutex::new(EspNvs::new(nvs_part.clone(), "doorsys", true)?));

    log::info!("Starting application");

    let (door_tx, door_rx) = mpsc::channel();
    setup_door(peripherals.pins.gpio10, door_rx)?;

    setup_button(door_tx.clone());

    setup_reader(door_tx.clone(), doorsys_nvs.clone())?;

    network::setup_wireless(peripherals.modem, sysloop.clone(), nvs_part.clone())?;

    let mqtt_client = network::setup_mqtt(doorsys_nvs.clone(), door_tx.clone())?;

    health_check(mqtt_client)?;

    log::info!("Application fully functional");

    Ok(())
}
