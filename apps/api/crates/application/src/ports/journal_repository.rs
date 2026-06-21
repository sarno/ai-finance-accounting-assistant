use async_trait::async_trait;
use uuid::Uuid;

use crate::errors::AppError;
use finance_assistant_domain::entities::journal::{JournalEntry, JournalLine};

#[async_trait]
pub trait JournalRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<JournalEntry, AppError>;
    async fn find_by_company(
        &self,
        company_id: Uuid,
        page: u32,
        per_page: u32,
    ) -> Result<Vec<JournalEntry>, AppError>;
    async fn save(&self, entry: &JournalEntry) -> Result<(), AppError>;
    async fn update(&self, entry: &JournalEntry) -> Result<(), AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
    async fn save_lines(&self, lines: &[JournalLine]) -> Result<(), AppError>;
    async fn count_by_company(&self, company_id: Uuid) -> Result<i64, AppError>;
}
