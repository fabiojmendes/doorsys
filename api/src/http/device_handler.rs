use super::HttpResult;
use crate::domain::device::{Device, DeviceRepository};
use axum::{extract::State, Json};

pub async fn list(State(device_repo): State<DeviceRepository>) -> HttpResult<Json<Vec<Device>>> {
    let device_list = device_repo.fetch_all().await?;
    Ok(Json(device_list))
}
