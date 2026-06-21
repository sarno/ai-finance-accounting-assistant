use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::value_objects::{AccountCode, AccountType};

/// Chart of Account entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub company_id: Uuid,
    pub code: AccountCode,
    pub name: String,
    pub account_type: AccountType,
    pub parent_id: Option<Uuid>,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Account {
    pub fn new(
        company_id: Uuid,
        code: AccountCode,
        name: String,
        account_type: AccountType,
        parent_id: Option<Uuid>,
    ) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id: Uuid::new_v4(),
            company_id,
            code,
            name,
            account_type,
            parent_id,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = OffsetDateTime::now_utc();
    }
}
