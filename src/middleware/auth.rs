use axum::{
    extract::{State, Request},
    middleware::Next,
    response::Response,
};
use http::header;
use jsonwebtoken::{decode, DecodingKey, Validation};
use uuid::Uuid;

use crate::{
    models::jwt::Claims,
    errors::app_error::AppError,
};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AuthState {
    pub employee_id: Uuid,
    pub email: String,
    pub permissions: Vec<i32>,
}

pub async fn auth_middleware(
    State((pool, jwt_secret)): State<(PgPool, String)>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Extraire le token du header Authorization
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_str| auth_str.strip_prefix("Bearer "));

    let token = auth_header
        .ok_or_else(|| AppError::Validation("Token d'authentification manquant".to_string()))?;

    // Décoder et valider le token JWT
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Validation("Token JWT invalide".to_string()))?;

    let claims = token_data.claims;

    // Vérifier si le token a expiré
    if claims.is_expired() {
        return Err(AppError::Validation("Token JWT expiré".to_string()));
    }

    // Vérifier que l'employé existe toujours et est actif
    let employee = sqlx::query!(
        "SELECT pk_employee_id FROM employees WHERE pk_employee_id = $1 AND deactivated_at IS NULL",
        claims.sub
    )
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::Validation("Employé non trouvé ou désactivé".to_string()))?;

    // Créer l'état d'authentification
    let auth_state = AuthState {
        employee_id: employee.pk_employee_id,
        email: claims.email,
        permissions: claims.permissions,
    };

    // Ajouter l'état d'authentification à la requête
    request.extensions_mut().insert(auth_state);

    Ok(next.run(request).await)
}