use axum::{middleware::{from_fn, from_fn_with_state}, routing::{get, post}, Router};

use crate::{middleware::{auth_middleware, with_required_permissions, AppState}, workday::handlers::{create_workday, get_all_workdays, get_all_workdays_by_period}};

pub fn protected_workday_routes(
    app_state: AppState
) -> Router {
    Router::new()
        .route("/workdays", get(get_all_workdays).route_layer(from_fn(with_required_permissions(vec![1]))))
        .route("/workdays/period", get(get_all_workdays_by_period).route_layer(from_fn(with_required_permissions(vec![1]))))
        .route("/workdays", post(create_workday).route_layer(from_fn(with_required_permissions(vec![2]))))
        .layer(from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ))
        .with_state(app_state)
}