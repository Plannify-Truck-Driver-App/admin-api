use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid, // employee_id
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub authorizations: Vec<i32>,
    pub exp: i64, // expiration timestamp
    pub iat: i64, // issued at timestamp
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: Uuid, // employee_id
    pub exp: i64,  // expiration timestamp
    pub iat: i64,  // issued at timestamp
}

impl Claims {
    pub fn new(
        employee_id: Uuid,
        email: String,
        firstname: String,
        lastname: String,
        authorizations: Vec<i32>,
        minutes_valid: u32,
    ) -> Self {
        let now = Utc::now();
        let exp = now + chrono::Duration::minutes(minutes_valid.into());
        
        Self {
            sub: employee_id,
            email,
            firstname,
            lastname,
            authorizations,
            exp: exp.timestamp(),
            iat: now.timestamp(),
        }
    }

    pub fn is_expired(&self) -> bool {
        let now = Utc::now().timestamp();
        now > self.exp
    }
}

impl RefreshClaims {
    pub fn new(employee_id: Uuid, minutes_valid: u32) -> Self {
        let now = Utc::now();
        let exp = now + chrono::Duration::minutes(minutes_valid.into());
        
        Self {
            sub: employee_id,
            exp: exp.timestamp(),
            iat: now.timestamp(),
        }
    }

    pub fn is_expired(&self) -> bool {
        let now = Utc::now().timestamp();
        now > self.exp
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeLoginRequest {
    pub professional_email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeInfo {
    pub id: Uuid,
    pub firstname: String,
    pub lastname: String,
    pub professional_email: String,
    pub permissions: Vec<i32>,
}
