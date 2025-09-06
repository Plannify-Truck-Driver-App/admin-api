use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

use crate::{models::paginate::{default_limit, default_page, default_sort_order}};

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
    #[serde(skip_serializing)]
    pub professional_email_password: String,
    pub created_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub deactivated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Validate)]
pub struct LightEmployee {
    pub pk_employee_id: Uuid,
    pub firstname: String,
    pub lastname: String,
    pub gender: Option<String>,
    pub professional_email: String,
}

#[derive(Debug, Deserialize)]
pub struct GetAllEmployeesQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub firstname: Option<String>,
    #[serde(default)]
    pub lastname: Option<String>,
    #[serde(default)]
    pub gender: Option<String>,
    #[serde(default)]
    pub personal_email: Option<String>,
    #[serde(default)]
    pub phone_number: Option<String>,
    #[serde(default)]
    pub professional_email: Option<String>,
    #[serde(default)]
    pub deactivated: Option<bool>,
    #[serde(default = "default_sort_order")]
    pub sort_order: String,
}

impl Default for GetAllEmployeesQuery {
    fn default() -> Self {
        Self {
            page: default_page(),
            limit: default_limit(),
            firstname: None,
            lastname: None,
            gender: None,
            personal_email: None,
            phone_number: None,
            professional_email: None,
            deactivated: None,
            sort_order: default_sort_order(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EmployeeCreateRequest {
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


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum EntityType {
    DRIVER,
    EMPLOYEE,
}

impl FromStr for EntityType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DRIVER" => Ok(EntityType::DRIVER),
            "EMPLOYEE" => Ok(EntityType::EMPLOYEE),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum CrudType {
    C,
    R,
    U,
    D,
}

impl FromStr for CrudType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(CrudType::C),
            "R" => Ok(CrudType::R),
            "U" => Ok(CrudType::U),
            "D" => Ok(CrudType::D),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct EmployeeAuthorization {
    pub pk_employee_authorization_id: i32,
    pub authorization_feature_code: String,
    pub authorization_index: i32,
    pub crud_type: CrudType,
    pub description: String,
    pub category_name_code: String,
    pub category_entity_type: EntityType,
    pub category_index: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EmployeeLevel {
    pub pk_employee_level_id: i32,
    pub level_index: i32,
    pub level_label: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EmployeeLevelWithAuthorizations {
    pub pk_employee_level_id: i32,
    pub level_index: i32,
    pub level_label: String,
    pub authorizations: Vec<EmployeeAuthorization>,
}

pub struct EmployeeAccreditationRow {
    pub pk_employee_accreditation_authorization_id: Uuid,
    pub fk_recipient_employee_id: Uuid,
    pub fk_employee_level_id: i32,
    pub fk_authorizing_employee_id: Option<Uuid>,
    pub start_at: DateTime<Utc>,
    pub end_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EmployeeAccreditation {
    pub accreditation_id: Uuid,
    pub recipient_employee: LightEmployee,
    pub employee_level: EmployeeLevel,
    pub authorizing_employee: Option<LightEmployee>,
    pub start_at: DateTime<Utc>,
    pub end_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignAccreditationRequest {
    pub employee_id: Uuid,
    pub level_id: i32,
    pub start_at: DateTime<Utc>,
    pub end_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAccreditationRequest {
    pub level_id: i32,
    pub start_at: DateTime<Utc>,
    pub end_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EmployeeDerogation {
    pub pk_derogation_id: i32,
    pub recipient_employee: LightEmployee,
    pub employee_authorization: EmployeeAuthorization,
    pub authorizing_employee: LightEmployee,
    pub derogation_reason: Option<String>,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

pub struct EmployeeDerogationRow {
    pub pk_employee_authorization_derogation_id: i32,
    pub fk_recipient_employee_id: Uuid,
    pub fk_employee_authorization_type_id: i32,
    pub fk_authorizing_employee_id: Uuid,
    pub derogation_reason: Option<String>,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEmployeeDerogationRequest {
    pub recipient_employee_id: Uuid,
    pub employee_authorization_type_id: i32,
    pub derogation_reason: Option<String>,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
}