use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid, // employee_id
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub permissions: Vec<i32>, // authorization IDs
    pub exp: i64, // expiration timestamp
    pub iat: i64, // issued at timestamp
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub employee: EmployeeInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeInfo {
    pub id: Uuid,
    pub firstname: String,
    pub lastname: String,
    pub professional_email: String,
    pub permissions: Vec<i32>,
}

impl Claims {
    pub fn new(
        employee_id: Uuid,
        email: String,
        firstname: String,
        lastname: String,
        permissions: Vec<i32>,
        expiration_hours: i64,
    ) -> Self {
        let now = Utc::now();
        let exp = now + chrono::Duration::hours(expiration_hours);
        
        Self {
            sub: employee_id,
            email,
            firstname,
            lastname,
            permissions,
            exp: exp.timestamp(),
            iat: now.timestamp(),
        }
    }
    
    pub fn is_expired(&self) -> bool {
        let now = Utc::now().timestamp();
        now > self.exp
    }
}
