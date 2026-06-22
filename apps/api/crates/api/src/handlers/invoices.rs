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
use finance_assistant_app::errors::AppError;
use finance_assistant_app::dto::invoice::{
    CreatePurchaseInvoiceRequest, CreatePurchaseInvoiceLineRequest, CreateSalesInvoiceRequest, PurchaseInvoiceResponse,
    SalesInvoiceResponse,
};

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

    let total = state
        .invoice_service
        .count_sales_invoices(company_id)
        .await?;
    let response = state
        .invoice_service
        .list_sales_invoices(company_id, page, per_page)
        .await?;

    let mut headers = HeaderMap::new();
    headers.insert("x-total-count", total.to_string().parse().unwrap());
    headers.insert(
        "access-control-expose-headers",
        "x-total-count".parse().unwrap(),
    );

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

/// GET /api/purchase-invoices
pub async fn list_purchase_invoices(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(params): Query<ListInvoicesParams>,
) -> Result<impl IntoResponse, ApiError> {
    let company_id = params.company_id.unwrap_or(user.company_id);
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);

    let total = state
        .invoice_service
        .count_purchase_invoices(company_id)
        .await?;
    let response = state
        .invoice_service
        .list_purchase_invoices(company_id, page, per_page)
        .await?;

    let mut headers = HeaderMap::new();
    headers.insert("x-total-count", total.to_string().parse().unwrap());
    headers.insert(
        "access-control-expose-headers",
        "x-total-count".parse().unwrap(),
    );

    Ok((headers, Json(response)))
}

/// POST /api/purchase-invoices/draft
pub async fn create_purchase_draft(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreatePurchaseInvoiceRequest>,
) -> Result<Json<PurchaseInvoiceResponse>, ApiError> {
    let response = state
        .invoice_service
        .create_purchase_draft(req, user.id)
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
    let response = state.invoice_service.update_sales_draft(id, req).await?;
    Ok(Json(response))
}

/// GET /api/purchase-invoices/:id
pub async fn get_purchase_invoice(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<PurchaseInvoiceResponse>, ApiError> {
    let response = state.invoice_service.get_purchase_invoice(id).await?;
    Ok(Json(response))
}

/// PUT /api/purchase-invoices/:id
pub async fn update_purchase_invoice(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(req): Json<CreatePurchaseInvoiceRequest>,
) -> Result<Json<PurchaseInvoiceResponse>, ApiError> {
    let response = state.invoice_service.update_purchase_draft(id, req).await?;
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

/// DELETE /api/purchase-invoices/:id
pub async fn delete_purchase_invoice(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    state.invoice_service.delete_purchase_invoice(id).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}

/// POST /api/sales-invoices/:id/submit
pub async fn submit_approval(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<SalesInvoiceResponse>, ApiError> {
    state
        .approval_service
        .submit_sales_invoice_approval(id, user.id)
        .await?;
    let response = state.invoice_service.get_sales_invoice(id).await?;
    Ok(Json(response))
}

/// POST /api/purchase-invoices/:id/submit
pub async fn submit_purchase_approval(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<PurchaseInvoiceResponse>, ApiError> {
    state
        .approval_service
        .submit_purchase_invoice_approval(id, user.id)
        .await?;
    let response = state.invoice_service.get_purchase_invoice(id).await?;
    Ok(Json(response))
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePurchaseFromDocumentRequest {
    pub uploaded_document_id: Uuid,
    pub supplier_id: Uuid,
    pub extracted_fields: ExtractedFields,
    pub lines: Vec<InvoiceLineRequest>,
    pub ai_confidence: f64,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtractedFields {
    pub invoice_no: String,
    pub invoice_date: String,
    pub due_date: String,
    pub subtotal: rust_decimal::Decimal,
    pub tax_amount: rust_decimal::Decimal,
    pub total_amount: rust_decimal::Decimal,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceLineRequest {
    pub description: String,
    pub quantity: rust_decimal::Decimal,
    pub unit_price: rust_decimal::Decimal,
    pub expense_account_id: Uuid,
    pub tax_type_id: Option<Uuid>,
}

fn parse_date(date_str: &str) -> Result<time::Date, ApiError> {
    time::Date::parse(date_str, &time::format_description::well_known::Rfc3339)
        .or_else(|_| time::Date::parse(date_str, &time::macros::format_description!("[year]-[month]-[day]")))
        .map_err(|e| ApiError(AppError::Validation {
            message: format!("Invalid date format '{}': {}", date_str, e),
        }))
}

/// POST /api/purchase-invoices/from-document
pub async fn create_purchase_from_document(
    State(state): State<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreatePurchaseFromDocumentRequest>,
) -> Result<Json<PurchaseInvoiceResponse>, ApiError> {
    let invoice_date = parse_date(&req.extracted_fields.invoice_date)?;
    let due_date = parse_date(&req.extracted_fields.due_date)?;

    let doc = state.document_service.get_document(req.uploaded_document_id).await.ok();
    let attachment_url = doc.map(|d| {
        format!(
            "{}/{}",
            state.config.storage_base_url.trim_end_matches('/'),
            d.storage_path
        )
    });

    let invoice_req = CreatePurchaseInvoiceRequest {
        company_id: user.company_id,
        branch_id: None,
        supplier_invoice_number: req.extracted_fields.invoice_no.clone(),
        internal_reference: req.extracted_fields.invoice_no,
        supplier_id: req.supplier_id,
        invoice_date,
        due_date,
        lines: req.lines.into_iter().enumerate().map(|(idx, l)| {
            CreatePurchaseInvoiceLineRequest {
                item_id: None,
                description: l.description,
                quantity: l.quantity,
                unit_price: l.unit_price,
                discount_amount: None,
                tax_type_id: l.tax_type_id,
                account_id: l.expense_account_id,
                sort_order: (idx + 1) as i32,
            }
        }).collect(),
        notes: Some(format!("Dibuat dari unggahan dokumen (Confidence: {}%)", req.ai_confidence)),
        attachment_url,
        uploaded_document_id: Some(req.uploaded_document_id),
        ai_confidence: Some(rust_decimal::Decimal::from_f64_retain(req.ai_confidence).unwrap_or_default()),
    };

    let response = state
        .invoice_service
        .create_purchase_draft(invoice_req, user.id)
        .await?;

    Ok(Json(response))
}

