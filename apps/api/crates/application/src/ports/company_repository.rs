use async_trait::async_trait;
use uuid::Uuid;

use finance_assistant_domain::entities::company::Company;
use crate::errors::AppError;

#[async_trait]
pub trait CompanyRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Company, AppError>;
    async fn find_all(&self) -> Result<Vec<Company>, AppError>;
    async fn save(&self, company: &Company) -> Result<(), AppError>;
    async fn update(&self, company: &Company) -> Result<(), AppError>;
}
