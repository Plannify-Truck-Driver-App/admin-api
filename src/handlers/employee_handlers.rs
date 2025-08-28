use axum::{
    extract::{Path, Query, State},
    Json,
};
use std::sync::Arc;

use crate::{
    errors::app_error::AppError,
    models::{employee::{
        Employee, EmployeeAccreditation, EmployeeAuthorization, EmployeeLevel, EmployeeLevelWithAuthorizations, GetAllEmployeesQuery
    }, paginate::{
        PaginateQuery,
        PaginatedResponse,
        PaginationInfo,
        PAGINATE_MAX_LIMIT
    }},
    services::employee_service::EmployeeService
};

pub async fn get_all_employees(
    Query(filters): Query<GetAllEmployeesQuery>,
    State(employee_service): State<Arc<EmployeeService>>,
) -> Result<Json<PaginatedResponse<Employee>>, AppError> {
    if filters.page <= 0 {
        return Err(AppError::Validation("Page must be greater than 0".to_string()));
    }
    if filters.limit <= 0 || filters.limit > PAGINATE_MAX_LIMIT {
        return Err(AppError::Validation(format!("Limit must be between 1 and {}.", PAGINATE_MAX_LIMIT)));
    }

    let response = employee_service.get_all_employees(&filters).await?;

    Ok(Json(response))
}

pub async fn get_employee_by_id(
    Path(employee_id): Path<String>,
    State(employee_service): State<Arc<EmployeeService>>,
) -> Result<Json<Employee>, AppError> {
    let employee = employee_service.get_employee_by_id(&employee_id).await?;
    Ok(Json(employee))
}

pub async fn get_employee_all_accreditations(
    Query(filters): Query<PaginateQuery>,
    Path(employee_id): Path<String>,
    State(employee_service): State<Arc<EmployeeService>>,
) -> Result<Json<PaginatedResponse<EmployeeAccreditation>>, AppError> {
    if filters.page <= 0 {
        return Err(AppError::Validation("Page must be greater than 0".to_string()));
    }
    if filters.limit <= 0 || filters.limit > PAGINATE_MAX_LIMIT {
        return Err(AppError::Validation(format!("Limit must be between 1 and {}.", PAGINATE_MAX_LIMIT)));
    }

    let (accreditations, total) = employee_service.get_employee_accreditations_by_employee_id(&employee_id, &filters).await?;

    let response = PaginatedResponse {
        data: accreditations,
        pagination: PaginationInfo {
            total,
            page: filters.page,
            limit: filters.limit,
        },
    };

    Ok(Json(response))
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
) -> Result<Json<EmployeeLevelWithAuthorizations>, AppError> {
    let level = employee_service.get_employee_level_with_authorizations_by_id(level_id).await?;
    Ok(Json(level))
}

pub async fn get_all_accreditations(
    Query(filters): Query<PaginateQuery>,
    State(employee_service): State<Arc<EmployeeService>>,
) -> Result<Json<PaginatedResponse<EmployeeAccreditation>>, AppError> {
    if filters.page <= 0 {
        return Err(AppError::Validation("Page must be greater than 0".to_string()));
    }
    if filters.limit <= 0 || filters.limit > PAGINATE_MAX_LIMIT {
        return Err(AppError::Validation(format!("Limit must be between 1 and {}.", PAGINATE_MAX_LIMIT)));
    }
    
    let (accreditations, total) = employee_service.get_all_employee_accreditations(&filters).await?;

    let response = PaginatedResponse {
        data: accreditations,
        pagination: PaginationInfo {
            total,
            page: filters.page,
            limit: filters.limit,
        },
    };

    Ok(Json(response))
}