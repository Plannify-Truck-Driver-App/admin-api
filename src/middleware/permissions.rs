use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

use crate::{
    middleware::AuthState,
    errors::app_error::AppError,
};

/// Middleware to check if the employee has the required permissions
pub async fn require_permissions(
    required_permissions: Vec<i32>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // get auth state from extensions
    let auth_state = request
        .extensions()
        .get::<AuthState>()
        .ok_or_else(|| AppError::Validation("Authentication required".to_string()))?;

    // check if the employee has all the required permissions
    let has_all_permissions = required_permissions
        .iter()
        .all(|&required| auth_state.permissions.contains(&required));

    if !has_all_permissions {
        return Err(AppError::InsufficientPermissions(required_permissions));
    }

    Ok(next.run(request).await)
}
