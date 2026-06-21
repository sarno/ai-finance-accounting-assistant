use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use finance_assistant_app::{
    errors::AppError,
    ports::company_repository::CompanyRepository,
};
use finance_assistant_domain::entities::company::Company;

pub struct PgCompanyRepository {
    pool: PgPool,
}

impl PgCompanyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CompanyRepository for PgCompanyRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Company, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, name, tax_number, address, currency, is_active, created_at, updated_at
            FROM companies
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
                resource: "Company".to_string(),
                id: id.to_string(),
            }),
        };

        Ok(Company {
            id: row.get("id"),
            name: row.get("name"),
            tax_number: row.get("tax_number"),
            address: row.get("address"),
            currency: row.get("currency"),
            is_active: row.get("is_active"),
            created_at: row.get::<time::OffsetDateTime, _>("created_at"),
            updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
        })
    }

    async fn find_all(&self) -> Result<Vec<Company>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, tax_number, address, currency, is_active, created_at, updated_at
            FROM companies
            ORDER BY name ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let companies = rows
            .into_iter()
            .map(|row| Company {
                id: row.get("id"),
                name: row.get("name"),
                tax_number: row.get("tax_number"),
                address: row.get("address"),
                currency: row.get("currency"),
                is_active: row.get("is_active"),
                created_at: row.get::<time::OffsetDateTime, _>("created_at"),
                updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
            })
            .collect();

        Ok(companies)
    }

    async fn save(&self, company: &Company) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO companies (id, name, tax_number, address, currency, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(company.id)
        .bind(&company.name)
        .bind(&company.tax_number)
        .bind(&company.address)
        .bind(&company.currency)
        .bind(company.is_active)
        .bind(company.created_at)
        .bind(company.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }

    async fn update(&self, company: &Company) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE companies
            SET name = $2, tax_number = $3, address = $4, currency = $5, is_active = $6, updated_at = $7
            WHERE id = $1
            "#,
        )
        .bind(company.id)
        .bind(&company.name)
        .bind(&company.tax_number)
        .bind(&company.address)
        .bind(&company.currency)
        .bind(company.is_active)
        .bind(company.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }
}
