use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

use crate::{
    middleware::AuthState,
    errors::app_error::AppError,
};

/// Middleware pour vérifier que l'employé a les permissions requises
pub async fn require_permissions(
    required_permissions: Vec<i32>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Récupérer l'état d'authentification depuis les extensions
    let auth_state = request
        .extensions()
        .get::<AuthState>()
        .ok_or_else(|| AppError::Validation("Authentification requise".to_string()))?;

    // Vérifier que l'employé a toutes les permissions requises
    let has_all_permissions = required_permissions
        .iter()
        .all(|&required| auth_state.permissions.contains(&required));

    if !has_all_permissions {
        return Err(AppError::Validation(format!(
            "Permissions insuffisantes. Permissions requises: {:?}, Permissions actuelles: {:?}",
            required_permissions, auth_state.permissions
        )));
    }

    Ok(next.run(request).await)
}
