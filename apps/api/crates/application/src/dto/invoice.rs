use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use finance_assistant_domain::entities::invoice::{SalesInvoice, InvoiceLine};

// ─── Request ──────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSalesInvoiceRequest {
    pub company_id: Uuid,
    pub branch_id: Option<Uuid>,
    pub invoice_number: String,
    pub customer_id: Uuid,
    #[serde(with = "crate::dto::date_format")]
    pub invoice_date: time::Date,
    #[serde(with = "crate::dto::date_format")]
    pub due_date: time::Date,
    pub lines: Vec<CreateInvoiceLineRequest>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInvoiceLineRequest {
    pub description: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub discount_amount: Option<Decimal>,
    pub tax_type_id: Option<Uuid>,
    pub account_id: Uuid,
    pub sort_order: i32,
}

// ─── Response ─────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SalesInvoiceResponse {
    pub id: Uuid,
    pub company_id: Uuid,
    pub branch_id: Option<Uuid>,
    pub invoice_number: String,
    pub customer_id: Uuid,
    #[serde(with = "crate::dto::date_format")]
    pub invoice_date: time::Date,
    #[serde(with = "crate::dto::date_format")]
    pub due_date: time::Date,
    pub lines: Vec<InvoiceLineResponse>,
    pub subtotal: Decimal,
    pub tax_amount: Decimal,
    pub total_amount: Decimal,
    pub status: String,
    pub notes: Option<String>,
    pub journal_entry_id: Option<Uuid>,
    pub created_by: Uuid,
    #[serde(with = "crate::dto::datetime_format")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "crate::dto::datetime_format")]
    pub updated_at: time::OffsetDateTime,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceLineResponse {
    pub id: Uuid,
    pub description: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub discount_amount: Decimal,
    pub tax_type_id: Option<Uuid>,
    pub tax_rate: Option<Decimal>,
    pub tax_amount: Decimal,
    pub line_total: Decimal,
    pub account_id: Uuid,
    pub sort_order: i32,
}

impl From<SalesInvoice> for SalesInvoiceResponse {
    fn from(si: SalesInvoice) -> Self {
        Self {
            id: si.id,
            company_id: si.company_id,
            branch_id: si.branch_id,
            invoice_number: si.invoice_number,
            customer_id: si.customer_id,
            invoice_date: si.invoice_date,
            due_date: si.due_date,
            lines: si.lines.into_iter().map(InvoiceLineResponse::from).collect(),
            subtotal: si.subtotal,
            tax_amount: si.tax_amount,
            total_amount: si.total_amount,
            status: si.status.to_string(),
            notes: si.notes,
            journal_entry_id: si.journal_entry_id,
            created_by: si.created_by,
            created_at: si.created_at,
            updated_at: si.updated_at,
        }
    }
}

impl From<InvoiceLine> for InvoiceLineResponse {
    fn from(il: InvoiceLine) -> Self {
        Self {
            id: il.id,
            description: il.description,
            quantity: il.quantity,
            unit_price: il.unit_price,
            discount_amount: il.discount_amount,
            tax_type_id: il.tax_type_id,
            tax_rate: il.tax_rate,
            tax_amount: il.tax_amount,
            line_total: il.line_total,
            account_id: il.account_id,
            sort_order: il.sort_order,
        }
    }
}
