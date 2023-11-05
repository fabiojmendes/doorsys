use doorsys_protocol::UserAction;
use esp_idf_svc::mqtt::client::{Details, Event, QoS};
use esp_idf_svc::mqtt::client::{EspMqttClient, EspMqttMessage, MqttClientConfiguration};
use std::sync::mpsc::Sender;

use crate::user::UserDB;

const MQTT_URL: &str = env!("MQTT_URL");
const MQTT_USER: &str = env!("MQTT_USER");
const MQTT_PASS: &str = env!("MQTT_PASS");

const BINCODE_CONFIG: bincode::config::Configuration = bincode::config::standard();

pub fn setup_mqtt(user_db: UserDB, door_tx: Sender<()>) -> anyhow::Result<EspMqttClient<'static>> {
    let mqtt_config = MqttClientConfiguration {
        client_id: Some("doorsys"),
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
    match msg.topic() {
        Some("doorsys/user") => process_user_message(msg, user_db),
        Some("doorsys/open") => proccess_open_message(msg, door_tx),
        Some(topic) => log::warn!("unknown topic {}", topic),
        None => log::error!("no topic provided"),
    }
}

fn proccess_open_message(msg: &EspMqttMessage, door_tx: &Sender<()>) {
    log::info!("open door from mqtt {:?}", msg);
    if let Err(e) = door_tx.send(()) {
        log::error!("error sending door message {}", e);
    }
}

fn process_user_message(msg: &EspMqttMessage, user_db: UserDB) {
    if msg.details() != &Details::Complete {
        log::error!("incomplete message, dropping {:?}", msg);
        return;
    }

    log::info!("msg: {:?}, data: {:?}", msg, msg.data());
    match bincode::decode_from_slice(msg.data(), BINCODE_CONFIG) {
        Ok((UserAction::Add(code), _)) => {
            if let Err(e) = user_db.add(code) {
                log::error!("Error adding new code {}", e);
            }
        }
        Ok((UserAction::Del(code), _)) => {
            if let Err(e) = user_db.delete(code) {
                log::error!("Error deleting code {}", e);
            }
        }
        Ok((UserAction::Replace { old, new }, _)) => {
            if let Err(e) = user_db.replace(old, new) {
                log::error!("Error replacing code {}", e);
            }
        }
        Ok((UserAction::Bulk(codes), _)) => {
            if let Err(e) = user_db.bulk(codes) {
                log::error!("Error bulk inserting codes {}", e);
            }
        }
        Err(e) => {
            log::error!("decoding error: {}", e);
        }
    };
}
