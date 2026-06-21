use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use finance_assistant_app::{errors::AppError, ports::item_repository::ItemRepository};
use finance_assistant_domain::entities::item::{Item, ItemCategory};

pub struct PgItemRepository {
    pool: PgPool,
}

impl PgItemRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ItemRepository for PgItemRepository {
    // ─── Item Categories ─────────────────────────────────────────────────────

    async fn find_category_by_id(&self, id: Uuid) -> Result<ItemCategory, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, company_id, name, description, is_active, created_at, updated_at
            FROM item_categories
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let row = match row {
            Some(r) => r,
            None => {
                return Err(AppError::NotFound {
                    resource: "ItemCategory".to_string(),
                    id: id.to_string(),
                })
            }
        };

        Ok(ItemCategory {
            id: row.get("id"),
            company_id: row.get("company_id"),
            name: row.get("name"),
            description: row.get("description"),
            is_active: row.get("is_active"),
            created_at: row.get::<time::OffsetDateTime, _>("created_at"),
            updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
        })
    }

    async fn find_categories_by_company(
        &self,
        company_id: Uuid,
    ) -> Result<Vec<ItemCategory>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, company_id, name, description, is_active, created_at, updated_at
            FROM item_categories
            WHERE company_id = $1
            ORDER BY name ASC
            "#,
        )
        .bind(company_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let categories = rows
            .into_iter()
            .map(|row| ItemCategory {
                id: row.get("id"),
                company_id: row.get("company_id"),
                name: row.get("name"),
                description: row.get("description"),
                is_active: row.get("is_active"),
                created_at: row.get::<time::OffsetDateTime, _>("created_at"),
                updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
            })
            .collect();

        Ok(categories)
    }

    async fn save_category(&self, category: &ItemCategory) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO item_categories (id, company_id, name, description, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(category.id)
        .bind(category.company_id)
        .bind(&category.name)
        .bind(&category.description)
        .bind(category.is_active)
        .bind(category.created_at)
        .bind(category.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }

    async fn update_category(&self, category: &ItemCategory) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE item_categories
            SET name = $2, description = $3, is_active = $4, updated_at = $5
            WHERE id = $1
            "#,
        )
        .bind(category.id)
        .bind(&category.name)
        .bind(&category.description)
        .bind(category.is_active)
        .bind(category.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }

    async fn delete_category(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            r#"
            DELETE FROM item_categories
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }

    // ─── Items ───────────────────────────────────────────────────────────────

    async fn find_item_by_id(&self, id: Uuid) -> Result<Item, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, company_id, category_id, code, name, description, unit_price, sale_account_id, purchase_account_id, tax_type_id, is_active, created_at, updated_at
            FROM items
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let row = match row {
            Some(r) => r,
            None => {
                return Err(AppError::NotFound {
                    resource: "Item".to_string(),
                    id: id.to_string(),
                })
            }
        };

        Ok(Item {
            id: row.get("id"),
            company_id: row.get("company_id"),
            category_id: row.get("category_id"),
            code: row.get("code"),
            name: row.get("name"),
            description: row.get("description"),
            unit_price: row.get("unit_price"),
            sale_account_id: row.get("sale_account_id"),
            purchase_account_id: row.get("purchase_account_id"),
            tax_type_id: row.get("tax_type_id"),
            is_active: row.get("is_active"),
            created_at: row.get::<time::OffsetDateTime, _>("created_at"),
            updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
        })
    }

    async fn find_items_by_company(&self, company_id: Uuid) -> Result<Vec<Item>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, company_id, category_id, code, name, description, unit_price, sale_account_id, purchase_account_id, tax_type_id, is_active, created_at, updated_at
            FROM items
            WHERE company_id = $1
            ORDER BY code ASC
            "#,
        )
        .bind(company_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let items = rows
            .into_iter()
            .map(|row| Item {
                id: row.get("id"),
                company_id: row.get("company_id"),
                category_id: row.get("category_id"),
                code: row.get("code"),
                name: row.get("name"),
                description: row.get("description"),
                unit_price: row.get("unit_price"),
                sale_account_id: row.get("sale_account_id"),
                purchase_account_id: row.get("purchase_account_id"),
                tax_type_id: row.get("tax_type_id"),
                is_active: row.get("is_active"),
                created_at: row.get::<time::OffsetDateTime, _>("created_at"),
                updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
            })
            .collect();

        Ok(items)
    }

    async fn save_item(&self, item: &Item) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO items (id, company_id, category_id, code, name, description, unit_price, sale_account_id, purchase_account_id, tax_type_id, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            "#,
        )
        .bind(item.id)
        .bind(item.company_id)
        .bind(item.category_id)
        .bind(&item.code)
        .bind(&item.name)
        .bind(&item.description)
        .bind(item.unit_price)
        .bind(item.sale_account_id)
        .bind(item.purchase_account_id)
        .bind(item.tax_type_id)
        .bind(item.is_active)
        .bind(item.created_at)
        .bind(item.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }

    async fn update_item(&self, item: &Item) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE items
            SET category_id = $2, code = $3, name = $4, description = $5, unit_price = $6, sale_account_id = $7, purchase_account_id = $8, tax_type_id = $9, is_active = $10, updated_at = $11
            WHERE id = $1
            "#,
        )
        .bind(item.id)
        .bind(item.category_id)
        .bind(&item.code)
        .bind(&item.name)
        .bind(&item.description)
        .bind(item.unit_price)
        .bind(item.sale_account_id)
        .bind(item.purchase_account_id)
        .bind(item.tax_type_id)
        .bind(item.is_active)
        .bind(item.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }

    async fn delete_item(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            r#"
            DELETE FROM items
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }
}
