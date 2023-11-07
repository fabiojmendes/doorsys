use super::HttpResult;
use crate::{
    domain::staff::{NewStaff, Staff, StaffRepository},
    mqtt,
};
use axum::{
    extract::{Path, State},
    Json,
};
use doorsys_protocol::UserAction;
use rand::Rng;
use rumqttc::{AsyncClient, QoS};

fn generate_pin() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(100000..=999999)
}

pub async fn create(
    State(staff_repo): State<StaffRepository>,
    State(mqtt_client): State<AsyncClient>,
    Json(new_staff): Json<NewStaff>,
) -> HttpResult<Json<Staff>> {
    let pin = generate_pin();
    let staff = staff_repo.create(&new_staff, pin).await?;

    let user_add = UserAction::Add(pin);
    if let Ok(payload) = bincode::encode_to_vec(user_add, mqtt::BINCODE_CONFIG) {
        mqtt_client
            .publish("doorsys/user", QoS::AtLeastOnce, false, payload)
            .await?;
    }

    if let Some(fob) = staff.fob {
        let user_add = UserAction::Add(fob);
        if let Ok(payload) = bincode::encode_to_vec(user_add, mqtt::BINCODE_CONFIG) {
            mqtt_client
                .publish("doorsys/user", QoS::AtLeastOnce, false, payload)
                .await?;
        }
    }
    Ok(Json(staff))
}

pub async fn get(
    State(staff_repo): State<StaffRepository>,
    Path(id): Path<i64>,
) -> HttpResult<Json<Staff>> {
    let staff = staff_repo.fetch_one(id).await?;
    Ok(Json(staff))
}

pub async fn list(
    State(staff_repo): State<StaffRepository>,
    Path(customer_id): Path<i64>,
) -> HttpResult<Json<Vec<Staff>>> {
    let staff_list = staff_repo.fetch_all(customer_id).await?;
    Ok(Json(staff_list))
}

pub async fn update(
    State(staff_repo): State<StaffRepository>,
    State(mqtt_client): State<AsyncClient>,
    Path(id): Path<i64>,
    Json(update_staff): Json<NewStaff>,
) -> HttpResult<Json<Staff>> {
    let old_staff = staff_repo.fetch_one(id).await?;
    let staff = staff_repo.update(id, &update_staff).await?;

    if let Some(action) = match (old_staff.fob, staff.fob) {
        (Some(old), Some(new)) if old != new => Some(UserAction::Replace { old, new }),
        (None, Some(fob)) => Some(UserAction::Add(fob)),
        (Some(fob), None) => Some(UserAction::Del(fob)),
        _ => None,
    } {
        if let Ok(payload) = bincode::encode_to_vec(action, mqtt::BINCODE_CONFIG) {
            mqtt_client
                .publish("doorsys/user", QoS::AtLeastOnce, false, payload)
                .await?;
        }
    }
    Ok(Json(staff))
}

pub async fn update_pin(
    State(staff_repo): State<StaffRepository>,
    State(mqtt_client): State<AsyncClient>,
    Path(id): Path<i64>,
) -> HttpResult<Json<Staff>> {
    let old_staff = staff_repo.fetch_one(id).await?;
    let old_pin = old_staff.pin;
    let new_pin = generate_pin();
    let staff = staff_repo.update_pin(id, new_pin).await?;

    let replace_pin = UserAction::Replace {
        old: old_pin,
        new: new_pin,
    };
    if let Ok(payload) = bincode::encode_to_vec(replace_pin, mqtt::BINCODE_CONFIG) {
        mqtt_client
            .publish("doorsys/user", QoS::AtLeastOnce, false, payload)
            .await?;
    }
    Ok(Json(staff))
}

pub async fn delete(
    State(staff_repo): State<StaffRepository>,
    Path(id): Path<i64>,
) -> HttpResult<()> {
    staff_repo.delete(id).await?;
    Ok(())
}
