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
    
    #[error("Conflict: {0} (Code: {1})")]
    Conflict(String, String),

    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Internal server error")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::Serialization(_) => (StatusCode::BAD_REQUEST, "Data format error"),
            AppError::Conflict(ref message, ref _error_code) => (StatusCode::CONFLICT, message.as_str()),
            AppError::NotFound(ref message) => (StatusCode::NOT_FOUND, message.as_str()),
            AppError::Validation(ref message) => (StatusCode::BAD_REQUEST, message.as_str()),
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
