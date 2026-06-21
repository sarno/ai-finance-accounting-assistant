use async_trait::async_trait;
use uuid::Uuid;
use finance_assistant_domain::entities::item::{ItemCategory, Item};
use crate::errors::AppError;

#[async_trait]
pub trait ItemRepository: Send + Sync {
    // Categories
    async fn find_category_by_id(&self, id: Uuid) -> Result<ItemCategory, AppError>;
    async fn find_categories_by_company(&self, company_id: Uuid) -> Result<Vec<ItemCategory>, AppError>;
    async fn save_category(&self, category: &ItemCategory) -> Result<(), AppError>;
    async fn update_category(&self, category: &ItemCategory) -> Result<(), AppError>;
    async fn delete_category(&self, id: Uuid) -> Result<(), AppError>;

    // Items
    async fn find_item_by_id(&self, id: Uuid) -> Result<Item, AppError>;
    async fn find_items_by_company(&self, company_id: Uuid) -> Result<Vec<Item>, AppError>;
    async fn save_item(&self, item: &Item) -> Result<(), AppError>;
    async fn update_item(&self, item: &Item) -> Result<(), AppError>;
    async fn delete_item(&self, id: Uuid) -> Result<(), AppError>;
}
