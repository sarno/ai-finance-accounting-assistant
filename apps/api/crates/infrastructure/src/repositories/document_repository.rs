use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use finance_assistant_app::{errors::AppError, ports::document_repository::DocumentRepository};
use finance_assistant_domain::entities::uploaded_document::UploadedDocument;

pub struct PgDocumentRepository {
    pool: PgPool,
}

impl PgDocumentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DocumentRepository for PgDocumentRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<UploadedDocument, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, company_id, original_file_name, storage_path, mime_type, size_bytes,
                   document_type, status, extracted_fields, validation_results, ai_confidence,
                   uploaded_by, uploaded_at, error_message
            FROM uploaded_documents
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
                    resource: "UploadedDocument".to_string(),
                    id: id.to_string(),
                })
            }
        };

        Ok(UploadedDocument {
            id: row.get("id"),
            company_id: row.get("company_id"),
            original_file_name: row.get("original_file_name"),
            storage_path: row.get("storage_path"),
            mime_type: row.get("mime_type"),
            size_bytes: row.get("size_bytes"),
            document_type: row.get("document_type"),
            status: row.get("status"),
            extracted_fields: row.get("extracted_fields"),
            validation_results: row.get("validation_results"),
            ai_confidence: row.get("ai_confidence"),
            uploaded_by: row.get("uploaded_by"),
            uploaded_at: row.get::<time::OffsetDateTime, _>("uploaded_at"),
            error_message: row.get("error_message"),
        })
    }

    async fn find_by_company(
        &self,
        company_id: Uuid,
        page: u32,
        per_page: u32,
    ) -> Result<Vec<UploadedDocument>, AppError> {
        let limit = per_page as i64;
        let offset = ((page.max(1) - 1) * per_page) as i64;

        let rows = sqlx::query(
            r#"
            SELECT id, company_id, original_file_name, storage_path, mime_type, size_bytes,
                   document_type, status, extracted_fields, validation_results, ai_confidence,
                   uploaded_by, uploaded_at, error_message
            FROM uploaded_documents
            WHERE company_id = $1
            ORDER BY uploaded_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(company_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let docs = rows
            .into_iter()
            .map(|row| UploadedDocument {
                id: row.get("id"),
                company_id: row.get("company_id"),
                original_file_name: row.get("original_file_name"),
                storage_path: row.get("storage_path"),
                mime_type: row.get("mime_type"),
                size_bytes: row.get("size_bytes"),
                document_type: row.get("document_type"),
                status: row.get("status"),
                extracted_fields: row.get("extracted_fields"),
                validation_results: row.get("validation_results"),
                ai_confidence: row.get("ai_confidence"),
                uploaded_by: row.get("uploaded_by"),
                uploaded_at: row.get::<time::OffsetDateTime, _>("uploaded_at"),
                error_message: row.get("error_message"),
            })
            .collect();

        Ok(docs)
    }

    async fn count_by_company(&self, company_id: Uuid) -> Result<u64, AppError> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM uploaded_documents WHERE company_id = $1"
        )
        .bind(company_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(count as u64)
    }

    async fn save(&self, doc: &UploadedDocument) -> Result<UploadedDocument, AppError> {
        sqlx::query(
            r#"
            INSERT INTO uploaded_documents (
                id, company_id, original_file_name, storage_path, mime_type, size_bytes,
                document_type, status, extracted_fields, validation_results, ai_confidence,
                uploaded_by, uploaded_at, error_message
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            "#,
        )
        .bind(doc.id)
        .bind(doc.company_id)
        .bind(&doc.original_file_name)
        .bind(&doc.storage_path)
        .bind(&doc.mime_type)
        .bind(doc.size_bytes)
        .bind(&doc.document_type)
        .bind(&doc.status)
        .bind(&doc.extracted_fields)
        .bind(&doc.validation_results)
        .bind(doc.ai_confidence)
        .bind(doc.uploaded_by)
        .bind(doc.uploaded_at)
        .bind(&doc.error_message)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(doc.clone())
    }

    async fn update(&self, doc: &UploadedDocument) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE uploaded_documents
            SET original_file_name = $2, storage_path = $3, mime_type = $4, size_bytes = $5,
                document_type = $6, status = $7, extracted_fields = $8, validation_results = $9,
                ai_confidence = $10, uploaded_by = $11, uploaded_at = $12, error_message = $13
            WHERE id = $1
            "#,
        )
        .bind(doc.id)
        .bind(&doc.original_file_name)
        .bind(&doc.storage_path)
        .bind(&doc.mime_type)
        .bind(doc.size_bytes)
        .bind(&doc.document_type)
        .bind(&doc.status)
        .bind(&doc.extracted_fields)
        .bind(&doc.validation_results)
        .bind(doc.ai_confidence)
        .bind(doc.uploaded_by)
        .bind(doc.uploaded_at)
        .bind(&doc.error_message)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM uploaded_documents WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }
}
