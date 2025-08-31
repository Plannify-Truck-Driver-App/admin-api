use axum::{
    routing::{get, post}, Router
};
use http::StatusCode;
use crate::{auth::handlers::{login, refresh_token}, middleware::AppState};

pub fn public_auth_routes(
    app_state: AppState,
) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/auth/login", post(login))
        .route("/auth/refresh", post(refresh_token))
        .with_state(app_state.clone())
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
