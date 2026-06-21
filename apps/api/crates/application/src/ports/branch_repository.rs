use async_trait::async_trait;
use uuid::Uuid;

use crate::errors::AppError;
use finance_assistant_domain::entities::branch::Branch;

#[async_trait]
pub trait BranchRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Branch, AppError>;
    async fn find_by_company(&self, company_id: Uuid) -> Result<Vec<Branch>, AppError>;
    async fn save(&self, branch: &Branch) -> Result<(), AppError>;
    async fn update(&self, branch: &Branch) -> Result<(), AppError>;
}
