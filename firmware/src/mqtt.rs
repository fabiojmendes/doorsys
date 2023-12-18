use doorsys_protocol::UserAction;
use esp_idf_svc::mqtt::client::{Details, Event, QoS};
use esp_idf_svc::mqtt::client::{EspMqttClient, EspMqttMessage, MqttClientConfiguration};
use std::sync::mpsc::Sender;

use crate::user::UserDB;

const MQTT_URL: &str = env!("MQTT_URL");
const MQTT_USER: &str = env!("MQTT_USER");
const MQTT_PASS: &str = env!("MQTT_PASS");

const BINCODE_CONFIG: bincode::config::Configuration = bincode::config::standard();

static mut SHARED_BUF: Vec<u8> = Vec::new();
static mut SHARED_TOPIC: String = String::new();

pub fn setup_mqtt(user_db: UserDB, door_tx: Sender<()>) -> anyhow::Result<EspMqttClient<'static>> {
    let mqtt_config = MqttClientConfiguration {
        client_id: Some("doorsys-v2"),
        username: Some(MQTT_USER),
        password: Some(MQTT_PASS),
        disable_clean_session: true,
        ..Default::default()
    };

    let mut client = EspMqttClient::new(MQTT_URL, &mqtt_config, move |res| match res {
        Ok(Event::Received(msg)) => route_message(msg, user_db.clone(), &door_tx),
        Ok(event) => log::info!("mqtt event: {:?}", event),
        Err(e) => log::warn!("from mqtt: {:?} {:?}", res, e),
    })?;

    client.subscribe("doorsys/user", QoS::AtLeastOnce)?;
    client.subscribe("doorsys/open", QoS::AtMostOnce)?;

    Ok(client)
}

fn route_message(msg: &EspMqttMessage, user_db: UserDB, door_tx: &Sender<()>) {
    log::info!(
        "Message received {:?} {:?}, {} bytes",
        msg.topic(),
        msg.details(),
        msg.data().len()
    );
    let (topic, data) = match msg.details() {
        Details::InitialChunk(init) => unsafe {
            SHARED_BUF = Vec::with_capacity(init.total_data_size);
            SHARED_BUF.extend_from_slice(msg.data());
            SHARED_TOPIC = String::from(msg.topic().unwrap());
            return;
        },
        Details::SubsequentChunk(_sub) => unsafe {
            SHARED_BUF.extend_from_slice(msg.data());
            if SHARED_BUF.len() != SHARED_BUF.capacity() {
                return;
            }
            (&*SHARED_TOPIC, &*SHARED_BUF)
        },
        Details::Complete => (msg.topic().unwrap(), msg.data()),
    };
    match topic {
        "doorsys/user" => process_user_message(data, user_db),
        "doorsys/open" => proccess_open_message(door_tx),
        _ => log::warn!("unknown topic {}", topic),
    };
}

fn proccess_open_message(door_tx: &Sender<()>) {
    log::info!("Open door from mqtt");
    if let Err(e) = door_tx.send(()) {
        log::error!("error sending door message {}", e);
    }
}

fn process_user_message(data: &[u8], user_db: UserDB) {
    match bincode::decode_from_slice(data, BINCODE_CONFIG) {
        Ok((UserAction::Add(code), _)) => {
            log::info!("Adding code {}", code);
            if let Err(e) = user_db.add(code) {
                log::error!("Error adding new code {}", e);
            }
        }
        Ok((UserAction::Del(code), _)) => {
            log::info!("Deleting code {}", code);
            if let Err(e) = user_db.delete(code) {
                log::error!("Error deleting code {}", e);
            }
        }
        Ok((UserAction::Replace { old, new }, _)) => {
            log::info!("Replacing code {} with {}", old, new);
            if let Err(e) = user_db.replace(old, new) {
                log::error!("Error replacing code {}", e);
            }
        }
        Ok((UserAction::Bulk(codes), _)) => {
            log::info!("Bulk adding codes {}", codes.len());
            if let Err(e) = user_db.bulk(codes) {
                log::error!("Error bulk inserting codes {}", e);
            }
        }
        Err(e) => {
            log::error!("decoding error: {}", e);
        }
    };
}
