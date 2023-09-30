use super::HttpResult;
use crate::domain::user::{NewUser, User, UserRepository};
use axum::{
    extract::{Path, State},
    Json,
};

pub async fn create(
    State(user_repo): State<UserRepository>,
    Json(user_form): Json<NewUser>,
) -> HttpResult<Json<User>> {
    let user = user_repo.create(&user_form).await?;
    Ok(Json(user))
}

pub async fn update(
    State(user_repo): State<UserRepository>,
    Path(id): Path<i64>,
    Json(new_user): Json<NewUser>,
) -> HttpResult<Json<User>> {
    let user = user_repo.update(id, &new_user).await?;
    Ok(Json(user))
}

pub async fn get(
    State(user_repo): State<UserRepository>,
    Path(id): Path<i64>,
) -> HttpResult<Json<User>> {
    let user = user_repo.fetch_one(id).await?;
    Ok(Json(user))
}

pub async fn list(State(user_repo): State<UserRepository>) -> HttpResult<Json<Vec<User>>> {
    let users = user_repo.fetch_all().await?;
    Ok(Json(users))
}
