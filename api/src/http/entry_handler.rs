use super::HttpResult;
use crate::domain::entry_log::{EntryLogDisplay, EntryLogRepository};
use axum::{
    extract::{Query, State},
    Json,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Filter {
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    customer_id: Option<i64>,
}

pub async fn list(
    State(entry_log_repo): State<EntryLogRepository>,
    filter: Query<Filter>,
) -> HttpResult<Json<Vec<EntryLogDisplay>>> {
    let date_range = filter.start_date..filter.end_date;
    tracing::info!("Getting entry_logs for {:?}", filter);
    let entry_list = entry_log_repo
        .fetch_all(date_range, filter.customer_id)
        .await?;
    Ok(Json(entry_list))
}
