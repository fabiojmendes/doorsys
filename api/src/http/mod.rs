use crate::domain::{
    code::CodeRepository, customer::CustomerRepository, entry_log::EntryLogRepository,
};
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

pub mod code_handler;
pub mod customer_handler;
pub mod entry_handler;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub customer_repo: CustomerRepository,
    pub code_repo: CodeRepository,
    pub entry_log_repo: EntryLogRepository,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(input: &AppState) -> Self {
        input.pool.clone()
    }
}

impl FromRef<AppState> for CustomerRepository {
    fn from_ref(input: &AppState) -> Self {
        input.customer_repo.clone()
    }
}

impl FromRef<AppState> for CodeRepository {
    fn from_ref(input: &AppState) -> Self {
        input.code_repo.clone()
    }
}

impl FromRef<AppState> for EntryLogRepository {
    fn from_ref(input: &AppState) -> Self {
        input.entry_log_repo.clone()
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

        tracing::error!("request error: {:?}", self);

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
    let customer_repo = CustomerRepository { pool: pool.clone() };
    let code_repo = CodeRepository { pool: pool.clone() };
    let entry_log_repo = EntryLogRepository { pool: pool.clone() };
    let app_state = AppState {
        pool,
        customer_repo,
        code_repo,
        entry_log_repo,
    };

    let app = Router::new()
        .route("/", get(health))
        .route(
            "/customers",
            get(customer_handler::list).post(customer_handler::create),
        )
        .route(
            "/customers/:id",
            get(customer_handler::get).put(customer_handler::update),
        )
        .route("/customers/:id/codes", get(code_handler::list))
        .route("/codes", post(code_handler::create))
        .route(
            "/codes/:code",
            get(code_handler::get)
                .delete(code_handler::delete)
                .put(code_handler::update),
        )
        .route("/entry_logs", get(entry_handler::list))
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
