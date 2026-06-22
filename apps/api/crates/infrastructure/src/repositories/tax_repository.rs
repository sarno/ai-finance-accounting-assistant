use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use finance_assistant_app::{errors::AppError, ports::tax_repository::TaxRepository};
use finance_assistant_domain::entities::tax::{TaxCategory, TaxType};

pub struct PgTaxRepository {
    pool: PgPool,
}

impl PgTaxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn map_category_to_str(cat: &TaxCategory) -> &'static str {
    match cat {
        TaxCategory::VatOutput => "vat_output",
        TaxCategory::VatInput => "vat_input",
        TaxCategory::WithholdingPph21 => "withholding_pph21",
        TaxCategory::WithholdingPph23 => "withholding_pph23",
        TaxCategory::WithholdingPph25 => "withholding_pph25",
        TaxCategory::WithholdingPphFinal => "withholding_pph_final",
    }
}

fn map_str_to_category(s: &str) -> TaxCategory {
    match s.to_lowercase().as_str() {
        "vat_output" | "vatoutput" => TaxCategory::VatOutput,
        "vat_input" | "vatinput" => TaxCategory::VatInput,
        "withholding_pph21" | "withholdingpph21" => TaxCategory::WithholdingPph21,
        "withholding_pph23" | "withholdingpph23" => TaxCategory::WithholdingPph23,
        "withholding_pph25" | "withholdingpph25" => TaxCategory::WithholdingPph25,
        "withholding_pph_final" | "withholdingpphfinal" => TaxCategory::WithholdingPphFinal,
        _ => TaxCategory::VatOutput,
    }
}

#[async_trait]
impl TaxRepository for PgTaxRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<TaxType, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, company_id, code, name, category, default_rate, payable_account_id, effective_from, effective_to, is_active, created_at, updated_at
            FROM tax_types
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let row = match row {
            Some(r) => r,
            None => {
                return Err(AppError::NotFound {
                    resource: "TaxType".to_string(),
                    id: id.to_string(),
                })
            }
        };

        let cat_str: String = row.get("category");

        Ok(TaxType {
            id: row.get("id"),
            company_id: row.get("company_id"),
            code: row.get("code"),
            name: row.get("name"),
            category: map_str_to_category(&cat_str),
            default_rate: row.get("default_rate"),
            payable_account_id: row.get("payable_account_id"),
            effective_from: row.get("effective_from"),
            effective_to: row.get("effective_to"),
            is_active: row.get("is_active"),
            created_at: row.get::<time::OffsetDateTime, _>("created_at"),
            updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
        })
    }

    async fn find_all_by_company(&self, company_id: Uuid) -> Result<Vec<TaxType>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, company_id, code, name, category, default_rate, payable_account_id, effective_from, effective_to, is_active, created_at, updated_at
            FROM tax_types
            WHERE company_id = $1
            ORDER BY code ASC
            "#,
        )
        .bind(company_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let tax_types = rows
            .into_iter()
            .map(|row| {
                let cat_str: String = row.get("category");
                TaxType {
                    id: row.get("id"),
                    company_id: row.get("company_id"),
                    code: row.get("code"),
                    name: row.get("name"),
                    category: map_str_to_category(&cat_str),
                    default_rate: row.get("default_rate"),
                    payable_account_id: row.get("payable_account_id"),
                    effective_from: row.get("effective_from"),
                    effective_to: row.get("effective_to"),
                    is_active: row.get("is_active"),
                    created_at: row.get::<time::OffsetDateTime, _>("created_at"),
                    updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
                }
            })
            .collect();

        Ok(tax_types)
    }

    async fn save(&self, tax_type: &TaxType) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO tax_types (id, company_id, code, name, category, default_rate, payable_account_id, effective_from, effective_to, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
        )
        .bind(tax_type.id)
        .bind(tax_type.company_id)
        .bind(&tax_type.code)
        .bind(&tax_type.name)
        .bind(map_category_to_str(&tax_type.category))
        .bind(tax_type.default_rate)
        .bind(tax_type.payable_account_id)
        .bind(tax_type.effective_from)
        .bind(tax_type.effective_to)
        .bind(tax_type.is_active)
        .bind(tax_type.created_at)
        .bind(tax_type.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }

    async fn update(&self, tax_type: &TaxType) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE tax_types
            SET name = $2, category = $3, default_rate = $4, payable_account_id = $5, effective_from = $6, effective_to = $7, is_active = $8, updated_at = $9
            WHERE id = $1
            "#,
        )
        .bind(tax_type.id)
        .bind(&tax_type.name)
        .bind(map_category_to_str(&tax_type.category))
        .bind(tax_type.default_rate)
        .bind(tax_type.payable_account_id)
        .bind(tax_type.effective_from)
        .bind(tax_type.effective_to)
        .bind(tax_type.is_active)
        .bind(tax_type.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }
}

use finance_assistant_app::ports::tax_repository::TaxRecordRepository;
use finance_assistant_domain::entities::tax::{TaxRecord, TaxRecordStatus};

#[async_trait::async_trait]
impl TaxRecordRepository for PgTaxRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<TaxRecord, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, company_id, tax_type_id, source_document_type, source_document_id, tax_period, tax_base_amount, tax_rate, tax_amount, status, counterparty_name, counterparty_npwp, created_at, updated_at
            FROM tax_records
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let row = match row {
            Some(r) => r,
            None => {
                return Err(AppError::NotFound {
                    resource: "TaxRecord".to_string(),
                    id: id.to_string(),
                })
            }
        };

        let status_str: String = row.get("status");

        Ok(TaxRecord {
            id: row.get("id"),
            company_id: row.get("company_id"),
            tax_type_id: row.get("tax_type_id"),
            source_document_type: row.get("source_document_type"),
            source_document_id: row.get("source_document_id"),
            tax_period: row.get("tax_period"),
            tax_base_amount: row.get("tax_base_amount"),
            tax_rate: row.get("tax_rate"),
            tax_amount: row.get("tax_amount"),
            status: TaxRecordStatus::from_str(&status_str),
            counterparty_name: row.get("counterparty_name"),
            counterparty_npwp: row.get("counterparty_npwp"),
            created_at: row.get::<time::OffsetDateTime, _>("created_at"),
            updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
        })
    }

    async fn find_all_by_company(&self, company_id: Uuid, page: u32, per_page: u32) -> Result<Vec<TaxRecord>, AppError> {
        let offset = (page.saturating_sub(1)) * per_page;
        let rows = sqlx::query(
            r#"
            SELECT id, company_id, tax_type_id, source_document_type, source_document_id, tax_period, tax_base_amount, tax_rate, tax_amount, status, counterparty_name, counterparty_npwp, created_at, updated_at
            FROM tax_records
            WHERE company_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(company_id)
        .bind(per_page as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let records = rows
            .into_iter()
            .map(|row| {
                let status_str: String = row.get("status");
                TaxRecord {
                    id: row.get("id"),
                    company_id: row.get("company_id"),
                    tax_type_id: row.get("tax_type_id"),
                    source_document_type: row.get("source_document_type"),
                    source_document_id: row.get("source_document_id"),
                    tax_period: row.get("tax_period"),
                    tax_base_amount: row.get("tax_base_amount"),
                    tax_rate: row.get("tax_rate"),
                    tax_amount: row.get("tax_amount"),
                    status: TaxRecordStatus::from_str(&status_str),
                    counterparty_name: row.get("counterparty_name"),
                    counterparty_npwp: row.get("counterparty_npwp"),
                    created_at: row.get::<time::OffsetDateTime, _>("created_at"),
                    updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
                }
            })
            .collect();

        Ok(records)
    }

    async fn count_by_company(&self, company_id: Uuid) -> Result<u64, AppError> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM tax_records WHERE company_id = $1
            "#,
        )
        .bind(company_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(count as u64)
    }

    async fn save(&self, r: &TaxRecord) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO tax_records (id, company_id, tax_type_id, source_document_type, source_document_id, tax_period, tax_base_amount, tax_rate, tax_amount, status, counterparty_name, counterparty_npwp, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            "#,
        )
        .bind(r.id)
        .bind(r.company_id)
        .bind(r.tax_type_id)
        .bind(&r.source_document_type)
        .bind(r.source_document_id)
        .bind(r.tax_period)
        .bind(r.tax_base_amount)
        .bind(r.tax_rate)
        .bind(r.tax_amount)
        .bind(r.status.as_str())
        .bind(&r.counterparty_name)
        .bind(&r.counterparty_npwp)
        .bind(r.created_at)
        .bind(r.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }

    async fn update(&self, r: &TaxRecord) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE tax_records
            SET status = $2, counterparty_name = $3, counterparty_npwp = $4, updated_at = $5
            WHERE id = $1
            "#,
        )
        .bind(r.id)
        .bind(r.status.as_str())
        .bind(&r.counterparty_name)
        .bind(&r.counterparty_npwp)
        .bind(r.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }

    async fn get_summary(&self, company_id: Uuid, start_date: time::Date, end_date: time::Date) -> Result<Vec<TaxRecord>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, company_id, tax_type_id, source_document_type, source_document_id, tax_period, tax_base_amount, tax_rate, tax_amount, status, counterparty_name, counterparty_npwp, created_at, updated_at
            FROM tax_records
            WHERE company_id = $1 AND tax_period >= $2 AND tax_period <= $3
            ORDER BY tax_period ASC, created_at ASC
            "#,
        )
        .bind(company_id)
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let records = rows
            .into_iter()
            .map(|row| {
                let status_str: String = row.get("status");
                TaxRecord {
                    id: row.get("id"),
                    company_id: row.get("company_id"),
                    tax_type_id: row.get("tax_type_id"),
                    source_document_type: row.get("source_document_type"),
                    source_document_id: row.get("source_document_id"),
                    tax_period: row.get("tax_period"),
                    tax_base_amount: row.get("tax_base_amount"),
                    tax_rate: row.get("tax_rate"),
                    tax_amount: row.get("tax_amount"),
                    status: TaxRecordStatus::from_str(&status_str),
                    counterparty_name: row.get("counterparty_name"),
                    counterparty_npwp: row.get("counterparty_npwp"),
                    created_at: row.get::<time::OffsetDateTime, _>("created_at"),
                    updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
                }
            })
            .collect();

        Ok(records)
    }
}

use finance_assistant_app::ports::tax_repository::TaxCalendarRepository;
use finance_assistant_domain::entities::tax::{TaxCalendarEntry, TaxPaymentStatus, TaxFilingStatus};

#[async_trait::async_trait]
impl TaxCalendarRepository for PgTaxRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<TaxCalendarEntry, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, company_id, tax_type_id, tax_period, payment_due_date, filing_due_date, payment_status, filing_status, reminder_sent_at, created_at, updated_at
            FROM tax_calendar
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let row = match row {
            Some(r) => r,
            None => {
                return Err(AppError::NotFound {
                    resource: "TaxCalendarEntry".to_string(),
                    id: id.to_string(),
                })
            }
        };

        let payment_status_str: String = row.get("payment_status");
        let filing_status_str: String = row.get("filing_status");

        Ok(TaxCalendarEntry {
            id: row.get("id"),
            company_id: row.get("company_id"),
            tax_type_id: row.get("tax_type_id"),
            tax_period: row.get("tax_period"),
            payment_due_date: row.get("payment_due_date"),
            filing_due_date: row.get("filing_due_date"),
            payment_status: TaxPaymentStatus::from_str(&payment_status_str),
            filing_status: TaxFilingStatus::from_str(&filing_status_str),
            reminder_sent_at: row.get("reminder_sent_at"),
            created_at: row.get::<time::OffsetDateTime, _>("created_at"),
            updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
        })
    }

    async fn find_all_by_company(&self, company_id: Uuid) -> Result<Vec<TaxCalendarEntry>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, company_id, tax_type_id, tax_period, payment_due_date, filing_due_date, payment_status, filing_status, reminder_sent_at, created_at, updated_at
            FROM tax_calendar
            WHERE company_id = $1
            ORDER BY tax_period DESC
            "#,
        )
        .bind(company_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let entries = rows
            .into_iter()
            .map(|row| {
                let payment_status_str: String = row.get("payment_status");
                let filing_status_str: String = row.get("filing_status");
                TaxCalendarEntry {
                    id: row.get("id"),
                    company_id: row.get("company_id"),
                    tax_type_id: row.get("tax_type_id"),
                    tax_period: row.get("tax_period"),
                    payment_due_date: row.get("payment_due_date"),
                    filing_due_date: row.get("filing_due_date"),
                    payment_status: TaxPaymentStatus::from_str(&payment_status_str),
                    filing_status: TaxFilingStatus::from_str(&filing_status_str),
                    reminder_sent_at: row.get("reminder_sent_at"),
                    created_at: row.get::<time::OffsetDateTime, _>("created_at"),
                    updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
                }
            })
            .collect();

        Ok(entries)
    }

    async fn find_upcoming_reminders(&self, company_id: Uuid, before_date: time::Date) -> Result<Vec<TaxCalendarEntry>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, company_id, tax_type_id, tax_period, payment_due_date, filing_due_date, payment_status, filing_status, reminder_sent_at, created_at, updated_at
            FROM tax_calendar
            WHERE company_id = $1 
              AND (payment_due_date <= $2 OR filing_due_date <= $2)
              AND (payment_status = 'unpaid' OR filing_status = 'unfiled')
              AND reminder_sent_at IS NULL
            "#,
        )
        .bind(company_id)
        .bind(before_date)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let entries = rows
            .into_iter()
            .map(|row| {
                let payment_status_str: String = row.get("payment_status");
                let filing_status_str: String = row.get("filing_status");
                TaxCalendarEntry {
                    id: row.get("id"),
                    company_id: row.get("company_id"),
                    tax_type_id: row.get("tax_type_id"),
                    tax_period: row.get("tax_period"),
                    payment_due_date: row.get("payment_due_date"),
                    filing_due_date: row.get("filing_due_date"),
                    payment_status: TaxPaymentStatus::from_str(&payment_status_str),
                    filing_status: TaxFilingStatus::from_str(&filing_status_str),
                    reminder_sent_at: row.get("reminder_sent_at"),
                    created_at: row.get::<time::OffsetDateTime, _>("created_at"),
                    updated_at: row.get::<time::OffsetDateTime, _>("updated_at"),
                }
            })
            .collect();

        Ok(entries)
    }

    async fn save(&self, entry: &TaxCalendarEntry) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO tax_calendar (id, company_id, tax_type_id, tax_period, payment_due_date, filing_due_date, payment_status, filing_status, reminder_sent_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (company_id, tax_type_id, tax_period) DO UPDATE
            SET payment_due_date = EXCLUDED.payment_due_date,
                filing_due_date = EXCLUDED.filing_due_date,
                payment_status = EXCLUDED.payment_status,
                filing_status = EXCLUDED.filing_status,
                reminder_sent_at = EXCLUDED.reminder_sent_at,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(entry.id)
        .bind(entry.company_id)
        .bind(entry.tax_type_id)
        .bind(entry.tax_period)
        .bind(entry.payment_due_date)
        .bind(entry.filing_due_date)
        .bind(entry.payment_status.as_str())
        .bind(entry.filing_status.as_str())
        .bind(entry.reminder_sent_at)
        .bind(entry.created_at)
        .bind(entry.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }

    async fn update(&self, entry: &TaxCalendarEntry) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE tax_calendar
            SET payment_due_date = $2,
                filing_due_date = $3,
                payment_status = $4,
                filing_status = $5,
                reminder_sent_at = $6,
                updated_at = $7
            WHERE id = $1
            "#,
        )
        .bind(entry.id)
        .bind(entry.payment_due_date)
        .bind(entry.filing_due_date)
        .bind(entry.payment_status.as_str())
        .bind(entry.filing_status.as_str())
        .bind(entry.reminder_sent_at)
        .bind(entry.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }
}

