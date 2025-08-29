use axum::{
    middleware::{from_fn, from_fn_with_state}, routing::get, Router
};
use crate::{
    handlers::employee_handlers::{
        get_all_accreditations, get_all_authorizations, get_all_employees, get_all_levels, get_employee_all_accreditations, get_employee_by_id, get_level_by_id
    }, 
    middleware::{auth_middleware, with_required_permissions, MiddlewareState}, 
    services::employee_service::EmployeeService
};
use std::sync::Arc;

pub fn protected_employees_routes(
    jwt_secret: String,
    employee_service: Arc<EmployeeService>,
) -> Router {
    let auth_state = MiddlewareState { jwt_secret };

    Router::new()
        .route("/employees", get(get_all_employees).route_layer(from_fn(with_required_permissions(vec![20]))))
        .route("/employees/{id}", get(get_employee_by_id).route_layer(from_fn(with_required_permissions(vec![20]))))
        .route("/employees/levels", get(get_all_levels).route_layer(from_fn(with_required_permissions(vec![33]))))
        .route("/employees/levels/{id}", get(get_level_by_id).route_layer(from_fn(with_required_permissions(vec![33]))))
        .route("/employees/authorizations", get(get_all_authorizations).route_layer(from_fn(with_required_permissions(vec![32]))))
        .route("/employees/accreditations", get(get_all_accreditations).route_layer(from_fn(with_required_permissions(vec![34]))))
        .route("/employees/{id}/accreditations", get(get_employee_all_accreditations).route_layer(from_fn(with_required_permissions(vec![34]))))
        .layer(from_fn_with_state(
            auth_state,
            auth_middleware,
        ))
        .with_state(employee_service.clone())
}
