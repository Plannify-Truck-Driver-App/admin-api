use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde_json::Value;
use validator::Validate;

use crate::models::paginate::{default_limit, default_page, default_sort_order};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Driver {
    pub pk_driver_id: Uuid,
    pub firstname: String,
    pub lastname: String,
    pub gender: Option<String>,
    pub email: String,
    pub phone_number: Option<String>,
    pub is_searchable: bool,
    pub allow_request_professional_agreement: bool,
    pub language: String,
    pub rest_json: Option<Value>,
    pub mail_preferences: i32,
    pub created_at: DateTime<Utc>,
    pub verified_at: Option<DateTime<Utc>>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub deactivated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateDriverRequest {
    #[validate(length(min = 1, max = 255, message = "Firstname is required and cannot be longer than 255 characters"))]
    pub firstname: String,
    
    #[validate(length(min = 1, max = 255, message = "Lastname is required and cannot be longer than 255 characters"))]
    pub lastname: String,
    
    #[validate(length(equal = 1, message = "Gender must be 'M', 'F' or undefined"))]
    pub gender: Option<String>,
    
    #[validate(email(message = "Invalid email format"))]
    #[validate(length(max = 255, message = "Email cannot be longer than 255 characters"))]
    pub email: String,
    
    #[validate(length(min = 8, message = "Password must contain at least 8 characters"))]
    pub password: String,
    
    #[validate(length(max = 20, message = "Phone number cannot be longer than 20 characters"))]
    pub phone_number: Option<String>,
    
    pub is_searchable: Option<bool>,
    pub allow_request_professional_agreement: Option<bool>,
    
    #[validate(length(equal = 2, message = "Language must be a 2 characters code (ex: fr, en)"))]
    pub language: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateDriverRequest {
    #[validate(length(min = 1, max = 255, message = "Firstname cannot be empty and cannot be longer than 255 characters"))]
    pub firstname: Option<String>,
    
    #[validate(length(min = 1, max = 255, message = "Lastname cannot be empty and cannot be longer than 255 characters"))]
    pub lastname: Option<String>,
    
    #[validate(length(equal = 1, message = "Gender must be 'M', 'F' or null"))]
    pub gender: Option<String>,
    
    #[validate(email(message = "Invalid email format"))]
    #[validate(length(max = 255, message = "Email cannot be longer than 255 characters"))]
    pub email: Option<String>,
    
    #[validate(length(max = 20, message = "Phone number cannot be longer than 20 characters"))]
    pub phone_number: Option<String>,
    
    pub is_searchable: Option<bool>,
    pub allow_request_professional_agreement: Option<bool>,
    
    #[validate(length(equal = 2, message = "Language must be a 2 characters code (ex: fr, en)"))]
    pub language: Option<String>,
    
    pub rest_json: Option<Value>,
    
    #[validate(range(min = 0, message = "Email preferences must be higher than 0"))]
    pub mail_preferences: Option<i32>,
    
    pub verified_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct GetAllDriversQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub pk_driver_id: Option<String>,
    #[serde(default)]
    pub firstname: Option<String>,
    #[serde(default)]
    pub lastname: Option<String>,
    #[serde(default)]
    pub gender: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub phone_number: Option<String>,
    #[serde(default)]
    pub is_searchable: Option<bool>,
    #[serde(default)]
    pub allow_request_professional_agreement: Option<bool>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub rest_json: Option<bool>,
    #[serde(default)]
    pub verified: Option<bool>,
    #[serde(default)]
    pub deactivated: Option<bool>,
    #[serde(default = "default_sort_order")]
    pub sort_order: String,
}

impl Default for GetAllDriversQuery {
    fn default() -> Self {
        Self {
            page: default_page(),
            limit: default_limit(),
            pk_driver_id: None,
            firstname: None,
            lastname: None,
            gender: None,
            email: None,
            phone_number: None,
            is_searchable: None,
            allow_request_professional_agreement: None,
            language: None,
            rest_json: None,
            verified: None,
            deactivated: None,
            sort_order: default_sort_order(),
        }
    }
}