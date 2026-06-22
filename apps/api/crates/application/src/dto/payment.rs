use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use finance_assistant_domain::entities::payment::{Payment, PaymentType, CounterpartyType, AllocatedDocumentType};

// ─── Request ──────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePaymentRequest {
    pub company_id: Uuid,
    pub payment_type: String, // "payment_received", "payment_paid"
    pub counterparty_type: String, // "customer", "supplier"
    pub counterparty_id: Uuid,
    #[serde(with = "crate::dto::date_format")]
    pub payment_date: time::Date,
    pub bank_account_id: Uuid,
    pub amount: Decimal,
    pub allocations: Vec<CreatePaymentAllocationRequest>,
    pub notes: Option<String>,
    pub attachment_url: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePaymentAllocationRequest {
    pub document_type: String, // "sales_invoice", "purchase_invoice"
    pub document_id: Uuid,
    pub allocated_amount: Decimal,
}

// ─── Response ─────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentResponse {
    pub id: Uuid,
    pub company_id: Uuid,
    pub reference_number: String,
    pub payment_type: String,
    pub counterparty_type: String,
    pub counterparty_id: Uuid,
    #[serde(with = "crate::dto::date_format")]
    pub payment_date: time::Date,
    pub bank_account_id: Uuid,
    pub amount: Decimal,
    pub allocations: Vec<PaymentAllocationResponse>,
    pub status: String,
    pub notes: Option<String>,
    pub journal_entry_id: Option<Uuid>,
    pub attachment_url: Option<String>,
    pub created_by: Uuid,
    #[serde(with = "crate::dto::datetime_format")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "crate::dto::datetime_format")]
    pub updated_at: time::OffsetDateTime,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentAllocationResponse {
    pub id: Uuid,
    pub payment_id: Uuid,
    pub document_type: String,
    pub document_id: Uuid,
    pub allocated_amount: Decimal,
}

impl From<Payment> for PaymentResponse {
    fn from(p: Payment) -> Self {
        let payment_type = match p.payment_type {
            PaymentType::PaymentReceived => "payment_received".to_string(),
            PaymentType::PaymentPaid => "payment_paid".to_string(),
        };

        let counterparty_type = match p.counterparty_type {
            CounterpartyType::Customer => "customer".to_string(),
            CounterpartyType::Supplier => "supplier".to_string(),
        };

        let allocations = p.allocations
            .into_iter()
            .map(|a| {
                let document_type = match a.document_type {
                    AllocatedDocumentType::SalesInvoice => "sales_invoice".to_string(),
                    AllocatedDocumentType::PurchaseInvoice => "purchase_invoice".to_string(),
                };
                PaymentAllocationResponse {
                    id: a.id,
                    payment_id: a.payment_id,
                    document_type,
                    document_id: a.document_id,
                    allocated_amount: a.allocated_amount,
                }
            })
            .collect();

        Self {
            id: p.id,
            company_id: p.company_id,
            reference_number: p.reference_number,
            payment_type,
            counterparty_type,
            counterparty_id: p.counterparty_id,
            payment_date: p.payment_date,
            bank_account_id: p.bank_account_id,
            amount: p.amount,
            allocations,
            status: p.status.to_string(),
            notes: p.notes,
            journal_entry_id: p.journal_entry_id,
            attachment_url: p.attachment_url,
            created_by: p.created_by,
            created_at: p.created_at,
            updated_at: p.updated_at,
        }
    }
}
