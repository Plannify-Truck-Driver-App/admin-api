use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Validate)]
pub struct Employee {
    pub pk_employee_id: Uuid,
    pub firstname: String,
    pub lastname: String,
    pub gender: Option<String>,
    pub personal_email: String,
    #[serde(skip_serializing)]
    pub login_password_hash: String,
    pub phone_number: Option<String>,
    pub professional_email: String,
    pub professional_email_password: String,
    pub created_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub deactivated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EmployeeCreate {
    #[validate(length(min = 1, max = 255))]
    pub firstname: String,
    #[validate(length(min = 1, max = 255))]
    pub lastname: String,
    pub gender: Option<String>,
    #[validate(email)]
    pub personal_email: String,
    #[validate(length(min = 8))]
    pub login_password: String,
    pub phone_number: Option<String>,
    #[validate(email)]
    pub professional_email: String,
    #[validate(length(min = 1, max = 40))]
    pub professional_email_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EmployeeLoginRequest {
    #[validate(email)]
    pub professional_email: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EmployeeLoginResponse {
    pub token: String,
    pub token_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeWithPermissions {
    pub employee: Employee,
    pub permissions: Vec<i32>,
}