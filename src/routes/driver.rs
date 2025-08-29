use axum::{
    extract::Request, middleware::{from_fn, from_fn_with_state, Next}, routing::{delete, get, post, put}, Router
};
use crate::{
    handlers::driver_handlers::{
        create_driver, deactivate_driver, get_all_drivers, get_driver_by_id, update_driver
    }, middleware::{auth_middleware, require_permissions, MiddlewareState}, services::{driver_service::DriverService}
};
use sqlx::PgPool;
use std::sync::Arc;

pub fn protected_driver_routes(
    pool: PgPool,
    jwt_secret: String,
    driver_service: Arc<DriverService>,
) -> Router {
    let auth_state = MiddlewareState { pool, jwt_secret };

    Router::new()
        .route("/drivers", get(get_all_drivers))
        .route("/drivers", post(create_driver))
        .route("/drivers/{id}", get(get_driver_by_id))
        .route("/drivers/{id}", put(update_driver))
        .route("/drivers/{id}", delete(deactivate_driver))
        .route_layer(from_fn(|req: Request, next: Next| {
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
        .layer(from_fn_with_state(
            auth_state,
            auth_middleware,
        ))
        .with_state(driver_service.clone())
}
