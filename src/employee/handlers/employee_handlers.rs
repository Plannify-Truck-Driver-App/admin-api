use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{
    employee::models::{Employee, EmployeeCreateRequest, GetAllEmployeesQuery}, errors::app_error::AppError, middleware::{validate_request, AppState}, models::paginate::{PaginatedResponse, PAGINATE_MAX_LIMIT}
};

pub async fn get_all_employees(
    Query(filters): Query<GetAllEmployeesQuery>,
    State(app_state): State<AppState>,
) -> Result<Json<PaginatedResponse<Employee>>, AppError> {
    if filters.page <= 0 {
        return Err(AppError::Validation("Page must be greater than 0".to_string()));
    }
    if filters.limit <= 0 || filters.limit > PAGINATE_MAX_LIMIT {
        return Err(AppError::Validation(format!("Limit must be between 1 and {}.", PAGINATE_MAX_LIMIT)));
    }

    let response = app_state.employee_service.get_all_employees(&filters).await?;

    Ok(Json(response))
}

pub async fn get_employee_by_id(
    Path(employee_id): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<Employee>, AppError> {
    let employee = app_state.employee_service.get_employee_by_id(&employee_id).await?;
    Ok(Json(employee))
}

pub async fn create_employee(
    State(app_state): State<AppState>,
    Json(new_employee): Json<EmployeeCreateRequest>,
) -> Result<Json<Employee>, AppError> {
    validate_request(&new_employee)?;

    let created_employee = app_state.employee_service.create_employee(&new_employee).await?;
    Ok(Json(created_employee))
}