// Placeholder stubs for remaining port traits.

use async_trait::async_trait;
use uuid::Uuid;
use finance_assistant_domain::entities::{
    invoice::{SalesInvoice, PurchaseInvoice},
    payment::Payment,
    approval::ApprovalRequest,
    tax::TaxType,
};
use crate::errors::AppError;

#[async_trait]
pub trait InvoiceRepository: Send + Sync {
    async fn find_sales_by_id(&self, id: Uuid) -> Result<SalesInvoice, AppError>;
    async fn find_sales_by_company(&self, company_id: Uuid, page: u32, per_page: u32) -> Result<Vec<SalesInvoice>, AppError>;
    async fn count_sales_by_company(&self, company_id: Uuid) -> Result<u64, AppError>;
    async fn save_sales(&self, invoice: &SalesInvoice) -> Result<(), AppError>;
    async fn update_sales(&self, invoice: &SalesInvoice) -> Result<(), AppError>;
    async fn delete_sales(&self, id: Uuid) -> Result<(), AppError>;
    async fn find_purchase_by_id(&self, id: Uuid) -> Result<PurchaseInvoice, AppError>;
    async fn save_purchase(&self, invoice: &PurchaseInvoice) -> Result<(), AppError>;
    async fn update_purchase(&self, invoice: &PurchaseInvoice) -> Result<(), AppError>;
    async fn find_duplicate_purchase(&self, company_id: Uuid, supplier_id: Uuid, invoice_number: &str) -> Result<bool, AppError>;
}

#[async_trait]
pub trait PaymentRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Payment, AppError>;
    async fn save(&self, payment: &Payment) -> Result<(), AppError>;
    async fn update(&self, payment: &Payment) -> Result<(), AppError>;
}

#[async_trait]
pub trait ApprovalRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<ApprovalRequest, AppError>;
    async fn find_pending_by_company(&self, company_id: Uuid) -> Result<Vec<ApprovalRequest>, AppError>;
    async fn save(&self, request: &ApprovalRequest) -> Result<(), AppError>;
    async fn update(&self, request: &ApprovalRequest) -> Result<(), AppError>;
}

#[async_trait]
pub trait TaxRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<TaxType, AppError>;
    async fn find_effective_by_company(&self, company_id: Uuid, date: &time::Date) -> Result<Vec<TaxType>, AppError>;
    async fn save(&self, tax_type: &TaxType) -> Result<(), AppError>;
    async fn update(&self, tax_type: &TaxType) -> Result<(), AppError>;
}

#[async_trait]
pub trait ReportRepository: Send + Sync {
    async fn cash_position(&self, company_id: Uuid, as_of: time::Date) -> Result<crate::dto::report::CashPositionReport, AppError>;
    async fn profit_loss(&self, company_id: Uuid, period_from: time::Date, period_to: time::Date) -> Result<crate::dto::report::ProfitLossReport, AppError>;
}
