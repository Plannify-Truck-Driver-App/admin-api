use axum::{extract::{Path, Query, State}, Extension, Json};

use crate::{employee::models::{CreateEmployeeDerogationRequest, EmployeeDerogation}, errors::app_error::AppError, middleware::{AppState, AuthState}, models::paginate::{PaginateQuery, PaginatedResponse, PaginationInfo, PAGINATE_MAX_LIMIT}};

pub async fn get_all_derogations(
    Query(filters): Query<PaginateQuery>,
    State(app_state): State<AppState>,
) -> Result<Json<PaginatedResponse<EmployeeDerogation>>, AppError> {
    if filters.page <= 0 {
        return Err(AppError::Validation("Page must be greater than 0".to_string()));
    }
    if filters.limit <= 0 || filters.limit > PAGINATE_MAX_LIMIT {
        return Err(AppError::Validation(format!("Limit must be between 1 and {}.", PAGINATE_MAX_LIMIT)));
    }

    let (derogations, total) = app_state.employee_service.get_all_employee_derogations(&filters).await?;

    let response = PaginatedResponse {
        data: derogations,
        pagination: PaginationInfo {
            total,
            page: filters.page,
            limit: filters.limit,
        },
    };

    Ok(Json(response))
}

pub async fn get_derogation_by_id(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<EmployeeDerogation>, AppError> {
    let derogation = app_state.employee_service.get_employee_derogation_by_id(id).await?;
    Ok(Json(derogation))
}

pub async fn create_derogation(
    State(app_state): State<AppState>,
    Extension(auth_state): Extension<AuthState>,
    Json(request): Json<CreateEmployeeDerogationRequest>,
) -> Result<Json<EmployeeDerogation>, AppError> {
    let employee_authorizing_id = auth_state.employee_id.to_string();
    let derogation = app_state.employee_service.create_employee_derogation(&app_state.auth_service, &request, &employee_authorizing_id).await?;
    Ok(Json(derogation))
}

pub async fn delete_derogation(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
    Extension(auth_state): Extension<AuthState>,
) -> Result<Json<()>, AppError> {
    app_state.employee_service.delete_employee_derogation_by_id(id, &app_state.auth_service, &auth_state.employee_id.to_string()).await?;
    Ok(Json(()))
}