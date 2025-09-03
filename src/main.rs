use axum::Router;
use redis::aio::ConnectionManager;
use sqlx::PgPool;
use tracing_subscriber::EnvFilter;
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer}};
use tracing::info;

use crate::{
    auth::{
        routes::public_auth_routes,
        services::AuthService
    }, 
    driver::{
        routes::protected_driver_routes, 
        services::DriverService
    },
    workday::{
        routes::protected_workday_routes,
        services::WorkdayService
    },
    employee::{
        routes::protected_employees_routes, 
        services::EmployeeService
    }, 
    middleware::AppState
};

mod models;
mod errors;
mod middleware;
mod driver;
mod auth;
mod employee;
mod workday;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();
    
    // Connect to the database
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be defined");
    
    let pool = PgPool::connect(&database_url).await?;
    info!("Database connection established");

    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be defined");

    let redis_url = std::env::var("REDIS_URL")
        .expect("REDIS_URL must be defined");

    let redis_client = redis::Client::open(redis_url)
        .expect("Failed to create Redis client");
    let redis_manager: ConnectionManager = ConnectionManager::new(redis_client)
        .await
        .expect("Failed to get Redis connection manager");

    let driver_service = Arc::new(DriverService::new(pool.clone()));
    let auth_service = Arc::new(AuthService::new(pool.clone(), redis_manager));
    let employee_service = Arc::new(EmployeeService::new(pool.clone()));
    let workday_service = Arc::new(WorkdayService::new(pool.clone()));

    let app_state = AppState {
        auth_service,
        employee_service,
        driver_service,
        workday_service,
        jwt_secret,
    };

    // CORS configuration
    let cors = CorsLayer::permissive();

    let admin_router = Router::new()
        .merge(public_auth_routes(app_state.clone()))
        .merge(protected_driver_routes(app_state.clone()))
        .merge(protected_workday_routes(app_state.clone()))
        .merge(protected_employees_routes(app_state.clone()));

    let app = Router::new()
        .nest("/admin", admin_router)
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_response(DefaultOnResponse::new().include_headers(true)),
        );
    
    let addr = "[::]:3000";
    info!("Server started on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}