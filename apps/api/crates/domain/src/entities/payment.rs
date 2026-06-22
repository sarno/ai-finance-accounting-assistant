use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::value_objects::DocumentStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: Uuid,
    pub company_id: Uuid,
    pub reference_number: String,
    pub payment_type: PaymentType,
    pub counterparty_type: CounterpartyType,
    pub counterparty_id: Uuid,
    pub payment_date: time::Date,
    pub bank_account_id: Uuid,
    pub amount: Decimal,
    pub allocations: Vec<PaymentAllocation>,
    pub status: DocumentStatus,
    pub notes: Option<String>,
    pub journal_entry_id: Option<Uuid>,
    pub attachment_url: Option<String>,
    pub created_by: Uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentType {
    PaymentReceived, // from customer
    PaymentPaid,     // to supplier
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CounterpartyType {
    Customer,
    Supplier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentAllocation {
    pub id: Uuid,
    pub payment_id: Uuid,
    pub document_type: AllocatedDocumentType,
    pub document_id: Uuid,
    pub allocated_amount: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AllocatedDocumentType {
    SalesInvoice,
    PurchaseInvoice,
}
