use crate::domain::{code::CodeRepository, user::UserRepository};
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
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub mod code;
pub mod user;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub user_repo: UserRepository,
    pub code_repo: CodeRepository,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(input: &AppState) -> Self {
        input.pool.clone()
    }
}

impl FromRef<AppState> for UserRepository {
    fn from_ref(input: &AppState) -> Self {
        input.user_repo.clone()
    }
}

impl FromRef<AppState> for CodeRepository {
    fn from_ref(input: &AppState) -> Self {
        input.code_repo.clone()
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
    let user_repo = UserRepository { pool: pool.clone() };
    let code_repo = CodeRepository { pool: pool.clone() };
    let app_state = AppState {
        pool,
        user_repo,
        code_repo,
    };

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(health))
        .route("/users", get(user::list).post(user::create))
        .route("/users/:id", get(user::get).put(user::update))
        .route("/users/:id/codes", get(code::list))
        .route("/codes", post(code::create))
        .route(
            "/codes/:code",
            get(code::get).delete(code::delete).put(code::update),
        )
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
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
