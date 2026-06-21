use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   // user_id as UUID string
    pub company_id: String,
    pub roles: Vec<String>,
    pub exp: i64,
    pub iat: i64,
}

/// Authenticated user extracted from JWT — used as Axum extractor.
#[derive(Debug, Clone)]
pub struct AuthenticatedUser(pub UserContext);

#[derive(Debug, Clone)]
pub struct UserContext {
    pub id: Uuid,
    pub company_id: Uuid,
    pub roles: Vec<String>,
}

/// Axum middleware to verify JWT bearer token.
pub async fn require_auth(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Response {
    let token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|t| t.to_string());

    let token = match token {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({
                    "errorCode": "UNAUTHORIZED",
                    "message": "Missing or invalid Authorization header"
                })),
            )
                .into_response();
        }
    };

    let claims = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
        &Validation::default(),
    );

    match claims {
        Ok(token_data) => {
            let user = UserContext {
                id: Uuid::parse_str(&token_data.claims.sub).unwrap_or_default(),
                company_id: Uuid::parse_str(&token_data.claims.company_id).unwrap_or_default(),
                roles: token_data.claims.roles,
            };
            request.extensions_mut().insert(AuthenticatedUser(user));
            next.run(request).await
        }
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            axum::Json(serde_json::json!({
                "errorCode": "UNAUTHORIZED",
                "message": "Invalid or expired token"
            })),
        )
            .into_response(),
    }
}

// Axum FromRequestParts extractor for AuthenticatedUser
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthenticatedUser>()
            .cloned()
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    axum::Json(serde_json::json!({
                        "errorCode": "UNAUTHORIZED",
                        "message": "Not authenticated"
                    })),
                )
            })
    }
}
