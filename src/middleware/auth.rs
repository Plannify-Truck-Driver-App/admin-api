use axum::{
    extract::{State, Request},
    middleware::Next,
    response::Response,
};
use http::header;
use jsonwebtoken::{decode, DecodingKey, Validation};
use uuid::Uuid;

use crate::{
    errors::app_error::AppError, middleware::require_permissions, models::jwt::Claims
};

#[derive(Clone)]
pub struct AuthState {
    pub employee_id: Uuid,
    pub authorizations: Vec<i32>,
}

#[derive(Clone)]
pub struct MiddlewareState {
    pub jwt_secret: String,
}

pub async fn auth_middleware(
    State(MiddlewareState { jwt_secret, .. }): State<MiddlewareState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // extract token from Authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_str| auth_str.strip_prefix("Bearer "));

    let token = auth_header
        .ok_or_else(|| AppError::Validation("Authentication token missing".to_string()))?;

    // decode and validate JWT token
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Validation("Invalid JWT token".to_string()))?;

    let claims = token_data.claims;

    // check if the token has expired
    if claims.is_expired() {
        return Err(AppError::Validation("JWT token expired".to_string()));
    }

    // create auth state
    let auth_state = AuthState {
        employee_id: claims.sub,
        authorizations: claims.authorizations,
    };

    // add auth state to request
    request.extensions_mut().insert(auth_state);

    Ok(next.run(request).await)
}

pub fn with_required_permissions(
    required_permissions: Vec<i32>,
) -> impl Fn(Request, Next) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Response, AppError>> + Send>
    >
    + Clone
    + Send
    + Sync
    + 'static
{
    move |req: Request, next: Next| {
        let required_permissions = required_permissions.clone();
        Box::pin(async move {
            require_permissions(required_permissions, req, next).await
        })
    }
}
