use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use finance_assistant_app::{errors::AppError, ports::branch_repository::BranchRepository};
use finance_assistant_domain::entities::branch::Branch;

pub struct PgBranchRepository {
    pool: PgPool,
}

impl PgBranchRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BranchRepository for PgBranchRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Branch, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, company_id, code, name, address, phone, is_active, created_at, updated_at
            FROM branches
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
                    resource: "Branch".to_string(),
                    id: id.to_string(),
                })
            }
        };

        Ok(Branch {
            id: row.get("id"),
            company_id: row.get("company_id"),
            code: row.get("code"),
            name: row.get("name"),
            address: row.get("address"),
            phone: row.get("phone"),
            is_active: row.get("is_active"),
            created_at: row.get::<time::OffsetDateTime, _>("created_at"),
            updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
        })
    }

    async fn find_by_company(&self, company_id: Uuid) -> Result<Vec<Branch>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, company_id, code, name, address, phone, is_active, created_at, updated_at
            FROM branches
            WHERE company_id = $1
            ORDER BY code ASC
            "#,
        )
        .bind(company_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let branches = rows
            .into_iter()
            .map(|row| Branch {
                id: row.get("id"),
                company_id: row.get("company_id"),
                code: row.get("code"),
                name: row.get("name"),
                address: row.get("address"),
                phone: row.get("phone"),
                is_active: row.get("is_active"),
                created_at: row.get::<time::OffsetDateTime, _>("created_at"),
                updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
            })
            .collect();

        Ok(branches)
    }

    async fn save(&self, branch: &Branch) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO branches (id, company_id, code, name, address, phone, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(branch.id)
        .bind(branch.company_id)
        .bind(&branch.code)
        .bind(&branch.name)
        .bind(&branch.address)
        .bind(&branch.phone)
        .bind(branch.is_active)
        .bind(branch.created_at)
        .bind(branch.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }

    async fn update(&self, branch: &Branch) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE branches
            SET code = $2, name = $3, address = $4, phone = $5, is_active = $6, updated_at = $7
            WHERE id = $1
            "#,
        )
        .bind(branch.id)
        .bind(&branch.code)
        .bind(&branch.name)
        .bind(&branch.address)
        .bind(&branch.phone)
        .bind(branch.is_active)
        .bind(branch.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }
}
