use anyhow::Context;
use axum::{
    extract::{FromRef, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router, Server,
};
use serde_json::json;
use sqlx::PgPool;

pub mod admin;
pub mod code;
pub mod user;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(input: &AppState) -> Self {
        input.pool.clone()
    }
}

pub type HttpResult<T, E = AppError> = core::result::Result<T, E>;

#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = StatusCode::INTERNAL_SERVER_ERROR;
        let payload = json!({
            "code": status.as_u16(),
            "success": status.is_success(),
            "msg": format!("{}", self.0)
        });

        (status, payload.to_string()).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub async fn serve(pool: PgPool) -> anyhow::Result<()> {
    let app_state = AppState { pool };

    let app = Router::new()
        .route("/", get(health))
        .route("/users", get(user::list).post(user::create))
        .route("/users/:id", get(user::get).put(user::update))
        .route("/codes", post(code::create))
        .route(
            "/codes/:id",
            get(code::get).delete(code::delete).put(code::update),
        )
        .with_state(app_state);

    Server::bind(&"127.0.0.1:3000".parse()?)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}

async fn health(State(pool): State<PgPool>) -> HttpResult<Json<serde_json::Value>> {
    sqlx::query("select 1").execute(&pool).await?;
    Ok(Json(json!({"ok": true})))
}
