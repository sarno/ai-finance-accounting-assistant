use std::sync::Arc;
use uuid::Uuid;
use tracing::instrument;

use finance_assistant_domain::{
    entities::approval::{ApprovalRequest, ApprovalDocumentType, ApprovalStatus},
    value_objects::DocumentStatus,
};

use crate::{
    dto::approval::ApprovalResponse,
    errors::AppError,
    ports::{
        approval_repository::ApprovalRepository,
        journal_repository::JournalRepository,
        audit_log_repository::AuditLogRepository,
    },
};

pub struct ApprovalService {
    approval_repo: Arc<dyn ApprovalRepository>,
    journal_repo: Arc<dyn JournalRepository>,
    audit_repo: Arc<dyn AuditLogRepository>,
}

impl ApprovalService {
    pub fn new(
        approval_repo: Arc<dyn ApprovalRepository>,
        journal_repo: Arc<dyn JournalRepository>,
        audit_repo: Arc<dyn AuditLogRepository>,
    ) -> Self {
        Self {
            approval_repo,
            journal_repo,
            audit_repo,
        }
    }

    /// Submit a journal entry for approval.
    #[instrument(skip(self))]
    pub async fn submit_journal_approval(&self, journal_id: Uuid, requested_by: Uuid) -> Result<ApprovalResponse, AppError> {
        let mut journal = self.journal_repo.find_by_id(journal_id).await?;
        
        if journal.status != DocumentStatus::Draft && journal.status != DocumentStatus::Rejected {
            return Err(AppError::Validation {
                message: "Only draft or rejected journals can be submitted for approval".to_string(),
            });
        }

        // Update journal status
        journal.status = DocumentStatus::WaitingApproval;
        journal.updated_at = time::OffsetDateTime::now_utc();
        self.journal_repo.update(&journal).await?;

        // Create or reactivate approval request
        let now = time::OffsetDateTime::now_utc();
        let existing = self.approval_repo.find_by_document(journal_id).await?;

        let request = match existing {
            Some(mut req) => {
                req.status = ApprovalStatus::Pending;
                req.requested_by = requested_by;
                req.comment = None;
                req.reviewed_by = None;
                req.reviewed_at = None;
                req.updated_at = now;
                self.approval_repo.update(&req).await?;
                req
            }
            None => {
                let req = ApprovalRequest {
                    id: Uuid::new_v4(),
                    company_id: journal.company_id,
                    document_type: ApprovalDocumentType::JournalEntry,
                    document_id: journal_id,
                    status: ApprovalStatus::Pending,
                    requested_by,
                    reviewed_by: None,
                    reviewed_at: None,
                    comment: None,
                    created_at: now,
                    updated_at: now,
                };
                self.approval_repo.save(&req).await?;
                req
            }
        };

        tracing::info!(journal_id = %journal_id, approval_id = %request.id, "Journal submitted for approval");
        Ok(ApprovalResponse::from(request))
    }

    /// Approve an approval request.
    #[instrument(skip(self))]
    pub async fn approve_request(&self, id: Uuid, reviewed_by: Uuid, comment: Option<String>) -> Result<ApprovalResponse, AppError> {
        let mut req = self.approval_repo.find_by_id(id).await?;
        if req.status != ApprovalStatus::Pending {
            return Err(AppError::Validation {
                message: "This request has already been reviewed".to_string(),
            });
        }

        let now = time::OffsetDateTime::now_utc();
        req.status = ApprovalStatus::Approved;
        req.reviewed_by = Some(reviewed_by);
        req.reviewed_at = Some(now);
        req.comment = comment;
        req.updated_at = now;

        self.approval_repo.update(&req).await?;

        // Transition target document based on type
        match req.document_type {
            ApprovalDocumentType::JournalEntry => {
                let mut journal = self.journal_repo.find_by_id(req.document_id).await?;
                journal.status = DocumentStatus::Approved;
                journal.updated_at = now;
                self.journal_repo.update(&journal).await?;
            }
            _ => {
                // Other document types will be implemented as tasks progress
            }
        }

        tracing::info!(approval_id = %id, reviewed_by = %reviewed_by, "Approval request approved");
        Ok(ApprovalResponse::from(req))
    }

    /// Reject an approval request.
    #[instrument(skip(self))]
    pub async fn reject_request(&self, id: Uuid, reviewed_by: Uuid, comment: Option<String>) -> Result<ApprovalResponse, AppError> {
        let mut req = self.approval_repo.find_by_id(id).await?;
        if req.status != ApprovalStatus::Pending {
            return Err(AppError::Validation {
                message: "This request has already been reviewed".to_string(),
            });
        }

        let now = time::OffsetDateTime::now_utc();
        req.status = ApprovalStatus::Rejected;
        req.reviewed_by = Some(reviewed_by);
        req.reviewed_at = Some(now);
        req.comment = comment;
        req.updated_at = now;

        self.approval_repo.update(&req).await?;

        // Transition target document back to draft or rejected status
        match req.document_type {
            ApprovalDocumentType::JournalEntry => {
                let mut journal = self.journal_repo.find_by_id(req.document_id).await?;
                journal.status = DocumentStatus::Rejected;
                journal.updated_at = now;
                self.journal_repo.update(&journal).await?;
            }
            _ => {
                // Other document types will be implemented as tasks progress
            }
        }

        tracing::info!(approval_id = %id, reviewed_by = %reviewed_by, "Approval request rejected");
        Ok(ApprovalResponse::from(req))
    }

    /// List all pending approval requests for a company.
    pub async fn list_pending(&self, company_id: Uuid) -> Result<Vec<ApprovalResponse>, AppError> {
        let list = self.approval_repo.find_pending_by_company(company_id).await?;
        Ok(list.into_iter().map(ApprovalResponse::from).collect())
    }

    /// Get details of an approval request by its ID.
    pub async fn get_by_id(&self, id: Uuid) -> Result<ApprovalResponse, AppError> {
        let req = self.approval_repo.find_by_id(id).await?;
        Ok(ApprovalResponse::from(req))
    }
}
