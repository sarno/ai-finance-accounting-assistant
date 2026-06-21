use axum::{
    extract::{Path, State},
    response::Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{errors::*, middleware::auth_middleware::AuthenticatedUser, state::AppState};
use finance_assistant_app::dto::approval::{ApprovalResponse, ApproveRequest, RejectRequest};

/// GET /api/approvals
/// List pending approval requests for the company.
pub async fn list_pending_approvals(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<Json<Vec<ApprovalResponse>>, ApiError> {
    let response = state.approval_service.list_pending(user.company_id).await?;

    Ok(Json(response))
}

/// GET /api/approvals/:id
/// Get details of a single approval request.
pub async fn get_approval(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ApprovalResponse>, ApiError> {
    let response = state.approval_service.get_by_id(id).await?;
    Ok(Json(response))
}

/// POST /api/approvals/:id/approve
/// Approve request with comment.
pub async fn approve_request(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(req): Json<ApproveRequest>,
) -> Result<Json<ApprovalResponse>, ApiError> {
    let response = state
        .approval_service
        .approve_request(id, user.id, req.comment)
        .await?;

    Ok(Json(response))
}

/// POST /api/approvals/:id/reject
/// Reject request with comment.
pub async fn reject_request(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(req): Json<RejectRequest>,
) -> Result<Json<ApprovalResponse>, ApiError> {
    let response = state
        .approval_service
        .reject_request(id, user.id, Some(req.reason))
        .await?;

    Ok(Json(response))
}
