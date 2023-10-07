use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Staff {
    pub id: i64,
    pub customer_id: i64,
    pub name: String,
    pub phone: String,
    pub pin: String,
    pub fob: Option<String>,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewStaff {
    pub customer_id: i64,
    pub name: String,
    pub phone: String,
    pub fob: Option<String>,
}

#[derive(Clone)]
pub struct StaffRepository {
    pub pool: PgPool,
}

impl StaffRepository {
    pub async fn create(&self, new_staff: &NewStaff, pin: &str) -> Result<Staff, sqlx::Error> {
        sqlx::query_as!(
            Staff,
            "insert into staff (customer_id, name, phone, pin, fob) values ($1, $2, $3, $4, $5) returning *",
            new_staff.customer_id,
            new_staff.name,
            new_staff.phone,
            pin,
            new_staff.fob,
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update(&self, id: i64, update_staff: &NewStaff) -> Result<Staff, sqlx::Error> {
        sqlx::query_as!(
            Staff,
            "update staff set name = $1, phone = $2, fob = $3 where id = $4 returning *",
            update_staff.name,
            update_staff.phone,
            update_staff.fob,
            id,
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update_pin(&self, id: i64, new_pin: &str) -> Result<Staff, sqlx::Error> {
        sqlx::query_as!(
            Staff,
            "update staff set pin = $1 where id = $2 returning *",
            new_pin,
            id,
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn fetch_all(&self, customer_id: i64) -> Result<Vec<Staff>, sqlx::Error> {
        sqlx::query_as!(
            Staff,
            "select * from staff where customer_id = $1",
            customer_id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn fetch_one(&self, id: i64) -> Result<Staff, sqlx::Error> {
        sqlx::query_as!(Staff, "select * from staff where id = $1", id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!("delete from staff where id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
