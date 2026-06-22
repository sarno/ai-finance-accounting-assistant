use axum::{
    extract::{Multipart, State, Query, Path},
    response::Json,
};
use std::sync::Arc;
use uuid::Uuid;
use std::path::Path as StdPath;

use crate::{errors::ApiError, state::AppState, middleware::auth_middleware::AuthenticatedUser};
use finance_assistant_app::errors::AppError;
use finance_assistant_domain::entities::uploaded_document::UploadedDocument;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadResponse {
    pub id: Uuid,
    pub original_file_name: String,
    pub storage_path: String,
    pub mime_type: Option<String>,
    pub size_bytes: Option<i64>,
    pub document_type: Option<String>,
    pub status: String,
    pub url: String,
}

pub async fn upload_file(
    State(state): State<Arc<AppState>>,
    auth: AuthenticatedUser,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, ApiError> {
    let mut file_data = None;
    let mut file_name = None;
    let mut mime_type = None;
    let mut document_type = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to parse multipart field: {}", e))?
    {
        let name = field.name().unwrap_or("").to_string();
        if name == "file" {
            let content_type = field.content_type().map(|s| s.to_string());
            let original_name = field.file_name().unwrap_or("").to_string();
            let data = field
                .bytes()
                .await
                .map_err(|e| anyhow::anyhow!("Failed to read field bytes: {}", e))?;
            file_name = Some(original_name);
            file_data = Some(data.to_vec());
            mime_type = content_type;
        } else if name == "documentType" {
            if let Ok(value) = field.text().await {
                document_type = Some(value);
            }
        }
    }

    let original_name = match file_name {
        Some(name) if !name.is_empty() => name,
        _ => {
            return Err(ApiError(AppError::Validation {
                message: "No file was uploaded or file field is missing".to_string(),
            }))
        }
    };

    let data = file_data.ok_or_else(|| {
        ApiError(AppError::Validation {
            message: "File data is empty".to_string(),
        })
    })?;

    // Validate file extension
    let extension = StdPath::new(&original_name)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase());

    match extension {
        Some(ref e) if e == "pdf" || e == "png" || e == "jpg" || e == "jpeg" || e == "gif" || e == "webp" => e.clone(),
        _ => {
            return Err(ApiError(AppError::Validation {
                message: "Only PDF and image files (PNG, JPG, JPEG, GIF, WEBP) are allowed".to_string(),
            }))
        }
    };

    // Limit file size, e.g., 10MB
    if data.len() > 10 * 1024 * 1024 {
        return Err(ApiError(AppError::Validation {
            message: "File size exceeds the 10MB limit".to_string(),
        }));
    }

    let size_bytes = data.len() as i64;

    // Use document service to upload and persist metadata
    let doc = state.document_service.upload_document(
        auth.0.company_id,
        original_name,
        data,
        mime_type,
        Some(size_bytes),
        document_type,
        Some(auth.0.id),
    ).await?;

    let file_url = format!(
        "{}/{}",
        state.config.storage_base_url.trim_end_matches('/'),
        doc.storage_path
    );

    // Spawn background Tokio task for OCR extraction
    let doc_svc_clone = state.document_service.clone();
    let doc_id = doc.id;
    tokio::spawn(async move {
        if let Err(e) = doc_svc_clone.process_ocr(doc_id).await {
            tracing::error!("Background OCR failed for document {}: {:?}", doc_id, e);
        } else {
            tracing::info!("Background OCR successfully completed for document {}", doc_id);
        }
    });

    Ok(Json(UploadResponse {
        id: doc.id,
        original_file_name: doc.original_file_name,
        storage_path: doc.storage_path,
        mime_type: doc.mime_type,
        size_bytes: doc.size_bytes,
        document_type: doc.document_type,
        status: doc.status,
        url: file_url,
    }))
}

#[derive(serde::Deserialize)]
pub struct ListParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

pub async fn list_documents(
    State(state): State<Arc<AppState>>,
    auth: AuthenticatedUser,
    Query(params): Query<ListParams>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);
    let docs = state.document_service.list_documents(auth.0.company_id, page, per_page).await?;
    let total = state.document_service.count_documents(auth.0.company_id).await?;

    Ok(Json(serde_json::json!({
        "data": docs,
        "total": total,
        "page": page,
        "perPage": per_page
    })))
}

pub async fn get_document(
    State(state): State<Arc<AppState>>,
    auth: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<UploadedDocument>, ApiError> {
    let doc = state.document_service.get_document(id).await?;
    // Security check: must belong to the same company
    if doc.company_id != auth.0.company_id {
        return Err(ApiError(AppError::Unauthorized {
            reason: "Access denied to document".to_string(),
        }));
    }
    Ok(Json(doc))
}
