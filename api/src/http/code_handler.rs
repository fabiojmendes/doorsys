use super::HttpResult;
use crate::domain::code::{Code, CodeRepository};
use axum::{
    extract::{Path, State},
    Json,
};

pub async fn create(
    State(code_repo): State<CodeRepository>,
    Json(code): Json<Code>,
) -> HttpResult<Json<Code>> {
    let code = code_repo.create(code).await?;
    Ok(Json(code))
}

pub async fn get(
    State(code_repo): State<CodeRepository>,
    Path(code): Path<String>,
) -> HttpResult<Json<Code>> {
    let code = code_repo.fetch_one(&code).await?;
    Ok(Json(code))
}

pub async fn list(
    State(code_repo): State<CodeRepository>,
    Path(customer_id): Path<i64>,
) -> HttpResult<Json<Vec<Code>>> {
    let code_list = code_repo.fetch_all(customer_id).await?;
    Ok(Json(code_list))
}

pub async fn update(
    State(code_repo): State<CodeRepository>,
    Path(old_code): Path<String>,
    Json(new_code): Json<String>,
) -> HttpResult<Json<Code>> {
    let code = code_repo.update(&old_code, &new_code).await?;
    Ok(Json(code))
}

pub async fn delete(
    State(code_repo): State<CodeRepository>,
    Path(code): Path<String>,
) -> HttpResult<()> {
    code_repo.delete(&code).await?;
    Ok(())
}
