use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{errors::DomainError, value_objects::DocumentStatus};

// ─── Sales Invoice ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesInvoice {
    pub id: Uuid,
    pub company_id: Uuid,
    pub branch_id: Option<Uuid>,
    pub invoice_number: String,
    pub customer_id: Uuid,
    pub invoice_date: time::Date,
    pub due_date: time::Date,
    pub lines: Vec<InvoiceLine>,
    pub subtotal: Decimal,
    pub tax_amount: Decimal,
    pub total_amount: Decimal,
    pub status: DocumentStatus,
    pub notes: Option<String>,
    pub journal_entry_id: Option<Uuid>,
    pub created_by: Uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

// ─── Purchase Invoice ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseInvoice {
    pub id: Uuid,
    pub company_id: Uuid,
    pub branch_id: Option<Uuid>,
    pub supplier_invoice_number: String,
    pub internal_reference: String,
    pub supplier_id: Uuid,
    pub invoice_date: time::Date,
    pub due_date: time::Date,
    pub lines: Vec<InvoiceLine>,
    pub subtotal: Decimal,
    pub tax_amount: Decimal,
    pub total_amount: Decimal,
    pub status: DocumentStatus,
    pub ai_confidence: Option<Decimal>,
    pub uploaded_document_id: Option<Uuid>,
    pub journal_entry_id: Option<Uuid>,
    pub notes: Option<String>,
    pub created_by: Uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

// ─── Invoice Line ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceLine {
    pub id: Uuid,
    pub item_id: Option<Uuid>,
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

impl InvoiceLine {
    /// Calculate net amount before tax.
    pub fn net_amount(&self) -> Decimal {
        (self.quantity * self.unit_price) - self.discount_amount
    }
}

impl SalesInvoice {
    pub fn ensure_editable(&self) -> Result<(), DomainError> {
        if self.status == DocumentStatus::Posted {
            return Err(DomainError::PostedDocumentImmutable);
        }
        Ok(())
    }
}

impl PurchaseInvoice {
    pub fn ensure_editable(&self) -> Result<(), DomainError> {
        if self.status == DocumentStatus::Posted {
            return Err(DomainError::PostedDocumentImmutable);
        }
        Ok(())
    }
}
