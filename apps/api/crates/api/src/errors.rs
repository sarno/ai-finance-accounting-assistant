use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use finance_assistant_app::errors::AppError;
use finance_assistant_domain::errors::DomainError;

/// Newtype wrapper for AppError to comply with orphan rules when implementing IntoResponse.
#[derive(Debug)]
pub struct ApiError(pub AppError);

impl From<AppError> for ApiError {
    fn from(err: AppError) -> Self {
        ApiError(err)
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError(AppError::Internal(err))
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_code, message) = match &self.0 {
            AppError::Domain(DomainError::JournalNotBalanced {
                total_debit,
                total_credit,
            }) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "JOURNAL_NOT_BALANCED",
                format!("Total debit ({total_debit}) must equal credit ({total_credit})"),
            ),
            AppError::Domain(DomainError::PostedDocumentImmutable) => (
                StatusCode::CONFLICT,
                "POSTED_DOCUMENT_IMMUTABLE",
                "Posted document cannot be edited".to_string(),
            ),
            AppError::Domain(DomainError::ApprovalRequired) => (
                StatusCode::CONFLICT,
                "APPROVAL_REQUIRED",
                "Approval is required before posting".to_string(),
            ),
            AppError::Domain(DomainError::PermissionDenied { action }) => (
                StatusCode::FORBIDDEN,
                "PERMISSION_DENIED",
                format!("Permission denied: {action}"),
            ),
            AppError::Domain(d) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "DOMAIN_ERROR",
                d.to_string(),
            ),
            AppError::NotFound { resource, id } => (
                StatusCode::NOT_FOUND,
                "NOT_FOUND",
                format!("{resource} not found: {id}"),
            ),
            AppError::Unauthorized { reason } => {
                (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", reason.clone())
            }
            AppError::Forbidden { action } => (StatusCode::FORBIDDEN, "FORBIDDEN", action.clone()),
            AppError::Conflict { message } => (StatusCode::CONFLICT, "CONFLICT", message.clone()),
            AppError::Validation { message } => {
                (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", message.clone())
            }
            AppError::Internal(e) => {
                tracing::error!("Internal error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "An internal error occurred".to_string(),
                )
            }
            AppError::ExternalService { service, message } => {
                tracing::error!("External service error ({}): {}", service, message);
                (
                    StatusCode::BAD_GATEWAY,
                    "EXTERNAL_SERVICE_ERROR",
                    format!("{service} error"),
                )
            }
        };

        (
            status,
            Json(json!({ "errorCode": error_code, "message": message })),
        )
            .into_response()
    }
}
