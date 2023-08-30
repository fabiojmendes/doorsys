// Reference: https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/system/freertos.html

mod buttons;
mod door;
mod mqtt;
mod network;
mod wiegand;

use doorsys_protocol::Audit;
use embedded_svc::mqtt::client::QoS;
use esp_idf_hal as _;
use esp_idf_hal::gpio::OutputPin;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::mqtt::client::EspMqttClient;
use esp_idf_svc::nvs::{EspDefaultNvsPartition, EspNvs, NvsDefault};
use esp_idf_svc::systime::EspSystemTime;
use esp_idf_sys::{
    esp, gpio_install_isr_service, heap_caps_get_free_size, heap_caps_get_largest_free_block,
    heap_caps_get_minimum_free_size, heap_caps_get_total_size, ESP_INTR_FLAG_IRAM,
    MALLOC_CAP_DEFAULT,
};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
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

fn setup_reader(
    door_tx: Sender<()>,
    nvs: Arc<Mutex<EspNvs<NvsDefault>>>,
    audit_tx: Sender<Audit>,
) -> anyhow::Result<()> {
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
                        let mut audit = Audit {
                            code: pin.clone(),
                            timestamp: SystemTime::now(),
                            success: false,
                        };
                        match contains {
                            Ok(Some(_)) => {
                                door_tx.send(()).unwrap();
                                audit.success = true;
                            }
                            Ok(None) => log::warn!("invalid pin: {}", &pin),
                            Err(e) => log::error!("error reading nvs {}", e),
                        }
                        if let Err(e) = audit_tx.send(audit) {
                            log::error!("error sending audit record: {}", e);
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
                    let mut audit = Audit {
                        code: rfid.to_string(),
                        timestamp: SystemTime::now(),
                        success: false,
                    };

                    let rfid_str = rfid.to_string();
                    let contains = { nvs.lock().unwrap().get_u8(&rfid_str) };
                    match contains {
                        Ok(Some(_)) => {
                            door_tx.send(()).unwrap();
                            audit.success = true;
                        }
                        Ok(None) => log::warn!("invalid card id: {}", &rfid_str),
                        Err(e) => log::error!("error reading nvs {}", e),
                    }
                    if let Err(e) = audit_tx.send(audit) {
                        log::error!("error sending audit record: {}", e);
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

fn setup_audit_publiher(mqtt_client: Arc<Mutex<EspMqttClient>>, audit_rx: Receiver<Audit>) {
    thread::spawn(move || {
        let config = bincode::config::standard();

        for audit in audit_rx {
            match bincode::encode_to_vec(audit, config) {
                Ok(buffer) => {
                    if let Err(e) = mqtt_client.lock().unwrap().enqueue(
                        "doorsys/audit",
                        QoS::AtLeastOnce,
                        false,
                        &buffer,
                    ) {
                        log::error!("error sending audit: {}", e);
                    }
                }
                Err(e) => {
                    log::error!("error encoding audit: {}", e);
                }
            }
        }
    });
}

fn health_check(mqtt_client: Arc<Mutex<EspMqttClient>>) -> anyhow::Result<()> {
    let systime = EspSystemTime {};

    thread::spawn(move || loop {
        let time = systime.now().as_nanos();
        let heap = unsafe {
            let total = heap_caps_get_total_size(MALLOC_CAP_DEFAULT);
            let free = heap_caps_get_free_size(MALLOC_CAP_DEFAULT);
            let minimum = heap_caps_get_minimum_free_size(MALLOC_CAP_DEFAULT);
            let largest_free = heap_caps_get_largest_free_block(MALLOC_CAP_DEFAULT);
            format!("heap,host=doorsys free={free},total={total},minimum={minimum},largest_free={largest_free} {time}")
        };
        log::info!("{}", heap);
        if let Err(e) = mqtt_client.lock().unwrap().enqueue(
            "doorsys/status",
            QoS::AtMostOnce,
            false,
            heap.as_bytes(),
        ) {
            log::warn!("mqtt enqueue error: {}", e);
        }

        thread::sleep(Duration::from_secs(60));
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

    let (audit_tx, audit_rx) = mpsc::channel();
    setup_reader(door_tx.clone(), doorsys_nvs.clone(), audit_tx)?;

    network::setup_wireless(peripherals.modem, sysloop.clone(), nvs_part.clone())?;

    let mqtt_client = Arc::new(Mutex::new(mqtt::setup_mqtt(
        doorsys_nvs.clone(),
        door_tx.clone(),
    )?));

    setup_audit_publiher(mqtt_client.clone(), audit_rx);

    health_check(mqtt_client.clone())?;

    log::info!("Application fully functional");

    Ok(())
}
