use axum::{
    extract::State,
    Json,
};
use std::sync::Arc;

use crate::{
    models::{
        employee::{EmployeeLoginRequest, EmployeeCreate, Employee},
        jwt::{AuthResponse, RefreshTokenRequest},
    },
    database::{driver_service::Database, auth_service::AuthService},
    errors::app_error::AppError,
};
use validator::Validate;

// Simple validation function using the Validate trait
fn validate_request<T: Validate>(req: &T) -> Result<(), AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(format!("The request content is not valid: {}", e)))
}

pub async fn login(
    State((_db, auth_service)): State<(Arc<Database>, Arc<AuthService>)>,
    Json(login): Json<EmployeeLoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let response = auth_service.login(&login).await?;
    Ok(Json(response))
}

pub async fn refresh_token(
    State((_db, auth_service)): State<(Arc<Database>, Arc<AuthService>)>,
    Json(refresh_req): Json<RefreshTokenRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let response = auth_service.refresh_token(&refresh_req).await?;
    Ok(Json(response))
}

pub async fn register(
    State((_db, auth_service)): State<(Arc<Database>, Arc<AuthService>)>,
    Json(employee_data): Json<EmployeeCreate>,
) -> Result<Json<Employee>, AppError> {
    // Validate the request content
    validate_request(&employee_data)?;
    
    let employee = auth_service.create_employee(&employee_data).await?;
    Ok(Json(employee))
}