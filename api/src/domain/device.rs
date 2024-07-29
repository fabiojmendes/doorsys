use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub id: i64,
    pub name: String,
    pub net_id: String,
}

#[derive(Clone)]
pub struct DeviceRepository {
    pub pool: PgPool,
}

impl DeviceRepository {
    pub async fn fetch_all(&self) -> Result<Vec<Device>, sqlx::Error> {
        sqlx::query_as!(Device, r#"select * from device order by name"#)
            .fetch_all(&self.pool)
            .await
    }
}
