//! Port traits — interfaces that the infrastructure layer must implement.
//! These are dependency-inverted so domain/app logic never depends on sqlx directly.

pub mod account_repository;
pub mod journal_repository;
pub mod invoice_repository;
pub mod payment_repository;
pub mod approval_repository;
pub mod user_repository;
pub mod tax_repository;
pub mod audit_log_repository;
pub mod report_repository;
pub mod storage_port;
pub mod ai_client_port;
pub mod company_repository;
pub mod branch_repository;
pub mod customer_repository;
pub mod supplier_repository;
pub mod bank_account_repository;

