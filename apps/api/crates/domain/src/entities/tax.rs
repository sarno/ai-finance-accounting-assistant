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
