use argon2::{password_hash::{rand_core::OsRng, PasswordHasher, SaltString}, Algorithm, Argon2, Params, Version};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{driver::models::{CreateDriverRequest, Driver, GetAllDriversQuery, UpdateDriverRequest}, errors::app_error::AppError};

pub struct DriverService {
    pool: PgPool,
}

impl DriverService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Get all users with filters
    pub async fn get_all_drivers(&self, filters: &GetAllDriversQuery) -> Result<(Vec<Driver>, u64), AppError> {
        let offset = (filters.page - 1) * filters.limit;
        
        // Build the WHERE clause and parameters
        let mut where_conditions = Vec::new();
        let mut param_count = 0;
        
        // Add filters for each field
        if let Some(ref _driver_id) = filters.pk_driver_id {
            param_count += 1;
            where_conditions.push(format!("pk_driver_id = ${}", param_count));
        }
        
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
        
        if let Some(ref _email) = filters.email {
            param_count += 1;
            where_conditions.push(format!("email ILIKE ${}", param_count));
        }
        
        if let Some(ref _phone_number) = filters.phone_number {
            param_count += 1;
            where_conditions.push(format!("phone_number ILIKE ${}", param_count));
        }
        
        if let Some(_is_searchable) = filters.is_searchable {
            param_count += 1;
            where_conditions.push(format!("is_searchable = ${}", param_count));
        }
        
        if let Some(_allow_request) = filters.allow_request_professional_agreement {
            param_count += 1;
            where_conditions.push(format!("allow_request_professional_agreement = ${}", param_count));
        }
        
        if let Some(ref _language) = filters.language {
            param_count += 1;
            where_conditions.push(format!("language = ${}", param_count));
        }
        
        if let Some(has_rest_json) = filters.rest_json {
            if has_rest_json {
                where_conditions.push("rest_json IS NOT NULL AND rest_json != 'null'".to_string());
            } else {
                where_conditions.push("(rest_json IS NULL OR rest_json = 'null')".to_string());
            }
        }
        
        if let Some(is_verified) = filters.verified {
            if is_verified {
                where_conditions.push("verified_at IS NOT NULL".to_string());
            } else {
                where_conditions.push("verified_at IS NULL".to_string());
            }
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
            "SELECT COUNT(*) as count FROM \"drivers\" {}",
            where_clause
        );
        
        let total_count = if filters.pk_driver_id.is_some() || filters.firstname.is_some() || filters.lastname.is_some() || 
                           filters.gender.is_some() || filters.email.is_some() || filters.phone_number.is_some() ||
                           filters.is_searchable.is_some() || filters.allow_request_professional_agreement.is_some() ||
                           filters.language.is_some() {
            // Use dynamic query with parameters
            let mut query = sqlx::query_scalar::<_, i64>(&total_count_query);
            
            if let Some(ref driver_id) = filters.pk_driver_id {
                query = query.bind(driver_id);
            }
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
            if let Some(ref email) = filters.email {
                query = query.bind(format!("%{}%", email));
            }
            if let Some(ref phone_number) = filters.phone_number {
                query = query.bind(format!("%{}%", phone_number));
            }
            if let Some(is_searchable) = filters.is_searchable {
                query = query.bind(is_searchable);
            }
            if let Some(allow_request) = filters.allow_request_professional_agreement {
                query = query.bind(allow_request);
            }
            if let Some(ref language) = filters.language {
                query = query.bind(language);
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
            "SELECT pk_driver_id, firstname, lastname, gender, email, phone_number, is_searchable, allow_request_professional_agreement, language, rest_json, mail_preferences, created_at, verified_at, last_login_at, deactivated_at FROM \"drivers\" {} ORDER BY created_at {} LIMIT ${} OFFSET ${}",
            where_clause,
            order_direction,
            param_count + 1,
            param_count + 2
        );
        
        // Get paginated users with filters
        let drivers = if filters.pk_driver_id.is_some() || filters.firstname.is_some() || filters.lastname.is_some() || 
                     filters.gender.is_some() || filters.email.is_some() || filters.phone_number.is_some() ||
                     filters.is_searchable.is_some() || filters.allow_request_professional_agreement.is_some() ||
                     filters.language.is_some() {
            // Use dynamic query with parameters
            let mut query = sqlx::query_as::<_, Driver>(&select_query);
            
            if let Some(ref driver_id) = filters.pk_driver_id {
                query = query.bind(driver_id);
            }
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
            if let Some(ref email) = filters.email {
                query = query.bind(format!("%{}%", email));
            }
            if let Some(ref phone_number) = filters.phone_number {
                query = query.bind(format!("%{}%", phone_number));
            }
            if let Some(is_searchable) = filters.is_searchable {
                query = query.bind(is_searchable);
            }
            if let Some(allow_request) = filters.allow_request_professional_agreement {
                query = query.bind(allow_request);
            }
            if let Some(ref language) = filters.language {
                query = query.bind(language);
            }
            
            query = query.bind(filters.limit as i64);
            query = query.bind(offset as i64);
            
            query.fetch_all(&self.pool).await?
        } else {
            // No filters, use simple query
            sqlx::query_as::<_, Driver>(&select_query)
                .bind(filters.limit as i64)
                .bind(offset as i64)
                .fetch_all(&self.pool)
                .await?
        };
        
        Ok((drivers, total))
    }

    // Get a user by ID
    pub async fn get_driver_by_id(&self, driver_id: &Uuid) -> Result<Driver, AppError> {
        let driver = sqlx::query_as!(
            Driver,
            "SELECT pk_driver_id, firstname, lastname, gender, email, phone_number, is_searchable, allow_request_professional_agreement, language, rest_json, mail_preferences, created_at, verified_at, last_login_at, deactivated_at FROM \"drivers\" WHERE pk_driver_id = $1",
            driver_id
        )
        .fetch_optional(&self.pool)
        .await;

        match driver {
            Ok(Some(driver)) => Ok(driver),
            Ok(None) => Err(AppError::NotFound("Driver not found".to_string(), "DRIVER_NOT_FOUND".to_string())),
            Err(e) => Err(e.into())
        }
    }

    fn to_title_case(name: &str) -> String {
        name.trim().split(|c: char| c.is_whitespace() || c == '-')
            .map(|word| {
                if word.is_empty() {
                    return String::new();
                }
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str().to_lowercase().as_str(),
                }
            })
            .collect::<Vec<String>>()
            .join("-")
    }

    // Create a new user
    pub async fn create_driver(&self, create_req: &CreateDriverRequest) -> Result<Driver, AppError> {
        let driver_exists = self.email_exists(&create_req.email).await?;
        if driver_exists {
            return Err(AppError::Conflict("A driver with this email already exists".to_string(), "DRIVER_EMAIL_ALREADY_EXISTS".to_string()));
        }

        let salt = SaltString::generate(&mut OsRng);
        let params = Params::new(19 * 1024, 2, 1, None)
            .map_err(|_| AppError::Internal("An error occurred while creating Argon2 parameters".to_string()))?; // 19 MiB, 2 itérations, 1 thread
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

        let password_hash = argon2.hash_password(create_req.password.as_bytes(), &salt)
            .map_err(|_| AppError::Internal("An error occurred while hashing the password".to_string()))
            .map(|hash| hash.to_string())?;

        let driver = sqlx::query_as!(
            Driver,
            r#"
            INSERT INTO "drivers" (firstname, lastname, gender, email, password_hash, phone_number, is_searchable, allow_request_professional_agreement, language, rest_json, mail_preferences, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, NOW())
            RETURNING pk_driver_id, firstname, lastname, gender, email, phone_number, is_searchable, allow_request_professional_agreement, language, rest_json, mail_preferences, created_at, verified_at, last_login_at, deactivated_at
            "#,
            Self::to_title_case(&create_req.firstname),
            Self::to_title_case(&create_req.lastname),
            create_req.gender,
            create_req.email.to_lowercase(),
            password_hash,
            create_req.phone_number,
            create_req.is_searchable,
            create_req.allow_request_professional_agreement,
            create_req.language,
            serde_json::Value::Null,
            32
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(driver)
    }

    // Update a user
    pub async fn update_driver(&self, driver_id: &Uuid, update_req: &UpdateDriverRequest) -> Result<Driver, AppError> {
        // Use a simple approach with separate queries for each field
        if let Some(ref firstname) = update_req.firstname {
            sqlx::query!("UPDATE \"drivers\" SET firstname = $1 WHERE pk_driver_id = $2", Self::to_title_case(firstname), driver_id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(ref lastname) = update_req.lastname {
            sqlx::query!("UPDATE \"drivers\" SET lastname = $1 WHERE pk_driver_id = $2", Self::to_title_case(lastname), driver_id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(ref gender) = update_req.gender {
            sqlx::query!("UPDATE \"drivers\" SET gender = $1 WHERE pk_driver_id = $2", gender, driver_id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(ref email) = update_req.email {
            sqlx::query!("UPDATE \"drivers\" SET email = $1 WHERE pk_driver_id = $2", email.to_lowercase(), driver_id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(ref phone_number) = update_req.phone_number {
            sqlx::query!("UPDATE \"drivers\" SET phone_number = $1 WHERE pk_driver_id = $2", phone_number, driver_id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(is_searchable) = update_req.is_searchable {
            sqlx::query!("UPDATE \"drivers\" SET is_searchable = $1 WHERE pk_driver_id = $2", is_searchable, driver_id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(allow_request_professional_agreement) = update_req.allow_request_professional_agreement {
            sqlx::query!("UPDATE \"drivers\" SET allow_request_professional_agreement = $1 WHERE pk_driver_id = $2", allow_request_professional_agreement, driver_id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(ref language) = update_req.language {
            sqlx::query!("UPDATE \"drivers\" SET language = $1 WHERE pk_driver_id = $2", language, driver_id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(ref rest_json) = update_req.rest_json {
            sqlx::query!("UPDATE \"drivers\" SET rest_json = $1 WHERE pk_driver_id = $2", rest_json, driver_id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(mail_preferences) = update_req.mail_preferences {
            sqlx::query!("UPDATE \"drivers\" SET mail_preferences = $1 WHERE pk_driver_id = $2", mail_preferences, driver_id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(verified_at) = update_req.verified_at {
            sqlx::query!("UPDATE \"drivers\" SET verified_at = $1 WHERE pk_driver_id = $2", verified_at, driver_id)
                .execute(&self.pool)
                .await?;
        }

        // Get the updated user
        self.get_driver_by_id(driver_id).await
    }

    // Deactivate a user (soft delete)
    pub async fn deactivate_driver(&self, driver_id: &Uuid) -> Result<(), AppError> {
        let driver = self.get_driver_by_id(driver_id).await?;
        if driver.deactivated_at.is_some() {
            return Err(AppError::NotFound("Driver has already been deactivated".to_string(), "DRIVER_ALREADY_DEACTIVATED".to_string()));
        }

        let result = sqlx::query!(
            "UPDATE \"drivers\" SET deactivated_at = NOW() WHERE pk_driver_id = $1",
            driver_id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Driver not found".to_string(), "DRIVER_NOT_FOUND".to_string()));
        }

        Ok(())
    }

    pub async fn reactivate_driver(&self, driver_id: &Uuid) -> Result<(), AppError> {
        let driver = self.get_driver_by_id(driver_id).await?;
        if driver.deactivated_at.is_none() {
            return Err(AppError::NotFound("Driver is not deactivated".to_string(), "DRIVER_NOT_DEACTIVATED".to_string()));
        }

        let result = sqlx::query!(
            "UPDATE \"drivers\" SET deactivated_at = NULL WHERE pk_driver_id = $1",
            driver_id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Driver not found".to_string(), "DRIVER_NOT_FOUND".to_string()));
        }

        Ok(())
    }

    // Check if an email exists
    pub async fn email_exists(&self, email: &str) -> Result<bool, AppError> {
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM \"drivers\" WHERE email = $1",
            email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count.count.unwrap_or(0) > 0)
    }

    // Check if an email exists for another user
    pub async fn email_exists_except_driver(&self, email: &str, driver_id: &Uuid) -> Result<bool, AppError> {
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM \"drivers\" WHERE email = $1 AND pk_driver_id != $2",
            email,
            driver_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count.count.unwrap_or(0) > 0)
    }
}
