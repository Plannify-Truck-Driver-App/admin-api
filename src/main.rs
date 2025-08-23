use axum::{
    routing::{get, post, put, delete},
    http::StatusCode,
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;

mod models;
mod handlers;
mod database;
mod errors;

use crate::handlers::user_handlers::*;
use crate::database::user_service::Database;

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
    let db = Arc::new(Database::new(pool));
    
    info!("Database connection established");
    
    // CORS configuration
    let cors = CorsLayer::permissive();
    
    // Routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/users", get(get_all_users))
        .route("/users", post(create_user))
        .route("/users/{id}", get(get_user_by_id))
        .route("/users/{id}", put(update_user))
        .route("/users/{id}", delete(delete_user))
        .layer(cors)
        .with_state(db);
    
    let addr = "[::]:3000";
    info!("Server started on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
