use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccount {
    pub id: Uuid,
    pub company_id: Uuid,
    pub account_id: Uuid,
    pub bank_name: String,
    pub account_number: String,
    pub account_name: String,
    pub currency: String,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl BankAccount {
    pub fn new(
        company_id: Uuid,
        account_id: Uuid,
        bank_name: String,
        account_number: String,
        account_name: String,
        currency: String,
    ) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id: Uuid::new_v4(),
            company_id,
            account_id,
            bank_name,
            account_number,
            account_name,
            currency,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
}
