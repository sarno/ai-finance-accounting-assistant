use rust_decimal::Decimal;
use std::sync::Arc;
use sqlx::{PgPool, Row};
use time::OffsetDateTime;
use uuid::Uuid;
use serde_json::json;

use finance_assistant_domain::entities::uploaded_document::UploadedDocument;
use crate::{
    errors::AppError,
    ports::{
        document_repository::DocumentRepository,
        storage_port::StoragePort,
        ai_client_port::AiClientPort,
        invoice_repository::InvoiceRepository,
    },
};

pub struct DocumentService {
    document_repo: Arc<dyn DocumentRepository>,
    storage_port: Arc<dyn StoragePort>,
    ai_client: Arc<dyn AiClientPort>,
    invoice_repo: Arc<dyn InvoiceRepository>,
    pool: PgPool,
}

impl DocumentService {
    pub fn new(
        document_repo: Arc<dyn DocumentRepository>,
        storage_port: Arc<dyn StoragePort>,
        ai_client: Arc<dyn AiClientPort>,
        invoice_repo: Arc<dyn InvoiceRepository>,
        pool: PgPool,
    ) -> Self {
        Self {
            document_repo,
            storage_port,
            ai_client,
            invoice_repo,
            pool,
        }
    }

    pub async fn upload_document(
        &self,
        company_id: Uuid,
        original_file_name: String,
        file_data: Vec<u8>,
        mime_type: Option<String>,
        size_bytes: Option<i64>,
        document_type: Option<String>,
        uploaded_by: Option<Uuid>,
    ) -> Result<UploadedDocument, AppError> {
        // Create unique file name
        let ext = std::path::Path::new(&original_file_name)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("bin")
            .to_lowercase();
        let new_filename = format!("{}.{}", Uuid::new_v4(), ext);

        // Upload to storage
        let storage_path = self.storage_port
            .upload(&new_filename, file_data, mime_type.as_deref().unwrap_or("application/octet-stream"))
            .await?;

        // Save to DB
        let doc = UploadedDocument {
            id: Uuid::new_v4(),
            company_id,
            original_file_name,
            storage_path,
            mime_type,
            size_bytes,
            document_type: Some(document_type.unwrap_or_else(|| "PurchaseInvoice".to_string())),
            status: "pending".to_string(),
            extracted_fields: None,
            validation_results: None,
            ai_confidence: None,
            uploaded_by,
            uploaded_at: OffsetDateTime::now_utc(),
            error_message: None,
        };

        let saved = self.document_repo.save(&doc).await?;
        Ok(saved)
    }

    pub async fn get_document(&self, id: Uuid) -> Result<UploadedDocument, AppError> {
        self.document_repo.find_by_id(id).await
    }

    pub async fn list_documents(&self, company_id: Uuid, page: u32, per_page: u32) -> Result<Vec<UploadedDocument>, AppError> {
        self.document_repo.find_by_company(company_id, page, per_page).await
    }

    pub async fn count_documents(&self, company_id: Uuid) -> Result<u64, AppError> {
        self.document_repo.count_by_company(company_id).await
    }

    pub async fn process_ocr(&self, id: Uuid) -> Result<(), AppError> {
        // 1. Get document
        let mut doc = self.document_repo.find_by_id(id).await?;
        
        // Update status to processing
        doc.status = "processing".to_string();
        self.document_repo.update(&doc).await?;

        // 2. Download file data
        let file_bytes = match self.storage_port.download(&doc.storage_path).await {
            Ok(bytes) => bytes,
            Err(e) => {
                doc.status = "failed".to_string();
                doc.error_message = Some(format!("Failed to download file: {}", e));
                self.document_repo.update(&doc).await?;
                return Err(e);
            }
        };

        // 3. Call AI extraction
        let mime = doc.mime_type.clone().unwrap_or_else(|| "application/octet-stream".to_string());
        
        let extraction_result = match self.ai_client.extract_invoice_fields(&file_bytes, &mime).await {
            Ok(fields) => fields,
            Err(e) => {
                doc.status = "failed".to_string();
                doc.error_message = Some(format!("AI OCR Extraction failed: {}", e));
                self.document_repo.update(&doc).await?;
                return Err(e);
            }
        };

        // 4. Run validation checklist
        let company_id = doc.company_id;
        
        // Check 1: Supplier exists
        let supplier_name = extraction_result.supplier_name.clone().unwrap_or_default();
        let supplier_row = sqlx::query("SELECT id FROM suppliers WHERE name ILIKE $1 AND company_id = $2 LIMIT 1")
            .bind(&supplier_name)
            .bind(company_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        
        let supplier_id: Option<Uuid> = supplier_row.map(|r| r.get("id"));
        let supplier_exists = supplier_id.is_some();

        // Check 2: Duplicate invoice check
        let invoice_number = extraction_result.invoice_number.clone().unwrap_or_default();
        let is_duplicate = if let Some(s_id) = supplier_id {
            if !invoice_number.is_empty() {
                self.invoice_repo.find_duplicate_purchase(company_id, s_id, &invoice_number).await?
            } else {
                false
            }
        } else {
            false
        };
        let invoice_number_not_duplicate = !is_duplicate;

        // Check 3: Subtotal + tax = total
        let subtotal_opt = extraction_result.subtotal.as_deref().and_then(parse_decimal_from_ai_str);
        let tax_opt = extraction_result.tax_amount.as_deref().and_then(parse_decimal_from_ai_str);
        let total_opt = extraction_result.total_amount.as_deref().and_then(parse_decimal_from_ai_str);
        
        let amounts_match = match (subtotal_opt, tax_opt, total_opt) {
            (Some(s), Some(t), Some(tot)) => s + t == tot,
            (Some(s), None, Some(tot)) => s == tot, // no tax
            _ => false,
        };

        // Check 4: Due date is valid
        let inv_date_opt = extraction_result.invoice_date.as_deref().and_then(parse_date_from_ai_str);
        let due_date_opt = extraction_result.due_date.as_deref().and_then(parse_date_from_ai_str);
        let due_date_valid = match (inv_date_opt, due_date_opt) {
            (Some(idate), Some(ddate)) => ddate >= idate,
            _ => false,
        };

        // Check 5: Tax rate config exists
        let tax_types_row = sqlx::query("SELECT id FROM tax_types WHERE is_active = true AND company_id = $1 LIMIT 1")
            .bind(company_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        let tax_rate_valid = tax_types_row.is_some();

        // 5. Build validation results JSON
        let mut checklist = vec![];
        checklist.push(json!({
            "name": "supplier_exists",
            "passed": supplier_exists,
            "message": if supplier_exists {
                format!("Supplier '{}' ditemukan di database", supplier_name)
            } else {
                format!("Supplier '{}' tidak ditemukan. Disarankan membuat supplier baru.", supplier_name)
            }
        }));
        checklist.push(json!({
            "name": "invoice_number_not_duplicate",
            "passed": invoice_number_not_duplicate,
            "message": if invoice_number_not_duplicate {
                "Nomor invoice unik untuk supplier ini".to_string()
            } else {
                format!("Nomor invoice '{}' sudah pernah dicatat sebelumnya untuk supplier ini", invoice_number)
            }
        }));
        checklist.push(json!({
            "name": "amounts_match",
            "passed": amounts_match,
            "message": if amounts_match {
                format!("Perhitungan nilai cocok: Subtotal + PPN = Total ({})", total_opt.unwrap_or_default())
            } else {
                "Perhitungan nilai tidak cocok: Subtotal + PPN != Total".to_string()
            }
        }));
        checklist.push(json!({
            "name": "due_date_valid",
            "passed": due_date_valid,
            "message": if due_date_valid {
                "Tanggal jatuh tempo valid (sama dengan atau setelah tanggal invoice)".to_string()
            } else {
                "Tanggal jatuh tempo tidak boleh sebelum tanggal invoice".to_string()
            }
        }));
        checklist.push(json!({
            "name": "tax_rate_valid",
            "passed": tax_rate_valid,
            "message": if tax_rate_valid {
                "Konfigurasi tipe pajak aktif ditemukan".to_string()
            } else {
                "Belum ada konfigurasi tipe pajak aktif (seperti PPN) untuk perusahaan ini".to_string()
            }
        }));

        let validation_results = json!({
            "supplierExists": supplier_exists,
            "supplierId": supplier_id,
            "invoiceNumberNotDuplicate": invoice_number_not_duplicate,
            "amountsMatch": amounts_match,
            "dueDateValid": due_date_valid,
            "taxRateValid": tax_rate_valid,
            "checklist": checklist
        });

        // 6. Save back to doc
        doc.status = "completed".to_string();
        doc.extracted_fields = Some(serde_json::to_value(&extraction_result).unwrap());
        doc.validation_results = Some(validation_results);
        doc.ai_confidence = Some(extraction_result.confidence);
        
        self.document_repo.update(&doc).await?;
        Ok(())
    }
}

// ─── Parsers Helper ──────────────────────────────────────────────────────────

fn parse_decimal_from_ai_str(s: &str) -> Option<Decimal> {
    let mut cleaned = s.replace("Rp", "")
                       .replace("rp", "")
                       .replace(" ", "")
                       .replace(".", "");
    if cleaned.contains(',') {
        cleaned = cleaned.replace(",", ".");
    }
    cleaned.parse::<Decimal>().ok()
}

fn parse_date_from_ai_str(s: &str) -> Option<time::Date> {
    time::Date::parse(s, &time::format_description::well_known::Rfc3339)
        .or_else(|_| time::Date::parse(s, &time::macros::format_description!("[year]-[month]-[day]")))
        .ok()
}
