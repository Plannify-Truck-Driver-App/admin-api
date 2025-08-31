use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{employee::models::{AssignAccreditationRequest, CrudType, Employee, EmployeeAccreditation, EmployeeAccreditationRow, EmployeeAuthorization, EmployeeLevel, EmployeeLevelWithAuthorizations, EntityType, GetAllEmployeesQuery, LightEmployee, UpdateAccreditationRequest}, errors::app_error::AppError, models::paginate::{PaginateQuery, PaginatedResponse, PaginationInfo}};
use futures::stream::StreamExt;

pub struct EmployeeService {
    pool: PgPool,
}

impl EmployeeService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all_employees(&self, filters: &GetAllEmployeesQuery) -> Result<PaginatedResponse<Employee>, AppError> {
        let offset = (filters.page - 1) * filters.limit;
        
        // Build the WHERE clause and parameters
        let mut where_conditions = Vec::new();
        let mut param_count = 0;
        
        if let Some(ref _firstname) = filters.firstname {
            param_count += 1;
            where_conditions.push(format!("firstname ILIKE ${}", param_count));
        }
        
        if let Some(ref _lastname) = filters.lastname {
            param_count += 1;
            where_conditions.push(format!("lastname ILIKE ${}", param_count));
        }
        
        if let Some(ref gender) = filters.gender {
            if gender == "none" {
                where_conditions.push("gender IS NULL".to_string());
            } else {
                param_count += 1;
                where_conditions.push(format!("gender = ${}", param_count));
            }
        }
        
        if let Some(ref _personal_email) = filters.personal_email {
            param_count += 1;
            where_conditions.push(format!("personal_email ILIKE ${}", param_count));
        }
        
        if let Some(ref _phone_number) = filters.phone_number {
            param_count += 1;
            where_conditions.push(format!("phone_number ILIKE ${}", param_count));
        }
        
        if let Some(ref _professional_email) = filters.professional_email {
            param_count += 1;
            where_conditions.push(format!("professional_email ILIKE ${}", param_count));
        }
        
        if let Some(is_deactivated) = filters.deactivated {
            if is_deactivated {
                where_conditions.push("deactivated_at IS NOT NULL".to_string());
            } else {
                where_conditions.push("deactivated_at IS NULL".to_string());
            }
        }
        
        // Build the complete WHERE clause
        let where_clause = if where_conditions.is_empty() {
            "".to_string()
        } else {
            let clause = where_conditions.join(" AND ");
            format!("WHERE {}", clause)
        };
        
        // Get total count with filters
        let total_count_query = format!(
            "SELECT COUNT(*) as count FROM \"employees\" {}",
            where_clause
        );

        let total_count = if filters.firstname.is_some() || filters.lastname.is_some() || 
                           filters.gender.is_some() || filters.personal_email.is_some() || filters.phone_number.is_some() ||
                           filters.professional_email.is_some() {
            // Use dynamic query with parameters
            let mut query = sqlx::query_scalar::<_, i64>(&total_count_query);

            if let Some(ref firstname) = filters.firstname {
                query = query.bind(format!("%{}%", firstname));
            }
            if let Some(ref lastname) = filters.lastname {
                query = query.bind(format!("%{}%", lastname));
            }
            if let Some(ref gender) = filters.gender {
                if gender != "none" {
                    query = query.bind(gender);
                }
            }
            if let Some(ref personal_email) = filters.personal_email {
                query = query.bind(format!("%{}%", personal_email));
            }
            if let Some(ref phone_number) = filters.phone_number {
                query = query.bind(format!("%{}%", phone_number));
            }
            if let Some(ref professional_email) = filters.professional_email {
                query = query.bind(format!("%{}%", professional_email));
            }
            
            query.fetch_one(&self.pool).await?
        } else {
            // No filters, use simple query
            sqlx::query_scalar::<_, i64>(&total_count_query)
                .fetch_one(&self.pool)
                .await?
        };
        
        let total = total_count as u64;
        
        // Determine sort order (default to ASC for chronological order)
        let order_direction = match filters.sort_order.to_lowercase().as_str() {
            "desc" => "DESC",
            _ => "ASC", // default to ASC (chronological order)
        };
        
        // Build the SELECT query with filters
        let select_query = format!(
            "SELECT pk_employee_id, firstname, lastname, gender, personal_email, login_password_hash, phone_number, professional_email, professional_email_password, created_at, last_login_at, deactivated_at FROM \"employees\" {} ORDER BY created_at {} LIMIT ${} OFFSET ${}",
            where_clause,
            order_direction,
            param_count + 1,
            param_count + 2
        );
        
        // Get paginated users with filters
        let employees = if filters.firstname.is_some() || filters.lastname.is_some() || 
                     filters.gender.is_some() || filters.personal_email.is_some() || filters.phone_number.is_some() ||
                     filters.professional_email.is_some() || filters.deactivated.is_some() {
            // Use dynamic query with parameters
            let mut query = sqlx::query_as::<_, Employee>(&select_query);

            if let Some(ref firstname) = filters.firstname {
                query = query.bind(format!("%{}%", firstname));
            }
            if let Some(ref lastname) = filters.lastname {
                query = query.bind(format!("%{}%", lastname));
            }
            if let Some(ref gender) = filters.gender {
                if gender != "none" {
                    query = query.bind(gender);
                }
            }
            if let Some(ref personal_email) = filters.personal_email {
                query = query.bind(format!("%{}%", personal_email));
            }
            if let Some(ref phone_number) = filters.phone_number {
                query = query.bind(format!("%{}%", phone_number));
            }
            if let Some(ref professional_email) = filters.professional_email {
                query = query.bind(format!("%{}%", professional_email));
            }
            
            query = query.bind(filters.limit as i64);
            query = query.bind(offset as i64);
            
            query.fetch_all(&self.pool).await?
        } else {
            // No filters, use simple query
            sqlx::query_as::<_, Employee>(&select_query)
                .bind(filters.limit as i64)
                .bind(offset as i64)
                .fetch_all(&self.pool)
                .await?
        };

        let result = PaginatedResponse {
            data: employees,
            pagination: PaginationInfo {
                page: filters.page,
                limit: filters.limit,
                total,
            },
        };

        Ok(result)
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

    pub async fn get_current_employee_level_by_employee_id(&self, employee_id: &str) -> Result<EmployeeLevel, AppError> {
        let employee_uuid_id: Uuid = Uuid::parse_str(employee_id).expect("Employee ID is not a valid UUID");

        let levels = sqlx::query_as!(
            EmployeeLevel,
            r#"
            SELECT el.pk_employee_level_id, el.level_index, el.level_label
            FROM employee_levels el
            JOIN employee_accreditation_authorizations eaa ON el.pk_employee_level_id = eaa.fk_employee_level_id
            WHERE eaa.fk_recipient_employee_id = $1
            AND eaa.start_at <= NOW()
            AND (eaa.end_at IS NULL OR eaa.end_at > NOW())
            "#,
            employee_uuid_id
        )
        .fetch_all(&self.pool).await?;

        if levels.is_empty() {
            return Err(AppError::NotFound("No current employee level found".to_string()));
        }

        Ok(levels.into_iter().next().unwrap())
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

        let accreditations_row = sqlx::query_as!(
            EmployeeAccreditationRow,
            r#"
            SELECT
                eaa.pk_employee_accreditation_authorization_id,
                eaa.fk_recipient_employee_id,
                eaa.fk_employee_level_id,
                eaa.fk_authorizing_employee_id,
                eaa.start_at,
                eaa.end_at,
                eaa.created_at
            FROM employee_accreditation_authorizations eaa
            ORDER BY eaa.start_at ASC
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
                        accreditation_id: row.pk_employee_accreditation_authorization_id,
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

    pub async fn get_all_employee_accreditations_from(&self, employee_id: &str, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Result<Vec<EmployeeAccreditation>, AppError> {
        let employee_uuid_id: Uuid = Uuid::parse_str(employee_id).expect("Employee ID is not a valid UUID");

        let accreditations_row = sqlx::query_as!(
            EmployeeAccreditationRow,
            r#"
            SELECT
                eaa.pk_employee_accreditation_authorization_id,
                eaa.fk_recipient_employee_id,
                eaa.fk_employee_level_id,
                eaa.fk_authorizing_employee_id,
                eaa.start_at,
                eaa.end_at,
                eaa.created_at
            FROM employee_accreditation_authorizations eaa
            WHERE eaa.fk_recipient_employee_id = $1
            AND eaa.start_at <= $3
            AND (eaa.end_at >= $2 OR eaa.end_at IS NULL)
            ORDER BY eaa.start_at ASC
            "#,
            employee_uuid_id,
            start_date,
            end_date
        )
        .fetch_all(&self.pool)
        .await?;

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
                        accreditation_id: row.pk_employee_accreditation_authorization_id,
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

        Ok(accreditations)
    }

    pub async fn assign_accreditation(&self, assign_req: &AssignAccreditationRequest, employee_authorizing_id: &str) -> Result<EmployeeAccreditation, AppError> {
        let employee_uuid_id: Uuid = assign_req.employee_id;

        if assign_req.start_at < (Utc::now() - chrono::Duration::minutes(5)) {
            return Err(AppError::Validation("The start date must be in the future.".to_string()));
        }

        if assign_req.end_at.is_some() && assign_req.end_at.unwrap() <= assign_req.start_at {
            return Err(AppError::Validation("The end date must be after the start date.".to_string()));
        }
        
        let employee = self.get_light_employee_by_id(&employee_uuid_id.to_string()).await?;
        
        let level = self.get_employee_level_by_id(assign_req.level_id).await?;

        let authorizing_employee = self.get_light_employee_by_id(employee_authorizing_id).await?;
        let authorizing_employee_level = self.get_current_employee_level_by_employee_id(employee_authorizing_id).await?;

        if level.level_index >= authorizing_employee_level.level_index && authorizing_employee_level.level_index != 1 {
            return Err(AppError::Forbidden("You can't assign a higher or equal level than your own.".to_string()));
        }

        let existing_accreditations = self.get_all_employee_accreditations_from(&employee_uuid_id.to_string(), assign_req.start_at, assign_req.end_at.unwrap_or(DateTime::<Utc>::MAX_UTC)).await?;
        if existing_accreditations.len() > 0 {
            return Err(AppError::Conflict("An accreditation already exists for this employee in the specified time range.".to_string(), "ACCREDITATION_ALREADY_EXISTS_FOR_THIS_PERIOD".to_string()));
        }

        let accreditation_row = sqlx::query_as!(
            EmployeeAccreditationRow,
            r#"
            INSERT INTO employee_accreditation_authorizations (fk_recipient_employee_id, fk_employee_level_id, fk_authorizing_employee_id, start_at, end_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING pk_employee_accreditation_authorization_id, fk_recipient_employee_id, fk_employee_level_id, fk_authorizing_employee_id, start_at, end_at, created_at
            "#,
            employee_uuid_id,
            assign_req.level_id,
            Uuid::parse_str(employee_authorizing_id).expect("Authorizing Employee ID is not a valid UUID"),
            assign_req.start_at,
            assign_req.end_at
        )
        .fetch_one(&self.pool)
        .await?;

        let accreditation = EmployeeAccreditation {
            accreditation_id: accreditation_row.pk_employee_accreditation_authorization_id,
            recipient_employee: employee,
            employee_level: level,
            authorizing_employee: Some(authorizing_employee),
            start_at: accreditation_row.start_at,
            end_at: accreditation_row.end_at,
            created_at: accreditation_row.created_at,
        };

        Ok(accreditation)
    }

    pub async fn get_employee_accreditation_by_id(&self, accreditation_id: &Uuid) -> Result<EmployeeAccreditation, AppError> {
        let accreditation_row = sqlx::query_as!(
            EmployeeAccreditationRow,
            r#"
            SELECT pk_employee_accreditation_authorization_id, fk_recipient_employee_id, fk_employee_level_id, fk_authorizing_employee_id, start_at, end_at, created_at
            FROM employee_accreditation_authorizations
            WHERE pk_employee_accreditation_authorization_id = $1
            "#,
            accreditation_id
        )
        .fetch_one(&self.pool)
        .await?;

        let employee = self.get_light_employee_by_id(&accreditation_row.fk_recipient_employee_id.to_string()).await?;
        let level = self.get_employee_level_by_id(accreditation_row.fk_employee_level_id).await?;
        let authorizing_employee = match accreditation_row.fk_authorizing_employee_id {
            Some(authorizing_id) => Some(self.get_light_employee_by_id(&authorizing_id.to_string()).await?),
            None => None,
        };

        let accreditation = EmployeeAccreditation {
            accreditation_id: accreditation_row.pk_employee_accreditation_authorization_id,
            recipient_employee: employee,
            employee_level: level,
            authorizing_employee,
            start_at: accreditation_row.start_at,
            end_at: accreditation_row.end_at,
            created_at: accreditation_row.created_at,
        };

        Ok(accreditation)
    }

    pub async fn update_accreditation(&self, accreditation_id: &Uuid, update_req: &UpdateAccreditationRequest, employee_authorizing_id: &str) -> Result<EmployeeAccreditation, AppError> {
        if update_req.start_at < (Utc::now() - chrono::Duration::minutes(5)) {
            return Err(AppError::Validation("The start date must be in the future.".to_string()));
        }

        if update_req.end_at.is_some() && update_req.end_at.unwrap() <= update_req.start_at {
            return Err(AppError::Validation("The end date must be after the start date.".to_string()));
        }
        
        let level = self.get_employee_level_by_id(update_req.level_id).await?;

        let authorizing_employee_level = self.get_current_employee_level_by_employee_id(employee_authorizing_id).await?;

        if level.level_index >= authorizing_employee_level.level_index && authorizing_employee_level.level_index != 1 {
            return Err(AppError::Forbidden("You can't assign a higher or equal level than your own.".to_string()));
        }

        let accreditation = self.get_employee_accreditation_by_id(accreditation_id).await?;

        let existing_accreditations = self.get_all_employee_accreditations_from(&accreditation.recipient_employee.pk_employee_id.to_string(), update_req.start_at, update_req.end_at.unwrap_or(DateTime::<Utc>::MAX_UTC)).await?;
        let existing_accreditations: Vec<EmployeeAccreditation> = existing_accreditations.into_iter().filter(|acc| &acc.accreditation_id != accreditation_id).collect();
        if existing_accreditations.len() > 0 {
            return Err(AppError::Conflict("An accreditation already exists for this employee in the specified time range.".to_string(), "ACCREDITATION_ALREADY_EXISTS_FOR_THIS_PERIOD".to_string()));
        }

        let accreditation_row = sqlx::query_as!(
            EmployeeAccreditationRow,
            r#"
            UPDATE employee_accreditation_authorizations
            SET fk_employee_level_id = $1, start_at = $2, end_at = $3
            WHERE pk_employee_accreditation_authorization_id = $4
            RETURNING pk_employee_accreditation_authorization_id, fk_recipient_employee_id, fk_employee_level_id, fk_authorizing_employee_id, start_at, end_at, created_at
            "#,
            update_req.level_id,
            update_req.start_at,
            update_req.end_at,
            accreditation_id
        )
        .fetch_one(&self.pool)
        .await?;

        let accreditation_updated = EmployeeAccreditation {
            accreditation_id: accreditation_row.pk_employee_accreditation_authorization_id,
            recipient_employee: self.get_light_employee_by_id(&accreditation_row.fk_recipient_employee_id.to_string()).await?,
            employee_level: self.get_employee_level_by_id(accreditation_row.fk_employee_level_id).await?,
            authorizing_employee: match accreditation_row.fk_authorizing_employee_id {
                Some(authorizing_id) => Some(self.get_light_employee_by_id(&authorizing_id.to_string()).await?),
                None => None,
            },
            start_at: accreditation_row.start_at,
            end_at: accreditation_row.end_at,
            created_at: accreditation_row.created_at,
        };

        Ok(accreditation_updated)
    }

    pub async fn delete_accreditation(&self, accreditation_id: &Uuid, employee_id: &str) -> Result<(), AppError> {
        let accreditation = self.get_employee_accreditation_by_id(accreditation_id).await?;
        let employee_level = self.get_current_employee_level_by_employee_id(employee_id).await?;

        if employee_level.level_index >= accreditation.employee_level.level_index {
            return Err(AppError::Forbidden("You can't delete an accreditation with a higher or equal level than your own.".to_string()));
        }

        let result = sqlx::query!(
            r#"
            DELETE FROM employee_accreditation_authorizations
            WHERE pk_employee_accreditation_authorization_id = $1
            "#,
            accreditation_id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Accreditation not found".to_string()));
        }

        Ok(())
    }

    pub async fn get_employee_accreditations_by_employee_id(&self, employee_id: &str, filters: &PaginateQuery) -> Result<(Vec<EmployeeAccreditation>, u64), AppError> {
        let employee_uuid_id: Uuid = Uuid::parse_str(employee_id).expect("Employee ID is not a valid UUID");

        let total_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) as count FROM employee_accreditation_authorizations WHERE fk_recipient_employee_id = $1")
                .bind(employee_uuid_id)
                .fetch_one(&self.pool)
                .await? as u64;

        let accreditations_row = sqlx::query_as!(
            EmployeeAccreditationRow,
            r#"
            SELECT
                eaa.pk_employee_accreditation_authorization_id,
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
                        accreditation_id: row.pk_employee_accreditation_authorization_id,
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