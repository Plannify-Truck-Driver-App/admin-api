use axum::{
    middleware::{from_fn, from_fn_with_state}, routing::{delete, get, post, put}, Router
};
use crate::{driver::{handlers::{create_driver, deactivate_driver, get_all_drivers, get_driver_by_id, update_driver}}, middleware::{auth_middleware, with_required_permissions, AppState}};

pub fn protected_driver_routes(
    app_state: AppState
) -> Router {
    Router::new()
        .route("/drivers", get(get_all_drivers).route_layer(from_fn(with_required_permissions(vec![1]))))
        .route("/drivers", post(create_driver).route_layer(from_fn(with_required_permissions(vec![2]))))
        .route("/drivers/{id}", get(get_driver_by_id).route_layer(from_fn(with_required_permissions(vec![1]))))
        .route("/drivers/{id}", put(update_driver).route_layer(from_fn(with_required_permissions(vec![3]))))
        .route("/drivers/{id}", delete(deactivate_driver).route_layer(from_fn(with_required_permissions(vec![4]))))
        .layer(from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ))
        .with_state(app_state)
}
