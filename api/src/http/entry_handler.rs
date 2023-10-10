use super::HttpResult;
use crate::domain::entry_log::{EntryLogDisplay, EntryLogRepository};
use axum::{extract::State, Json};

pub async fn list(
    State(entry_log_repo): State<EntryLogRepository>,
) -> HttpResult<Json<Vec<EntryLogDisplay>>> {
    let entry_list = entry_log_repo.fetch_all().await?;
    Ok(Json(entry_list))
}
