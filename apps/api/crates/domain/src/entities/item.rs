use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemCategory {
    pub id: Uuid,
    pub company_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: Uuid,
    pub company_id: Uuid,
    pub category_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub unit_price: Decimal,
    pub sale_account_id: Option<Uuid>,
    pub purchase_account_id: Option<Uuid>,
    pub tax_type_id: Option<Uuid>,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
