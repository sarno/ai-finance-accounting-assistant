use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use finance_assistant_app::{
    errors::AppError,
    ports::bank_account_repository::BankAccountRepository,
};
use finance_assistant_domain::entities::bank_account::BankAccount;

pub struct PgBankAccountRepository {
    pool: PgPool,
}

impl PgBankAccountRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BankAccountRepository for PgBankAccountRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<BankAccount, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, company_id, account_id, bank_name, account_number, account_name, currency, is_active, created_at, updated_at
            FROM bank_accounts
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
                resource: "BankAccount".to_string(),
                id: id.to_string(),
            }),
        };

        Ok(BankAccount {
            id: row.get("id"),
            company_id: row.get("company_id"),
            account_id: row.get("account_id"),
            bank_name: row.get("bank_name"),
            account_number: row.get("account_number"),
            account_name: row.get("account_name"),
            currency: row.get("currency"),
            is_active: row.get("is_active"),
            created_at: row.get::<time::OffsetDateTime, _>("created_at"),
            updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
        })
    }

    async fn find_all_by_company(&self, company_id: Uuid) -> Result<Vec<BankAccount>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, company_id, account_id, bank_name, account_number, account_name, currency, is_active, created_at, updated_at
            FROM bank_accounts
            WHERE company_id = $1
            ORDER BY bank_name ASC
            "#,
        )
        .bind(company_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let accounts = rows
            .into_iter()
            .map(|row| BankAccount {
                id: row.get("id"),
                company_id: row.get("company_id"),
                account_id: row.get("account_id"),
                bank_name: row.get("bank_name"),
                account_number: row.get("account_number"),
                account_name: row.get("account_name"),
                currency: row.get("currency"),
                is_active: row.get("is_active"),
                created_at: row.get::<time::OffsetDateTime, _>("created_at"),
                updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
            })
            .collect();

        Ok(accounts)
    }

    async fn save(&self, bank_account: &BankAccount) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO bank_accounts (id, company_id, account_id, bank_name, account_number, account_name, currency, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
        )
        .bind(bank_account.id)
        .bind(bank_account.company_id)
        .bind(bank_account.account_id)
        .bind(&bank_account.bank_name)
        .bind(&bank_account.account_number)
        .bind(&bank_account.account_name)
        .bind(&bank_account.currency)
        .bind(bank_account.is_active)
        .bind(bank_account.created_at)
        .bind(bank_account.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }

    async fn update(&self, bank_account: &BankAccount) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE bank_accounts
            SET account_id = $2, bank_name = $3, account_number = $4, account_name = $5, currency = $6, is_active = $7, updated_at = $8
            WHERE id = $1
            "#,
        )
        .bind(bank_account.id)
        .bind(bank_account.account_id)
        .bind(&bank_account.bank_name)
        .bind(&bank_account.account_number)
        .bind(&bank_account.account_name)
        .bind(&bank_account.currency)
        .bind(bank_account.is_active)
        .bind(bank_account.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }
}
