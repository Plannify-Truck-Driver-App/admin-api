use axum::{
    extract::State,
    Json,
};
use tracing::debug;

use crate::{
    auth::models::{AuthResponse, RefreshTokenRequest}, employee::models::{Employee, EmployeeCreate, EmployeeLoginRequest}, errors::app_error::AppError, middleware::{validate_request, AppState}
};

pub async fn login(
    State(app_state): State<AppState>,
    Json(login): Json<EmployeeLoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    debug!("Logging in user: {}", login.professional_email);
    validate_request(&login)?;

    let response = app_state.auth_service.login(&login).await?;
    Ok(Json(response))
}

pub async fn refresh_token(
    State(app_state): State<AppState>,
    Json(refresh_req): Json<RefreshTokenRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    debug!("Refreshing token for token: {}", refresh_req.refresh_token);
    let response = app_state.auth_service.refresh_token(&refresh_req).await?;
    Ok(Json(response))
}

pub async fn register(
    State(app_state): State<AppState>,
    Json(employee_data): Json<EmployeeCreate>,
) -> Result<Json<Employee>, AppError> {
    // validate the request content
    validate_request(&employee_data)?;

    let employee = app_state.auth_service.create_employee(&employee_data).await?;
    Ok(Json(employee))
}