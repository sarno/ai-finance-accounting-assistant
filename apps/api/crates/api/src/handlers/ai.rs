use axum::{
    extract::State,
    http::HeaderMap,
    response::Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;
use sqlx::Row;

use crate::{
    errors::ApiError,
    state::AppState,
};
use finance_assistant_app::{
    dto::{
        invoice::{
            CreateSalesInvoiceRequest, CreatePurchaseInvoiceRequest, CreateInvoiceLineRequest,
            CreatePurchaseInvoiceLineRequest, SalesInvoiceResponse, PurchaseInvoiceResponse,
        },
        payment::{CreatePaymentRequest, CreatePaymentAllocationRequest, PaymentResponse},
        journal::{CreateJournalDraftRequest, CreateJournalLineRequest, JournalResponse},
        approval::ApprovalResponse,
    },
    errors::AppError,
};

// Helper function to check Bearer token auth matches AI_SERVICE_TOKEN
fn check_ai_auth(headers: &HeaderMap, state: &AppState) -> Result<(), ApiError> {
    let auth_header = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    match auth_header {
        Some(token) if token == state.config.ai_service_token => Ok(()),
        _ => Err(ApiError(AppError::Unauthorized {
            reason: "Invalid or missing AI service token".to_string(),
        })),
    }
}

// Helper to get first user in company as the actor_user_id (for auditing/created_by)
async fn get_company_user(company_id: Uuid, state: &AppState) -> Uuid {
    sqlx::query("SELECT id FROM users WHERE company_id = $1 LIMIT 1")
        .bind(company_id)
        .fetch_one(&state.db_pool)
        .await
        .map(|row| row.get::<Uuid, _>("id"))
        .unwrap_or_else(|_| Uuid::nil())
}

// Helper to write database audit logs for AI transactions
async fn log_ai_action(
    state: &AppState,
    company_id: Uuid,
    user_id: Uuid,
    entity_type: &str,
    entity_id: Uuid,
    action: &str,
    headers: &HeaderMap,
) {
    let ip = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    let ua = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let _ = sqlx::query(
        "INSERT INTO audit_logs (company_id, actor_user_id, actor_type, entity_type, entity_id, action, ip_address, user_agent)
         VALUES ($1, $2, 'ai', $3, $4, $5, $6, $7)"
    )
    .bind(company_id)
    .bind(if user_id.is_nil() { None } else { Some(user_id) })
    .bind(entity_type)
    .bind(entity_id)
    .bind(action)
    .bind(ip)
    .bind(ua)
    .execute(&state.db_pool)
    .await;
}

// Helper to parse dates supporting RFC3339 or standard YYYY-MM-DD
fn parse_date(date_str: &str) -> Result<time::Date, ApiError> {
    time::Date::parse(date_str, &time::format_description::well_known::Rfc3339)
        .or_else(|_| time::Date::parse(date_str, &time::macros::format_description!("[year]-[month]-[day]")))
        .map_err(|e| ApiError(AppError::Validation {
            message: format!("Invalid date format '{}': {}", date_str, e),
        }))
}

// ─── 1. Query Report ──────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryReportRequest {
    pub company_id: Uuid,
    pub intent: String, // "CashPosition" | "AccountsReceivable" | "AccountsPayable" | "ProfitLoss" | "TaxSummary" | "TrialBalance" | "GeneralLedger"
    pub period: Option<String>, // "YYYY-MM"
    pub as_of: Option<String>, // "YYYY-MM-DD"
    pub start_date: Option<String>, // "YYYY-MM-DD"
    pub end_date: Option<String>, // "YYYY-MM-DD"
}

pub async fn query_report(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<QueryReportRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    check_ai_auth(&headers, &state)?;

    let now_date = time::OffsetDateTime::now_utc().date();
    
    // Parse single target date
    let as_of = match &req.as_of {
        Some(s) => parse_date(s)?,
        None => now_date,
    };

    // Parse date ranges
    let mut start_date = time::Date::from_calendar_date(now_date.year(), now_date.month(), 1).unwrap();
    let mut end_date = now_date;

    if let Some(ref p) = req.period {
        let parts: Vec<&str> = p.split('-').collect();
        if parts.len() == 2 {
            if let (Ok(y), Ok(m)) = (parts[0].parse::<i32>(), parts[1].parse::<u8>()) {
                if let Ok(month) = time::Month::try_from(m) {
                    if let Ok(start) = time::Date::from_calendar_date(y, month, 1) {
                        start_date = start;
                        let next_month = month.next();
                        let next_year = if next_month == time::Month::January { y + 1 } else { y };
                        let next_month_start = time::Date::from_calendar_date(next_year, next_month, 1).unwrap();
                        end_date = next_month_start.previous_day().unwrap();
                    }
                }
            }
        }
    } else {
        if let Some(ref s) = req.start_date {
            start_date = parse_date(s)?;
        }
        if let Some(ref e) = req.end_date {
            end_date = parse_date(e)?;
        }
    }

    let result_val = match req.intent.as_str() {
        "CashPosition" => {
            let res = state.report_service.get_cash_position(req.company_id, as_of).await?;
            serde_json::to_value(res).unwrap()
        }
        "AccountsReceivable" => {
            let res = state.report_service.get_accounts_receivable_aging(req.company_id, as_of).await?;
            serde_json::to_value(res).unwrap()
        }
        "AccountsPayable" => {
            let res = state.report_service.get_accounts_payable_aging(req.company_id, as_of).await?;
            serde_json::to_value(res).unwrap()
        }
        "ProfitLoss" => {
            let res = state.report_service.get_profit_loss(req.company_id, start_date, end_date).await?;
            serde_json::to_value(res).unwrap()
        }
        "TaxSummary" => {
            let res = state.master_data_service.get_tax_summary(req.company_id, start_date, end_date).await?;
            serde_json::to_value(res).unwrap()
        }
        "TrialBalance" => {
            let res = state.report_service.get_trial_balance(req.company_id, as_of).await?;
            serde_json::to_value(res).unwrap()
        }
        "GeneralLedger" => {
            let res = state.report_service.get_general_ledger(req.company_id, start_date, end_date, None).await?;
            serde_json::to_value(res).unwrap()
        }
        _ => return Err(ApiError(AppError::Validation {
            message: format!("Unknown AI query intent '{}'", req.intent),
        })),
    };

    Ok(Json(result_val))
}

// ─── 2. Create Draft Invoice ──────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDraftInvoiceRequest {
    pub company_id: Uuid,
    pub invoice_type: String, // "Sales" | "Purchase"
    pub contact_id: Uuid, // customer_id (Sales) or supplier_id (Purchase)
    pub invoice_date: String,
    pub due_date: String,
    pub invoice_number: String,
    pub notes: Option<String>,
    pub items: Vec<CreateDraftInvoiceLine>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDraftInvoiceLine {
    pub item_id: Option<Uuid>,
    pub description: String,
    pub quantity: rust_decimal::Decimal,
    pub unit_price: rust_decimal::Decimal,
    pub tax_type_id: Option<Uuid>,
    pub account_id: Uuid,
}

pub async fn create_draft_invoice(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<CreateDraftInvoiceRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    check_ai_auth(&headers, &state)?;
    let user_id = get_company_user(req.company_id, &state).await;

    let invoice_date = parse_date(&req.invoice_date)?;
    let due_date = parse_date(&req.due_date)?;

    match req.invoice_type.as_str() {
        "Sales" => {
            let sales_req = CreateSalesInvoiceRequest {
                company_id: req.company_id,
                branch_id: None,
                invoice_number: req.invoice_number,
                customer_id: req.contact_id,
                invoice_date,
                due_date,
                lines: req.items.iter().enumerate().map(|(idx, line)| CreateInvoiceLineRequest {
                    item_id: line.item_id,
                    description: line.description.clone(),
                    quantity: line.quantity,
                    unit_price: line.unit_price,
                    discount_amount: None,
                    tax_type_id: line.tax_type_id,
                    account_id: line.account_id,
                    sort_order: (idx + 1) as i32,
                }).collect(),
                notes: req.notes,
            };
            let created = state.invoice_service.create_sales_draft(sales_req, user_id).await?;
            let res_id = created.id;
            log_ai_action(&state, req.company_id, user_id, "SalesInvoice", res_id, "create_draft", &headers).await;
            
            let res_response = SalesInvoiceResponse::from(created);
            Ok(Json(serde_json::to_value(res_response).unwrap()))
        }
        "Purchase" => {
            let purchase_req = CreatePurchaseInvoiceRequest {
                company_id: req.company_id,
                branch_id: None,
                supplier_invoice_number: req.invoice_number.clone(),
                internal_reference: req.invoice_number,
                supplier_id: req.contact_id,
                invoice_date,
                due_date,
                lines: req.items.iter().enumerate().map(|(idx, line)| CreatePurchaseInvoiceLineRequest {
                    item_id: line.item_id,
                    description: line.description.clone(),
                    quantity: line.quantity,
                    unit_price: line.unit_price,
                    discount_amount: None,
                    tax_type_id: line.tax_type_id,
                    account_id: line.account_id,
                    sort_order: (idx + 1) as i32,
                }).collect(),
                notes: req.notes,
                attachment_url: None,
                uploaded_document_id: None,
                ai_confidence: None,
            };
            let created = state.invoice_service.create_purchase_draft(purchase_req, user_id).await?;
            let res_id = created.id;
            log_ai_action(&state, req.company_id, user_id, "PurchaseInvoice", res_id, "create_draft", &headers).await;

            let res_response = PurchaseInvoiceResponse::from(created);
            Ok(Json(serde_json::to_value(res_response).unwrap()))
        }
        _ => Err(ApiError(AppError::Validation {
            message: format!("Invalid invoiceType '{}' (expected 'Sales' or 'Purchase')", req.invoice_type),
        })),
    }
}

// ─── 3. Create Draft Payment ──────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDraftPaymentRequest {
    pub company_id: Uuid,
    pub payment_type: String, // "payment_received" | "payment_paid"
    pub counterparty_type: String, // "customer" | "supplier"
    pub counterparty_id: Uuid,
    pub payment_date: String,
    pub bank_account_id: Uuid,
    pub amount: rust_decimal::Decimal,
    pub notes: Option<String>,
    pub allocations: Vec<CreateDraftPaymentAllocation>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDraftPaymentAllocation {
    pub document_type: String, // "sales_invoice" | "purchase_invoice"
    pub document_id: Uuid,
    pub allocated_amount: rust_decimal::Decimal,
}

pub async fn create_draft_payment(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<CreateDraftPaymentRequest>,
) -> Result<Json<PaymentResponse>, ApiError> {
    check_ai_auth(&headers, &state)?;
    let user_id = get_company_user(req.company_id, &state).await;

    let payment_date = parse_date(&req.payment_date)?;

    let app_req = CreatePaymentRequest {
        company_id: req.company_id,
        payment_type: req.payment_type,
        counterparty_type: req.counterparty_type,
        counterparty_id: req.counterparty_id,
        payment_date,
        bank_account_id: req.bank_account_id,
        amount: req.amount,
        allocations: req.allocations.into_iter().map(|a| CreatePaymentAllocationRequest {
            document_type: a.document_type,
            document_id: a.document_id,
            allocated_amount: a.allocated_amount,
        }).collect(),
        notes: req.notes,
        attachment_url: None,
    };

    let created = state.payment_service.create_payment(app_req, user_id).await?;
    let res_id = created.id;
    log_ai_action(&state, req.company_id, user_id, "Payment", res_id, "create_draft", &headers).await;

    Ok(Json(PaymentResponse::from(created)))
}

// ─── 4. Create Draft Journal ──────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDraftJournalRequest {
    pub company_id: Uuid,
    pub description: String,
    pub transaction_date: String,
    pub reference_number: Option<String>,
    pub lines: Vec<CreateDraftJournalLine>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDraftJournalLine {
    pub account_id: Uuid,
    pub debit: rust_decimal::Decimal,
    pub credit: rust_decimal::Decimal,
    pub description: Option<String>,
}

pub async fn create_draft_journal(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<CreateDraftJournalRequest>,
) -> Result<Json<JournalResponse>, ApiError> {
    check_ai_auth(&headers, &state)?;
    let user_id = get_company_user(req.company_id, &state).await;

    let transaction_date = parse_date(&req.transaction_date)?;

    let app_req = CreateJournalDraftRequest {
        company_id: req.company_id,
        branch_id: None,
        reference_number: req.reference_number,
        description: req.description,
        transaction_date,
        lines: req.lines.into_iter().map(|l| CreateJournalLineRequest {
            account_id: l.account_id,
            debit: l.debit,
            credit: l.credit,
            description: l.description,
        }).collect(),
    };

    let created = state.journal_service.create_draft(app_req, user_id).await?;
    let res_id = created.id;
    log_ai_action(&state, req.company_id, user_id, "JournalEntry", res_id, "create_draft", &headers).await;

    Ok(Json(JournalResponse::from(created)))
}

// ─── 5. Submit Approval Command ───────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitApprovalCommandRequest {
    pub company_id: Uuid,
    pub document_reference: String, // e.g. "INV-2026-001" or "JNL-2026-0034"
    pub action: String, // "Approve" | "Reject"
    pub comment: Option<String>,
}

pub async fn submit_approval_command(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<SubmitApprovalCommandRequest>,
) -> Result<Json<ApprovalResponse>, ApiError> {
    check_ai_auth(&headers, &state)?;
    let user_id = get_company_user(req.company_id, &state).await;

    let ref_str = req.document_reference.trim();

    // 1. Try resolving document UUID by its reference number across transaction tables
    let mut doc_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM sales_invoices WHERE invoice_number = $1 AND company_id = $2"
    )
    .bind(ref_str)
    .bind(req.company_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(anyhow::Error::from)?;

    let mut doc_type = "SalesInvoice";

    if doc_id.is_none() {
        doc_id = sqlx::query_scalar(
            "SELECT id FROM purchase_invoices WHERE (supplier_invoice_number = $1 OR internal_reference = $1) AND company_id = $2"
        )
        .bind(ref_str)
        .bind(req.company_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(anyhow::Error::from)?;
        if doc_id.is_some() {
            doc_type = "PurchaseInvoice";
        }
    }

    if doc_id.is_none() {
        doc_id = sqlx::query_scalar(
            "SELECT id FROM journal_entries WHERE reference_number = $1 AND company_id = $2"
        )
        .bind(ref_str)
        .bind(req.company_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(anyhow::Error::from)?;
        if doc_id.is_some() {
            doc_type = "JournalEntry";
        }
    }

    if doc_id.is_none() {
        doc_id = sqlx::query_scalar(
            "SELECT id FROM payments WHERE reference_number = $1 AND company_id = $2"
        )
        .bind(ref_str)
        .bind(req.company_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(anyhow::Error::from)?;
        if doc_id.is_some() {
            doc_type = "Payment";
        }
    }

    let resolved_id = match doc_id {
        Some(id) => id,
        None => return Err(ApiError(AppError::Validation {
            message: format!("No active document found matching reference '{}'", ref_str),
        })),
    };

    // 2. Locate active pending approval request
    let pending_approval_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM approval_requests WHERE document_id = $1 AND document_type = $2 AND status = 'pending' AND company_id = $3"
    )
    .bind(resolved_id)
    .bind(doc_type)
    .bind(req.company_id)
    .fetch_optional(&state.db_pool)
    .await
    .map_err(anyhow::Error::from)?;

    let approval_request_id = match pending_approval_id {
        Some(id) => id,
        None => return Err(ApiError(AppError::Validation {
            message: format!("No pending approval request exists for reference '{}'", ref_str),
        })),
    };

    // 3. Process Approve / Reject action
    let result = match req.action.as_str() {
        "Approve" => {
            let res = state.approval_service.approve_request(
                approval_request_id,
                user_id,
                Some(req.comment.clone().unwrap_or_else(|| "Approved via AI assistant chat command".to_string())),
            )
            .await?;
            log_ai_action(&state, req.company_id, user_id, "ApprovalRequest", approval_request_id, "approve", &headers).await;
            res
        }
        "Reject" => {
            let res = state.approval_service.reject_request(
                approval_request_id,
                user_id,
                Some(req.comment.clone().unwrap_or_else(|| "Rejected via AI assistant chat command".to_string())),
            )
            .await?;
            log_ai_action(&state, req.company_id, user_id, "ApprovalRequest", approval_request_id, "reject", &headers).await;
            res
        }
        _ => return Err(ApiError(AppError::Validation {
            message: format!("Invalid approval command action '{}' (expected 'Approve' or 'Reject')", req.action),
        })),
    };

    Ok(Json(result))
}
