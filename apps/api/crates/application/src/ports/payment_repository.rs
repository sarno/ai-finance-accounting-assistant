use async_trait::async_trait;
use uuid::Uuid;

use finance_assistant_domain::entities::payment::Payment;
use crate::errors::AppError;

#[async_trait]
pub trait PaymentRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Payment, AppError>;
    async fn find_by_company(&self, company_id: Uuid, page: u32, per_page: u32) -> Result<Vec<Payment>, AppError>;
    async fn count_by_company(&self, company_id: Uuid) -> Result<u64, AppError>;
    async fn save(&self, payment: &Payment) -> Result<Payment, AppError>;
    async fn update(&self, payment: &Payment) -> Result<(), AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}
