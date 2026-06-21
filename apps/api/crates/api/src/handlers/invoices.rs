use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
    response::IntoResponse,
    response::Json,
};
use std::sync::Arc;
use uuid::Uuid;
use serde::Deserialize;

use finance_assistant_app::dto::invoice::{CreateSalesInvoiceRequest, SalesInvoiceResponse};
use crate::{errors::ApiError, middleware::auth_middleware::AuthenticatedUser, state::AppState};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListInvoicesParams {
    pub company_id: Option<Uuid>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

/// GET /api/sales-invoices
pub async fn list_sales_invoices(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(params): Query<ListInvoicesParams>,
) -> Result<impl IntoResponse, ApiError> {
    let company_id = params.company_id.unwrap_or(user.company_id);
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);

    let total = state.invoice_service.count_sales_invoices(company_id).await?;
    let response = state
        .invoice_service
        .list_sales_invoices(company_id, page, per_page)
        .await?;

    let mut headers = HeaderMap::new();
    headers.insert("x-total-count", total.to_string().parse().unwrap());
    headers.insert("access-control-expose-headers", "x-total-count".parse().unwrap());

    Ok((headers, Json(response)))
}

/// POST /api/sales-invoices/draft
pub async fn create_sales_draft(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreateSalesInvoiceRequest>,
) -> Result<Json<SalesInvoiceResponse>, ApiError> {
    let response = state
        .invoice_service
        .create_sales_draft(req, user.id)
        .await?;
    Ok(Json(response))
}

/// GET /api/sales-invoices/:id
pub async fn get_sales_invoice(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<SalesInvoiceResponse>, ApiError> {
    let response = state.invoice_service.get_sales_invoice(id).await?;
    Ok(Json(response))
}

/// PUT /api/sales-invoices/:id
pub async fn update_sales_invoice(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(req): Json<CreateSalesInvoiceRequest>,
) -> Result<Json<SalesInvoiceResponse>, ApiError> {
    let response = state
        .invoice_service
        .update_sales_draft(id, req)
        .await?;
    Ok(Json(response))
}

/// DELETE /api/sales-invoices/:id
pub async fn delete_sales_invoice(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    state.invoice_service.delete_sales_invoice(id).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}

/// POST /api/sales-invoices/:id/submit
pub async fn submit_approval(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<SalesInvoiceResponse>, ApiError> {
    state.approval_service.submit_sales_invoice_approval(id, user.id).await?;
    let response = state.invoice_service.get_sales_invoice(id).await?;
    Ok(Json(response))
}
