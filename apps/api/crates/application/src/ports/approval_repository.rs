use async_trait::async_trait;
use uuid::Uuid;

use crate::errors::AppError;
use finance_assistant_domain::entities::approval::ApprovalRequest;

#[async_trait]
pub trait ApprovalRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<ApprovalRequest, AppError>;
    async fn find_pending_by_company(
        &self,
        company_id: Uuid,
    ) -> Result<Vec<ApprovalRequest>, AppError>;
    async fn find_by_document(
        &self,
        document_id: Uuid,
    ) -> Result<Option<ApprovalRequest>, AppError>;
    async fn save(&self, request: &ApprovalRequest) -> Result<(), AppError>;
    async fn update(&self, request: &ApprovalRequest) -> Result<(), AppError>;
}
