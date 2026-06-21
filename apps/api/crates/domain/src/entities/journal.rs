use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;
use crate::{
    errors::DomainError,
    value_objects::DocumentStatus,
};

/// A journal entry (set of balanced debit/credit lines).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: Uuid,
    pub company_id: Uuid,
    pub branch_id: Option<Uuid>,
    pub reference_number: String,
    pub description: String,
    pub transaction_date: time::Date,
    pub lines: Vec<JournalLine>,
    pub status: DocumentStatus,
    pub source: JournalSource,
    pub source_document_id: Option<Uuid>,
    pub created_by: Uuid,
    pub posted_by: Option<Uuid>,
    pub posted_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

/// A single debit or credit line within a journal entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalLine {
    pub id: Uuid,
    pub journal_entry_id: Uuid,
    pub account_id: Uuid,
    pub debit: Decimal,
    pub credit: Decimal,
    pub description: Option<String>,
    pub sort_order: i32,
}

/// Where did this journal originate?
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JournalSource {
    Manual,
    SalesInvoice,
    PurchaseInvoice,
    PaymentReceived,
    PaymentPaid,
    Expense,
    Adjustment,
    Reversal,
}

impl JournalEntry {
    /// Domain rule: debit total must equal credit total.
    pub fn validate_balance(&self) -> Result<(), DomainError> {
        let total_debit: Decimal = self.lines.iter().map(|l| l.debit).sum();
        let total_credit: Decimal = self.lines.iter().map(|l| l.credit).sum();

        if total_debit != total_credit {
            return Err(DomainError::JournalNotBalanced {
                total_debit,
                total_credit,
            });
        }
        Ok(())
    }

    /// Mark journal as posted. Fails if not approved or already posted.
    pub fn post(&mut self, posted_by: Uuid) -> Result<(), DomainError> {
        if self.status != DocumentStatus::Approved {
            return Err(DomainError::InvalidStatusTransition {
                from: format!("{:?}", self.status),
                to: "Posted".to_string(),
            });
        }
        self.validate_balance()?;
        self.status = DocumentStatus::Posted;
        self.posted_by = Some(posted_by);
        self.posted_at = Some(OffsetDateTime::now_utc());
        self.updated_at = OffsetDateTime::now_utc();
        Ok(())
    }

    /// Posted journals are immutable — cannot be edited directly.
    pub fn ensure_editable(&self) -> Result<(), DomainError> {
        if self.status == DocumentStatus::Posted {
            return Err(DomainError::PostedDocumentImmutable);
        }
        Ok(())
    }
}

impl std::fmt::Display for JournalSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Manual => "manual",
            Self::SalesInvoice => "sales_invoice",
            Self::PurchaseInvoice => "purchase_invoice",
            Self::PaymentReceived => "payment_received",
            Self::PaymentPaid => "payment_paid",
            Self::Expense => "expense",
            Self::Adjustment => "adjustment",
            Self::Reversal => "reversal",
        };
        write!(f, "{}", s)
    }
}
