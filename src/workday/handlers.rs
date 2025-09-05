use axum::{extract::{Query, State}, Json};
use http::StatusCode;
use tracing::debug;

use crate::{errors::app_error::AppError, middleware::AppState, models::paginate::{PaginatedResponse, PaginationInfo, PAGINATE_MAX_LIMIT}, workday::models::{CreateWorkdayRequest, GetAllWorkdaysByPeriodQuery, GetAllWorkdaysQuery, UpdateWorkdayRequest, Workday}};

pub async fn get_all_workdays(
    Query(filters): Query<GetAllWorkdaysQuery>,
    State(app_state): State<AppState>,
) -> Result<Json<PaginatedResponse<Workday>>, AppError> {
    debug!("Get all workdays with filters: {:?}", filters);
    if filters.page <= 0 {
        return Err(AppError::Validation("Page must be greater than 0".to_string()));
    }
    if filters.limit <= 0 || filters.limit > PAGINATE_MAX_LIMIT {
        return Err(AppError::Validation(format!("Limit must be between 1 and {}.", PAGINATE_MAX_LIMIT)));
    }

    let (workdays, total_count) = app_state.workday_service.get_all_workdays(&filters).await?;

    let result = PaginatedResponse {
        data: workdays,
        pagination: PaginationInfo {
            page: filters.page,
            limit: filters.limit,
            total: total_count,
        },
    };

    Ok(Json(result))
}

pub async fn get_all_workdays_by_period(
    Query(filters): Query<GetAllWorkdaysByPeriodQuery>,
    State(app_state): State<AppState>,
) -> Result<Json<PaginatedResponse<Workday>>, AppError> {
    debug!("Get all workdays by period with filters: {:?}", filters);
    if filters.page <= 0 {
        return Err(AppError::Validation("Page must be greater than 0".to_string()));
    }

    if filters.limit <= 0 || filters.limit > PAGINATE_MAX_LIMIT {
        return Err(AppError::Validation(format!("Limit must be between 1 and {}.", PAGINATE_MAX_LIMIT)));
    }

    let (workdays, total_count) = app_state.workday_service.get_all_workdays_by_period(&filters).await?;

    let result = PaginatedResponse {
        data: workdays,
        pagination: PaginationInfo {
            page: filters.page,
            limit: filters.limit,
            total: total_count,
        },
    };

    Ok(Json(result))
}

pub async fn create_workday(
    State(app_state): State<AppState>,
    Json(create_req): Json<CreateWorkdayRequest>,
) -> Result<(StatusCode, Json<Workday>), AppError> {
    debug!("Create workday request: {:?}", create_req);

    let created_workday = app_state.workday_service.create_workday(&create_req).await?;

    Ok((StatusCode::CREATED, Json(created_workday)))
}

pub async fn update_workday(
    State(app_state): State<AppState>,
    Json(update_req): Json<UpdateWorkdayRequest>,
) -> Result<(StatusCode, Json<Workday>), AppError> {
    debug!("Update workday request: {:?}", update_req);

    let updated_workday = app_state.workday_service.update_workday(&update_req).await?;

    Ok((StatusCode::OK, Json(updated_workday)))
}