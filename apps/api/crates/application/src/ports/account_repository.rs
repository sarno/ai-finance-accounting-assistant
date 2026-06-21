use async_trait::async_trait;
use uuid::Uuid;

use crate::errors::AppError;
use finance_assistant_domain::entities::account::Account;
use finance_assistant_domain::value_objects::AccountCode;

#[async_trait]
pub trait AccountRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Account, AppError>;
    async fn find_by_code(
        &self,
        company_id: Uuid,
        code: &AccountCode,
    ) -> Result<Option<Account>, AppError>;
    async fn find_all_by_company(&self, company_id: Uuid) -> Result<Vec<Account>, AppError>;
    async fn save(&self, account: &Account) -> Result<(), AppError>;
    async fn update(&self, account: &Account) -> Result<(), AppError>;
}
