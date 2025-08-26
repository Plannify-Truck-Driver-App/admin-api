use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::app_error::AppError, models::{employee::{CrudType, Employee, EmployeeAccreditation, EmployeeAuthorization, EmployeeLevel, EmployeeLevelWithAuthorizations, EntityType, LightEmployee}, paginate::PaginateQuery}};
use futures::stream::StreamExt;

pub struct EmployeeService {
    pool: PgPool,
}

impl EmployeeService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_employee_by_id(&self, employee_id: &str) -> Result<Employee, AppError> {
        let _employee_uuid_id: Uuid = Uuid::parse_str(employee_id).expect("Employee ID is not a valid UUID");

        sqlx::query_as!(
            Employee,
            r#"SELECT * FROM employees WHERE pk_employee_id = $1"#,
            _employee_uuid_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| AppError::NotFound("Employee not found".to_string()))
    }

    pub async fn get_light_employee_by_id(&self, employee_id: &str) -> Result<LightEmployee, AppError> {
        let _employee_uuid_id: Uuid = Uuid::parse_str(employee_id).expect("Employee ID is not a valid UUID");
    
        sqlx::query_as!(
            LightEmployee,
            r#"SELECT pk_employee_id, firstname, lastname, gender, professional_email FROM employees WHERE pk_employee_id = $1"#,
            _employee_uuid_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| AppError::NotFound("Employee not found".to_string()))
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

    pub async fn get_employee_level_with_authorizations_by_id(&self, level_id: i32) -> Result<EmployeeLevelWithAuthorizations, AppError> {
        let level = self.get_employee_level_by_id(level_id).await?;
        let authorizations = self.get_all_employee_authorizations_by_level_id(level_id).await?;
        Ok(EmployeeLevelWithAuthorizations {
            pk_employee_level_id: level.pk_employee_level_id,
            level_index: level.level_index,
            level_label: level.level_label,
            authorizations,
        })
    }

    pub async fn get_all_employee_accreditations(&self, filters: &PaginateQuery) -> Result<(Vec<EmployeeAccreditation>, u64), AppError> {
        let total_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) as count FROM employee_accreditation_authorizations")
                .fetch_one(&self.pool)
                .await? as u64;

        struct EmployeeAccreditationRow {
            fk_recipient_employee_id: Uuid,
            fk_employee_level_id: i32,
            fk_authorizing_employee_id: Option<Uuid>,
            start_at: DateTime<Utc>,
            end_at: Option<DateTime<Utc>>,
            created_at: DateTime<Utc>,
        }

        let accreditations_row = sqlx::query_as!(
            EmployeeAccreditationRow,
            r#"
            SELECT
                eaa.fk_recipient_employee_id,
                eaa.fk_employee_level_id,
                eaa.fk_authorizing_employee_id,
                eaa.start_at,
                eaa.end_at,
                eaa.created_at
            FROM employee_accreditation_authorizations eaa
            ORDER BY created_at ASC
            LIMIT $1
            OFFSET $2
            "#,
            filters.limit as i64,
            ((filters.page - 1) * filters.limit) as i64
        )
        .fetch_all(&self.pool).await?;

        let accreditations = futures::stream::iter(accreditations_row)
            .filter_map(|row| {
                let fk_recipient_employee_id = row.fk_recipient_employee_id;
                let fk_employee_level_id = row.fk_employee_level_id;
                let fk_authorizing_employee_id = row.fk_authorizing_employee_id;
                let start_at = row.start_at;
                let end_at = row.end_at;
                let created_at = row.created_at;

                async move {
                    let recipient_employee = match self.get_light_employee_by_id(&fk_recipient_employee_id.to_string()).await.ok() {
                        Some(emp) => {
                            emp
                        },
                        None => {
                            return None;
                        },
                    };
                    
                    let employee_level = match self.get_employee_level_by_id(fk_employee_level_id).await.ok() {
                        Some(level) => {
                            level
                        },
                        None => {
                            return None;
                        },
                    };
                    
                    let authorizing_employee: Option<LightEmployee> = match fk_authorizing_employee_id {
                        Some(authorizing_id) => {
                            match self.get_light_employee_by_id(&authorizing_id.to_string()).await.ok() {
                                Some(emp) => {
                                    Some(emp)
                                },
                                None => {
                                    return None;
                                },
                            }
                        },
                        None => {
                            None
                        },
                    };

                    Some(EmployeeAccreditation {
                        recipient_employee,
                        employee_level,
                        authorizing_employee,
                        start_at,
                        end_at,
                        created_at,
                    })
                }
            })
            .collect::<Vec<_>>()
            .await;

        Ok((accreditations, total_count))
    }

    pub async fn get_employee_accreditations_by_employee_id(&self, employee_id: &str, filters: &PaginateQuery) -> Result<(Vec<EmployeeAccreditation>, u64), AppError> {
        let employee_uuid_id: Uuid = Uuid::parse_str(employee_id).expect("Employee ID is not a valid UUID");

        let total_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) as count FROM employee_accreditation_authorizations WHERE fk_recipient_employee_id = $1")
                .bind(employee_uuid_id)
                .fetch_one(&self.pool)
                .await? as u64;

        struct EmployeeAccreditationRow {
            fk_recipient_employee_id: Uuid,
            fk_employee_level_id: i32,
            fk_authorizing_employee_id: Option<Uuid>,
            start_at: DateTime<Utc>,
            end_at: Option<DateTime<Utc>>,
            created_at: DateTime<Utc>,
        }

        let accreditations_row = sqlx::query_as!(
            EmployeeAccreditationRow,
            r#"
            SELECT
                eaa.fk_recipient_employee_id,
                eaa.fk_employee_level_id,
                eaa.fk_authorizing_employee_id,
                eaa.start_at,
                eaa.end_at,
                eaa.created_at
            FROM employee_accreditation_authorizations eaa
            WHERE eaa.fk_recipient_employee_id = $1
            ORDER BY created_at ASC
            LIMIT $2
            OFFSET $3
            "#,
            employee_uuid_id,
            filters.limit as i64,
            ((filters.page - 1) * filters.limit) as i64
        )
        .fetch_all(&self.pool).await?;

        let accreditations = futures::stream::iter(accreditations_row)
            .filter_map(|row| {
                let fk_recipient_employee_id = row.fk_recipient_employee_id;
                let fk_employee_level_id = row.fk_employee_level_id;
                let fk_authorizing_employee_id = row.fk_authorizing_employee_id;
                let start_at = row.start_at;
                let end_at = row.end_at;
                let created_at = row.created_at;

                async move {
                    let recipient_employee = match self.get_light_employee_by_id(&fk_recipient_employee_id.to_string()).await.ok() {
                        Some(emp) => {
                            emp
                        },
                        None => {
                            return None;
                        },
                    };
                    
                    let employee_level = match self.get_employee_level_by_id(fk_employee_level_id).await.ok() {
                        Some(level) => {
                            level
                        },
                        None => {
                            return None;
                        },
                    };
                    
                    let authorizing_employee: Option<LightEmployee> = match fk_authorizing_employee_id {
                        Some(authorizing_id) => {
                            match self.get_light_employee_by_id(&authorizing_id.to_string()).await.ok() {
                                Some(emp) => {
                                    Some(emp)
                                },
                                None => {
                                    return None;
                                },
                            }
                        },
                        None => {
                            None
                        },
                    };

                    Some(EmployeeAccreditation {
                        recipient_employee,
                        employee_level,
                        authorizing_employee,
                        start_at,
                        end_at,
                        created_at,
                    })
                }
            })
            .collect::<Vec<_>>()
            .await;

        Ok((accreditations, total_count))
    }
}