use axum::{
    routing::{get, post}, Router
};
use http::StatusCode;
use crate::{
    handlers::auth_handlers::{login, refresh_token},
    services::{auth_service::AuthService}
};
use std::sync::Arc;

pub fn public_auth_routes(
    auth_service: Arc<AuthService>,
) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/auth/login", post(login))
        .route("/auth/refresh", post(refresh_token))
        .with_state(auth_service.clone())
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
