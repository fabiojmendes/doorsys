use std::time::Duration;

use bincode::config::Configuration;
use chrono::{DateTime, Utc};
use doorsys_protocol::Audit;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use sqlx::PgPool;
use tokio::task;

use crate::domain::entry_log::EntryLogRepository;

pub const BINCODE_CONFIG: Configuration = bincode::config::standard();

pub async fn start(pool: PgPool) -> anyhow::Result<AsyncClient> {
    let mut mqtt_opts = MqttOptions::new("doorsys-api", "rpi.home", 1883);
    mqtt_opts
        .set_keep_alive(Duration::from_secs(5))
        .set_credentials("esp", "aurora")
        .set_clean_session(false);

    let (client, mut connection) = AsyncClient::new(mqtt_opts, 10);
    client.subscribe("doorsys/audit", QoS::AtLeastOnce).await?;

    task::spawn(async move {
        let entry_repo = EntryLogRepository { pool };

        loop {
            let notification = connection.poll().await;
            tracing::trace!("Notification = {:?}", &notification);
            if let Ok(Event::Incoming(Packet::Publish(p))) = notification {
                tracing::info!(
                    "topic: {}, qos: {:?}, data: {:?}",
                    p.topic,
                    p.qos,
                    p.payload
                );
                if let Ok((audit, len)) =
                    bincode::decode_from_slice::<Audit, _>(&p.payload, BINCODE_CONFIG)
                {
                    tracing::info!("Audit({}): {:?}", len, audit);
                    let code_type = audit.code_type.to_string();
                    let date: DateTime<Utc> = audit.timestamp.into();
                    match entry_repo
                        .create_with_code(&audit.code, &code_type, audit.success, &date)
                        .await
                    {
                        Ok(log) => {
                            tracing::info!("Log created {:?}", log);
                        }
                        Err(e) => {
                            tracing::error!("Error creating entry log {}", e);
                        }
                    }
                }
            }
        }
    });

    Ok(client)
}
