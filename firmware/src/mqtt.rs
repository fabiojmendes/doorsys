use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use doorsys_protocol::UserAction;
use esp_idf_svc::mqtt::client::{
    Details, EspMqttClient, EventPayload, MqttClientConfiguration, QoS,
};

use crate::user::UserDB;

const MQTT_URL: &str = env!("MQTT_URL");
const MQTT_USER: &str = env!("MQTT_USER");
const MQTT_PASS: &str = env!("MQTT_PASS");

const BINCODE_CONFIG: bincode::config::Configuration = bincode::config::standard();

static mut SHARED_BUF: Vec<u8> = Vec::new();
static mut SHARED_TOPIC: String = String::new();

pub type MqttClient = EspMqttClient<'static>;

pub fn setup_mqtt(net_id: &str, user_db: UserDB) -> anyhow::Result<Arc<Mutex<MqttClient>>> {
    let mqtt_config = MqttClientConfiguration {
        client_id: Some(net_id),
        username: Some(MQTT_USER),
        password: Some(MQTT_PASS),
        disable_clean_session: true,
        ..Default::default()
    };

    let (sender, receiver) = mpsc::channel();

    let client =
        EspMqttClient::new_cb(MQTT_URL, &mqtt_config, move |event| match event.payload() {
            EventPayload::Received {
                id: _,
                topic,
                data,
                details,
            } => route_message(topic, data, details, &user_db),
            EventPayload::Connected(session) => {
                log::info!("Connected session = {session}");
                if !session {
                    sender.send(()).unwrap();
                }
            }
            EventPayload::Error(e) => log::error!("from mqtt: {:?}", e),
            event => log::info!("mqtt event: {:?}", event),
        })?;
    let client = Arc::new(Mutex::new(client));
    let client_clone = client.clone();

    thread::spawn(move || {
        while let Ok(_) = receiver.recv() {
            let topic = "doorsys/user";
            match client.lock().unwrap().subscribe(topic, QoS::AtLeastOnce) {
                Ok(id) => log::info!("Subscribed to {topic} {id}"),
                Err(e) => log::error!("Failed to subscribe to topic {topic}: {e}"),
            };
        }
    });

    Ok(client_clone)
}

fn route_message(topic: Option<&str>, data: &[u8], details: Details, user_db: &UserDB) {
    log::info!(
        "Message received {:?} {:?}, {} bytes",
        topic,
        details,
        data.len()
    );
    let (topic, data) = match details {
        Details::InitialChunk(init) => unsafe {
            SHARED_BUF = Vec::with_capacity(init.total_data_size);
            SHARED_BUF.extend_from_slice(data);
            SHARED_TOPIC = String::from(topic.unwrap());
            return;
        },
        Details::SubsequentChunk(_sub) => unsafe {
            SHARED_BUF.extend_from_slice(data);
            if SHARED_BUF.len() != SHARED_BUF.capacity() {
                return;
            }
            (&*SHARED_TOPIC, &*SHARED_BUF)
        },
        Details::Complete => (topic.unwrap(), data),
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
