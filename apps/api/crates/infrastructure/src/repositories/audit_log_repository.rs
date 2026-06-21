use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use finance_assistant_app::{errors::AppError, ports::audit_log_repository::AuditLogRepository};
use finance_assistant_domain::entities::audit::AuditLog;

pub struct PgAuditLogRepository {
    pool: PgPool,
}

impl PgAuditLogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditLogRepository for PgAuditLogRepository {
    /// Append-only INSERT — no UPDATE or DELETE allowed.
    async fn append(&self, log: &AuditLog) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO audit_logs (
                id, company_id, actor_user_id, actor_type,
                entity_type, entity_id, action,
                before_snapshot, after_snapshot,
                ip_address, user_agent, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
        )
        .bind(log.id)
        .bind(log.company_id)
        .bind(log.actor_user_id)
        .bind(log.actor_type.to_string())
        .bind(&log.entity_type)
        .bind(log.entity_id)
        .bind(&log.action)
        .bind(&log.before_snapshot)
        .bind(&log.after_snapshot)
        .bind(&log.ip_address)
        .bind(&log.user_agent)
        .bind(log.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }

    async fn find_by_entity(
        &self,
        _entity_type: &str,
        _entity_id: Uuid,
    ) -> Result<Vec<AuditLog>, AppError> {
        Ok(vec![])
    }
}
