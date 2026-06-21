use async_trait::async_trait;
use uuid::Uuid;

use finance_assistant_domain::entities::bank_account::BankAccount;
use crate::errors::AppError;

#[async_trait]
pub trait BankAccountRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<BankAccount, AppError>;
    async fn find_all_by_company(&self, company_id: Uuid) -> Result<Vec<BankAccount>, AppError>;
    async fn save(&self, bank_account: &BankAccount) -> Result<(), AppError>;
    async fn update(&self, bank_account: &BankAccount) -> Result<(), AppError>;
}
