use std::env;

mod domain;
mod http;
mod mqtt;

use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let format = tracing_subscriber::fmt::format().without_time().compact();
    tracing_subscriber::fmt().event_format(format).init();

    tracing::info!("Starting Server");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    tracing::info!("Connected to database, executing migrations");
    sqlx::migrate!().run(&pool).await?;

    let mqtt_url = env::var("MQTT_URL")?;
    let mqtt_client = mqtt::start(pool.clone(), &mqtt_url).await?;

    http::serve(pool, mqtt_client).await
}
