use std::{
    thread,
    time::{Duration, SystemTime},
};

use bincode::config::Configuration;
use chrono::{DateTime, Utc};
use doorsys_protocol::{Audit, CodeType, UserAction};
use rumqttc::{Client, Event, MqttOptions, Packet, QoS};

const CONFIG: Configuration = bincode::config::standard();

fn main() {
    println!("Hello, world!");

    let mut mqtt_opts = MqttOptions::new("doorsys-cli", "rpi.home", 1883);
    mqtt_opts
        .set_keep_alive(Duration::from_secs(5))
        .set_credentials("esp", "aurora")
        .set_clean_session(false);

    let (mut client, mut connection) = Client::new(mqtt_opts, 10);

    thread::spawn(move || {
        client
            .publish("doorsys/open", QoS::AtMostOnce, false, vec![])
            .unwrap();

        let user_add = UserAction::Del(String::from("1234"));
        if let Ok(payload) = bincode::encode_to_vec(user_add, CONFIG) {
            client
                .publish("doorsys/user", QoS::AtMostOnce, false, payload)
                .unwrap();
        }

        let audit = Audit {
            timestamp: SystemTime::now(),
            code: "12983".to_owned(),
            code_type: CodeType::Pin,
            success: false,
        };
        if let Ok(payload) = bincode::encode_to_vec(audit, CONFIG) {
            client
                .publish("doorsys/audit", QoS::AtLeastOnce, false, payload)
                .unwrap();
        }
    });

    // Iterate to poll the eventloop for connection progress
    for (i, notification) in connection.iter().enumerate() {
        println!("[{i}] Notification = {:?}", &notification);
        if let Ok(Event::Incoming(Packet::Publish(p))) = notification {
            println!(
                "topic: {}, qos: {:?}, data: {:?}",
                p.topic, p.qos, p.payload
            );
            if let Ok((audit, count)) = bincode::decode_from_slice::<Audit, _>(&p.payload, CONFIG) {
                println!("Audit: {:?} {}", audit, count);
                println!("systime: {:?}", &audit.timestamp);
                let date: DateTime<Utc> = audit.timestamp.into();
                println!("datetime: {}", date);
            }
        }
    }
}
