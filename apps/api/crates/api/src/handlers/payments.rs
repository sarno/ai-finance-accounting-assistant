use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
    response::IntoResponse,
    response::Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{errors::ApiError, middleware::auth_middleware::AuthenticatedUser, state::AppState};
use finance_assistant_app::dto::payment::{CreatePaymentRequest, PaymentResponse};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPaymentsParams {
    pub company_id: Option<Uuid>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

/// GET /api/payments
pub async fn list_payments(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(params): Query<ListPaymentsParams>,
) -> Result<impl IntoResponse, ApiError> {
    let company_id = params.company_id.unwrap_or(user.company_id);
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);

    let total = state
        .payment_service
        .count_payments(company_id)
        .await?;
    let response = state
        .payment_service
        .list_payments(company_id, page, per_page)
        .await?;

    let mut headers = HeaderMap::new();
    headers.insert("x-total-count", total.to_string().parse().unwrap());
    headers.insert(
        "access-control-expose-headers",
        "x-total-count".parse().unwrap(),
    );

    Ok((headers, Json(response)))
}

/// POST /api/payments/draft
pub async fn create_payment_draft(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreatePaymentRequest>,
) -> Result<Json<PaymentResponse>, ApiError> {
    let response = state
        .payment_service
        .create_payment(req, user.id)
        .await?;
    Ok(Json(response))
}

/// GET /api/payments/:id
pub async fn get_payment(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<PaymentResponse>, ApiError> {
    let response = state.payment_service.get_payment(id).await?;
    Ok(Json(response))
}

/// PUT /api/payments/:id
pub async fn update_payment(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(req): Json<CreatePaymentRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    state.payment_service.update_payment(id, req).await?;
    let response = state.payment_service.get_payment(id).await?;
    Ok(Json(serde_json::json!({ "success": true, "payment": response })))
}

/// DELETE /api/payments/:id
pub async fn delete_payment(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    state.payment_service.delete_payment(id).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}

/// POST /api/payments/:id/submit
pub async fn submit_payment_approval(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<PaymentResponse>, ApiError> {
    state
        .approval_service
        .submit_payment_approval(id, user.id)
        .await?;
    let response = state.payment_service.get_payment(id).await?;
    Ok(Json(response))
}
