use axum::{extract::{Path, State}, Json};

use crate::{employee::models::{EmployeeLevel, EmployeeLevelWithAuthorizations}, errors::app_error::AppError, middleware::AppState};

pub async fn get_all_levels(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<EmployeeLevel>>, AppError> {
    let levels = app_state.employee_service.get_all_employee_levels().await?;
    Ok(Json(levels))
}

pub async fn get_level_by_id(
    Path(level_id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<EmployeeLevelWithAuthorizations>, AppError> {
    let level = app_state.employee_service.get_employee_level_with_authorizations_by_id(level_id).await?;
    Ok(Json(level))
}