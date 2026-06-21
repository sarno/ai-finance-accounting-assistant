use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use finance_assistant_domain::{
    entities::journal::{JournalEntry, JournalLine, JournalSource},
    rules::journal_rules,
    value_objects::DocumentStatus,
};

use crate::{
    dto::journal::{CreateJournalDraftRequest, JournalResponse},
    errors::AppError,
    ports::{audit_log_repository::AuditLogRepository, journal_repository::JournalRepository},
};

pub struct JournalService {
    journal_repo: Arc<dyn JournalRepository>,
    audit_repo: Arc<dyn AuditLogRepository>,
}

impl JournalService {
    pub fn new(
        journal_repo: Arc<dyn JournalRepository>,
        audit_repo: Arc<dyn AuditLogRepository>,
    ) -> Self {
        Self {
            journal_repo,
            audit_repo,
        }
    }

    /// Create a draft journal entry (AI or manual).
    /// Does NOT post. Requires separate approval + post steps.
    #[instrument(skip(self, req), fields(company_id = %req.company_id))]
    pub async fn create_draft(
        &self,
        req: CreateJournalDraftRequest,
        created_by: Uuid,
    ) -> Result<JournalResponse, AppError> {
        // Validate balance before even saving
        let lines: Vec<JournalLine> = req
            .lines
            .into_iter()
            .enumerate()
            .map(|(i, l)| JournalLine {
                id: Uuid::new_v4(),
                journal_entry_id: Uuid::nil(), // will be replaced after save
                account_id: l.account_id,
                debit: l.debit,
                credit: l.credit,
                description: l.description,
                sort_order: i as i32,
            })
            .collect();

        journal_rules::validate_journal_balance(&lines)?;
        journal_rules::validate_journal_line_sides(&lines)?;

        let now = time::OffsetDateTime::now_utc();
        let ref_num = match req.reference_number {
            Some(ref_num) if !ref_num.trim().is_empty() => ref_num,
            _ => {
                let count = self.journal_repo.count_by_company(req.company_id).await?;
                let next_num = count + 1;
                format!("JV/{}/{:03}", now.year(), next_num)
            }
        };

        let entry = JournalEntry {
            id: Uuid::new_v4(),
            company_id: req.company_id,
            branch_id: req.branch_id,
            reference_number: ref_num,
            description: req.description,
            transaction_date: req.transaction_date,
            lines,
            status: DocumentStatus::Draft,
            source: JournalSource::Manual,
            source_document_id: None,
            created_by,
            posted_by: None,
            posted_at: None,
            created_at: now,
            updated_at: now,
        };

        self.journal_repo.save(&entry).await?;

        tracing::info!(journal_id = %entry.id, "Draft journal created");

        Ok(JournalResponse::from(entry))
    }

    /// Post a journal. Requires status == Approved.
    #[instrument(skip(self), fields(journal_id = %id))]
    pub async fn post_journal(
        &self,
        id: Uuid,
        posted_by: Uuid,
    ) -> Result<JournalResponse, AppError> {
        let mut entry = self.journal_repo.find_by_id(id).await?;

        entry.post(posted_by).map_err(AppError::Domain)?;

        self.journal_repo.update(&entry).await?;

        tracing::info!(journal_id = %id, posted_by = %posted_by, "Journal posted");

        Ok(JournalResponse::from(entry))
    }

    /// Retrieve a single journal entry by its ID.
    pub async fn get_journal(&self, id: Uuid) -> Result<JournalResponse, AppError> {
        let entry = self.journal_repo.find_by_id(id).await?;
        Ok(JournalResponse::from(entry))
    }

    /// List general journal entries for a company (paginated).
    pub async fn list_journals(
        &self,
        company_id: Uuid,
        page: u32,
        per_page: u32,
    ) -> Result<Vec<JournalResponse>, AppError> {
        let entries = self
            .journal_repo
            .find_by_company(company_id, page, per_page)
            .await?;
        Ok(entries.into_iter().map(JournalResponse::from).collect())
    }

    /// Submit a draft journal for approval.
    pub async fn submit_approval(&self, id: Uuid) -> Result<JournalResponse, AppError> {
        let mut entry = self.journal_repo.find_by_id(id).await?;
        if entry.status != DocumentStatus::Draft {
            return Err(AppError::Validation {
                message: "Only draft journals can be submitted for approval".to_string(),
            });
        }
        entry.status = DocumentStatus::WaitingApproval;
        entry.updated_at = time::OffsetDateTime::now_utc();
        self.journal_repo.update(&entry).await?;
        Ok(JournalResponse::from(entry))
    }

    /// Approve a journal entry (direct helper, transitions to Approved status).
    pub async fn approve_journal(&self, id: Uuid) -> Result<JournalResponse, AppError> {
        let mut entry = self.journal_repo.find_by_id(id).await?;
        entry.status = DocumentStatus::Approved;
        entry.updated_at = time::OffsetDateTime::now_utc();
        self.journal_repo.update(&entry).await?;
        Ok(JournalResponse::from(entry))
    }

    /// Update a draft journal entry.
    pub async fn update_draft(
        &self,
        id: Uuid,
        req: CreateJournalDraftRequest,
    ) -> Result<JournalResponse, AppError> {
        let mut entry = self.journal_repo.find_by_id(id).await?;
        if entry.status != DocumentStatus::Draft {
            return Err(AppError::Validation {
                message: "Only draft journal entries can be updated".to_string(),
            });
        }

        let lines: Vec<JournalLine> = req
            .lines
            .into_iter()
            .enumerate()
            .map(|(i, l)| JournalLine {
                id: Uuid::new_v4(),
                journal_entry_id: id,
                account_id: l.account_id,
                debit: l.debit,
                credit: l.credit,
                description: l.description,
                sort_order: i as i32,
            })
            .collect();

        journal_rules::validate_journal_balance(&lines)?;
        journal_rules::validate_journal_line_sides(&lines)?;

        entry.branch_id = req.branch_id;
        if let Some(ref_num) = req.reference_number {
            if !ref_num.trim().is_empty() {
                entry.reference_number = ref_num;
            }
        }
        entry.description = req.description;
        entry.transaction_date = req.transaction_date;
        entry.lines = lines;
        entry.updated_at = time::OffsetDateTime::now_utc();

        self.journal_repo.update(&entry).await?;

        Ok(JournalResponse::from(entry))
    }

    /// Delete a draft journal entry.
    pub async fn delete_journal(&self, id: Uuid) -> Result<(), AppError> {
        let entry = self.journal_repo.find_by_id(id).await?;
        if entry.status != DocumentStatus::Draft {
            return Err(AppError::Validation {
                message: "Only draft journal entries can be deleted".to_string(),
            });
        }
        self.journal_repo.delete(id).await?;
        Ok(())
    }

    /// Count journal entries for a company.
    pub async fn count_journals(&self, company_id: Uuid) -> Result<i64, AppError> {
        let count = self.journal_repo.count_by_company(company_id).await?;
        Ok(count)
    }
}
