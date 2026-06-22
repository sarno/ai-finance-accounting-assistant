use axum::{
    extract::{Multipart, State},
    response::Json,
};
use std::sync::Arc;
use uuid::Uuid;
use std::path::Path;
use tokio::io::AsyncWriteExt;

use crate::{errors::ApiError, state::AppState};
use finance_assistant_app::errors::AppError;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadResponse {
    pub url: String,
}

pub async fn upload_file(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, ApiError> {
    let mut file_data = None;
    let mut file_name = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to parse multipart field: {}", e))?
    {
        let name = field.name().unwrap_or("").to_string();
        if name == "file" {
            let original_name = field.file_name().unwrap_or("").to_string();
            let data = field
                .bytes()
                .await
                .map_err(|e| anyhow::anyhow!("Failed to read field bytes: {}", e))?;
            file_name = Some(original_name);
            file_data = Some(data);
            break;
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
    let extension = Path::new(&original_name)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase());

    let ext = match extension {
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

    // Ensure storage path exists
    let storage_path = &state.config.storage_base_path;
    tokio::fs::create_dir_all(storage_path)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create storage directory: {}", e))?;

    // Create unique file name
    let new_filename = format!("{}.{}", Uuid::new_v4(), ext);
    let full_path = Path::new(storage_path).join(&new_filename);

    // Save to disk
    let mut file = tokio::fs::File::create(&full_path)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create destination file: {}", e))?;
    file.write_all(&data)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to write to file: {}", e))?;

    let file_url = format!(
        "{}/{}",
        state.config.storage_base_url.trim_end_matches('/'),
        new_filename
    );

    Ok(Json(UploadResponse { url: file_url }))
}
