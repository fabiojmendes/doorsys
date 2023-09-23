use axum::{
    debug_handler,
    extract::{Path, State},
    Form, Json,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::domain::Code;

use super::HttpResult;

#[derive(Debug, Deserialize)]
pub struct CodeForm {
    pub user_id: i64,
    pub code: String,
}

pub async fn add_code(
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

#[debug_handler]
pub async fn get_code(State(pool): State<PgPool>, Path(id): Path<i64>) -> HttpResult<Json<Code>> {
    let code = sqlx::query_file_as!(Code, "queries/code_select.sql", id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(code))
}
pub async fn delete_code(State(pool): State<PgPool>, Path(id): Path<i64>) -> HttpResult<()> {
    sqlx::query_file!("queries/code_delete.sql", id)
        .execute(&pool)
        .await?;

    Ok(())
}
