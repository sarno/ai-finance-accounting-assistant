use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use finance_assistant_app::{
    errors::AppError,
    ports::customer_repository::CustomerRepository,
};
use finance_assistant_domain::entities::customer::Customer;

pub struct PgCustomerRepository {
    pool: PgPool,
}

impl PgCustomerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CustomerRepository for PgCustomerRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Customer, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, company_id, name, tax_number, email, phone, address, is_active, created_at, updated_at
            FROM customers
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
                resource: "Customer".to_string(),
                id: id.to_string(),
            }),
        };

        Ok(Customer {
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

    async fn find_all_by_company(&self, company_id: Uuid) -> Result<Vec<Customer>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, company_id, name, tax_number, email, phone, address, is_active, created_at, updated_at
            FROM customers
            WHERE company_id = $1
            ORDER BY name ASC
            "#,
        )
        .bind(company_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let customers = rows
            .into_iter()
            .map(|row| Customer {
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

        Ok(customers)
    }

    async fn save(&self, customer: &Customer) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO customers (id, company_id, name, tax_number, email, phone, address, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
        )
        .bind(customer.id)
        .bind(customer.company_id)
        .bind(&customer.name)
        .bind(&customer.tax_number)
        .bind(&customer.email)
        .bind(&customer.phone)
        .bind(&customer.address)
        .bind(customer.is_active)
        .bind(customer.created_at)
        .bind(customer.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }

    async fn update(&self, customer: &Customer) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE customers
            SET name = $2, tax_number = $3, email = $4, phone = $5, address = $6, is_active = $7, updated_at = $8
            WHERE id = $1
            "#,
        )
        .bind(customer.id)
        .bind(&customer.name)
        .bind(&customer.tax_number)
        .bind(&customer.email)
        .bind(&customer.phone)
        .bind(&customer.address)
        .bind(customer.is_active)
        .bind(customer.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }
}
