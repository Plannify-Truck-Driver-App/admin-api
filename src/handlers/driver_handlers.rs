use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    Json,
};
use tracing::debug;
use std::sync::Arc;

use crate::models::driver::{CreateDriverRequest, GetAllDriversQuery, PaginatedResponse, PaginationInfo, UpdateDriverRequest, Driver};
use crate::database::driver_service::Database;
use crate::errors::app_error::AppError;
use uuid::Uuid;
use validator::Validate;

// Simple validation function using the Validate trait
fn validate_request<T: Validate>(req: &T) -> Result<(), AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(format!("The request content is not valid: {}", e)))
}

pub async fn get_all_drivers(
    Query(filters): Query<GetAllDriversQuery>,
    State(db): State<Arc<Database>>,
) -> Result<Json<PaginatedResponse<Driver>>, AppError> {
    debug!("Get all drivers request: {:?}", filters);

    // Validate pagination parameters
    if filters.page <= 0 {
        return Err(AppError::Validation("Page must be greater than 0".to_string()));
    }
    if filters.limit <= 0 || filters.limit > 100 {
        return Err(AppError::Validation("Limit must be between 1 and 100".to_string()));
    }
    
    let (drivers, total) = db.get_all_drivers(&filters).await?;
    
    let pagination_info = PaginationInfo {
        page: filters.page,
        limit: filters.limit,
        total
    };
    
    let response = PaginatedResponse {
        data: drivers,
        pagination: pagination_info,
    };
    
    Ok(Json(response))
}

pub async fn get_driver_by_id(
    Path(driver_id): Path<String>,
    State(db): State<Arc<Database>>,
) -> Result<Json<Driver>, AppError> {
    let driver_uuid = driver_id.parse::<Uuid>()
        .map_err(|_| AppError::Validation("Driver ID is not valid".to_string()))?;
    let driver = db.get_driver_by_id(&driver_uuid).await?;
    Ok(Json(driver))
}

pub async fn create_driver(
    State(db): State<Arc<Database>>,
    Json(create_req): Json<CreateDriverRequest>,
) -> Result<(StatusCode, Json<Driver>), AppError> {
    debug!("Create driver request: {:?}", create_req);
    
    // Validate the request content
    validate_request(&create_req).map_err(|e| AppError::Validation(e.to_string()))?;
    
    // Check if the email already exists    
    if db.email_exists(&create_req.email).await? {
        return Err(AppError::Conflict("a driver with this email already exists".to_string(), "DRIVER_EMAIL_ALREADY_EXISTS".to_string()));
    }
    
    let created_driver = db.create_driver(&create_req).await?;
    
    Ok((StatusCode::CREATED, Json(created_driver)))
}

pub async fn update_driver(
    Path(driver_id): Path<String>,
    State(db): State<Arc<Database>>,
    Json(update_req): Json<UpdateDriverRequest>,
) -> Result<Json<Driver>, AppError> {
    let driver_uuid = driver_id.parse::<Uuid>()
        .map_err(|_| AppError::Validation("Driver ID is not valid".to_string()))?;
    
    // Validate the request content
    validate_request(&update_req)?;
    
    // Check if the user exists
    let _existing_driver = db.get_driver_by_id(&driver_uuid).await?;
    
    // If the email is modified, check if it already exists
    if let Some(ref email) = update_req.email {
        if db.email_exists_except_driver(email, &driver_uuid).await? {
            return Err(AppError::Conflict("An other driver already uses this email".to_string(), "DRIVER_EMAIL_ALREADY_EXISTS".to_string()));
        }
    }
    
    let updated_driver = db.update_driver(&driver_uuid, &update_req).await?;
    Ok(Json(updated_driver))
}

pub async fn delete_driver(
    Path(driver_id): Path<String>,
    State(db): State<Arc<Database>>,
) -> Result<StatusCode, AppError> {
    let driver_uuid = driver_id.parse::<Uuid>()
        .map_err(|_| AppError::Validation("Driver ID is not valid".to_string()))?;
    
    // Check if the user exists
    let _existing_driver = db.get_driver_by_id(&driver_uuid).await?;
    
    db.delete_driver(&driver_uuid).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use crate::models::driver::GetAllDriversQuery;

    #[test]
    fn test_get_all_drivers_query_deserialization() {
        // Test without any filters
        let query = serde_json::from_str::<GetAllDriversQuery>(r#"{"page": 1, "limit": 20}"#).unwrap();
        assert_eq!(query.page, 1);
        assert_eq!(query.limit, 20);
        assert_eq!(query.pk_driver_id, None);
        assert_eq!(query.email, None);
        assert_eq!(query.sort_order, "asc"); // default value

        // Test with text filters
        let query_with_filters = serde_json::from_str::<GetAllDriversQuery>(
            r#"{"page": 2, "limit": 10, "firstname": "john", "email": "gmail"}"#
        ).unwrap();
        assert_eq!(query_with_filters.page, 2);
        assert_eq!(query_with_filters.limit, 10);
        assert_eq!(query_with_filters.firstname, Some("john".to_string()));
        assert_eq!(query_with_filters.email, Some("gmail".to_string()));
        assert_eq!(query_with_filters.sort_order, "asc"); // default value

        // Test with boolean filters
        let query_with_bools = serde_json::from_str::<GetAllDriversQuery>(
            r#"{"page": 1, "limit": 50, "is_searchable": true, "verified": true}"#
        ).unwrap();
        assert_eq!(query_with_bools.is_searchable, Some(true));
        assert_eq!(query_with_bools.verified, Some(true));

        // Test with presence filters
        let query_with_presence = serde_json::from_str::<GetAllDriversQuery>(
            r#"{"page": 1, "limit": 20, "rest_json": true, "deactivated": false}"#
        ).unwrap();
        assert_eq!(query_with_presence.rest_json, Some(true));
        assert_eq!(query_with_presence.deactivated, Some(false));

        // Test with sort order
        let query_with_sort_desc = serde_json::from_str::<GetAllDriversQuery>(
            r#"{"page": 1, "limit": 20, "sort_order": "desc"}"#
        ).unwrap();
        assert_eq!(query_with_sort_desc.sort_order, "desc");

        let query_with_sort_asc = serde_json::from_str::<GetAllDriversQuery>(
            r#"{"page": 1, "limit": 20, "sort_order": "asc"}"#
        ).unwrap();
        assert_eq!(query_with_sort_asc.sort_order, "asc");

        // Test with default values
        let query_defaults = serde_json::from_str::<GetAllDriversQuery>(r#"{}"#).unwrap();
        assert_eq!(query_defaults.page, 1);
        assert_eq!(query_defaults.limit, 20);
        assert_eq!(query_defaults.firstname, None);
        assert_eq!(query_defaults.is_searchable, None);
        assert_eq!(query_defaults.sort_order, "asc"); // default value
    }

    #[test]
    fn test_pagination_validation() {
        // Test valid page and limit
        let valid_query = GetAllDriversQuery {
            page: 1,
            limit: 50,
            ..Default::default()
        };
        assert_eq!(valid_query.page, 1);
        assert_eq!(valid_query.limit, 50);

        // Test default values
        let default_query = GetAllDriversQuery {
            ..Default::default()
        };
        assert_eq!(default_query.page, 1);
        assert_eq!(default_query.limit, 20);
    }
}
