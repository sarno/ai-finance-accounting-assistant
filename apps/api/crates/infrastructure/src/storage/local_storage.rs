use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs;

use finance_assistant_app::{errors::AppError, ports::storage_port::StoragePort};

pub struct LocalStorageProvider {
    base_path: PathBuf,
    base_url: String,
}

impl LocalStorageProvider {
    pub fn new(base_path: impl Into<PathBuf>, base_url: impl Into<String>) -> Self {
        Self {
            base_path: base_path.into(),
            base_url: base_url.into(),
        }
    }
}

#[async_trait]
impl StoragePort for LocalStorageProvider {
    async fn upload(
        &self,
        filename: &str,
        data: Vec<u8>,
        _content_type: &str,
    ) -> Result<String, AppError> {
        let path = self.base_path.join(filename);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| AppError::Internal(e.into()))?;
        }
        fs::write(&path, data)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        Ok(filename.to_string())
    }

    async fn download(&self, path: &str) -> Result<Vec<u8>, AppError> {
        let full_path = self.base_path.join(path);
        fs::read(full_path)
            .await
            .map_err(|e| AppError::Internal(e.into()))
    }

    async fn delete(&self, path: &str) -> Result<(), AppError> {
        let full_path = self.base_path.join(path);
        fs::remove_file(full_path)
            .await
            .map_err(|e| AppError::Internal(e.into()))
    }

    fn public_url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url.trim_end_matches('/'), path)
    }
}
