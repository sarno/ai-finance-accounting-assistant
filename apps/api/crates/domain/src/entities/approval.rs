use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

/// An approval request linked to a document (invoice, journal, payment).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub id: Uuid,
    pub company_id: Uuid,
    pub document_type: ApprovalDocumentType,
    pub document_id: Uuid,
    pub status: ApprovalStatus,
    pub requested_by: Uuid,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<OffsetDateTime>,
    pub comment: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalDocumentType {
    SalesInvoice,
    PurchaseInvoice,
    PaymentReceived,
    PaymentPaid,
    JournalEntry,
    Expense,
    TaxPayment,
}

impl ApprovalDocumentType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SalesInvoice => "sales_invoice",
            Self::PurchaseInvoice => "purchase_invoice",
            Self::PaymentReceived => "payment_received",
            Self::PaymentPaid => "payment_paid",
            Self::JournalEntry => "journal_entry",
            Self::Expense => "expense",
            Self::TaxPayment => "tax_payment",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "sales_invoice" => Ok(Self::SalesInvoice),
            "purchase_invoice" => Ok(Self::PurchaseInvoice),
            "payment_received" => Ok(Self::PaymentReceived),
            "payment_paid" => Ok(Self::PaymentPaid),
            "journal_entry" => Ok(Self::JournalEntry),
            "expense" => Ok(Self::Expense),
            "tax_payment" => Ok(Self::TaxPayment),
            _ => Err(format!("Unknown document type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
    Cancelled,
}

impl ApprovalStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Approved => "approved",
            Self::Rejected => "rejected",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "pending" => Ok(Self::Pending),
            "approved" => Ok(Self::Approved),
            "rejected" => Ok(Self::Rejected),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(format!("Unknown approval status: {}", s)),
        }
    }
}
