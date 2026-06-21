use async_trait::async_trait;
use uuid::Uuid;

use finance_assistant_domain::entities::tax::TaxType;
use crate::errors::AppError;

#[async_trait]
pub trait TaxRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<TaxType, AppError>;
    async fn find_all_by_company(&self, company_id: Uuid) -> Result<Vec<TaxType>, AppError>;
    async fn save(&self, tax_type: &TaxType) -> Result<(), AppError>;
    async fn update(&self, tax_type: &TaxType) -> Result<(), AppError>;
}
