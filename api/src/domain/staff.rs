use chrono::{DateTime, Utc};
use doorsys_protocol::UserAction;
use rumqttc::{AsyncClient, QoS};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::mqtt;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Staff {
    pub id: i64,
    pub customer_id: i64,
    pub name: String,
    pub phone: String,
    pub pin: i32,
    pub fob: Option<i32>,
    pub active: bool,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewStaff {
    pub customer_id: i64,
    pub name: String,
    pub phone: String,
    pub fob: Option<i32>,
}

#[derive(Clone)]
pub struct StaffRepository {
    pub pool: PgPool,
}

impl StaffRepository {
    pub async fn create(&self, new_staff: &NewStaff, pin: i32) -> Result<Staff, sqlx::Error> {
        sqlx::query_as!(
            Staff,
            r#"insert into staff (customer_id, name, phone, pin, fob) values ($1, $2, $3, $4, $5) returning *"#,
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
            r#"update staff set name = $1, phone = $2, fob = $3 where id = $4 returning *"#,
            update_staff.name,
            update_staff.phone,
            update_staff.fob,
            id,
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update_pin(&self, id: i64, new_pin: i32) -> Result<Staff, sqlx::Error> {
        sqlx::query_as!(
            Staff,
            r#"update staff set pin = $1 where id = $2 returning *"#,
            new_pin,
            id,
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update_status(&self, id: i64, active: bool) -> Result<Staff, sqlx::Error> {
        sqlx::query_as!(
            Staff,
            r#"update staff set active = $1 where id = $2 returning *"#,
            active,
            id,
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn bulk_update_status(
        &self,
        customer_id: i64,
        active: bool,
    ) -> Result<Vec<Staff>, sqlx::Error> {
        sqlx::query_as!(
            Staff,
            r#"update staff set active = $1 where customer_id = $2 returning *"#,
            active,
            customer_id,
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn fetch_all(&self, customer_id: i64) -> Result<Vec<Staff>, sqlx::Error> {
        sqlx::query_as!(
            Staff,
            r#"select * from staff where customer_id = $1 order by name"#,
            customer_id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn fetch_one(&self, id: i64) -> Result<Staff, sqlx::Error> {
        sqlx::query_as!(Staff, r#"select * from staff where id = $1"#, id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn fetch_all_codes(&self) -> Result<Vec<Option<i32>>, sqlx::Error> {
        sqlx::query_scalar!(
            r#"
            with all_codes(code, active) as (
                select pin, active from staff 
                union 
                select fob, active from staff
            ) select code from all_codes where code is not null and active is true order by code
            "#,
        )
        .fetch_all(&self.pool)
        .await
    }
}

#[derive(Clone)]
pub struct StaffService {
    pub staff_repo: StaffRepository,
    pub mqtt_client: AsyncClient,
}

impl StaffService {
    pub async fn bulk_update_status(&self, customer_id: i64, active: bool) -> anyhow::Result<()> {
        let staff_list = self
            .staff_repo
            .bulk_update_status(customer_id, active)
            .await?;
        for staff in staff_list {
            self.send_mqtt_message(&staff).await?;
        }
        Ok(())
    }

    pub async fn update_status(&self, id: i64, active: bool) -> anyhow::Result<Staff> {
        let staff = self.staff_repo.update_status(id, active).await?;
        self.send_mqtt_message(&staff).await?;
        Ok(staff)
    }

    pub async fn send_mqtt_message(&self, staff: &Staff) -> anyhow::Result<()> {
        let pin_action = match staff.active {
            true => UserAction::Add(staff.pin),
            false => UserAction::Del(staff.pin),
        };

        let payload = bincode::encode_to_vec(pin_action, mqtt::BINCODE_CONFIG)?;
        self.mqtt_client
            .publish("doorsys/user", QoS::AtLeastOnce, false, payload)
            .await?;

        if let Some(fob) = staff.fob {
            let fob_action = match staff.active {
                true => UserAction::Add(fob),
                false => UserAction::Del(fob),
            };
            let payload = bincode::encode_to_vec(fob_action, mqtt::BINCODE_CONFIG)?;
            self.mqtt_client
                .publish("doorsys/user", QoS::AtLeastOnce, false, payload)
                .await?;
        }
        Ok(())
    }
}
