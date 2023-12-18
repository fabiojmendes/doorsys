use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryLog {
    pub id: i64,
    pub staff_id: Option<i64>,
    pub code: i32,
    pub code_type: String,
    pub success: bool,
    pub event_date: DateTime<Utc>,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryLogDisplay {
    pub id: i64,
    pub staff_id: Option<i64>,
    pub staff_name: Option<String>,
    pub customer_id: Option<i64>,
    pub customer_name: Option<String>,
    pub code: i32,
    pub code_type: String,
    pub success: bool,
    pub event_date: DateTime<Utc>,
}

#[derive(Clone)]
pub struct EntryLogRepository {
    pub pool: PgPool,
}

impl EntryLogRepository {
    pub async fn create_with_code(
        &self,
        code: i32,
        code_type: &str,
        success: bool,
        event_date: &DateTime<Utc>,
    ) -> Result<EntryLog, sqlx::Error> {
        sqlx::query_as!(
            EntryLog,
            r#"
            with temp(code) as (values($1::int))
            insert into entry_log (staff_id, code, code_type, success, event_date) 
                select s.id, t.code, $2, $3, $4
                from temp t
                left join staff s on s.pin = t.code or s.fob = t.code
            returning *
            "#,
            code,
            code_type,
            success,
            event_date
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn fetch_all(&self) -> Result<Vec<EntryLogDisplay>, sqlx::Error> {
        sqlx::query_as!(
            EntryLogDisplay,
            r#"
            select 
                e.id, 
                s.id as "staff_id?", 
                s.name as "staff_name?", 
                c.id as "customer_id?",
                c.name as "customer_name?",
                e.code,
                e.code_type,
                e.success,
                e.event_date
            from entry_log e
            left join staff s on s.id = e.staff_id
            left join customer c on s.customer_id = c.id
            order by e.event_date desc
            limit 50
            "#,
        )
        .fetch_all(&self.pool)
        .await
    }
}
