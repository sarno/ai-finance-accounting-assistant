use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use finance_assistant_domain::entities::journal::{JournalEntry, JournalLine, JournalSource};
use finance_assistant_domain::value_objects::DocumentStatus;

// ─── Request ──────────────────────────────────────────────────────────────────
 
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateJournalDraftRequest {
    pub company_id: Uuid,
    pub branch_id: Option<Uuid>,
    pub reference_number: Option<String>,
    pub description: String,
    #[serde(with = "crate::dto::date_format")]
    pub transaction_date: time::Date,
    pub lines: Vec<CreateJournalLineRequest>,
}
 
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateJournalLineRequest {
    pub account_id: Uuid,
    pub debit: Decimal,
    pub credit: Decimal,
    pub description: Option<String>,
}
 
// ─── Response ─────────────────────────────────────────────────────────────────
 
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JournalResponse {
    pub id: Uuid,
    pub company_id: Uuid,
    pub branch_id: Option<Uuid>,
    pub reference_number: String,
    pub description: String,
    #[serde(with = "crate::dto::date_format")]
    pub transaction_date: time::Date,
    pub lines: Vec<JournalLineResponse>,
    pub status: DocumentStatus,
    pub source: JournalSource,
    pub created_by: Uuid,
    pub created_at: time::OffsetDateTime,
}
 
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JournalLineResponse {
    pub id: Uuid,
    pub account_id: Uuid,
    pub debit: Decimal,
    pub credit: Decimal,
    pub description: Option<String>,
}

impl From<JournalEntry> for JournalResponse {
    fn from(e: JournalEntry) -> Self {
        Self {
            id: e.id,
            company_id: e.company_id,
            branch_id: e.branch_id,
            reference_number: e.reference_number,
            description: e.description,
            transaction_date: e.transaction_date,
            lines: e.lines.into_iter().map(JournalLineResponse::from).collect(),
            status: e.status,
            source: e.source,
            created_by: e.created_by,
            created_at: e.created_at,
        }
    }
}

impl From<JournalLine> for JournalLineResponse {
    fn from(l: JournalLine) -> Self {
        Self {
            id: l.id,
            account_id: l.account_id,
            debit: l.debit,
            credit: l.credit,
            description: l.description,
        }
    }
}
