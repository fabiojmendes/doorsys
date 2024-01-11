use std::{
    env,
    error::Error,
    thread,
    time::{Duration, SystemTime},
};

use bincode::config::Configuration;
use chrono::{DateTime, Utc};
use doorsys_protocol::{Audit, CodeType};
use rumqttc::{Client, Event, MqttOptions, Packet, QoS};

const CONFIG: Configuration = bincode::config::standard();

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting doorsys-cli");

    let mqtt_url = env::var("MQTT_URL")?;
    let mqtt_opts = MqttOptions::parse_url(mqtt_url)?;

    let (mut client, mut connection) = Client::new(mqtt_opts, 10);

    let audit = Audit {
        timestamp: SystemTime::now(),
        code_type: CodeType::Pin,
        code: 1234,
        success: true,
    };
    let payload = bincode::encode_to_vec(audit, CONFIG).unwrap();
    client
        .publish("doorsys/audit", QoS::AtLeastOnce, false, &*payload)
        .unwrap();
    client
        .publish("doorsys/audit", QoS::AtLeastOnce, false, &*payload)
        .unwrap();
    client
        .publish("doorsys/audit", QoS::AtLeastOnce, false, &*payload)
        .unwrap();
    // Iterate to poll the eventloop for connection progress
    for (i, notification) in connection.iter().enumerate() {
        match notification {
            Ok(Event::Incoming(Packet::Publish(p))) => {
                println!(
                    "topic: {}, qos: {:?}, data: {:?}",
                    p.topic, p.qos, p.payload
                );
                if let Ok((audit, count)) =
                    bincode::decode_from_slice::<Audit, _>(&p.payload, CONFIG)
                {
                    println!("Audit: {:?} {}", audit, count);
                    println!("systime: {:?}", &audit.timestamp);
                    let date: DateTime<Utc> = audit.timestamp.into();
                    println!("datetime: {}", date);
                }
            }
            Err(rumqttc::ConnectionError::Io(e)) => {
                println!("Connection refused {:?}", e);
                thread::sleep(Duration::from_secs(5));
            }
            _ => {
                println!("[{i}] Notification = {:?}", &notification);
            }
        }
    }

    Ok(())
}
