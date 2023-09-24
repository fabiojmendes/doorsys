use super::HttpResult;
use crate::domain::{Code, User, UserCode};
use axum::{
    extract::{Path, State},
    Form, Json,
};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct UserForm {
    pub name: String,
    pub email: String,
}

pub async fn create(
    State(pool): State<PgPool>,
    Form(user_form): Form<UserForm>,
) -> HttpResult<Json<User>> {
    let user = sqlx::query_file_as!(
        User,
        "queries/user_insert.sql",
        user_form.name,
        user_form.email
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(user))
}

pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
    Form(user_form): Form<UserForm>,
) -> HttpResult<Json<User>> {
    let user = sqlx::query_file_as!(
        User,
        "queries/user_update.sql",
        &user_form.name,
        &user_form.email,
        id,
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(user))
}

pub async fn get(State(pool): State<PgPool>, Path(id): Path<i64>) -> HttpResult<Json<User>> {
    let user = sqlx::query_file_as!(User, "queries/user_select.sql", id,)
        .fetch_one(&pool)
        .await?;

    Ok(Json(user))
}

pub async fn list(State(pool): State<PgPool>) -> HttpResult<Json<Vec<UserCode>>> {
    let users = sqlx::query_file_as!(UserCode, "queries/users_codes.sql")
        .fetch_all(&pool)
        .await?;

    Ok(Json(users))
}
