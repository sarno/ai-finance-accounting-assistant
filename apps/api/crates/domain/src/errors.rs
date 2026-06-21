use rust_decimal::Decimal;
use thiserror::Error;

/// All domain-level errors. These are pure business rule violations.
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Journal is not balanced: debit={total_debit}, credit={total_credit}")]
    JournalNotBalanced {
        total_debit: Decimal,
        total_credit: Decimal,
    },

    #[error("Posted document is immutable and cannot be edited directly")]
    PostedDocumentImmutable,

    #[error("Invalid status transition from {from} to {to}")]
    InvalidStatusTransition { from: String, to: String },

    #[error("Tax type is not effective for the given transaction date")]
    TaxTypeNotEffective,

    #[error("Approval is required before posting")]
    ApprovalRequired,

    #[error("User does not have permission to perform this action: {action}")]
    PermissionDenied { action: String },

    #[error("Duplicate supplier invoice number detected")]
    DuplicateInvoiceNumber,

    #[error("Validation error: {message}")]
    Validation { message: String },
}
