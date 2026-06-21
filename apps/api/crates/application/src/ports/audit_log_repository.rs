use async_trait::async_trait;
use uuid::Uuid;

use finance_assistant_domain::entities::audit::AuditLog;
use crate::errors::AppError;

#[async_trait]
pub trait AuditLogRepository: Send + Sync {
    /// Append-only write — audit logs must never be updated or deleted.
    async fn append(&self, log: &AuditLog) -> Result<(), AppError>;
    async fn find_by_entity(&self, entity_type: &str, entity_id: Uuid) -> Result<Vec<AuditLog>, AppError>;
}
