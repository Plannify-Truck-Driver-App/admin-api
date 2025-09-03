use chrono::Utc;
use redis::{aio::ConnectionManager, AsyncCommands};
use sqlx::PgPool;
use bcrypt::{hash, verify, DEFAULT_COST};
use tracing::debug;
use uuid::Uuid;
use jsonwebtoken::{encode, Header, EncodingKey};

use crate::{
    auth::models::{AuthResponse, Claims, RefreshClaims, RefreshTokenRequest}, employee::models::{Employee, EmployeeCreate, EmployeeLoginRequest}, errors::app_error::AppError
};

pub struct AuthService {
    pool: PgPool,
    redis: ConnectionManager,
    jwt_secret: String,
    access_token_duration_minutes: u32,
    refresh_token_duration_minutes: u32,
}

impl AuthService {
    pub fn new(pool: PgPool, redis: ConnectionManager) -> Self {
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be defined");
        let access_token_duration_minutes = std::env::var("ACCESS_TOKEN_DURATION_MINUTES")
            .expect("ACCESS_TOKEN_DURATION_MINUTES must be defined")
            .parse()
            .expect("ACCESS_TOKEN_DURATION_MINUTES must be a valid number");
        let refresh_token_duration_minutes = std::env::var("REFRESH_TOKEN_DURATION_MINUTES")
            .expect("REFRESH_TOKEN_DURATION_MINUTES must be defined")
            .parse()
            .expect("REFRESH_TOKEN_DURATION_MINUTES must be a valid number");
        
        Self {
            pool,
            redis,
            jwt_secret,
            access_token_duration_minutes,
            refresh_token_duration_minutes,
        }
    }

    pub async fn login(&self, login: &EmployeeLoginRequest) -> Result<AuthResponse, AppError> {
        // get employee by professional email
        let employee = sqlx::query_as!(
            Employee,
            r#"
            SELECT 
                pk_employee_id, firstname, lastname, gender, personal_email,
                login_password_hash, phone_number, professional_email,
                professional_email_password, created_at, last_login_at, deactivated_at
            FROM employees 
            WHERE professional_email = $1 AND deactivated_at IS NULL
            "#,
            login.professional_email
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(AppError::Validation("Invalid email or password".to_string()))?;

        // check password
        if !verify(&login.password, &employee.login_password_hash)
            .map_err(|_| AppError::Internal("Invalid email or password".to_string()))? {
            return Err(AppError::Validation("Invalid email or password".to_string()));
        }
        
        // get employee permissions
        let permissions = self.get_employee_permissions(employee.pk_employee_id).await?;
        
        // update last login
        sqlx::query!(
            "UPDATE employees SET last_login_at = $1 WHERE pk_employee_id = $2",
            Utc::now(),
            employee.pk_employee_id
        )
        .execute(&self.pool)
        .await?;
        
        // generate JWT tokens
        let access_token = self.generate_access_token(&employee, &permissions)?;
        let refresh_token = self.generate_refresh_token(&employee)?;
        
        Ok(AuthResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
        })
    }

    pub async fn refresh_token(&self, refresh_req: &RefreshTokenRequest) -> Result<AuthResponse, AppError> {
        // decode and validate refresh token
        let token_data = jsonwebtoken::decode::<RefreshClaims>(
            &refresh_req.refresh_token,
            &jsonwebtoken::DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|_| AppError::Validation("Refresh token invalide".to_string()))?;

        let claims = token_data.claims;

        // check if the token has expired
        if claims.is_expired() {
            return Err(AppError::Validation("Refresh token expiré".to_string()));
        }

        // check if the employee exists and is active
        let employee = sqlx::query_as!(
            Employee,
            r#"
            SELECT 
                pk_employee_id, firstname, lastname, gender, personal_email,
                login_password_hash, phone_number, professional_email,
                professional_email_password, created_at, last_login_at, deactivated_at
            FROM employees 
            WHERE pk_employee_id = $1 AND deactivated_at IS NULL
            "#,
            claims.sub
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(AppError::Validation("Employé non trouvé ou désactivé".to_string()))?;

        // get employee permissions
        let permissions = self.get_employee_permissions(employee.pk_employee_id).await?;
        
        // generate new access token
        let access_token = self.generate_access_token(&employee, &permissions)?;
        let refresh_token = self.generate_refresh_token(&employee)?;
        
        Ok(AuthResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
        })
    }
    
    pub async fn get_employee_permissions(&self, employee_id: Uuid) -> Result<Vec<i32>, AppError> {
        let redis_key = format!("employee:{}:permissions", employee_id);
        let mut conn = self.redis.clone();

        // Lire depuis Redis
        if let Ok(serialized) = conn.get::<_, String>(&redis_key).await {
            debug!("Cached permissions found for employee {}", employee_id);
            if let Ok(permission_ids) = serde_json::from_str::<Vec<i32>>(&serialized) {
                return Ok(permission_ids);
            }
        }

        debug!("Cached permissions not found for employee {}", employee_id);

        let permissions = sqlx::query!(
            r#"
            SELECT authorization_id
            FROM (
                SELECT eat.pk_employee_authorization_type_id as authorization_id
                FROM employee_accreditation_authorizations eaa
                JOIN employee_levels el ON eaa.fk_employee_level_id = el.pk_employee_level_id
                JOIN link_employee_authorization lea ON el.pk_employee_level_id = lea.fk_employee_level_id
                JOIN employee_authorization_types eat ON lea.fk_employee_authorization_type_id = eat.pk_employee_authorization_type_id
                JOIN employee_authorizations ea ON eat.fk_employee_authorization_id = ea.pk_employee_authorization_id
                WHERE eaa.fk_recipient_employee_id = $1
                AND eaa.start_at <= NOW()
                AND (eaa.end_at IS NULL OR eaa.end_at > NOW())

                UNION

                SELECT ead.fk_employee_authorization_type_id as authorization_id
                FROM employee_authorization_derogations ead
                WHERE ead.fk_recipient_employee_id = $1
                AND ead.start_at <= NOW()
                AND ead.end_at > NOW()
            )
            ORDER BY authorization_id
            "#,
            employee_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let permission_ids: Vec<i32> = permissions.iter()
            .filter_map(|p| p.authorization_id)
            .collect();

        let serialized = serde_json::to_string(&permission_ids)?;
        let _: () = conn.set_ex(redis_key, serialized, 60 * 60 * 24).await.map_err(|_| AppError::Internal("Failed to cache permissions in Redis".to_string()))?;

        Ok(permission_ids)
    }
    
    fn generate_access_token(&self, employee: &Employee, permissions: &[i32]) -> Result<String, AppError> {
        let claims = Claims::new(
            employee.pk_employee_id,
            employee.professional_email.clone(),
            employee.firstname.clone(),
            employee.lastname.clone(),
            permissions.to_vec(),
            self.access_token_duration_minutes, // 24 heures
        );
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|_| AppError::Internal("An error occurred while generating the access token".to_string()))?;
        
        Ok(token)
    }

    fn generate_refresh_token(&self, employee: &Employee) -> Result<String, AppError> {
        let claims = RefreshClaims::new(
            employee.pk_employee_id,
            self.refresh_token_duration_minutes, // 7 jours
        );
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|_| AppError::Internal("An error occurred while generating the refresh token".to_string()))?;
        
        Ok(token)
    }
    
    pub async fn create_employee(&self, employee_data: &EmployeeCreate) -> Result<Employee, AppError> {
        // check if the professional email already exists
        let existing = sqlx::query!(
            "SELECT pk_employee_id FROM employees WHERE professional_email = $1",
            employee_data.professional_email
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if existing.is_some() {
            return Err(AppError::Conflict("Un employé avec cet email professionnel existe déjà".to_string(), "EMAIL_EXISTS".to_string()));
        }
        
        // hash password
        let password_hash = hash(employee_data.login_password.as_bytes(), DEFAULT_COST)
            .map_err(|_| AppError::Internal("An error occurred while hashing the password".to_string()))?;
        
        // insert employee
        let employee = sqlx::query_as!(
            Employee,
            r#"
            INSERT INTO employees (
                firstname, lastname, gender, personal_email, login_password_hash,
                phone_number, professional_email, professional_email_password
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING 
                pk_employee_id, firstname, lastname, gender, personal_email,
                login_password_hash, phone_number, professional_email,
                professional_email_password, created_at, last_login_at, deactivated_at
            "#,
            employee_data.firstname,
            employee_data.lastname,
            employee_data.gender,
            employee_data.personal_email,
            password_hash,
            employee_data.phone_number,
            employee_data.professional_email,
            employee_data.professional_email_password
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(employee)
    }
}
