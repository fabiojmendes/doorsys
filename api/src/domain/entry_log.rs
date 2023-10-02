use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryLog {
    pub id: i64,
    pub code: String,
    pub created: DateTime<Utc>,
}

#[derive(Clone)]
pub struct EntryLogRepository {
    pub pool: PgPool,
}

impl EntryLogRepository {
    pub async fn create(&self, code: &str) -> Result<EntryLog, sqlx::Error> {
        sqlx::query_as!(
            EntryLog,
            "insert into entry_log (code) values ($1) returning *",
            code
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn fetch_all(&self) -> Result<Vec<EntryLog>, sqlx::Error> {
        sqlx::query_as!(EntryLog, "select * from entry_log order by created desc",)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn fetch_all_by_code(&self, code: &str) -> Result<Vec<EntryLog>, sqlx::Error> {
        sqlx::query_as!(EntryLog, "select * from entry_log where code = $1", code)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn fetch_all_by_customer(
        &self,
        customer_id: i64,
    ) -> Result<Vec<EntryLog>, sqlx::Error> {
        sqlx::query_as!(
            EntryLog,
            "select e.* from entry_log e join code c on e.code = c.code where c.customer_id = $1",
            customer_id
        )
        .fetch_all(&self.pool)
        .await
    }
}
