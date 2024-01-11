use std::sync::{Arc, Mutex};
use std::thread;

use doorsys_protocol::UserAction;
use esp_idf_svc::mqtt::client::{ConnState, Details, Event, Message, MessageImpl, QoS};
use esp_idf_svc::mqtt::client::{EspMqttClient, MqttClientConfiguration};
use esp_idf_svc::sys::EspError;

use crate::user::UserDB;

const MQTT_URL: &str = env!("MQTT_URL");
const MQTT_USER: &str = env!("MQTT_USER");
const MQTT_PASS: &str = env!("MQTT_PASS");

const BINCODE_CONFIG: bincode::config::Configuration = bincode::config::standard();

static mut SHARED_BUF: Vec<u8> = Vec::new();
static mut SHARED_TOPIC: String = String::new();

pub type MqttClient = EspMqttClient<'static, ConnState<MessageImpl, EspError>>;

pub fn setup_mqtt(user_db: UserDB) -> anyhow::Result<Arc<Mutex<MqttClient>>> {
    let mqtt_config = MqttClientConfiguration {
        client_id: Some("doorsys-v2"),
        username: Some(MQTT_USER),
        password: Some(MQTT_PASS),
        disable_clean_session: true,
        ..Default::default()
    };

    let (client, mut conn) = EspMqttClient::new_with_conn(MQTT_URL, &mqtt_config)?;
    let client = Arc::new(Mutex::new(client));
    let client_clone = client.clone();

    thread::spawn(move || {
        while let Some(res) = conn.next() {
            match res {
                Ok(Event::Received(msg)) => route_message(msg, &user_db),
                Ok(Event::Connected(_)) => {
                    log::info!("mqtt event: Connected");
                    if let Err(e) = client
                        .lock()
                        .unwrap()
                        .subscribe("doorsys/user", QoS::AtLeastOnce)
                    {
                        log::error!("error subscribing: {:?}", e);
                    }
                }
                Ok(event) => log::info!("mqtt event: {:?}", event),
                Err(e) => log::warn!("from mqtt: {:?} {:?}", res, e),
            }
        }
    });

    Ok(client_clone)
}

fn route_message(msg: impl Message, user_db: &UserDB) {
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
        _ => log::warn!("unknown topic {}", topic),
    };
}

fn process_user_message(data: &[u8], user_db: &UserDB) {
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
