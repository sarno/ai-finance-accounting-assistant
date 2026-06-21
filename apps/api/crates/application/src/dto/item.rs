use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use finance_assistant_domain::entities::item::{ItemCategory, Item};

// ─── Item Category DTOs ──────────────────────────────────────────────────────
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateItemCategoryRequest {
    pub company_id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateItemCategoryRequest {
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemCategoryResponse {
    pub id: Uuid,
    pub company_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    #[serde(with = "crate::dto::datetime_format")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "crate::dto::datetime_format")]
    pub updated_at: time::OffsetDateTime,
}

impl From<ItemCategory> for ItemCategoryResponse {
    fn from(c: ItemCategory) -> Self {
        Self {
            id: c.id,
            company_id: c.company_id,
            name: c.name,
            description: c.description,
            is_active: c.is_active,
            created_at: c.created_at,
            updated_at: c.updated_at,
        }
    }
}

// ─── Item DTOs ───────────────────────────────────────────────────────────────
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateItemRequest {
    pub company_id: Uuid,
    pub category_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub unit_price: Decimal,
    pub sale_account_id: Option<Uuid>,
    pub purchase_account_id: Option<Uuid>,
    pub tax_type_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateItemRequest {
    pub category_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub unit_price: Decimal,
    pub sale_account_id: Option<Uuid>,
    pub purchase_account_id: Option<Uuid>,
    pub tax_type_id: Option<Uuid>,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemResponse {
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
    #[serde(with = "crate::dto::datetime_format")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "crate::dto::datetime_format")]
    pub updated_at: time::OffsetDateTime,
}

impl From<Item> for ItemResponse {
    fn from(i: Item) -> Self {
        Self {
            id: i.id,
            company_id: i.company_id,
            category_id: i.category_id,
            code: i.code,
            name: i.name,
            description: i.description,
            unit_price: i.unit_price,
            sale_account_id: i.sale_account_id,
            purchase_account_id: i.purchase_account_id,
            tax_type_id: i.tax_type_id,
            is_active: i.is_active,
            created_at: i.created_at,
            updated_at: i.updated_at,
        }
    }
}
