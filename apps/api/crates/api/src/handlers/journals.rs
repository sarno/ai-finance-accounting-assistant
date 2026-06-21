use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
    response::{IntoResponse, Json},
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{errors::*, middleware::auth_middleware::AuthenticatedUser, state::AppState};
use finance_assistant_app::dto::journal::{CreateJournalDraftRequest, JournalResponse};

#[derive(serde::Deserialize)]
pub struct ListJournalsParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub company_id: Option<Uuid>,
}

/// POST /api/journals/draft
/// Create a draft journal entry (manual or from AI suggestion).
pub async fn create_draft(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> Result<Json<JournalResponse>, ApiError> {
    println!("Create draft raw JSON: {}", body);
    let req: CreateJournalDraftRequest = serde_json::from_value(body).map_err(|e| {
        ApiError(finance_assistant_app::errors::AppError::Validation {
            message: format!("JSON parse error: {}", e),
        })
    })?;
    println!("Create draft parsed struct: {:?}", req);
    let response = state.journal_service.create_draft(req, user.id).await?;

    Ok(Json(response))
}

/// GET /api/journals/:id
/// Retrieve a single journal entry by ID.
pub async fn get_journal(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<JournalResponse>, ApiError> {
    let response = state.journal_service.get_journal(id).await?;
    Ok(Json(response))
}

/// GET /api/journals
/// List journal entries for the company.
pub async fn list_journals(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(params): Query<ListJournalsParams>,
) -> Result<impl IntoResponse, ApiError> {
    let company_id = params.company_id.unwrap_or(user.company_id);
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);

    let total = state.journal_service.count_journals(company_id).await?;
    let response = state
        .journal_service
        .list_journals(company_id, page, per_page)
        .await?;

    let mut headers = HeaderMap::new();
    headers.insert("x-total-count", total.to_string().parse().unwrap());
    headers.insert(
        "access-control-expose-headers",
        "x-total-count".parse().unwrap(),
    );

    Ok((headers, Json(response)))
}

/// POST /api/journals/:id/submit — submit draft for approval.
pub async fn submit_approval(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<JournalResponse>, ApiError> {
    state
        .approval_service
        .submit_journal_approval(id, user.id)
        .await?;
    let response = state.journal_service.get_journal(id).await?;
    Ok(Json(response))
}

/// POST /api/journals/:id/post — post approved journal.
pub async fn post_journal(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<JournalResponse>, ApiError> {
    let response = state.journal_service.post_journal(id, user.id).await?;
    Ok(Json(response))
}

/// POST /api/journals/:id/approve — direct approve journal (helper/testing).
pub async fn approve_journal(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<JournalResponse>, ApiError> {
    let response = state.journal_service.approve_journal(id).await?;
    Ok(Json(response))
}

/// PUT /api/journals/:id
/// Update a draft journal entry.
pub async fn update_journal(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(body): Json<serde_json::Value>,
) -> Result<Json<JournalResponse>, ApiError> {
    let req: CreateJournalDraftRequest = serde_json::from_value(body).map_err(|e| {
        ApiError(finance_assistant_app::errors::AppError::Validation {
            message: format!("JSON parse error: {}", e),
        })
    })?;
    let response = state.journal_service.update_draft(id, req).await?;
    Ok(Json(response))
}

/// DELETE /api/journals/:id
/// Delete a draft journal entry.
pub async fn delete_journal(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    state.journal_service.delete_journal(id).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}
