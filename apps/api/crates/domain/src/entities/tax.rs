use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

/// Tax type configuration (stored with effective dates).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxType {
    pub id: Uuid,
    pub company_id: Uuid,
    pub code: String,
    pub name: String,
    pub category: TaxCategory,
    pub default_rate: Decimal,
    pub payable_account_id: Uuid,
    pub effective_from: time::Date,
    pub effective_to: Option<time::Date>,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaxCategory {
    VatOutput,
    VatInput,
    WithholdingPph21,
    WithholdingPph23,
    WithholdingPph25,
    WithholdingPphFinal,
}

impl TaxType {
    /// Get the applicable rate for a given date (respects effective_from / effective_to).
    pub fn is_effective_for(&self, date: &time::Date) -> bool {
        if date < &self.effective_from {
            return false;
        }
        if let Some(to) = self.effective_to {
            if date > &to {
                return false;
            }
        }
        self.is_active
    }
}

/// Tax transaction record representing individual tax items from transactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRecord {
    pub id: Uuid,
    pub company_id: Uuid,
    pub tax_type_id: Uuid,
    pub source_document_type: String,
    pub source_document_id: Uuid,
    pub tax_period: time::Date,
    pub tax_base_amount: Decimal,
    pub tax_rate: Decimal,
    pub tax_amount: Decimal,
    pub status: TaxRecordStatus,
    pub counterparty_name: Option<String>,
    pub counterparty_npwp: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaxRecordStatus {
    NotRequired,
    Required,
    Drafted,
    Validated,
    Reported,
    Paid,
    Archived,
}

impl TaxRecordStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::Required => "required",
            Self::Drafted => "drafted",
            Self::Validated => "validated",
            Self::Reported => "reported",
            Self::Paid => "paid",
            Self::Archived => "archived",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "not_required" | "notrequired" => Self::NotRequired,
            "required" => Self::Required,
            "drafted" => Self::Drafted,
            "validated" => Self::Validated,
            "reported" => Self::Reported,
            "paid" => Self::Paid,
            "archived" => Self::Archived,
            _ => Self::Drafted,
        }
    }
}

/// Tax calendar entry representing a specific tax filing and payment due date.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxCalendarEntry {
    pub id: Uuid,
    pub company_id: Uuid,
    pub tax_type_id: Uuid,
    pub tax_period: time::Date,
    pub payment_due_date: time::Date,
    pub filing_due_date: time::Date,
    pub payment_status: TaxPaymentStatus,
    pub filing_status: TaxFilingStatus,
    pub reminder_sent_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaxPaymentStatus {
    Unpaid,
    Paid,
}

impl TaxPaymentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unpaid => "unpaid",
            Self::Paid => "paid",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "paid" => Self::Paid,
            _ => Self::Unpaid,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaxFilingStatus {
    Unfiled,
    Filed,
}

impl TaxFilingStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unfiled => "unfiled",
            Self::Filed => "filed",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "filed" => Self::Filed,
            _ => Self::Unfiled,
        }
    }
}

