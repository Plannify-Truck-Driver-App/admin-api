use axum::{
    extract::State,
    Json,
};
use std::sync::Arc;

use crate::{
    database::{auth_service::AuthService, driver_service::Database}, errors::app_error::AppError, models::{
        employee::{Employee, EmployeeCreate, EmployeeLoginRequest, EmployeeLoginResponse}
    }
};
use validator::Validate;

pub async fn login(
    State((_db, auth_service)): State<(Arc<Database>, Arc<AuthService>)>,
    Json(login_data): Json<EmployeeLoginRequest>,
) -> Result<Json<EmployeeLoginResponse>, AppError> {
    // Valider les données d'entrée
    login_data.validate()
        .map_err(|e| AppError::Validation(format!("Données de connexion invalides: {}", e)))?;
    
    // Authentifier l'employé
    let auth_response = auth_service.authenticate_employee(&login_data).await?;
    
    Ok(Json(EmployeeLoginResponse {
        token: auth_response.token,
        token_type: auth_response.token_type
    }))
}

pub async fn register(
    State((_db, auth_service)): State<(Arc<Database>, Arc<AuthService>)>,
    Json(employee_data): Json<EmployeeCreate>,
) -> Result<Json<Employee>, AppError> {
    // Valider les données d'entrée
    employee_data.validate()
        .map_err(|e| AppError::Validation(format!("Données d'employé invalides: {}", e)))?;
    
    // Créer l'employé
    let employee = auth_service.create_employee(&employee_data).await?;
    
    Ok(Json(employee))
}