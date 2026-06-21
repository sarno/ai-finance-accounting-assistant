use rust_decimal::Decimal;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use finance_assistant_domain::{
    entities::approval::{ApprovalDocumentType, ApprovalRequest, ApprovalStatus},
    value_objects::DocumentStatus,
};

use crate::{
    dto::approval::ApprovalResponse,
    errors::AppError,
    ports::{
        account_repository::AccountRepository, approval_repository::ApprovalRepository,
        audit_log_repository::AuditLogRepository, invoice_repository::InvoiceRepository,
        journal_repository::JournalRepository, tax_repository::TaxRepository,
        user_repository::UserRepository,
    },
};

pub struct ApprovalService {
    approval_repo: Arc<dyn ApprovalRepository>,
    journal_repo: Arc<dyn JournalRepository>,
    audit_repo: Arc<dyn AuditLogRepository>,
    user_repo: Arc<dyn UserRepository>,
    invoice_repo: Arc<dyn InvoiceRepository>,
    account_repo: Arc<dyn AccountRepository>,
    tax_repo: Arc<dyn TaxRepository>,
}

impl ApprovalService {
    pub fn new(
        approval_repo: Arc<dyn ApprovalRepository>,
        journal_repo: Arc<dyn JournalRepository>,
        audit_repo: Arc<dyn AuditLogRepository>,
        user_repo: Arc<dyn UserRepository>,
        invoice_repo: Arc<dyn InvoiceRepository>,
        account_repo: Arc<dyn AccountRepository>,
        tax_repo: Arc<dyn TaxRepository>,
    ) -> Self {
        Self {
            approval_repo,
            journal_repo,
            audit_repo,
            user_repo,
            invoice_repo,
            account_repo,
            tax_repo,
        }
    }

    async fn enrich_response(&self, r: ApprovalRequest) -> ApprovalResponse {
        let requested_by_name = match self.user_repo.find_by_id(r.requested_by).await {
            Ok(user) => Some(user.full_name),
            Err(_) => None,
        };

        let document_reference = match r.document_type {
            ApprovalDocumentType::JournalEntry => {
                match self.journal_repo.find_by_id(r.document_id).await {
                    Ok(journal) => Some(journal.reference_number),
                    Err(_) => None,
                }
            }
            ApprovalDocumentType::SalesInvoice => {
                match self.invoice_repo.find_sales_by_id(r.document_id).await {
                    Ok(invoice) => Some(invoice.invoice_number),
                    Err(_) => None,
                }
            }
            _ => None,
        };

        ApprovalResponse {
            id: r.id,
            company_id: r.company_id,
            document_type: r.document_type.as_str().to_string(),
            document_id: r.document_id,
            status: r.status.as_str().to_string(),
            requested_by: r.requested_by,
            requested_by_name,
            document_reference,
            reviewed_by: r.reviewed_by,
            reviewed_at: r.reviewed_at,
            comment: r.comment,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }

    /// Submit a journal entry for approval.
    #[instrument(skip(self))]
    pub async fn submit_journal_approval(
        &self,
        journal_id: Uuid,
        requested_by: Uuid,
    ) -> Result<ApprovalResponse, AppError> {
        let mut journal = self.journal_repo.find_by_id(journal_id).await?;

        if journal.status != DocumentStatus::Draft && journal.status != DocumentStatus::Rejected {
            return Err(AppError::Validation {
                message: "Only draft or rejected journals can be submitted for approval"
                    .to_string(),
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
        Ok(self.enrich_response(request).await)
    }

    /// Submit a sales invoice for approval.
    #[instrument(skip(self))]
    pub async fn submit_sales_invoice_approval(
        &self,
        invoice_id: Uuid,
        requested_by: Uuid,
    ) -> Result<ApprovalResponse, AppError> {
        let mut invoice = self.invoice_repo.find_sales_by_id(invoice_id).await?;

        if invoice.status != DocumentStatus::Draft && invoice.status != DocumentStatus::Rejected {
            return Err(AppError::Validation {
                message: "Only draft or rejected invoices can be submitted for approval"
                    .to_string(),
            });
        }

        // Update invoice status
        invoice.status = DocumentStatus::WaitingApproval;
        invoice.updated_at = time::OffsetDateTime::now_utc();
        self.invoice_repo.update_sales(&invoice).await?;

        // Create or reactivate approval request
        let now = time::OffsetDateTime::now_utc();
        let existing = self.approval_repo.find_by_document(invoice_id).await?;

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
                    company_id: invoice.company_id,
                    document_type: ApprovalDocumentType::SalesInvoice,
                    document_id: invoice_id,
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

        tracing::info!(invoice_id = %invoice_id, approval_id = %request.id, "Sales invoice submitted for approval");
        Ok(self.enrich_response(request).await)
    }

    /// Approve an approval request.
    #[instrument(skip(self))]
    pub async fn approve_request(
        &self,
        id: Uuid,
        reviewed_by: Uuid,
        comment: Option<String>,
    ) -> Result<ApprovalResponse, AppError> {
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
            ApprovalDocumentType::SalesInvoice => {
                let mut invoice = self.invoice_repo.find_sales_by_id(req.document_id).await?;
                if invoice.status != DocumentStatus::WaitingApproval {
                    return Err(AppError::Validation {
                        message: "Only waiting approval invoices can be approved".to_string(),
                    });
                }

                // 1. Resolve Accounts Receivable account (code 1200)
                let ar_code =
                    finance_assistant_domain::value_objects::AccountCode("1200".to_string());
                let ar_account = self
                    .account_repo
                    .find_by_code(invoice.company_id, &ar_code)
                    .await?
                    .ok_or_else(|| AppError::Validation {
                        message: "Accounts Receivable account (1200) not found".to_string(),
                    })?;

                // 2. Generate Journal Entry
                let journal_id = Uuid::new_v4();
                let mut journal_lines = Vec::new();
                let mut sort_order = 0;

                for line in &invoice.lines {
                    // Credit Revenue
                    journal_lines.push(finance_assistant_domain::entities::journal::JournalLine {
                        id: Uuid::new_v4(),
                        journal_entry_id: journal_id,
                        account_id: line.account_id,
                        debit: Decimal::ZERO,
                        credit: line.net_amount(),
                        description: Some(line.description.clone()),
                        sort_order,
                    });
                    sort_order += 1;

                    // Credit VAT/Tax if tax_amount > 0
                    if line.tax_amount > Decimal::ZERO {
                        let tax_id = line.tax_type_id.ok_or_else(|| AppError::Validation {
                            message: "Missing tax type configuration for tax amount".to_string(),
                        })?;
                        let tax_config = self.tax_repo.find_by_id(tax_id).await?;
                        journal_lines.push(
                            finance_assistant_domain::entities::journal::JournalLine {
                                id: Uuid::new_v4(),
                                journal_entry_id: journal_id,
                                account_id: tax_config.payable_account_id,
                                debit: Decimal::ZERO,
                                credit: line.tax_amount,
                                description: Some(format!("Tax for: {}", line.description)),
                                sort_order,
                            },
                        );
                        sort_order += 1;
                    }
                }

                // Debit Accounts Receivable
                journal_lines.push(finance_assistant_domain::entities::journal::JournalLine {
                    id: Uuid::new_v4(),
                    journal_entry_id: journal_id,
                    account_id: ar_account.id,
                    debit: invoice.total_amount,
                    credit: Decimal::ZERO,
                    description: Some(format!("Receivable for invoice {}", invoice.invoice_number)),
                    sort_order,
                });

                let journal_entry = finance_assistant_domain::entities::journal::JournalEntry {
                    id: journal_id,
                    company_id: invoice.company_id,
                    branch_id: invoice.branch_id,
                    reference_number: invoice.invoice_number.clone(),
                    description: format!("Auto-posting sales invoice {}", invoice.invoice_number),
                    transaction_date: invoice.invoice_date,
                    lines: journal_lines,
                    status: DocumentStatus::Posted, // Directly posted
                    source:
                        finance_assistant_domain::entities::journal::JournalSource::SalesInvoice,
                    source_document_id: Some(invoice.id),
                    created_by: invoice.created_by,
                    posted_by: Some(reviewed_by),
                    posted_at: Some(now),
                    created_at: now,
                    updated_at: now,
                };

                // Save journal entry to database
                self.journal_repo.save(&journal_entry).await?;

                // 3. Transition invoice status to Posted
                invoice.status = DocumentStatus::Posted;
                invoice.journal_entry_id = Some(journal_id);
                invoice.updated_at = now;
                self.invoice_repo.update_sales(&invoice).await?;
            }
            _ => {
                // Other document types will be implemented as tasks progress
            }
        }

        tracing::info!(approval_id = %id, reviewed_by = %reviewed_by, "Approval request approved");
        Ok(self.enrich_response(req).await)
    }

    /// Reject an approval request.
    #[instrument(skip(self))]
    pub async fn reject_request(
        &self,
        id: Uuid,
        reviewed_by: Uuid,
        comment: Option<String>,
    ) -> Result<ApprovalResponse, AppError> {
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
            ApprovalDocumentType::SalesInvoice => {
                let mut invoice = self.invoice_repo.find_sales_by_id(req.document_id).await?;
                invoice.status = DocumentStatus::Rejected;
                invoice.updated_at = now;
                self.invoice_repo.update_sales(&invoice).await?;
            }
            _ => {
                // Other document types will be implemented as tasks progress
            }
        }

        tracing::info!(approval_id = %id, reviewed_by = %reviewed_by, "Approval request rejected");
        Ok(self.enrich_response(req).await)
    }

    /// List all pending approval requests for a company.
    pub async fn list_pending(&self, company_id: Uuid) -> Result<Vec<ApprovalResponse>, AppError> {
        let list = self
            .approval_repo
            .find_pending_by_company(company_id)
            .await?;
        let mut responses = Vec::new();
        for r in list {
            responses.push(self.enrich_response(r).await);
        }
        Ok(responses)
    }

    /// Get details of an approval request by its ID.
    pub async fn get_by_id(&self, id: Uuid) -> Result<ApprovalResponse, AppError> {
        let req = self.approval_repo.find_by_id(id).await?;
        Ok(self.enrich_response(req).await)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use finance_assistant_domain::entities::account::Account;
    use finance_assistant_domain::entities::audit::AuditLog;
    use finance_assistant_domain::entities::invoice::{InvoiceLine, PurchaseInvoice, SalesInvoice};
    use finance_assistant_domain::entities::journal::{JournalEntry, JournalLine};
    use finance_assistant_domain::entities::tax::{TaxCategory, TaxType};
    use finance_assistant_domain::entities::user::{User, UserRole};
    use finance_assistant_domain::value_objects::{AccountCode, AccountType, DocumentStatus};
    use std::str::FromStr;
    use std::sync::Mutex;
    use time::OffsetDateTime;

    struct MockApprovalRepository {
        request: Mutex<Option<ApprovalRequest>>,
    }

    #[async_trait::async_trait]
    impl ApprovalRepository for MockApprovalRepository {
        async fn find_by_id(&self, _id: Uuid) -> Result<ApprovalRequest, AppError> {
            let req = self
                .request
                .lock()
                .unwrap()
                .clone()
                .ok_or_else(|| AppError::NotFound {
                    resource: "ApprovalRequest".to_string(),
                    id: _id.to_string(),
                })?;
            Ok(req)
        }
        async fn find_pending_by_company(
            &self,
            _company_id: Uuid,
        ) -> Result<Vec<ApprovalRequest>, AppError> {
            Ok(vec![])
        }
        async fn find_by_document(
            &self,
            _document_id: Uuid,
        ) -> Result<Option<ApprovalRequest>, AppError> {
            Ok(self.request.lock().unwrap().clone())
        }
        async fn save(&self, request: &ApprovalRequest) -> Result<(), AppError> {
            *self.request.lock().unwrap() = Some(request.clone());
            Ok(())
        }
        async fn update(&self, request: &ApprovalRequest) -> Result<(), AppError> {
            *self.request.lock().unwrap() = Some(request.clone());
            Ok(())
        }
    }

    struct MockJournalRepository {
        entry: Mutex<Option<JournalEntry>>,
    }

    #[async_trait::async_trait]
    impl JournalRepository for MockJournalRepository {
        async fn find_by_id(&self, _id: Uuid) -> Result<JournalEntry, AppError> {
            let entry = self
                .entry
                .lock()
                .unwrap()
                .clone()
                .ok_or_else(|| AppError::NotFound {
                    resource: "JournalEntry".to_string(),
                    id: _id.to_string(),
                })?;
            Ok(entry)
        }
        async fn find_by_company(
            &self,
            _company_id: Uuid,
            _page: u32,
            _per_page: u32,
        ) -> Result<Vec<JournalEntry>, AppError> {
            Ok(vec![])
        }
        async fn save(&self, entry: &JournalEntry) -> Result<(), AppError> {
            *self.entry.lock().unwrap() = Some(entry.clone());
            Ok(())
        }
        async fn update(&self, entry: &JournalEntry) -> Result<(), AppError> {
            *self.entry.lock().unwrap() = Some(entry.clone());
            Ok(())
        }
        async fn delete(&self, _id: Uuid) -> Result<(), AppError> {
            Ok(())
        }
        async fn save_lines(&self, _lines: &[JournalLine]) -> Result<(), AppError> {
            Ok(())
        }
        async fn count_by_company(&self, _company_id: Uuid) -> Result<i64, AppError> {
            Ok(0)
        }
    }

    struct MockAuditLogRepository;

    #[async_trait::async_trait]
    impl AuditLogRepository for MockAuditLogRepository {
        async fn append(&self, _log: &AuditLog) -> Result<(), AppError> {
            Ok(())
        }
        async fn find_by_entity(
            &self,
            _entity_type: &str,
            _entity_id: Uuid,
        ) -> Result<Vec<AuditLog>, AppError> {
            Ok(vec![])
        }
    }

    struct MockUserRepository;

    #[async_trait::async_trait]
    impl UserRepository for MockUserRepository {
        async fn find_by_id(&self, id: Uuid) -> Result<User, AppError> {
            Ok(User {
                id,
                company_id: Uuid::new_v4(),
                email: "reviewer@test.com".to_string(),
                full_name: "John Reviewer".to_string(),
                password_hash: "hash".to_string(),
                roles: vec![UserRole::FinanceManager],
                is_active: true,
                last_login_at: None,
                created_at: OffsetDateTime::now_utc(),
                updated_at: OffsetDateTime::now_utc(),
            })
        }
        async fn find_by_email(&self, _email: &str) -> Result<Option<User>, AppError> {
            Ok(None)
        }
        async fn save(&self, _user: &User) -> Result<(), AppError> {
            Ok(())
        }
        async fn update(&self, _user: &User) -> Result<(), AppError> {
            Ok(())
        }
    }

    struct MockInvoiceRepository {
        sales: Mutex<Option<SalesInvoice>>,
    }

    #[async_trait::async_trait]
    impl InvoiceRepository for MockInvoiceRepository {
        async fn find_sales_by_id(&self, _id: Uuid) -> Result<SalesInvoice, AppError> {
            let inv = self
                .sales
                .lock()
                .unwrap()
                .clone()
                .ok_or_else(|| AppError::NotFound {
                    resource: "SalesInvoice".to_string(),
                    id: _id.to_string(),
                })?;
            Ok(inv)
        }
        async fn find_sales_by_company(
            &self,
            _company_id: Uuid,
            _page: u32,
            _per_page: u32,
        ) -> Result<Vec<SalesInvoice>, AppError> {
            Ok(vec![])
        }
        async fn count_sales_by_company(&self, _company_id: Uuid) -> Result<u64, AppError> {
            Ok(0)
        }
        async fn save_sales(&self, invoice: &SalesInvoice) -> Result<(), AppError> {
            *self.sales.lock().unwrap() = Some(invoice.clone());
            Ok(())
        }
        async fn update_sales(&self, invoice: &SalesInvoice) -> Result<(), AppError> {
            *self.sales.lock().unwrap() = Some(invoice.clone());
            Ok(())
        }
        async fn delete_sales(&self, _id: Uuid) -> Result<(), AppError> {
            Ok(())
        }
        async fn find_purchase_by_id(&self, _id: Uuid) -> Result<PurchaseInvoice, AppError> {
            Err(AppError::NotFound {
                resource: "PurchaseInvoice".to_string(),
                id: _id.to_string(),
            })
        }
        async fn save_purchase(&self, _invoice: &PurchaseInvoice) -> Result<(), AppError> {
            Ok(())
        }
        async fn update_purchase(&self, _invoice: &PurchaseInvoice) -> Result<(), AppError> {
            Ok(())
        }
        async fn find_duplicate_purchase(
            &self,
            _company_id: Uuid,
            _supplier_id: Uuid,
            _invoice_number: &str,
        ) -> Result<bool, AppError> {
            Ok(false)
        }
    }

    struct MockAccountRepository;

    #[async_trait::async_trait]
    impl AccountRepository for MockAccountRepository {
        async fn find_by_id(&self, id: Uuid) -> Result<Account, AppError> {
            Ok(Account {
                id,
                company_id: Uuid::new_v4(),
                code: AccountCode("1200".to_string()),
                name: "Accounts Receivable".to_string(),
                account_type: AccountType::Asset,
                parent_id: None,
                is_active: true,
                created_at: OffsetDateTime::now_utc(),
                updated_at: OffsetDateTime::now_utc(),
            })
        }
        async fn find_by_code(
            &self,
            company_id: Uuid,
            code: &AccountCode,
        ) -> Result<Option<Account>, AppError> {
            Ok(Some(Account {
                id: Uuid::new_v4(),
                company_id,
                code: code.clone(),
                name: "Mock Account".to_string(),
                account_type: AccountType::Asset,
                parent_id: None,
                is_active: true,
                created_at: OffsetDateTime::now_utc(),
                updated_at: OffsetDateTime::now_utc(),
            }))
        }
        async fn find_all_by_company(&self, _company_id: Uuid) -> Result<Vec<Account>, AppError> {
            Ok(vec![])
        }
        async fn save(&self, _account: &Account) -> Result<(), AppError> {
            Ok(())
        }
        async fn update(&self, _account: &Account) -> Result<(), AppError> {
            Ok(())
        }
    }

    struct MockTaxRepository;

    #[async_trait::async_trait]
    impl TaxRepository for MockTaxRepository {
        async fn find_by_id(&self, id: Uuid) -> Result<TaxType, AppError> {
            Ok(TaxType {
                id,
                company_id: Uuid::new_v4(),
                code: "VAT10".to_string(),
                name: "VAT 10%".to_string(),
                category: TaxCategory::VatOutput,
                default_rate: Decimal::from_str("0.10").unwrap(),
                payable_account_id: Uuid::new_v4(),
                effective_from: time::Date::from_calendar_date(2020, time::Month::January, 1)
                    .unwrap(),
                effective_to: None,
                is_active: true,
                created_at: OffsetDateTime::now_utc(),
                updated_at: OffsetDateTime::now_utc(),
            })
        }
        async fn find_all_by_company(&self, _company_id: Uuid) -> Result<Vec<TaxType>, AppError> {
            Ok(vec![])
        }
        async fn save(&self, _tax_type: &TaxType) -> Result<(), AppError> {
            Ok(())
        }
        async fn update(&self, _tax_type: &TaxType) -> Result<(), AppError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_submit_and_approve_sales_invoice() {
        let approval_repo = Arc::new(MockApprovalRepository {
            request: Mutex::new(None),
        });
        let journal_repo = Arc::new(MockJournalRepository {
            entry: Mutex::new(None),
        });
        let audit_repo = Arc::new(MockAuditLogRepository);
        let user_repo = Arc::new(MockUserRepository);
        let invoice_repo = Arc::new(MockInvoiceRepository {
            sales: Mutex::new(None),
        });
        let account_repo = Arc::new(MockAccountRepository);
        let tax_repo = Arc::new(MockTaxRepository);

        let service = ApprovalService::new(
            approval_repo.clone(),
            journal_repo.clone(),
            audit_repo,
            user_repo,
            invoice_repo.clone(),
            account_repo,
            tax_repo,
        );

        let company_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let invoice_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        // 1. Setup draft sales invoice in repo
        let draft_invoice = SalesInvoice {
            id: invoice_id,
            company_id,
            branch_id: None,
            invoice_number: "INV-100".to_string(),
            customer_id,
            invoice_date: time::Date::from_calendar_date(2026, time::Month::June, 21).unwrap(),
            due_date: time::Date::from_calendar_date(2026, time::Month::July, 21).unwrap(),
            lines: vec![InvoiceLine {
                id: Uuid::new_v4(),
                item_id: None,
                description: "Taxable Item".to_string(),
                quantity: Decimal::from(1),
                unit_price: Decimal::from(100),
                discount_amount: Decimal::ZERO,
                tax_type_id: Some(Uuid::new_v4()),
                tax_rate: Some(Decimal::from_str("0.10").unwrap()),
                tax_amount: Decimal::from(10),
                line_total: Decimal::from(110),
                account_id: Uuid::new_v4(),
                sort_order: 1,
            }],
            subtotal: Decimal::from(100),
            tax_amount: Decimal::from(10),
            total_amount: Decimal::from(110),
            status: DocumentStatus::Draft,
            notes: None,
            journal_entry_id: None,
            created_by: user_id,
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        };
        *invoice_repo.sales.lock().unwrap() = Some(draft_invoice);

        // 2. Submit approval
        let submit_res = service
            .submit_sales_invoice_approval(invoice_id, user_id)
            .await
            .unwrap();
        assert_eq!(submit_res.status, "pending");

        let updated_inv = invoice_repo.sales.lock().unwrap().clone().unwrap();
        assert_eq!(updated_inv.status, DocumentStatus::WaitingApproval);

        // 3. Approve request
        let approval_req_id = submit_res.id;
        let approve_res = service
            .approve_request(approval_req_id, user_id, Some("Approved".to_string()))
            .await
            .unwrap();
        assert_eq!(approve_res.status, "approved");

        // Verify status transitioned to Posted
        let posted_inv = invoice_repo.sales.lock().unwrap().clone().unwrap();
        assert_eq!(posted_inv.status, DocumentStatus::Posted);
        assert!(posted_inv.journal_entry_id.is_some());

        // Verify balanced journal entry is created
        let journal = journal_repo.entry.lock().unwrap().clone().unwrap();
        assert_eq!(journal.status, DocumentStatus::Posted);
        assert_eq!(journal.lines.len(), 3); // 1 Revenue credit, 1 Tax credit, 1 Receivable debit

        // Line 0: credit Revenue
        assert_eq!(journal.lines[0].credit, Decimal::from(100));
        assert_eq!(journal.lines[0].debit, Decimal::ZERO);

        // Line 1: credit Tax
        assert_eq!(journal.lines[1].credit, Decimal::from(10));
        assert_eq!(journal.lines[1].debit, Decimal::ZERO);

        // Line 2: debit Accounts Receivable
        assert_eq!(journal.lines[2].debit, Decimal::from(110));
        assert_eq!(journal.lines[2].credit, Decimal::ZERO);
    }
}
