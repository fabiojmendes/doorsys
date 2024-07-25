use std::time::Duration;

use bincode::config::Configuration;
use doorsys_protocol::Audit;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use sqlx::PgPool;
use tokio::{task, time};

use crate::domain::entry_log::EntryLogRepository;

pub const BINCODE_CONFIG: Configuration = bincode::config::standard();

pub async fn start(pool: PgPool, mqtt_url: &str) -> anyhow::Result<AsyncClient> {
    let mqtt_opts = MqttOptions::parse_url(mqtt_url)?;

    let (client, mut connection) = AsyncClient::new(mqtt_opts, 10);
    let cloned_client = client.clone();

    task::spawn(async move {
        let entry_repo = EntryLogRepository { pool };

        loop {
            match connection.poll().await {
                Ok(Event::Incoming(Packet::Publish(p))) => {
                    tracing::info!(
                        "topic: {}, qos: {:?}, size: {:?}",
                        p.topic,
                        p.qos,
                        p.payload.len()
                    );
                    match bincode::decode_from_slice::<Audit, _>(&p.payload, BINCODE_CONFIG) {
                        Ok((audit, len)) => {
                            let net_id = p.topic.split('/').nth(2);
                            tracing::info!(
                                "Audit({}) [{:?}]: {:?}",
                                len,
                                net_id.unwrap_or(""),
                                audit
                            );
                            match entry_repo
                                .create_with_code(
                                    audit.code,
                                    &audit.code_type.to_string(),
                                    net_id,
                                    audit.success,
                                    &audit.timestamp.into(),
                                )
                                .await
                            {
                                Ok(log) => {
                                    tracing::info!("Log created {:?}", log);
                                }
                                Err(sqlx::Error::Database(e)) => {
                                    if let Some(c) = e.constraint() {
                                        tracing::warn!("Duplicated entry log, skpping... {}", c);
                                    } else {
                                        tracing::error!("Database error creating entry log {}", e);
                                    }
                                }
                                Err(e) => {
                                    tracing::error!("Error creating entry log {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            tracing::error!("Error decoding message: {}", e);
                        }
                    }
                }
                Ok(Event::Incoming(Packet::ConnAck(_))) => {
                    tracing::info!("Connected to mqtt broker and subscribing to topics");
                    if let Err(e) = client.subscribe("doorsys/audit/+", QoS::AtLeastOnce).await {
                        tracing::error!("Error subscribing to topic {}", e);
                    }
                    if let Err(e) = client.subscribe("doorsys/audit", QoS::AtLeastOnce).await {
                        tracing::error!("Error subscribing to topic {}", e);
                    }
                }
                Err(rumqttc::ConnectionError::Io(e)) => {
                    tracing::error!("Connection refused {:?}", e);
                    time::sleep(Duration::from_secs(5)).await;
                }
                notification => {
                    tracing::trace!("Notification = {:?}", &notification);
                }
            }
        }
    });

    Ok(cloned_client)
}
