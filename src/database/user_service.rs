use sqlx::PgPool;
use anyhow::Result;
use bcrypt::{hash, DEFAULT_COST};
use uuid::Uuid;

use crate::models::user::{User, UpdateUserRequest, CreateUserRequest, GetAllUsersQuery};

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Get all users with filters
    pub async fn get_all_users(&self, filters: &GetAllUsersQuery) -> Result<(Vec<User>, u64)> {
        let offset = (filters.page - 1) * filters.limit;
        
        // Build the WHERE clause and parameters
        let mut where_conditions = Vec::new();
        let mut param_count = 0;
        
        // Add filters for each field
        if let Some(ref _user_id) = filters.pk_user_id {
            param_count += 1;
            where_conditions.push(format!("pk_user_id = ${}", param_count));
        }
        
        if let Some(ref _firstname) = filters.firstname {
            param_count += 1;
            where_conditions.push(format!("firstname ILIKE ${}", param_count));
        }
        
        if let Some(ref _lastname) = filters.lastname {
            param_count += 1;
            where_conditions.push(format!("lastname ILIKE ${}", param_count));
        }
        
        if let Some(ref _gender) = filters.gender {
            param_count += 1;
            where_conditions.push(format!("gender = ${}", param_count));
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
            "SELECT COUNT(*) as count FROM \"user\" {}",
            where_clause
        );
        
        let total_count = if filters.pk_user_id.is_some() || filters.firstname.is_some() || filters.lastname.is_some() || 
                           filters.gender.is_some() || filters.email.is_some() || filters.phone_number.is_some() ||
                           filters.is_searchable.is_some() || filters.allow_request_professional_agreement.is_some() ||
                           filters.language.is_some() {
            // Use dynamic query with parameters
            let mut query = sqlx::query_scalar::<_, i64>(&total_count_query);
            
            if let Some(ref user_id) = filters.pk_user_id {
                query = query.bind(user_id);
            }
            if let Some(ref firstname) = filters.firstname {
                query = query.bind(format!("%{}%", firstname));
            }
            if let Some(ref lastname) = filters.lastname {
                query = query.bind(format!("%{}%", lastname));
            }
            if let Some(ref gender) = filters.gender {
                query = query.bind(gender);
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
            "SELECT pk_user_id, firstname, lastname, gender, email, phone_number, is_searchable, allow_request_professional_agreement, language, rest_json, mail_preferences, created_at, verified_at, last_login_at, deactivated_at FROM \"user\" {} ORDER BY created_at {} LIMIT ${} OFFSET ${}",
            where_clause,
            order_direction,
            param_count + 1,
            param_count + 2
        );
        
        // Get paginated users with filters
        let users = if filters.pk_user_id.is_some() || filters.firstname.is_some() || filters.lastname.is_some() || 
                     filters.gender.is_some() || filters.email.is_some() || filters.phone_number.is_some() ||
                     filters.is_searchable.is_some() || filters.allow_request_professional_agreement.is_some() ||
                     filters.language.is_some() {
            // Use dynamic query with parameters
            let mut query = sqlx::query_as::<_, User>(&select_query);
            
            if let Some(ref user_id) = filters.pk_user_id {
                query = query.bind(user_id);
            }
            if let Some(ref firstname) = filters.firstname {
                query = query.bind(format!("%{}%", firstname));
            }
            if let Some(ref lastname) = filters.lastname {
                query = query.bind(format!("%{}%", lastname));
            }
            if let Some(ref gender) = filters.gender {
                query = query.bind(gender);
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
            sqlx::query_as::<_, User>(&select_query)
                .bind(filters.limit as i64)
                .bind(offset as i64)
                .fetch_all(&self.pool)
                .await?
        };
        
        Ok((users, total))
    }

    // Get a user by ID
    pub async fn get_user_by_id(&self, user_id: &Uuid) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            "SELECT pk_user_id, firstname, lastname, gender, email, phone_number, is_searchable, allow_request_professional_agreement, language, rest_json, mail_preferences, created_at, verified_at, last_login_at, deactivated_at FROM \"user\" WHERE pk_user_id = $1",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        user.ok_or_else(|| anyhow::anyhow!("User not found"))
    }

    // Create a new user
    pub async fn create_user(&self, create_req: &CreateUserRequest) -> Result<User> {
        let user_id = Uuid::new_v4();
        let password_hash = hash(&create_req.password, DEFAULT_COST)?;

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO "user" (pk_user_id, firstname, lastname, gender, email, password_hash, phone_number, is_searchable, allow_request_professional_agreement, language, rest_json, mail_preferences, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, NOW())
            RETURNING pk_user_id, firstname, lastname, gender, email, phone_number, is_searchable, allow_request_professional_agreement, language, rest_json, mail_preferences, created_at, verified_at, last_login_at, deactivated_at
            "#,
            user_id,
            create_req.firstname,
            create_req.lastname,
            create_req.gender,
            create_req.email,
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

        Ok(user)
    }

    // Update a user
    pub async fn update_user(&self, user_id: &Uuid, update_req: &UpdateUserRequest) -> Result<User> {
        // Use a simple approach with separate queries for each field
        if let Some(ref firstname) = update_req.firstname {
            sqlx::query!("UPDATE \"user\" SET firstname = $1 WHERE pk_user_id = $2", firstname, user_id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(ref lastname) = update_req.lastname {
            sqlx::query!("UPDATE \"user\" SET lastname = $1 WHERE pk_user_id = $2", lastname, user_id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(ref gender) = update_req.gender {
            sqlx::query!("UPDATE \"user\" SET gender = $1 WHERE pk_user_id = $2", gender, user_id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(ref email) = update_req.email {
            sqlx::query!("UPDATE \"user\" SET email = $1 WHERE pk_user_id = $2", email, user_id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(ref phone_number) = update_req.phone_number {
            sqlx::query!("UPDATE \"user\" SET phone_number = $1 WHERE pk_user_id = $2", phone_number, user_id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(is_searchable) = update_req.is_searchable {
            sqlx::query!("UPDATE \"user\" SET is_searchable = $1 WHERE pk_user_id = $2", is_searchable, user_id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(allow_request_professional_agreement) = update_req.allow_request_professional_agreement {
            sqlx::query!("UPDATE \"user\" SET allow_request_professional_agreement = $1 WHERE pk_user_id = $2", allow_request_professional_agreement, user_id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(ref language) = update_req.language {
            sqlx::query!("UPDATE \"user\" SET language = $1 WHERE pk_user_id = $2", language, user_id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(ref rest_json) = update_req.rest_json {
            sqlx::query!("UPDATE \"user\" SET rest_json = $1 WHERE pk_user_id = $2", rest_json, user_id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(mail_preferences) = update_req.mail_preferences {
            sqlx::query!("UPDATE \"user\" SET mail_preferences = $1 WHERE pk_user_id = $2", mail_preferences, user_id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(verified_at) = update_req.verified_at {
            sqlx::query!("UPDATE \"user\" SET verified_at = $1 WHERE pk_user_id = $2", verified_at, user_id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(last_login_at) = update_req.last_login_at {
            sqlx::query!("UPDATE \"user\" SET last_login_at = $1 WHERE pk_user_id = $2", last_login_at, user_id)
                .execute(&self.pool)
                .await?;
        }

        if let Some(deactivated_at) = update_req.deactivated_at {
            sqlx::query!("UPDATE \"user\" SET deactivated_at = $1 WHERE pk_user_id = $2", deactivated_at, user_id)
                .execute(&self.pool)
                .await?;
        }

        // Get the updated user
        self.get_user_by_id(user_id).await
    }

    // Delete a user (soft delete)
    pub async fn delete_user(&self, user_id: &Uuid) -> Result<()> {
        let result = sqlx::query!(
            "UPDATE \"user\" SET deactivated_at = NOW() WHERE pk_user_id = $1 AND deactivated_at IS NULL",
            user_id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("User not found or already deleted"));
        }

        Ok(())
    }

    // Check if an email exists
    pub async fn email_exists(&self, email: &str) -> Result<bool> {
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM \"user\" WHERE email = $1 AND deactivated_at IS NULL",
            email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count.count.unwrap_or(0) > 0)
    }

    // Check if an email exists for another user
    pub async fn email_exists_except_user(&self, email: &str, user_id: &Uuid) -> Result<bool> {
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM \"user\" WHERE email = $1 AND pk_user_id != $2 AND deactivated_at IS NULL",
            email,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count.count.unwrap_or(0) > 0)
    }

    // Authenticate a user by email and password
    // pub async fn authenticate_user(&self, email: &str, password: &str) -> Result<Option<User>> {
    //     // First, get the user with the password hash
    //     let user_with_password = sqlx::query!(
    //         r#"
    //         SELECT pk_user_id, firstname, lastname, gender, email, phone_number, is_searchable, 
    //                allow_request_professional_agreement, language, rest_json, mail_preferences, 
    //                created_at, verified_at, last_login_at, deactivated_at, password_hash
    //         FROM "user" 
    //         WHERE email = $1 AND deactivated_at IS NULL
    //         "#,
    //         email
    //     )
    //     .fetch_optional(&self.pool)
    //     .await?;

    //     if let Some(user_data) = user_with_password {
    //         // Check the password
    //         if bcrypt::verify(password, &user_data.password_hash)? {
    //             // Update last_login_at
    //             sqlx::query!(
    //                 "UPDATE \"user\" SET last_login_at = NOW() WHERE pk_user_id = $1",
    //                 user_data.pk_user_id
    //             )
    //             .execute(&self.pool)
    //             .await?;

    //             // Build the User object
    //             let user = User {
    //                 pk_user_id: user_data.pk_user_id,
    //                 firstname: user_data.firstname,
    //                 lastname: user_data.lastname,
    //                 gender: user_data.gender,
    //                 email: user_data.email,
    //                 phone_number: user_data.phone_number,
    //                 is_searchable: user_data.is_searchable,
    //                 allow_request_professional_agreement: user_data.allow_request_professional_agreement,
    //                 language: user_data.language,
    //                 rest_json: user_data.rest_json,
    //                 mail_preferences: user_data.mail_preferences,
    //                 created_at: user_data.created_at,
    //                 verified_at: user_data.verified_at,
    //                 last_login_at: Some(Utc::now()),
    //                 deactivated_at: user_data.deactivated_at,
    //             };

    //             Ok(Some(user))
    //         } else {
    //             Ok(None)
    //         }
    //     } else {
    //         Ok(None)
    //     }
    // }
}
