use axum::{
    extract::{Path, State},
    response::Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    errors::ApiError,
    middleware::auth_middleware::AuthenticatedUser,
    state::AppState,
};
use finance_assistant_app::dto::auth::{CreateUserRequest, UpdateUserRequest, UserSummary};
use finance_assistant_app::errors::AppError;

/// GET /api/users
pub async fn list_users(
    State(state): State<Arc<AppState>>,
    auth_user: AuthenticatedUser,
) -> Result<Json<Vec<UserSummary>>, ApiError> {
    let user = auth_user.0;
    
    // Authorization check: Only Admin or Owner can view/manage users
    if !user.roles.contains(&"admin".to_string()) && !user.roles.contains(&"owner".to_string()) {
        return Err(ApiError(AppError::Unauthorized {
            reason: "Only admin or owner can manage users".to_string(),
        }));
    }

    let summaries = state.auth_service.list_users(user.company_id).await?;
    Ok(Json(summaries))
}

/// POST /api/users
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    auth_user: AuthenticatedUser,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<UserSummary>, ApiError> {
    let user = auth_user.0;
    
    // Authorization check
    if !user.roles.contains(&"admin".to_string()) && !user.roles.contains(&"owner".to_string()) {
        return Err(ApiError(AppError::Unauthorized {
            reason: "Only admin or owner can manage users".to_string(),
        }));
    }

    let summary = state.auth_service.create_user(user.company_id, req).await?;
    Ok(Json(summary))
}

/// PUT /api/users/:id
pub async fn update_user(
    State(state): State<Arc<AppState>>,
    auth_user: AuthenticatedUser,
    Path(user_id): Path<Uuid>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<UserSummary>, ApiError> {
    let user = auth_user.0;
    
    // Authorization check
    if !user.roles.contains(&"admin".to_string()) && !user.roles.contains(&"owner".to_string()) {
        return Err(ApiError(AppError::Unauthorized {
            reason: "Only admin or owner can manage users".to_string(),
        }));
    }

    let summary = state.auth_service.update_user(user.company_id, user_id, req).await?;
    Ok(Json(summary))
}

/// DELETE /api/users/:id
pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    auth_user: AuthenticatedUser,
    Path(user_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let user = auth_user.0;
    
    // Authorization check
    if !user.roles.contains(&"admin".to_string()) && !user.roles.contains(&"owner".to_string()) {
        return Err(ApiError(AppError::Unauthorized {
            reason: "Only admin or owner can manage users".to_string(),
        }));
    }

    // Safety: user cannot delete themselves
    if user.id == user_id {
        return Err(ApiError(AppError::Validation {
            message: "Cannot delete your own user account".to_string(),
        }));
    }

    state.auth_service.delete_user(user.company_id, user_id).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}
