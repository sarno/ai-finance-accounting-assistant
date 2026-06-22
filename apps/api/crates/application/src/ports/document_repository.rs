use crate::errors::AppError;
use async_trait::async_trait;
use finance_assistant_domain::entities::uploaded_document::UploadedDocument;
use uuid::Uuid;

#[async_trait]
pub trait DocumentRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<UploadedDocument, AppError>;
    async fn find_by_company(
        &self,
        company_id: Uuid,
        page: u32,
        per_page: u32,
    ) -> Result<Vec<UploadedDocument>, AppError>;
    async fn count_by_company(&self, company_id: Uuid) -> Result<u64, AppError>;
    async fn save(&self, doc: &UploadedDocument) -> Result<UploadedDocument, AppError>;
    async fn update(&self, doc: &UploadedDocument) -> Result<(), AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}
