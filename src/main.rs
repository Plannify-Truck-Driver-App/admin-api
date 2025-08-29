use axum::{
    routing::{get, post},
    http::StatusCode,
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;

mod models;
mod handlers;
mod services;
mod errors;
mod middleware;
mod routes;

use crate::{handlers::{
    auth_handlers::{login, refresh_token}
}, routes::{driver::protected_driver_routes, employee::protected_employees_routes}};
use crate::services::{driver_service::DriverService, auth_service::AuthService, employee_service::EmployeeService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Load environment variables
    dotenvy::dotenv().ok();
    
    // Connect to the database
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be defined");
    
    let pool = PgPool::connect(&database_url).await?;
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be defined");
    
    let driver_service = Arc::new(DriverService::new(pool.clone()));
    let auth_service = Arc::new(AuthService::new(pool.clone()));
    let employee_service = Arc::new(EmployeeService::new(pool.clone()));
    
    info!("Database connection established");
    
    // CORS configuration
    let cors = CorsLayer::permissive();
    
    // public routes (no auth)
    let public_routes = Router::new()
        .route("/health", get(health_check))
        .route("/auth/login", post(login))
        .route("/auth/refresh", post(refresh_token))
        .with_state((driver_service.clone(), auth_service.clone()));

    // main app
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_driver_routes(
            pool.clone(),
            jwt_secret.clone(),
            driver_service.clone(),
        ))
        .merge(protected_employees_routes(
            pool.clone(),
            jwt_secret.clone(),
            employee_service.clone(),
        ))
        .layer(cors);
    
    let addr = "[::]:3000";
    info!("Server started on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
