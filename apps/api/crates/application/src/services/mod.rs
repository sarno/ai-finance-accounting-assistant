//! Application services (use-cases).
//!
//! Each service takes repository/port traits as dependencies via constructor injection.

pub mod auth_service;
pub mod journal_service;
pub mod invoice_service;
pub mod payment_service;
pub mod approval_service;
pub mod report_service;
pub mod ai_orchestration_service;
pub mod audit_service;
pub mod master_data_service;
pub mod item_service;

