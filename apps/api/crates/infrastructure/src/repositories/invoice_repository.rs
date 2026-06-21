use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use finance_assistant_app::{
    errors::AppError,
    ports::invoice_repository::InvoiceRepository,
};
use finance_assistant_domain::entities::invoice::{SalesInvoice, PurchaseInvoice, InvoiceLine};
use finance_assistant_domain::value_objects::DocumentStatus;

pub struct PgInvoiceRepository {
    pool: PgPool,
}

impl PgInvoiceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InvoiceRepository for PgInvoiceRepository {
    async fn find_sales_by_id(&self, id: Uuid) -> Result<SalesInvoice, AppError> {
        let invoice_row = sqlx::query(
            r#"
            SELECT id, company_id, branch_id, invoice_number, customer_id, invoice_date, due_date, subtotal, tax_amount, total_amount, status, notes, journal_entry_id, created_by, created_at, updated_at
            FROM sales_invoices
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let invoice_row = match invoice_row {
            Some(r) => r,
            None => return Err(AppError::NotFound {
                resource: "SalesInvoice".to_string(),
                id: id.to_string(),
            }),
        };

        let lines_rows = sqlx::query(
            r#"
            SELECT id, sales_invoice_id, description, quantity, unit_price, discount_amount, tax_type_id, tax_rate, tax_amount, line_total, account_id, sort_order
            FROM sales_invoice_lines
            WHERE sales_invoice_id = $1
            ORDER BY sort_order ASC
            "#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let lines = lines_rows
            .into_iter()
            .map(|r| InvoiceLine {
                id: r.get("id"),
                description: r.get("description"),
                quantity: r.get("quantity"),
                unit_price: r.get("unit_price"),
                discount_amount: r.get("discount_amount"),
                tax_type_id: r.get("tax_type_id"),
                tax_rate: r.get("tax_rate"),
                tax_amount: r.get("tax_amount"),
                line_total: r.get("line_total"),
                account_id: r.get("account_id"),
                sort_order: r.get("sort_order"),
            })
            .collect();

        let status: DocumentStatus = invoice_row.get("status");

        Ok(SalesInvoice {
            id: invoice_row.get("id"),
            company_id: invoice_row.get("company_id"),
            branch_id: invoice_row.get("branch_id"),
            invoice_number: invoice_row.get("invoice_number"),
            customer_id: invoice_row.get("customer_id"),
            invoice_date: invoice_row.get("invoice_date"),
            due_date: invoice_row.get("due_date"),
            lines,
            subtotal: invoice_row.get("subtotal"),
            tax_amount: invoice_row.get("tax_amount"),
            total_amount: invoice_row.get("total_amount"),
            status,
            notes: invoice_row.get("notes"),
            journal_entry_id: invoice_row.get("journal_entry_id"),
            created_by: invoice_row.get("created_by"),
            created_at: invoice_row.get("created_at"),
            updated_at: invoice_row.get("updated_at"),
        })
    }

    async fn find_sales_by_company(&self, company_id: Uuid, page: u32, per_page: u32) -> Result<Vec<SalesInvoice>, AppError> {
        let limit = per_page as i64;
        let offset = ((page.max(1) - 1) * per_page) as i64;

        let invoice_rows = sqlx::query(
            r#"
            SELECT id, company_id, branch_id, invoice_number, customer_id, invoice_date, due_date, subtotal, tax_amount, total_amount, status, notes, journal_entry_id, created_by, created_at, updated_at
            FROM sales_invoices
            WHERE company_id = $1
            ORDER BY invoice_date DESC, created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(company_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        if invoice_rows.is_empty() {
            return Ok(vec![]);
        }

        let invoice_ids: Vec<Uuid> = invoice_rows.iter().map(|r| r.get("id")).collect();

        let lines_rows = sqlx::query(
            r#"
            SELECT id, sales_invoice_id, description, quantity, unit_price, discount_amount, tax_type_id, tax_rate, tax_amount, line_total, account_id, sort_order
            FROM sales_invoice_lines
            WHERE sales_invoice_id = ANY($1)
            ORDER BY sort_order ASC
            "#,
        )
        .bind(&invoice_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let mut invoices = Vec::with_capacity(invoice_rows.len());

        for r in invoice_rows {
            let id: Uuid = r.get("id");
            let mut invoice_lines = Vec::new();

            for lr in &lines_rows {
                let si_id: Uuid = lr.get("sales_invoice_id");
                if si_id == id {
                    invoice_lines.push(InvoiceLine {
                        id: lr.get("id"),
                        description: lr.get("description"),
                        quantity: lr.get("quantity"),
                        unit_price: lr.get("unit_price"),
                        discount_amount: lr.get("discount_amount"),
                        tax_type_id: lr.get("tax_type_id"),
                        tax_rate: lr.get("tax_rate"),
                        tax_amount: lr.get("tax_amount"),
                        line_total: lr.get("line_total"),
                        account_id: lr.get("account_id"),
                        sort_order: lr.get("sort_order"),
                    });
                }
            }

            let status: DocumentStatus = r.get("status");

            invoices.push(SalesInvoice {
                id,
                company_id: r.get("company_id"),
                branch_id: r.get("branch_id"),
                invoice_number: r.get("invoice_number"),
                customer_id: r.get("customer_id"),
                invoice_date: r.get("invoice_date"),
                due_date: r.get("due_date"),
                lines: invoice_lines,
                subtotal: r.get("subtotal"),
                tax_amount: r.get("tax_amount"),
                total_amount: r.get("total_amount"),
                status,
                notes: r.get("notes"),
                journal_entry_id: r.get("journal_entry_id"),
                created_by: r.get("created_by"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
            });
        }

        Ok(invoices)
    }

    async fn count_sales_by_company(&self, company_id: Uuid) -> Result<u64, AppError> {
        let row = sqlx::query(
            r#"
            SELECT COUNT(*) as count
            FROM sales_invoices
            WHERE company_id = $1
            "#,
        )
        .bind(company_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let count: i64 = row.get("count");
        Ok(count as u64)
    }

    async fn save_sales(&self, invoice: &SalesInvoice) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await.map_err(|e| AppError::Internal(e.into()))?;

        sqlx::query(
            r#"
            INSERT INTO sales_invoices (
                id, company_id, branch_id, invoice_number, customer_id, invoice_date, due_date,
                subtotal, tax_amount, total_amount, status, notes, journal_entry_id, created_by, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11::document_status, $12, $13, $14, $15, $16)
            "#,
        )
        .bind(invoice.id)
        .bind(invoice.company_id)
        .bind(invoice.branch_id)
        .bind(&invoice.invoice_number)
        .bind(invoice.customer_id)
        .bind(invoice.invoice_date)
        .bind(invoice.due_date)
        .bind(invoice.subtotal)
        .bind(invoice.tax_amount)
        .bind(invoice.total_amount)
        .bind(invoice.status.clone())
        .bind(&invoice.notes)
        .bind(invoice.journal_entry_id)
        .bind(invoice.created_by)
        .bind(invoice.created_at)
        .bind(invoice.updated_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        for line in &invoice.lines {
            sqlx::query(
                r#"
                INSERT INTO sales_invoice_lines (
                    id, sales_invoice_id, description, quantity, unit_price, discount_amount,
                    tax_type_id, tax_rate, tax_amount, line_total, account_id, sort_order
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                "#,
            )
            .bind(line.id)
            .bind(invoice.id)
            .bind(&line.description)
            .bind(line.quantity)
            .bind(line.unit_price)
            .bind(line.discount_amount)
            .bind(line.tax_type_id)
            .bind(line.tax_rate)
            .bind(line.tax_amount)
            .bind(line.line_total)
            .bind(line.account_id)
            .bind(line.sort_order)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        }

        tx.commit().await.map_err(|e| AppError::Internal(e.into()))?;
        Ok(())
    }

    async fn update_sales(&self, invoice: &SalesInvoice) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await.map_err(|e| AppError::Internal(e.into()))?;

        sqlx::query(
            r#"
            UPDATE sales_invoices
            SET branch_id = $2,
                invoice_number = $3,
                customer_id = $4,
                invoice_date = $5,
                due_date = $6,
                subtotal = $7,
                tax_amount = $8,
                total_amount = $9,
                status = $10::document_status,
                notes = $11,
                journal_entry_id = $12,
                updated_at = $13
            WHERE id = $1
            "#,
        )
        .bind(invoice.id)
        .bind(invoice.branch_id)
        .bind(&invoice.invoice_number)
        .bind(invoice.customer_id)
        .bind(invoice.invoice_date)
        .bind(invoice.due_date)
        .bind(invoice.subtotal)
        .bind(invoice.tax_amount)
        .bind(invoice.total_amount)
        .bind(invoice.status.clone())
        .bind(&invoice.notes)
        .bind(invoice.journal_entry_id)
        .bind(invoice.updated_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        sqlx::query(
            r#"
            DELETE FROM sales_invoice_lines
            WHERE sales_invoice_id = $1
            "#,
        )
        .bind(invoice.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        for line in &invoice.lines {
            sqlx::query(
                r#"
                INSERT INTO sales_invoice_lines (
                    id, sales_invoice_id, description, quantity, unit_price, discount_amount,
                    tax_type_id, tax_rate, tax_amount, line_total, account_id, sort_order
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                "#,
            )
            .bind(line.id)
            .bind(invoice.id)
            .bind(&line.description)
            .bind(line.quantity)
            .bind(line.unit_price)
            .bind(line.discount_amount)
            .bind(line.tax_type_id)
            .bind(line.tax_rate)
            .bind(line.tax_amount)
            .bind(line.line_total)
            .bind(line.account_id)
            .bind(line.sort_order)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        }

        tx.commit().await.map_err(|e| AppError::Internal(e.into()))?;
        Ok(())
    }

    async fn delete_sales(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            r#"
            DELETE FROM sales_invoices
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;
        Ok(())
    }

    // Purchase Invoice stubs (Task 009)
    async fn find_purchase_by_id(&self, _id: Uuid) -> Result<PurchaseInvoice, AppError> {
        Err(AppError::NotFound { resource: "PurchaseInvoice".to_string(), id: _id.to_string() })
    }
    async fn save_purchase(&self, _invoice: &PurchaseInvoice) -> Result<(), AppError> {
        Err(AppError::Internal(anyhow::anyhow!("PurchaseInvoice save not implemented")))
    }
    async fn update_purchase(&self, _invoice: &PurchaseInvoice) -> Result<(), AppError> {
        Err(AppError::Internal(anyhow::anyhow!("PurchaseInvoice update not implemented")))
    }
    async fn find_duplicate_purchase(&self, _company_id: Uuid, _supplier_id: Uuid, _invoice_number: &str) -> Result<bool, AppError> {
        Ok(false)
    }
}
