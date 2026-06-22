use async_trait::async_trait;
use uuid::Uuid;

use crate::errors::AppError;
use finance_assistant_domain::entities::tax::{TaxType, TaxRecord};

#[async_trait]
pub trait TaxRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<TaxType, AppError>;
    async fn find_all_by_company(&self, company_id: Uuid) -> Result<Vec<TaxType>, AppError>;
    async fn save(&self, tax_type: &TaxType) -> Result<(), AppError>;
    async fn update(&self, tax_type: &TaxType) -> Result<(), AppError>;
}

#[async_trait]
pub trait TaxRecordRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<TaxRecord, AppError>;
    async fn find_all_by_company(&self, company_id: Uuid, page: u32, per_page: u32) -> Result<Vec<TaxRecord>, AppError>;
    async fn count_by_company(&self, company_id: Uuid) -> Result<u64, AppError>;
    async fn save(&self, tax_record: &TaxRecord) -> Result<(), AppError>;
    async fn update(&self, tax_record: &TaxRecord) -> Result<(), AppError>;
    async fn get_summary(&self, company_id: Uuid, start_date: time::Date, end_date: time::Date) -> Result<Vec<TaxRecord>, AppError>;
}

use finance_assistant_domain::entities::tax::TaxCalendarEntry;

#[async_trait]
pub trait TaxCalendarRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<TaxCalendarEntry, AppError>;
    async fn find_all_by_company(&self, company_id: Uuid) -> Result<Vec<TaxCalendarEntry>, AppError>;
    async fn find_upcoming_reminders(&self, company_id: Uuid, before_date: time::Date) -> Result<Vec<TaxCalendarEntry>, AppError>;
    async fn save(&self, entry: &TaxCalendarEntry) -> Result<(), AppError>;
    async fn update(&self, entry: &TaxCalendarEntry) -> Result<(), AppError>;
}

