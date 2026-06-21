use async_trait::async_trait;
use uuid::Uuid;

use crate::errors::AppError;
use finance_assistant_domain::entities::customer::Customer;

#[async_trait]
pub trait CustomerRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Customer, AppError>;
    async fn find_all_by_company(&self, company_id: Uuid) -> Result<Vec<Customer>, AppError>;
    async fn save(&self, customer: &Customer) -> Result<(), AppError>;
    async fn update(&self, customer: &Customer) -> Result<(), AppError>;
}
