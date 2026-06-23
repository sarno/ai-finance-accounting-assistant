use async_trait::async_trait;
use uuid::Uuid;

use crate::errors::AppError;
use finance_assistant_domain::entities::user::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<User, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn find_by_company(&self, company_id: Uuid) -> Result<Vec<User>, AppError>;
    async fn save(&self, user: &User) -> Result<(), AppError>;
    async fn update(&self, user: &User) -> Result<(), AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}
