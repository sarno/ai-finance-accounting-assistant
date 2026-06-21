use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use finance_assistant_app::{
    errors::AppError,
    ports::journal_repository::JournalRepository,
};
use finance_assistant_domain::{
    entities::journal::{JournalEntry, JournalLine, JournalSource},
    value_objects::DocumentStatus,
};

pub struct PgJournalRepository {
    pool: PgPool,
}

impl PgJournalRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn parse_journal_source(s: &str) -> JournalSource {
    match s.to_lowercase().as_str() {
        "manual" => JournalSource::Manual,
        "sales_invoice" | "salesinvoice" => JournalSource::SalesInvoice,
        "purchase_invoice" | "purchaseinvoice" => JournalSource::PurchaseInvoice,
        "payment_received" | "paymentreceived" => JournalSource::PaymentReceived,
        "payment_paid" | "paymentpaid" => JournalSource::PaymentPaid,
        "expense" => JournalSource::Expense,
        "adjustment" => JournalSource::Adjustment,
        "reversal" => JournalSource::Reversal,
        _ => JournalSource::Manual,
    }
}

#[async_trait]
impl JournalRepository for PgJournalRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<JournalEntry, AppError> {
        let entry_row = sqlx::query(
            r#"
            SELECT id, company_id, branch_id, reference_number, description, transaction_date, status, source, source_document_id, created_by, posted_by, posted_at, created_at, updated_at
            FROM journal_entries
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let entry_row = match entry_row {
            Some(r) => r,
            None => return Err(AppError::NotFound {
                resource: "JournalEntry".to_string(),
                id: id.to_string(),
            }),
        };

        let lines_rows = sqlx::query(
            r#"
            SELECT id, journal_entry_id, account_id, debit, credit, description, sort_order
            FROM journal_lines
            WHERE journal_entry_id = $1
            ORDER BY sort_order ASC
            "#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let lines = lines_rows
            .into_iter()
            .map(|r| JournalLine {
                id: r.get("id"),
                journal_entry_id: r.get("journal_entry_id"),
                account_id: r.get("account_id"),
                debit: r.get("debit"),
                credit: r.get("credit"),
                description: r.get("description"),
                sort_order: r.get("sort_order"),
            })
            .collect();

        let source_str: String = entry_row.get("source");
        let status: DocumentStatus = entry_row.get("status");

        Ok(JournalEntry {
            id: entry_row.get("id"),
            company_id: entry_row.get("company_id"),
            branch_id: entry_row.get("branch_id"),
            reference_number: entry_row.get("reference_number"),
            description: entry_row.get("description"),
            transaction_date: entry_row.get("transaction_date"),
            lines,
            status,
            source: parse_journal_source(&source_str),
            source_document_id: entry_row.get("source_document_id"),
            created_by: entry_row.get("created_by"),
            posted_by: entry_row.get("posted_by"),
            posted_at: entry_row.get("posted_at"),
            created_at: entry_row.get("created_at"),
            updated_at: entry_row.get("updated_at"),
        })
    }

    async fn find_by_company(
        &self,
        company_id: Uuid,
        page: u32,
        per_page: u32,
    ) -> Result<Vec<JournalEntry>, AppError> {
        let limit = per_page as i64;
        let offset = ((page.max(1) - 1) * per_page) as i64;

        let entry_rows = sqlx::query(
            r#"
            SELECT id, company_id, branch_id, reference_number, description, transaction_date, status, source, source_document_id, created_by, posted_by, posted_at, created_at, updated_at
            FROM journal_entries
            WHERE company_id = $1
            ORDER BY transaction_date DESC, created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(company_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        if entry_rows.is_empty() {
            return Ok(vec![]);
        }

        let entry_ids: Vec<Uuid> = entry_rows.iter().map(|r| r.get("id")).collect();

        let lines_rows = sqlx::query(
            r#"
            SELECT id, journal_entry_id, account_id, debit, credit, description, sort_order
            FROM journal_lines
            WHERE journal_entry_id = ANY($1)
            ORDER BY sort_order ASC
            "#,
        )
        .bind(&entry_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let mut entries = Vec::with_capacity(entry_rows.len());

        for r in entry_rows {
            let id: Uuid = r.get("id");
            let mut entry_lines = Vec::new();

            for lr in &lines_rows {
                let j_id: Uuid = lr.get("journal_entry_id");
                if j_id == id {
                    entry_lines.push(JournalLine {
                        id: lr.get("id"),
                        journal_entry_id: j_id,
                        account_id: lr.get("account_id"),
                        debit: lr.get("debit"),
                        credit: lr.get("credit"),
                        description: lr.get("description"),
                        sort_order: lr.get("sort_order"),
                    });
                }
            }

            let source_str: String = r.get("source");
            let status: DocumentStatus = r.get("status");

            entries.push(JournalEntry {
                id,
                company_id: r.get("company_id"),
                branch_id: r.get("branch_id"),
                reference_number: r.get("reference_number"),
                description: r.get("description"),
                transaction_date: r.get("transaction_date"),
                lines: entry_lines,
                status,
                source: parse_journal_source(&source_str),
                source_document_id: r.get("source_document_id"),
                created_by: r.get("created_by"),
                posted_by: r.get("posted_by"),
                posted_at: r.get("posted_at"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
            });
        }

        Ok(entries)
    }

    async fn save(&self, entry: &JournalEntry) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await.map_err(|e| AppError::Internal(e.into()))?;

        sqlx::query(
            r#"
            INSERT INTO journal_entries (
                id, company_id, branch_id, reference_number, description,
                transaction_date, status, source, source_document_id, created_by, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7::document_status, $8, $9, $10, $11, $12)
            "#,
        )
        .bind(entry.id)
        .bind(entry.company_id)
        .bind(entry.branch_id)
        .bind(&entry.reference_number)
        .bind(&entry.description)
        .bind(entry.transaction_date)
        .bind(entry.status.clone())
        .bind(entry.source.to_string())
        .bind(entry.source_document_id)
        .bind(entry.created_by)
        .bind(entry.created_at)
        .bind(entry.updated_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        for line in &entry.lines {
            sqlx::query(
                r#"
                INSERT INTO journal_lines (id, journal_entry_id, account_id, debit, credit, description, sort_order)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
            )
            .bind(line.id)
            .bind(entry.id) // Ensure matching ID
            .bind(line.account_id)
            .bind(line.debit)
            .bind(line.credit)
            .bind(&line.description)
            .bind(line.sort_order)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        }

        tx.commit().await.map_err(|e| AppError::Internal(e.into()))?;
        Ok(())
    }

    async fn update(&self, entry: &JournalEntry) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await.map_err(|e| AppError::Internal(e.into()))?;

        sqlx::query(
            r#"
            UPDATE journal_entries
            SET status = $2::document_status,
                posted_by = $3,
                posted_at = $4,
                updated_at = $5
            WHERE id = $1
            "#,
        )
        .bind(entry.id)
        .bind(entry.status.clone())
        .bind(entry.posted_by)
        .bind(entry.posted_at)
        .bind(entry.updated_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        // Re-sync lines: delete and re-insert
        sqlx::query(
            r#"
            DELETE FROM journal_lines
            WHERE journal_entry_id = $1
            "#,
        )
        .bind(entry.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        for line in &entry.lines {
            sqlx::query(
                r#"
                INSERT INTO journal_lines (id, journal_entry_id, account_id, debit, credit, description, sort_order)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
            )
            .bind(line.id)
            .bind(entry.id)
            .bind(line.account_id)
            .bind(line.debit)
            .bind(line.credit)
            .bind(&line.description)
            .bind(line.sort_order)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        }

        tx.commit().await.map_err(|e| AppError::Internal(e.into()))?;
        Ok(())
    }

    async fn save_lines(&self, lines: &[JournalLine]) -> Result<(), AppError> {
        // Line-saving is handled directly in save/update transactions for consistency,
        // but we implement this to satisfy the repository trait signature.
        for line in lines {
            sqlx::query(
                r#"
                INSERT INTO journal_lines (id, journal_entry_id, account_id, debit, credit, description, sort_order)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT (id) DO UPDATE
                SET account_id = EXCLUDED.account_id,
                    debit = EXCLUDED.debit,
                    credit = EXCLUDED.credit,
                    description = EXCLUDED.description,
                    sort_order = EXCLUDED.sort_order
                "#,
            )
            .bind(line.id)
            .bind(line.journal_entry_id)
            .bind(line.account_id)
            .bind(line.debit)
            .bind(line.credit)
            .bind(&line.description)
            .bind(line.sort_order)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        }
        Ok(())
    }

    async fn count_by_company(&self, company_id: Uuid) -> Result<i64, AppError> {
        let row = sqlx::query(
            "SELECT COUNT(*) FROM journal_entries WHERE company_id = $1"
        )
        .bind(company_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let count: i64 = row.get(0);
        Ok(count)
    }
}
