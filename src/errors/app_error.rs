use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Conflict: {0} (Code: {1})")]
    Conflict(String, String),

    #[error("Not found: {0}")]
    NotFound(String, String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Insufficient permissions: {0:?}")]
    InsufficientPermissions(Vec<i32>),
    
    #[error("Internal server error")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::Serialization(_) => (StatusCode::BAD_REQUEST, "Data format error"),
            AppError::BadRequest(ref message) => (StatusCode::BAD_REQUEST, message.as_str()),
            AppError::Conflict(ref message, ref _error_code) => (StatusCode::CONFLICT, message.as_str()),
            AppError::NotFound(ref message, ref _error_code) => (StatusCode::NOT_FOUND, message.as_str()),
            AppError::Forbidden(ref message) => (StatusCode::FORBIDDEN, message.as_str()),
            AppError::Validation(ref message) => (StatusCode::BAD_REQUEST, message.as_str()),
            AppError::InsufficientPermissions(ref _permissions) => (StatusCode::FORBIDDEN, "Insufficient permissions"),
            AppError::Internal(ref message) => (StatusCode::INTERNAL_SERVER_ERROR, message.as_str()),
        };

        let body = match self {
            AppError::Conflict(ref message, ref error_code) => {
                Json(json!({
                    "error": message,
                    "error_code": error_code,
                    "status": status.as_u16()
                }))
            },
            AppError::NotFound(ref message, ref error_code) => {
                Json(json!({
                    "error": message,
                    "error_code": error_code,
                    "status": status.as_u16()
                }))
            },
            AppError::InsufficientPermissions(ref permissions) => {
                Json(json!({
                    "error": "Insufficient permissions",
                    "error_code": "INSUFFICIENT_PERMISSIONS",
                    "status": 403,
                    "required_permissions": permissions
                }))
            },
            _ => {
                Json(json!({
                    "error": error_message,
                    "status": status.as_u16()
                }))
            }
        };

        (status, body).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(_err: anyhow::Error) -> Self {
        AppError::Internal("Internal server error".to_string())
    }
}
