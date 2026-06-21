use std::sync::Arc;
use uuid::Uuid;
use time::OffsetDateTime;
use finance_assistant_domain::entities::item::{ItemCategory, Item};
use crate::{
    dto::item::*,
    errors::AppError,
    ports::item_repository::ItemRepository,
};

pub struct ItemService {
    item_repo: Arc<dyn ItemRepository>,
}

impl ItemService {
    pub fn new(item_repo: Arc<dyn ItemRepository>) -> Self {
        Self { item_repo }
    }

    // ─── Item Categories ─────────────────────────────────────────────────────

    pub async fn create_category(&self, req: CreateItemCategoryRequest) -> Result<ItemCategoryResponse, AppError> {
        let now = OffsetDateTime::now_utc();
        let category = ItemCategory {
            id: Uuid::new_v4(),
            company_id: req.company_id,
            name: req.name,
            description: req.description,
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        self.item_repo.save_category(&category).await?;
        Ok(ItemCategoryResponse::from(category))
    }

    pub async fn get_category(&self, id: Uuid) -> Result<ItemCategoryResponse, AppError> {
        let category = self.item_repo.find_category_by_id(id).await?;
        Ok(ItemCategoryResponse::from(category))
    }

    pub async fn update_category(&self, id: Uuid, req: UpdateItemCategoryRequest) -> Result<ItemCategoryResponse, AppError> {
        let mut category = self.item_repo.find_category_by_id(id).await?;
        category.name = req.name;
        category.description = req.description;
        category.is_active = req.is_active;
        category.updated_at = OffsetDateTime::now_utc();

        self.item_repo.update_category(&category).await?;
        Ok(ItemCategoryResponse::from(category))
    }

    pub async fn delete_category(&self, id: Uuid) -> Result<(), AppError> {
        self.item_repo.delete_category(id).await?;
        Ok(())
    }

    pub async fn list_categories(&self, company_id: Uuid) -> Result<Vec<ItemCategoryResponse>, AppError> {
        let categories = self.item_repo.find_categories_by_company(company_id).await?;
        Ok(categories.into_iter().map(ItemCategoryResponse::from).collect())
    }

    // ─── Items ───────────────────────────────────────────────────────────────

    pub async fn create_item(&self, req: CreateItemRequest) -> Result<ItemResponse, AppError> {
        let now = OffsetDateTime::now_utc();
        let item = Item {
            id: Uuid::new_v4(),
            company_id: req.company_id,
            category_id: req.category_id,
            code: req.code,
            name: req.name,
            description: req.description,
            unit_price: req.unit_price,
            sale_account_id: req.sale_account_id,
            purchase_account_id: req.purchase_account_id,
            tax_type_id: req.tax_type_id,
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        self.item_repo.save_item(&item).await?;
        Ok(ItemResponse::from(item))
    }

    pub async fn get_item(&self, id: Uuid) -> Result<ItemResponse, AppError> {
        let item = self.item_repo.find_item_by_id(id).await?;
        Ok(ItemResponse::from(item))
    }

    pub async fn update_item(&self, id: Uuid, req: UpdateItemRequest) -> Result<ItemResponse, AppError> {
        let mut item = self.item_repo.find_item_by_id(id).await?;
        item.category_id = req.category_id;
        item.code = req.code;
        item.name = req.name;
        item.description = req.description;
        item.unit_price = req.unit_price;
        item.sale_account_id = req.sale_account_id;
        item.purchase_account_id = req.purchase_account_id;
        item.tax_type_id = req.tax_type_id;
        item.is_active = req.is_active;
        item.updated_at = OffsetDateTime::now_utc();

        self.item_repo.update_item(&item).await?;
        Ok(ItemResponse::from(item))
    }

    pub async fn delete_item(&self, id: Uuid) -> Result<(), AppError> {
        self.item_repo.delete_item(id).await?;
        Ok(())
    }

    pub async fn list_items(&self, company_id: Uuid) -> Result<Vec<ItemResponse>, AppError> {
        let items = self.item_repo.find_items_by_company(company_id).await?;
        Ok(items.into_iter().map(ItemResponse::from).collect())
    }
}
