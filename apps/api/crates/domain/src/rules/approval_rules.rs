use crate::{entities::approval::ApprovalStatus, errors::DomainError};

/// Verify the approval is in Approved state before posting.
pub fn require_approved(status: &ApprovalStatus) -> Result<(), DomainError> {
    if *status != ApprovalStatus::Approved {
        return Err(DomainError::ApprovalRequired);
    }
    Ok(())
}
