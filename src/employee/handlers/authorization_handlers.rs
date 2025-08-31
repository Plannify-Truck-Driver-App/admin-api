use axum::{extract::State, Json};

use crate::{employee::models::EmployeeAuthorization, errors::app_error::AppError, middleware::AppState};

pub async fn get_all_authorizations(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<EmployeeAuthorization>>, AppError> {
    let authorizations = app_state.employee_service.get_all_employee_authorizations().await?;
    Ok(Json(authorizations))
}