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
mod database;
mod errors;
mod middleware;

use crate::handlers::{
    driver_handlers::{get_all_drivers, get_driver_by_id, create_driver, update_driver, deactivate_driver},
    auth_handlers::{login, register}
};
use crate::database::{driver_service::Database, auth_service::AuthService};
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
    let db = Arc::new(Database::new(pool.clone()));
    
    // JWT secret (en production, utilisez une variable d'environnement sécurisée)
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be defined");
    
    let auth_service = Arc::new(AuthService::new(pool.clone(), jwt_secret.clone()));
    
    info!("Database connection established");
    
    // CORS configuration
    let cors = CorsLayer::permissive();
    
    // Routes publiques (sans authentification)
    let public_routes = Router::new()
        .route("/health", get(health_check))
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        .with_state((db.clone(), auth_service.clone()));
    
    // Routes protégées avec vérification des permissions granulaires des chauffeurs
    // Utilisation du nouveau système de permissions configuré
    let protected_driver_routes = Router::new()
        .route("/drivers", get(get_all_drivers))
        .route("/drivers", post(create_driver))
        .route("/drivers/{id}", get(get_driver_by_id))
        .route("/drivers/{id}", put(update_driver))
        .route("/drivers/{id}", delete(deactivate_driver))
        .route_layer(axum_middleware::from_fn(|req: axum::extract::Request, next: axum::middleware::Next| {
            // Récupérer la méthode et le chemin pour déterminer les permissions requises
            let method = req.method().as_str();
            let path = req.uri().path();
            
            let required_permissions = match (method, path) {
                ("GET", "/drivers") => vec![1],           // Liste des chauffeurs
                ("POST", "/drivers") => vec![2],          // Créer un chauffeur
                ("GET", path) if path.starts_with("/drivers/") => vec![1], // Lire un chauffeur
                ("PUT", path) if path.starts_with("/drivers/") => vec![3], // Modifier un chauffeur
                ("DELETE", path) if path.starts_with("/drivers/") => vec![4], // Désactiver un chauffeur
                _ => vec![], // Aucune permission requise (ne devrait pas arriver)
            };
            
            require_permissions(required_permissions, req, next)
        }))
        .layer(axum_middleware::from_fn_with_state(
            (pool.clone(), jwt_secret.clone()),
            auth_middleware
        ))
        .with_state(db.clone());
    
    // Application principale
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_driver_routes)
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
