use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use finance_assistant_app::{errors::AppError, ports::approval_repository::ApprovalRepository};
use finance_assistant_domain::entities::approval::{
    ApprovalDocumentType, ApprovalRequest, ApprovalStatus,
};

pub struct PgApprovalRepository {
    pool: PgPool,
}

impl PgApprovalRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ApprovalRepository for PgApprovalRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<ApprovalRequest, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, company_id, document_type, document_id, status, requested_by, reviewed_by, reviewed_at, comment, created_at, updated_at
            FROM approval_requests
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let row = match row {
            Some(r) => r,
            None => {
                return Err(AppError::NotFound {
                    resource: "ApprovalRequest".to_string(),
                    id: id.to_string(),
                })
            }
        };

        let doc_type_str: String = row.get("document_type");
        let document_type = ApprovalDocumentType::from_str(&doc_type_str)
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

        let status_str: String = row.get("status");
        let status = ApprovalStatus::from_str(&status_str)
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

        Ok(ApprovalRequest {
            id: row.get("id"),
            company_id: row.get("company_id"),
            document_type,
            document_id: row.get("document_id"),
            status,
            requested_by: row.get("requested_by"),
            reviewed_by: row.get("reviewed_by"),
            reviewed_at: row.get::<Option<time::OffsetDateTime>, _>("reviewed_at"),
            comment: row.get("comment"),
            created_at: row.get::<time::OffsetDateTime, _>("created_at"),
            updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
        })
    }

    async fn find_pending_by_company(
        &self,
        company_id: Uuid,
    ) -> Result<Vec<ApprovalRequest>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, company_id, document_type, document_id, status, requested_by, reviewed_by, reviewed_at, comment, created_at, updated_at
            FROM approval_requests
            WHERE company_id = $1 AND status = 'pending'
            ORDER BY created_at DESC
            "#,
        )
        .bind(company_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let mut requests = Vec::new();
        for row in rows {
            let doc_type_str: String = row.get("document_type");
            let document_type = ApprovalDocumentType::from_str(&doc_type_str)
                .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

            let status_str: String = row.get("status");
            let status = ApprovalStatus::from_str(&status_str)
                .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

            requests.push(ApprovalRequest {
                id: row.get("id"),
                company_id: row.get("company_id"),
                document_type,
                document_id: row.get("document_id"),
                status,
                requested_by: row.get("requested_by"),
                reviewed_by: row.get("reviewed_by"),
                reviewed_at: row.get::<Option<time::OffsetDateTime>, _>("reviewed_at"),
                comment: row.get("comment"),
                created_at: row.get::<time::OffsetDateTime, _>("created_at"),
                updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
            });
        }

        Ok(requests)
    }

    async fn find_by_document(
        &self,
        document_id: Uuid,
    ) -> Result<Option<ApprovalRequest>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, company_id, document_type, document_id, status, requested_by, reviewed_by, reviewed_at, comment, created_at, updated_at
            FROM approval_requests
            WHERE document_id = $1
            LIMIT 1
            "#,
        )
        .bind(document_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let row = match row {
            Some(r) => r,
            None => return Ok(None),
        };

        let doc_type_str: String = row.get("document_type");
        let document_type = ApprovalDocumentType::from_str(&doc_type_str)
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

        let status_str: String = row.get("status");
        let status = ApprovalStatus::from_str(&status_str)
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

        Ok(Some(ApprovalRequest {
            id: row.get("id"),
            company_id: row.get("company_id"),
            document_type,
            document_id: row.get("document_id"),
            status,
            requested_by: row.get("requested_by"),
            reviewed_by: row.get("reviewed_by"),
            reviewed_at: row.get::<Option<time::OffsetDateTime>, _>("reviewed_at"),
            comment: row.get("comment"),
            created_at: row.get::<time::OffsetDateTime, _>("created_at"),
            updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
        }))
    }

    async fn save(&self, request: &ApprovalRequest) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO approval_requests (
                id, company_id, document_type, document_id, status,
                requested_by, reviewed_by, reviewed_at, comment,
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(request.id)
        .bind(request.company_id)
        .bind(request.document_type.as_str())
        .bind(request.document_id)
        .bind(request.status.as_str())
        .bind(request.requested_by)
        .bind(request.reviewed_by)
        .bind(request.reviewed_at)
        .bind(&request.comment)
        .bind(request.created_at)
        .bind(request.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }

    async fn update(&self, request: &ApprovalRequest) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE approval_requests
            SET status = $1, reviewed_by = $2, reviewed_at = $3, comment = $4, updated_at = $5
            WHERE id = $6
            "#,
        )
        .bind(request.status.as_str())
        .bind(request.reviewed_by)
        .bind(request.reviewed_at)
        .bind(&request.comment)
        .bind(request.updated_at)
        .bind(request.id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }
}
