use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use finance_assistant_app::{
    errors::AppError,
    ports::supplier_repository::SupplierRepository,
};
use finance_assistant_domain::entities::supplier::Supplier;

pub struct PgSupplierRepository {
    pool: PgPool,
}

impl PgSupplierRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SupplierRepository for PgSupplierRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Supplier, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, company_id, name, tax_number, email, phone, address, is_active, created_at, updated_at
            FROM suppliers
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let row = match row {
            Some(r) => r,
            None => return Err(AppError::NotFound {
                resource: "Supplier".to_string(),
                id: id.to_string(),
            }),
        };

        Ok(Supplier {
            id: row.get("id"),
            company_id: row.get("company_id"),
            name: row.get("name"),
            tax_number: row.get("tax_number"),
            email: row.get("email"),
            phone: row.get("phone"),
            address: row.get("address"),
            is_active: row.get("is_active"),
            created_at: row.get::<time::OffsetDateTime, _>("created_at"),
            updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
        })
    }

    async fn find_all_by_company(&self, company_id: Uuid) -> Result<Vec<Supplier>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, company_id, name, tax_number, email, phone, address, is_active, created_at, updated_at
            FROM suppliers
            WHERE company_id = $1
            ORDER BY name ASC
            "#,
        )
        .bind(company_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let suppliers = rows
            .into_iter()
            .map(|row| Supplier {
                id: row.get("id"),
                company_id: row.get("company_id"),
                name: row.get("name"),
                tax_number: row.get("tax_number"),
                email: row.get("email"),
                phone: row.get("phone"),
                address: row.get("address"),
                is_active: row.get("is_active"),
                created_at: row.get::<time::OffsetDateTime, _>("created_at"),
                updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
            })
            .collect();

        Ok(suppliers)
    }

    async fn save(&self, supplier: &Supplier) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO suppliers (id, company_id, name, tax_number, email, phone, address, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
        )
        .bind(supplier.id)
        .bind(supplier.company_id)
        .bind(&supplier.name)
        .bind(&supplier.tax_number)
        .bind(&supplier.email)
        .bind(&supplier.phone)
        .bind(&supplier.address)
        .bind(supplier.is_active)
        .bind(supplier.created_at)
        .bind(supplier.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }

    async fn update(&self, supplier: &Supplier) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE suppliers
            SET name = $2, tax_number = $3, email = $4, phone = $5, address = $6, is_active = $7, updated_at = $8
            WHERE id = $1
            "#,
        )
        .bind(supplier.id)
        .bind(&supplier.name)
        .bind(&supplier.tax_number)
        .bind(&supplier.email)
        .bind(&supplier.phone)
        .bind(&supplier.address)
        .bind(supplier.is_active)
        .bind(supplier.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }
}
