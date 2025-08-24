use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;

use crate::{database::employee_service::EmployeeService, errors::app_error::AppError, models::employee::EmployeeAuthorization};

pub async fn get_all_authorizations(
    State(employee_service): State<Arc<EmployeeService>>,
) -> Result<Json<Vec<EmployeeAuthorization>>, AppError> {
    let levels = employee_service.get_all_employee_authorizations().await?;
    Ok(Json(levels))
}

// pub async fn get_level_by_id(
//     Path(level_id): Path<i32>,
//     State(employee_service): State<Arc<EmployeeService>>,
// ) -> Result<Json<EmployeeAuthorization>, AppError> {
//     let level = employee_service.get_employee_authorization_by_id(level_id).await?;
//     Ok(Json(level))
// }