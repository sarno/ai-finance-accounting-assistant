use async_trait::async_trait;
use uuid::Uuid;

use crate::errors::AppError;
use finance_assistant_domain::entities::supplier::Supplier;

#[async_trait]
pub trait SupplierRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Supplier, AppError>;
    async fn find_all_by_company(&self, company_id: Uuid) -> Result<Vec<Supplier>, AppError>;
    async fn save(&self, supplier: &Supplier) -> Result<(), AppError>;
    async fn update(&self, supplier: &Supplier) -> Result<(), AppError>;
}
