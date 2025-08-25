use axum::{
    routing::{get, post, put, delete},
    http::StatusCode,
    Router,
    middleware as axum_middleware,
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

use crate::handlers::{
    auth_handlers::{login, refresh_token}, driver_handlers::{create_driver, deactivate_driver, get_all_drivers, get_driver_by_id, update_driver}, employee_handlers::{get_all_authorizations, get_all_levels, get_level_by_id}
};
use crate::services::{driver_service::DriverService, auth_service::AuthService, employee_service::EmployeeService};
use crate::middleware::{
    auth_middleware, 
    require_permissions,
};

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
    
    let protected_driver_routes = Router::new()
        .route("/drivers", get(get_all_drivers))
        .route("/drivers", post(create_driver))
        .route("/drivers/{id}", get(get_driver_by_id))
        .route("/drivers/{id}", put(update_driver))
        .route("/drivers/{id}", delete(deactivate_driver))
        .route_layer(axum_middleware::from_fn(|req: axum::extract::Request, next: axum::middleware::Next| {
            let method = req.method().as_str();
            let path = req.uri().path();
            
            let required_permissions = match (method, path) {
                ("GET", "/drivers") => vec![1],           // drivers list
                ("POST", "/drivers") => vec![2],          // create driver
                ("GET", path) if path.starts_with("/drivers/") => vec![1], // read driver
                ("PUT", path) if path.starts_with("/drivers/") => vec![3], // update driver
                ("DELETE", path) if path.starts_with("/drivers/") => vec![4], // deactivate driver
                _ => vec![], // no permission required (should not happen)
            };
            
            require_permissions(required_permissions, req, next)
        }))
        .layer(axum_middleware::from_fn_with_state(
            (pool.clone(), jwt_secret.clone()),
            auth_middleware
        ))
        .with_state(driver_service.clone());

    let protected_authorizations_routes = Router::new()
        .route("/employees/authorizations", get(get_all_authorizations))
        //.route("/levels/{id}", get(get_level_by_id))
        .route_layer(axum_middleware::from_fn(|req: axum::extract::Request, next: axum::middleware::Next| {
            let method = req.method().as_str();
            let path = req.uri().path();
            
            let required_permissions = match (method, path) {
                ("GET", "/employees/authorizations") => vec![13], // read all authorizations
                _ => vec![], // no permission required (should not happen)
            };
            
            require_permissions(required_permissions, req, next)
        }))
        .layer(axum_middleware::from_fn_with_state(
            (pool.clone(), jwt_secret.clone()),
            auth_middleware
        ))
        .with_state(employee_service.clone());

    let protected_levels_routes = Router::new()
        .route("/employees/levels", get(get_all_levels))
        .route("/employees/levels/{id}", get(get_level_by_id))
        .route_layer(axum_middleware::from_fn(|req: axum::extract::Request, next: axum::middleware::Next| {
            let method = req.method().as_str();
            let path = req.uri().path();

            let required_permissions = match (method, path) {
                ("GET", "/employees/levels") => vec![14], // read all levels
                ("GET", path) if path.starts_with("/employees/levels/") => vec![14], // read level by id
                _ => vec![], // no permission required (should not happen)
            };

            require_permissions(required_permissions, req, next)
        }))
        .layer(axum_middleware::from_fn_with_state(
            (pool.clone(), jwt_secret.clone()),
            auth_middleware
        ))
        .with_state(employee_service.clone());
    
    // main app
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_driver_routes)
        .merge(protected_authorizations_routes)
        .merge(protected_levels_routes)
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
