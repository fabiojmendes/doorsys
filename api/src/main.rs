use std::env;

mod domain;
mod http;
mod mqtt;

use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    tracing::info!("Starting Server");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    let mqtt_client = mqtt::start(pool.clone()).await?;

    http::serve(pool, mqtt_client).await
}
