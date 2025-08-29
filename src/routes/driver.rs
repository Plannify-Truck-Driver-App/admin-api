use axum::{
    middleware::{from_fn, from_fn_with_state}, routing::{delete, get, post, put}, Router
};
use crate::{
    handlers::driver_handlers::{
        create_driver, deactivate_driver, get_all_drivers, get_driver_by_id, update_driver
    }, middleware::{auth_middleware, with_required_permissions, MiddlewareState}, services::driver_service::DriverService
};
use std::sync::Arc;

pub fn protected_driver_routes(
    jwt_secret: String,
    driver_service: Arc<DriverService>,
) -> Router {
    let auth_state = MiddlewareState { jwt_secret };

    Router::new()
        .route("/drivers", get(get_all_drivers).route_layer(from_fn(with_required_permissions(vec![1]))))
        .route("/drivers", post(create_driver).route_layer(from_fn(with_required_permissions(vec![2]))))
        .route("/drivers/{id}", get(get_driver_by_id).route_layer(from_fn(with_required_permissions(vec![1]))))
        .route("/drivers/{id}", put(update_driver).route_layer(from_fn(with_required_permissions(vec![3]))))
        .route("/drivers/{id}", delete(deactivate_driver).route_layer(from_fn(with_required_permissions(vec![4]))))
        .layer(from_fn_with_state(
            auth_state,
            auth_middleware,
        ))
        .with_state(driver_service.clone())
}
