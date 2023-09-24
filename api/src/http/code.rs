use super::HttpResult;
use crate::domain::Code;
use axum::{
    extract::{Path, State},
    Form, Json,
};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct CodeForm {
    user_id: i64,
    code: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCodeForm {
    code: String,
}

pub async fn create(
    State(pool): State<PgPool>,
    Form(code_form): Form<CodeForm>,
) -> HttpResult<Json<Code>> {
    let code = sqlx::query_file_as!(
        Code,
        "queries/code_insert.sql",
        &code_form.user_id,
        &code_form.code
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(code))
}

pub async fn get(State(pool): State<PgPool>, Path(id): Path<i64>) -> HttpResult<Json<Code>> {
    let code = sqlx::query_file_as!(Code, "queries/code_select.sql", id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(code))
}

pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
    Form(form): Form<UpdateCodeForm>,
) -> HttpResult<Json<Code>> {
    let code = sqlx::query_file_as!(Code, "queries/code_update.sql", &form.code, id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(code))
}

pub async fn delete(State(pool): State<PgPool>, Path(id): Path<i64>) -> HttpResult<()> {
    sqlx::query_file!("queries/code_delete.sql", id)
        .execute(&pool)
        .await?;

    Ok(())
}
