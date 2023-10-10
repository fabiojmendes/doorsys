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

fn generate_pin() -> String {
    let mut rng = rand::thread_rng();
    (0..6).map(|_| rng.gen_range(0..10).to_string()).collect()
}

pub async fn create(
    State(staff_repo): State<StaffRepository>,
    State(mqtt_client): State<AsyncClient>,
    Json(new_staff): Json<NewStaff>,
) -> HttpResult<Json<Staff>> {
    let pin = generate_pin();
    let staff = staff_repo.create(&new_staff, &pin).await?;

    let user_add = UserAction::Add(pin);
    if let Ok(payload) = bincode::encode_to_vec(user_add, mqtt::BINCODE_CONFIG) {
        mqtt_client
            .publish("doorsys/user", QoS::AtMostOnce, false, payload)
            .await?;
    }

    if let Some(fob) = &staff.fob {
        let user_add = UserAction::Add(fob.to_owned());
        if let Ok(payload) = bincode::encode_to_vec(user_add, mqtt::BINCODE_CONFIG) {
            mqtt_client
                .publish("doorsys/user", QoS::AtMostOnce, false, payload)
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
    Path(id): Path<i64>,
    Json(update_staff): Json<NewStaff>,
) -> HttpResult<Json<Staff>> {
    let staff = staff_repo.update(id, &update_staff).await?;
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
    let staff = staff_repo.update_pin(id, &new_pin).await?;

    let user_add = UserAction::Add(new_pin);
    if let Ok(payload) = bincode::encode_to_vec(user_add, mqtt::BINCODE_CONFIG) {
        mqtt_client
            .publish("doorsys/user", QoS::AtMostOnce, false, payload)
            .await?;
    }
    let user_del = UserAction::Del(old_pin);
    if let Ok(payload) = bincode::encode_to_vec(user_del, mqtt::BINCODE_CONFIG) {
        mqtt_client
            .publish("doorsys/user", QoS::AtMostOnce, false, payload)
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
