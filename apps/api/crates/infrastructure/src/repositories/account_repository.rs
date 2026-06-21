use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use finance_assistant_app::{errors::AppError, ports::account_repository::AccountRepository};
use finance_assistant_domain::{
    entities::account::Account,
    value_objects::{AccountCode, AccountType},
};

pub struct PgAccountRepository {
    pool: PgPool,
}

impl PgAccountRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AccountRepository for PgAccountRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Account, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, company_id, code, name, account_type, parent_id, is_active, created_at, updated_at
            FROM chart_of_accounts
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
                    resource: "Account".to_string(),
                    id: id.to_string(),
                })
            }
        };

        let type_str: String = row.get("account_type");
        let account_type = match type_str.to_lowercase().as_str() {
            "asset" => AccountType::Asset,
            "liability" => AccountType::Liability,
            "equity" => AccountType::Equity,
            "revenue" => AccountType::Revenue,
            "expense" => AccountType::Expense,
            _ => AccountType::Asset,
        };

        Ok(Account {
            id: row.get("id"),
            company_id: row.get("company_id"),
            code: AccountCode::new(row.get::<String, _>("code")),
            name: row.get("name"),
            account_type,
            parent_id: row.get("parent_id"),
            is_active: row.get("is_active"),
            created_at: row.get::<time::OffsetDateTime, _>("created_at"),
            updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
        })
    }

    async fn find_by_code(
        &self,
        company_id: Uuid,
        code: &AccountCode,
    ) -> Result<Option<Account>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, company_id, code, name, account_type, parent_id, is_active, created_at, updated_at
            FROM chart_of_accounts
            WHERE company_id = $1 AND code = $2
            "#,
        )
        .bind(company_id)
        .bind(code.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let row = match row {
            Some(r) => r,
            None => return Ok(None),
        };

        let type_str: String = row.get("account_type");
        let account_type = match type_str.to_lowercase().as_str() {
            "asset" => AccountType::Asset,
            "liability" => AccountType::Liability,
            "equity" => AccountType::Equity,
            "revenue" => AccountType::Revenue,
            "expense" => AccountType::Expense,
            _ => AccountType::Asset,
        };

        Ok(Some(Account {
            id: row.get("id"),
            company_id: row.get("company_id"),
            code: AccountCode::new(row.get::<String, _>("code")),
            name: row.get("name"),
            account_type,
            parent_id: row.get("parent_id"),
            is_active: row.get("is_active"),
            created_at: row.get::<time::OffsetDateTime, _>("created_at"),
            updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
        }))
    }

    async fn find_all_by_company(&self, company_id: Uuid) -> Result<Vec<Account>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, company_id, code, name, account_type, parent_id, is_active, created_at, updated_at
            FROM chart_of_accounts
            WHERE company_id = $1
            ORDER BY code ASC
            "#,
        )
        .bind(company_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let accounts = rows
            .into_iter()
            .map(|row| {
                let type_str: String = row.get("account_type");
                let account_type = match type_str.to_lowercase().as_str() {
                    "asset" => AccountType::Asset,
                    "liability" => AccountType::Liability,
                    "equity" => AccountType::Equity,
                    "revenue" => AccountType::Revenue,
                    "expense" => AccountType::Expense,
                    _ => AccountType::Asset,
                };
                Account {
                    id: row.get("id"),
                    company_id: row.get("company_id"),
                    code: AccountCode::new(row.get::<String, _>("code")),
                    name: row.get("name"),
                    account_type,
                    parent_id: row.get("parent_id"),
                    is_active: row.get("is_active"),
                    created_at: row.get::<time::OffsetDateTime, _>("created_at"),
                    updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
                }
            })
            .collect();

        Ok(accounts)
    }

    async fn save(&self, account: &Account) -> Result<(), AppError> {
        let type_str = match account.account_type {
            AccountType::Asset => "Asset",
            AccountType::Liability => "Liability",
            AccountType::Equity => "Equity",
            AccountType::Revenue => "Revenue",
            AccountType::Expense => "Expense",
        };

        sqlx::query(
            r#"
            INSERT INTO chart_of_accounts (id, company_id, code, name, account_type, parent_id, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(account.id)
        .bind(account.company_id)
        .bind(account.code.as_str())
        .bind(&account.name)
        .bind(type_str)
        .bind(account.parent_id)
        .bind(account.is_active)
        .bind(account.created_at)
        .bind(account.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }

    async fn update(&self, account: &Account) -> Result<(), AppError> {
        let type_str = match account.account_type {
            AccountType::Asset => "Asset",
            AccountType::Liability => "Liability",
            AccountType::Equity => "Equity",
            AccountType::Revenue => "Revenue",
            AccountType::Expense => "Expense",
        };

        sqlx::query(
            r#"
            UPDATE chart_of_accounts
            SET code = $2, name = $3, account_type = $4, parent_id = $5, is_active = $6, updated_at = $7
            WHERE id = $1
            "#,
        )
        .bind(account.id)
        .bind(account.code.as_str())
        .bind(&account.name)
        .bind(type_str)
        .bind(account.parent_id)
        .bind(account.is_active)
        .bind(account.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }
}
