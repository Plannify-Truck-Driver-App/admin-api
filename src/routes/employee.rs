use axum::{
    middleware::{from_fn, from_fn_with_state}, routing::get, Router
};
use crate::{
    handlers::employee_handlers::{
        get_all_accreditations, get_all_authorizations, get_all_employees, get_all_levels, get_employee_all_accreditations, get_employee_by_id, get_level_by_id
    }, 
    middleware::{auth_middleware, require_permissions, MiddlewareState}, 
    services::employee_service::EmployeeService
};
use sqlx::PgPool;
use std::sync::Arc;

pub fn protected_employees_routes(
    pool: PgPool,
    jwt_secret: String,
    employee_service: Arc<EmployeeService>,
) -> Router {
    let auth_state = MiddlewareState { pool, jwt_secret };

    Router::new()
        .route("/employees", get(get_all_employees))
        .route("/employees/{id}/accreditations", get(get_employee_all_accreditations))
        .route("/employees/{id}", get(get_employee_by_id))
        .route("/employees/levels", get(get_all_levels))
        .route("/employees/levels/{id}", get(get_level_by_id))
        .route("/employees/authorizations", get(get_all_authorizations))
        .route("/employees/accreditations", get(get_all_accreditations))
        .route_layer(from_fn(|req, next| {
            let method = req.method().as_str();
            let path = req.uri().path();

            let required_permissions = match (method, path) {
                ("GET", "/employees") => vec![20], // read all employees
                ("GET", path) if path.starts_with("/employees/") && path.ends_with("/accreditations") => vec![34],
                ("GET", path) if path.starts_with("/employees/") => vec![20],
                ("GET", "/employees/levels") => vec![33],
                ("GET", path) if path.starts_with("/employees/levels/") => vec![33],
                ("GET", "/employees/authorizations") => vec![32],
                ("GET", "/employees/accreditations") => vec![34],
                _ => vec![], // no permission required (should not happen)
            };

            require_permissions(required_permissions, req, next)
        }))
        .layer(from_fn_with_state(
            auth_state,
            auth_middleware,
        ))
        .with_state(employee_service.clone())
}
