use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Supplier {
    pub id: Uuid,
    pub company_id: Uuid,
    pub name: String,
    pub tax_number: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Supplier {
    pub fn new(
        company_id: Uuid,
        name: String,
        tax_number: Option<String>,
        email: Option<String>,
        phone: Option<String>,
        address: Option<String>,
    ) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id: Uuid::new_v4(),
            company_id,
            name,
            tax_number,
            email,
            phone,
            address,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
}
