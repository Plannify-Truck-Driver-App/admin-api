use axum::{middleware::{from_fn, from_fn_with_state}, routing::{get, post, put}, Router};

use crate::{middleware::{auth_middleware, with_required_permissions, AppState}, workday::handlers::{create_workday, get_all_workdays, get_all_workdays_by_period, update_workday}};

pub fn protected_workday_routes(
    app_state: AppState
) -> Router {
    Router::new()
        .route("/workdays", get(get_all_workdays).route_layer(from_fn(with_required_permissions(vec![5]))))
        .route("/workdays/period", get(get_all_workdays_by_period).route_layer(from_fn(with_required_permissions(vec![5]))))
        .route("/workdays", post(create_workday).route_layer(from_fn(with_required_permissions(vec![6]))))
        .route("/workdays", put(update_workday).route_layer(from_fn(with_required_permissions(vec![7]))))
        .layer(from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ))
        .with_state(app_state)
}