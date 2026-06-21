use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Strongly typed account code (e.g. "1001", "2100").
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountCode(pub String);

impl AccountCode {
    pub fn new(code: impl Into<String>) -> Self {
        Self(code.into())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AccountCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Account types for classification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

/// Money value object (always stored as Decimal + currency code).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    pub amount: Decimal,
    pub currency: CurrencyCode,
}

impl Money {
    pub fn idr(amount: Decimal) -> Self {
        Self { amount, currency: CurrencyCode::IDR }
    }

    pub fn zero() -> Self {
        Self { amount: Decimal::ZERO, currency: CurrencyCode::IDR }
    }
}

/// ISO 4217 currency code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurrencyCode {
    IDR,
    USD,
    EUR,
    SGD,
}

impl Default for CurrencyCode {
    fn default() -> Self {
        CurrencyCode::IDR
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "document_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DocumentStatus {
    Draft,
    WaitingReview,
    WaitingApproval,
    Approved,
    Posted,
    Rejected,
    Cancelled,
}

impl std::fmt::Display for DocumentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Draft => "draft",
            Self::WaitingReview => "waiting_review",
            Self::WaitingApproval => "waiting_approval",
            Self::Approved => "approved",
            Self::Posted => "posted",
            Self::Rejected => "rejected",
            Self::Cancelled => "cancelled",
        };
        write!(f, "{}", s)
    }
}
