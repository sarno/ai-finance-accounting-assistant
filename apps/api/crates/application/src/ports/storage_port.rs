use async_trait::async_trait;

use crate::errors::AppError;

/// Port for file storage (local, Synology NAS, S3-compatible).
#[async_trait]
pub trait StoragePort: Send + Sync {
    async fn upload(
        &self,
        filename: &str,
        data: Vec<u8>,
        content_type: &str,
    ) -> Result<String, AppError>;
    async fn download(&self, path: &str) -> Result<Vec<u8>, AppError>;
    async fn delete(&self, path: &str) -> Result<(), AppError>;
    fn public_url(&self, path: &str) -> String;
}
