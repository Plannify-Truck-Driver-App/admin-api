use axum::{extract::{Path, Query, State, Extension}, Json};
use uuid::Uuid;

use crate::{
    employee::models::{AssignAccreditationRequest, EmployeeAccreditation, UpdateAccreditationRequest},
    errors::app_error::AppError,
    middleware::{auth::AuthState, AppState},
    models::paginate::{PaginateQuery, PaginatedResponse, PaginationInfo, PAGINATE_MAX_LIMIT}
};

pub async fn get_all_accreditations(
    Query(filters): Query<PaginateQuery>,
    State(app_state): State<AppState>,
) -> Result<Json<PaginatedResponse<EmployeeAccreditation>>, AppError> {
    if filters.page <= 0 {
        return Err(AppError::Validation("Page must be greater than 0".to_string()));
    }
    if filters.limit <= 0 || filters.limit > PAGINATE_MAX_LIMIT {
        return Err(AppError::Validation(format!("Limit must be between 1 and {}.", PAGINATE_MAX_LIMIT)));
    }

    let (accreditations, total) = app_state.employee_service.get_all_employee_accreditations(&filters).await?;

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

pub async fn assign_accreditation(
    State(app_state): State<AppState>,
    Extension(auth_state): Extension<AuthState>,
    Json(assign_req): Json<AssignAccreditationRequest>,
) -> Result<Json<EmployeeAccreditation>, AppError> {
    let accreditation = app_state.employee_service.assign_accreditation(&assign_req, &auth_state.employee_id.to_string()).await?;
    Ok(Json(accreditation))
}

pub async fn update_accreditation(
    Path(accreditation_id): Path<String>,
    State(app_state): State<AppState>,
    Extension(auth_state): Extension<AuthState>,
    Json(update_req): Json<UpdateAccreditationRequest>,
) -> Result<Json<EmployeeAccreditation>, AppError> {
    let accreditation_uuid = accreditation_id.parse::<Uuid>()
        .map_err(|_| AppError::Validation("Accreditation ID is not valid".to_string()))?;

    let accreditation = app_state.employee_service.update_accreditation(&accreditation_uuid, &update_req, &auth_state.employee_id.to_string()).await?;
    Ok(Json(accreditation))
}

pub async fn delete_accreditation(
    Path(accreditation_id): Path<String>,
    State(app_state): State<AppState>,
    Extension(auth_state): Extension<AuthState>,
) -> Result<Json<()>, AppError> {
    let accreditation_uuid = accreditation_id.parse::<Uuid>()
        .map_err(|_| AppError::Validation("Accreditation ID is not valid".to_string()))?;

    app_state.employee_service.delete_accreditation(&accreditation_uuid, &auth_state.employee_id.to_string()).await?;
    Ok(Json(()))
}

pub async fn get_all_accreditations_by_employee_id(
    Query(filters): Query<PaginateQuery>,
    Path(employee_id): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<PaginatedResponse<EmployeeAccreditation>>, AppError> {
    if filters.page <= 0 {
        return Err(AppError::Validation("Page must be greater than 0".to_string()));
    }
    if filters.limit <= 0 || filters.limit > PAGINATE_MAX_LIMIT {
        return Err(AppError::Validation(format!("Limit must be between 1 and {}.", PAGINATE_MAX_LIMIT)));
    }

    let (accreditations, total) = app_state.employee_service.get_employee_accreditations_by_employee_id(&employee_id, &filters).await?;

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