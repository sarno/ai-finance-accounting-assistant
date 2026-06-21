use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: Uuid,
    pub name: String,
    pub tax_number: Option<String>,
    pub address: Option<String>,
    pub currency: String,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Company {
    pub fn new(
        name: String,
        tax_number: Option<String>,
        address: Option<String>,
        currency: String,
    ) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id: Uuid::new_v4(),
            name,
            tax_number,
            address,
            currency,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
}
