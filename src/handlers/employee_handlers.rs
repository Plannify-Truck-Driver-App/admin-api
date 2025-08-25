use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;

use crate::{services::employee_service::EmployeeService, errors::app_error::AppError, models::employee::{EmployeeAuthorization, EmployeeLevel}};

pub async fn get_employee_all_levels(
    Path(employee_id): Path<String>,
    State(employee_service): State<Arc<EmployeeService>>,
) -> Result<Json<Vec<EmployeeLevel>>, AppError> {
    let levels = employee_service.get_employee_levels_by_employee_id(&employee_id).await?;
    Ok(Json(levels))
}

pub async fn get_all_authorizations(
    State(employee_service): State<Arc<EmployeeService>>,
) -> Result<Json<Vec<EmployeeAuthorization>>, AppError> {
    let authorizations = employee_service.get_all_employee_authorizations().await?;
    Ok(Json(authorizations))
}

pub async fn get_all_levels(
    State(employee_service): State<Arc<EmployeeService>>,
) -> Result<Json<Vec<EmployeeLevel>>, AppError> {
    let levels = employee_service.get_all_employee_levels().await?;
    Ok(Json(levels))
}

pub async fn get_level_by_id(
    Path(level_id): Path<i32>,
    State(employee_service): State<Arc<EmployeeService>>,
) -> Result<Json<EmployeeLevel>, AppError> {
    let level = employee_service.get_employee_level_by_id(level_id).await?;
    Ok(Json(level))
}