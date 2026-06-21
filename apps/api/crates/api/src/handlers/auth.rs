use axum::{extract::State, response::Json};
use std::sync::Arc;

use finance_assistant_app::dto::auth::{
    LoginRequest, LoginResponse, RefreshTokenRequest, RefreshTokenResponse,
};

use crate::{errors::ApiError, state::AppState};

/// POST /api/auth/login
/// Authenticate user and return JWT access + refresh tokens.
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    let response = state.auth_service.login(req).await?;
    Ok(Json(response))
}

/// POST /api/auth/refresh
/// Refresh an expired access token using a valid refresh token.
pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RefreshTokenRequest>,
) -> Result<Json<RefreshTokenResponse>, ApiError> {
    let response = state.auth_service.refresh(req).await?;
    Ok(Json(response))
}
