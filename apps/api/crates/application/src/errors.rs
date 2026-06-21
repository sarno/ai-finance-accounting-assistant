use finance_assistant_domain::errors::DomainError;
use thiserror::Error;

/// Application-level errors (wraps domain + infra errors).
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Domain rule violation: {0}")]
    Domain(#[from] DomainError),

    #[error("Not found: {resource} with id={id}")]
    NotFound { resource: String, id: String },

    #[error("Unauthorized: {reason}")]
    Unauthorized { reason: String },

    #[error("Forbidden: {action}")]
    Forbidden { action: String },

    #[error("Conflict: {message}")]
    Conflict { message: String },

    #[error("Validation failed: {message}")]
    Validation { message: String },

    #[error("External service error: {service} — {message}")]
    ExternalService { service: String, message: String },

    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}
