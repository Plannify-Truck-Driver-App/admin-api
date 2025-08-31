use axum::{
    middleware::{from_fn, from_fn_with_state}, routing::{delete, get, post, put}, Router
};
use crate::{
    employee::handlers::{
            accreditation_handlers::{
                assign_accreditation, delete_accreditation, get_all_accreditations, get_all_accreditations_by_employee_id, update_accreditation
            }, authorization_handlers::get_all_authorizations, employee_handlers::{
                get_all_employees, get_employee_by_id
            }, level_handlers::{
                get_all_levels, get_level_by_id
            }
        }, middleware::{
        auth_middleware, with_required_permissions, AppState
    }
};

pub fn protected_employees_routes(
    app_state: AppState
) -> Router {
    Router::new()
        .route("/employees", get(get_all_employees).route_layer(from_fn(with_required_permissions(vec![20]))))
        .route("/employees/{id}", get(get_employee_by_id).route_layer(from_fn(with_required_permissions(vec![20]))))
        .route("/employees/levels", get(get_all_levels).route_layer(from_fn(with_required_permissions(vec![33]))))
        .route("/employees/levels/{id}", get(get_level_by_id).route_layer(from_fn(with_required_permissions(vec![33]))))
        .route("/employees/authorizations", get(get_all_authorizations).route_layer(from_fn(with_required_permissions(vec![32]))))
        .route("/employees/accreditations", get(get_all_accreditations).route_layer(from_fn(with_required_permissions(vec![34]))))
        .route("/employees/accreditations", post(assign_accreditation).route_layer(from_fn(with_required_permissions(vec![35]))))
        .route("/employees/accreditations/{id}", put(update_accreditation).route_layer(from_fn(with_required_permissions(vec![36]))))
        .route("/employees/accreditations/{id}", delete(delete_accreditation).route_layer(from_fn(with_required_permissions(vec![37]))))
        .route("/employees/{id}/accreditations", get(get_all_accreditations_by_employee_id).route_layer(from_fn(with_required_permissions(vec![34]))))
        .layer(from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ))
        .with_state(app_state)
}
