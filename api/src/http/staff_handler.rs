use super::HttpResult;
use crate::domain::staff::{NewStaff, Staff, StaffRepository};
use axum::{
    extract::{Path, State},
    Json,
};
use rand::Rng;

fn generate_pin() -> String {
    let mut rng = rand::thread_rng();
    (0..6).map(|_| rng.gen_range(0..10).to_string()).collect()
}

pub async fn create(
    State(staff_repo): State<StaffRepository>,
    Json(new_staff): Json<NewStaff>,
) -> HttpResult<Json<Staff>> {
    let pin = generate_pin();
    let staff = staff_repo.create(&new_staff, &pin).await?;
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
    Path(id): Path<i64>,
) -> HttpResult<Json<Staff>> {
    let new_pin = generate_pin();
    let staff = staff_repo.update_pin(id, &new_pin).await?;
    Ok(Json(staff))
}

pub async fn delete(
    State(staff_repo): State<StaffRepository>,
    Path(id): Path<i64>,
) -> HttpResult<()> {
    staff_repo.delete(id).await?;
    Ok(())
}
