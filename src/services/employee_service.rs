use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::app_error::AppError, models::employee::{CrudType, EmployeeAuthorization, EmployeeLevel, EntityType}};

pub struct EmployeeService {
    pool: PgPool,
}

impl EmployeeService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all_employee_authorizations(&self) -> Result<Vec<EmployeeAuthorization>, AppError> {
        let authorizations = sqlx::query!(
            r#"
            SELECT eat.pk_employee_authorization_type_id, ea.feature_code as authorization_feature_code, ea.authorization_index as authorization_index, eac.name_code as category_name_code, eac.entity_type as "entity_type: String", eac.category_index, eat.crud_type as "crud_type: String", eat.description
            FROM employee_authorizations ea
            JOIN employee_authorization_categories eac ON ea.fk_employee_authorization_category_id = eac.pk_employee_authorization_category_id
            JOIN employee_authorization_types eat ON ea.pk_employee_authorization_id = eat.fk_employee_authorization_id
            "#
        )
        .fetch_all(&self.pool).await?;

        let result: Vec<EmployeeAuthorization> = authorizations
            .into_iter()
            .filter_map(|row| {
                let entity_type = row.entity_type.parse::<EntityType>().ok()?;
                let crud_type = row.crud_type?.parse::<CrudType>().ok()?;
                
                Some(EmployeeAuthorization {
                    pk_employee_authorization_id: row.pk_employee_authorization_type_id,
                    authorization_feature_code: row.authorization_feature_code,
                    authorization_index: row.authorization_index,
                    category_name_code: row.category_name_code,
                    category_entity_type: entity_type,
                    category_index: row.category_index,
                    crud_type,
                    description: row.description,
                })
            })
            .collect();

        Ok(result)
    }

    pub async fn get_all_employee_authorizations_by_level_id(&self, level_id: i32) -> Result<Vec<EmployeeAuthorization>, AppError> {
        let authorizations = sqlx::query!(
            r#"
            SELECT eat.pk_employee_authorization_type_id, ea.feature_code as authorization_feature_code, ea.authorization_index as authorization_index, eac.name_code as category_name_code, eac.entity_type as "entity_type: String", eac.category_index, eat.crud_type as "crud_type: String", eat.description
            FROM employee_authorizations ea
            JOIN employee_authorization_categories eac ON ea.fk_employee_authorization_category_id = eac.pk_employee_authorization_category_id
            JOIN employee_authorization_types eat ON ea.pk_employee_authorization_id = eat.fk_employee_authorization_id
            JOIN link_employee_authorization lea ON lea.fk_employee_authorization_type_id = eat.pk_employee_authorization_type_id
            WHERE lea.fk_employee_level_id = $1
            "#,
            level_id
        )
        .fetch_all(&self.pool).await?;

        let result: Vec<EmployeeAuthorization> = authorizations
            .into_iter()
            .filter_map(|row| {
                let entity_type = row.entity_type.parse::<EntityType>().ok()?;
                let crud_type = row.crud_type?.parse::<CrudType>().ok()?;
                
                Some(EmployeeAuthorization {
                    pk_employee_authorization_id: row.pk_employee_authorization_type_id,
                    authorization_feature_code: row.authorization_feature_code,
                    authorization_index: row.authorization_index,
                    category_name_code: row.category_name_code,
                    category_entity_type: entity_type,
                    category_index: row.category_index,
                    crud_type,
                    description: row.description,
                })
            })
            .collect();

        Ok(result)
    }

    pub async fn get_employee_levels_by_employee_id(&self, employee_id: &str) -> Result<Vec<EmployeeLevel>, AppError> {
        let employee_uuid_id: Uuid = Uuid::parse_str(employee_id).expect("Employee ID is not a valid UUID");

        let levels = sqlx::query_as!(
            EmployeeLevel,
            r#"
            SELECT el.pk_employee_level_id, el.level_index, el.level_label
            FROM employee_levels el
            JOIN employee_accreditation_authorizations eaa ON el.pk_employee_level_id = eaa.fk_employee_level_id
            WHERE eaa.fk_recipient_employee_id = $1
            "#,
            employee_uuid_id
        )
        .fetch_all(&self.pool).await?;

        Ok(levels)
    }

    pub async fn get_all_employee_levels(&self) -> Result<Vec<EmployeeLevel>, AppError> {
        let levels = sqlx::query_as!(
            EmployeeLevel,
            r#"
            SELECT pk_employee_level_id, level_index, level_label
            FROM employee_levels
            "#
        )
        .fetch_all(&self.pool).await?;
        
        Ok(levels)
    }

    pub async fn get_employee_level_by_id(&self, level_id: i32) -> Result<EmployeeLevel, AppError> {
        let level = sqlx::query_as!(
            EmployeeLevel,
            r#"
            SELECT pk_employee_level_id, level_index, level_label
            FROM employee_levels
            WHERE pk_employee_level_id = $1
            "#,
            level_id
        )
        .fetch_optional(&self.pool).await?;

        if level.is_none() {
            return Err(AppError::NotFound("Level not found".to_string()));
        }

        Ok(level.unwrap())
    }
}   