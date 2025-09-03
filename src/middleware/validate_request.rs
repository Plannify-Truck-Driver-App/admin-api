use validator::Validate;

use crate::errors::app_error::AppError;

pub fn validate_request<T: Validate>(req: &T) -> Result<(), AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(format!("The request content is not valid: {}", e)))
}