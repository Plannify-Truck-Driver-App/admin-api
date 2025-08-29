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
    services::{driver_service::DriverService, auth_service::AuthService},
    errors::app_error::AppError,
};
use validator::Validate;

fn validate_request<T: Validate>(req: &T) -> Result<(), AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(format!("The request content is not valid: {}", e)))
}

pub async fn login(
    State(auth_service): State<Arc<AuthService>>,
    Json(login): Json<EmployeeLoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let response = auth_service.login(&login).await?;
    Ok(Json(response))
}

pub async fn refresh_token(
    State(auth_service): State<Arc<AuthService>>,
    Json(refresh_req): Json<RefreshTokenRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let response = auth_service.refresh_token(&refresh_req).await?;
    Ok(Json(response))
}

pub async fn register(
    State((_driver_service, auth_service)): State<(Arc<DriverService>, Arc<AuthService>)>,
    Json(employee_data): Json<EmployeeCreate>,
) -> Result<Json<Employee>, AppError> {
    // validate the request content
    validate_request(&employee_data)?;
    
    let employee = auth_service.create_employee(&employee_data).await?;
    Ok(Json(employee))
}