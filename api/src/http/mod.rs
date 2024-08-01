use crate::domain::{
    customer::CustomerRepository,
    device::DeviceRepository,
    entry_log::EntryLogRepository,
    staff::{StaffRepository, StaffService},
};
use anyhow::Context;
use axum::{
    extract::{FromRef, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use rumqttc::AsyncClient;
use serde_json::json;
use sqlx::PgPool;
use tokio::{
    net::TcpListener,
    signal::{self, unix::SignalKind},
};
use tower_http::trace::TraceLayer;

pub mod customer_handler;
pub mod device_handler;
pub mod entry_handler;
pub mod staff_handler;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub mqtt_client: AsyncClient,
    pub customer_repo: CustomerRepository,
    pub staff_repo: StaffRepository,
    pub entry_log_repo: EntryLogRepository,
    pub device_repo: DeviceRepository,
    pub staff_service: StaffService,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(input: &AppState) -> Self {
        input.pool.clone()
    }
}

impl FromRef<AppState> for AsyncClient {
    fn from_ref(input: &AppState) -> Self {
        input.mqtt_client.clone()
    }
}

impl FromRef<AppState> for CustomerRepository {
    fn from_ref(input: &AppState) -> Self {
        input.customer_repo.clone()
    }
}

impl FromRef<AppState> for StaffRepository {
    fn from_ref(input: &AppState) -> Self {
        input.staff_repo.clone()
    }
}

impl FromRef<AppState> for EntryLogRepository {
    fn from_ref(input: &AppState) -> Self {
        input.entry_log_repo.clone()
    }
}

impl FromRef<AppState> for DeviceRepository {
    fn from_ref(input: &AppState) -> Self {
        input.device_repo.clone()
    }
}

impl FromRef<AppState> for StaffService {
    fn from_ref(input: &AppState) -> Self {
        input.staff_service.clone()
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

pub async fn serve(pool: PgPool, mqtt_client: AsyncClient) -> anyhow::Result<()> {
    let customer_repo = CustomerRepository { pool: pool.clone() };
    let staff_repo = StaffRepository { pool: pool.clone() };
    let entry_log_repo = EntryLogRepository { pool: pool.clone() };
    let device_repo = DeviceRepository { pool: pool.clone() };
    let staff_service = StaffService {
        staff_repo: staff_repo.clone(),
        mqtt_client: mqtt_client.clone(),
    };
    let app_state = AppState {
        pool,
        mqtt_client,
        customer_repo,
        staff_repo,
        entry_log_repo,
        device_repo,
        staff_service,
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
        .route(
            "/customers/:id/status",
            put(customer_handler::update_status),
        )
        .route("/customers/:id/staff", get(staff_handler::list))
        .route("/staff", post(staff_handler::create))
        .route(
            "/staff/:id",
            get(staff_handler::get).put(staff_handler::update),
        )
        .route("/staff/:id/pin", post(staff_handler::update_pin))
        .route("/staff/:id/status", put(staff_handler::update_status))
        .route("/devices", get(device_handler::list))
        .route("/entry_logs", get(entry_handler::list))
        .route("/admin/bulk", post(staff_handler::bulk_load_codes))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let listerner = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listerner, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("error running HTTP server")
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

async fn health(State(pool): State<PgPool>) -> HttpResult<Json<serde_json::Value>> {
    sqlx::query("select 1").execute(&pool).await?;
    Ok(Json(json!({"ok": true})))
}
