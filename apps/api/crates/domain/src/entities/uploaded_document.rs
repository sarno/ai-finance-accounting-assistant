use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadedDocument {
    pub id: Uuid,
    pub company_id: Uuid,
    pub original_file_name: String,
    pub storage_path: String,
    pub mime_type: Option<String>,
    pub size_bytes: Option<i64>,
    pub document_type: Option<String>,
    pub status: String, // 'pending', 'processing', 'completed', 'failed'
    pub extracted_fields: Option<serde_json::Value>,
    pub validation_results: Option<serde_json::Value>,
    pub ai_confidence: Option<f64>,
    pub uploaded_by: Option<Uuid>,
    pub uploaded_at: OffsetDateTime,
    pub error_message: Option<String>,
}
