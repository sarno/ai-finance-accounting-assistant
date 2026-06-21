use crate::errors::AppError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Port for external AI/LLM communication.
#[async_trait]
pub trait AiClientPort: Send + Sync {
    async fn extract_invoice_fields(
        &self,
        document_bytes: &[u8],
        content_type: &str,
    ) -> Result<ExtractedInvoiceFields, AppError>;
    async fn answer_financial_query(
        &self,
        context: &str,
        question: &str,
    ) -> Result<String, AppError>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractedInvoiceFields {
    pub supplier_name: Option<String>,
    pub invoice_number: Option<String>,
    pub invoice_date: Option<String>,
    pub due_date: Option<String>,
    pub subtotal: Option<String>,
    pub tax_amount: Option<String>,
    pub total_amount: Option<String>,
    pub confidence: f64,
}
