use sqlx::PgPool;
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;
use chrono::Utc;
use jsonwebtoken::{encode, Header, EncodingKey};

use crate::{
    models::{
        employee::{Employee, EmployeeLoginRequest, EmployeeCreate},
        jwt::{Claims, AuthResponse, EmployeeInfo},
    },
    errors::app_error::AppError,
};

pub struct AuthService {
    pool: PgPool,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(pool: PgPool, jwt_secret: String) -> Self {
        Self { pool, jwt_secret }
    }
    
    pub async fn authenticate_employee(&self, login: &EmployeeLoginRequest) -> Result<AuthResponse, AppError> {
        // Récupérer l'employé par email professionnel
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
        .ok_or(AppError::Validation("Email ou mot de passe incorrect".to_string()))?;
        
        // Vérifier le mot de passe
        if !verify(&login.password, &employee.login_password_hash)
            .map_err(|_| AppError::Internal("Email ou mot de passe incorrect".to_string()))? {
            return Err(AppError::Validation("Email ou mot de passe incorrect".to_string()));
        }
        
        // Récupérer les permissions de l'employé
        let permissions = self.get_employee_permissions(employee.pk_employee_id).await?;
        
        // Mettre à jour la dernière connexion
        sqlx::query!(
            "UPDATE employees SET last_login_at = $1 WHERE pk_employee_id = $2",
            Utc::now(),
            employee.pk_employee_id
        )
        .execute(&self.pool)
        .await?;
        
        // Générer le token JWT
        let token = self.generate_jwt(&employee, &permissions)?;
        
        // Construire la réponse
        let employee_info = EmployeeInfo {
            id: employee.pk_employee_id,
            firstname: employee.firstname.clone(),
            lastname: employee.lastname.clone(),
            professional_email: employee.professional_email.clone(),
            permissions: permissions,
        };
        
        Ok(AuthResponse {
            token,
            token_type: "Bearer".to_string(),
            expires_in: 24 * 60 * 60, // 24 heures en secondes
            employee: employee_info,
        })
    }
    
    pub async fn get_employee_permissions(&self, employee_id: Uuid) -> Result<Vec<i32>, AppError> {
        let permissions = sqlx::query!(
            r#"
            SELECT DISTINCT
                eat.pk_employee_authorization_type_id as authorization_id
            FROM employee_accreditation_authorizations eaa
            JOIN employee_levels el ON eaa.fk_employee_level_id = el.pk_employee_level_id
            JOIN link_employee_authorization lea ON el.pk_employee_level_id = lea.fk_employee_level_id
            JOIN employee_authorization_types eat ON lea.fk_employee_authorization_type_id = eat.pk_employee_authorization_type_id
            JOIN employee_authorizations ea ON eat.fk_employee_authorization_id = ea.pk_employee_authorization_id
            WHERE eaa.fk_recipient_employee_id = $1
                AND (eaa.end_at IS NULL OR eaa.end_at > NOW())
                AND eaa.start_at <= NOW()
            ORDER BY eat.pk_employee_authorization_type_id
            "#,
            employee_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let permission_ids: Vec<i32> = permissions.iter().map(|p| p.authorization_id).collect();
        
        Ok(permission_ids)
    }
    
    fn generate_jwt(&self, employee: &Employee, permissions: &[i32]) -> Result<String, AppError> {
        let claims = Claims::new(
            employee.pk_employee_id,
            employee.professional_email.clone(),
            employee.firstname.clone(),
            employee.lastname.clone(),
            permissions.to_vec(),
            24, // 24 heures
        );
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|_| AppError::Internal("An error occurred while generating the JWT".to_string()))?;
        
        Ok(token)
    }
    
    pub async fn create_employee(&self, employee_data: &EmployeeCreate) -> Result<Employee, AppError> {
        // Vérifier que l'email professionnel n'existe pas déjà
        let existing = sqlx::query!(
            "SELECT pk_employee_id FROM employees WHERE professional_email = $1",
            employee_data.professional_email
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if existing.is_some() {
            return Err(AppError::Conflict("Un employé avec cet email professionnel existe déjà".to_string(), "EMAIL_EXISTS".to_string()));
        }
        
        // Hasher le mot de passe
        let password_hash = hash(employee_data.login_password.as_bytes(), DEFAULT_COST)
            .map_err(|_| AppError::Internal("An error occurred while hashing the password".to_string()))?;
        
        // Insérer l'employé
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
