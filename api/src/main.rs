mod domain;
mod http;
mod logging;
mod mqtt;

use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logging::init();
    tracing::info!("Starting Server");

    let pool = PgPoolOptions::new()
        .min_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    tracing::info!("Connected to database, executing migrations");
    sqlx::migrate!().run(&pool).await?;

    let mqtt_url = env::var("MQTT_URL")?;
    let mqtt_client = mqtt::start(pool.clone(), &mqtt_url).await?;

    http::serve(pool, mqtt_client).await
}
