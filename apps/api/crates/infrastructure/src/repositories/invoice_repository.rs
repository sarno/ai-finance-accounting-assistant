use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use finance_assistant_app::{errors::AppError, ports::invoice_repository::InvoiceRepository};
use finance_assistant_domain::entities::invoice::{InvoiceLine, PurchaseInvoice, SalesInvoice};
use finance_assistant_domain::value_objects::DocumentStatus;

pub struct PgInvoiceRepository {
    pool: PgPool,
}

impl PgInvoiceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn should_generate_sales_invoice_number(
        invoice_number: &str,
        invoice_date: time::Date,
    ) -> bool {
        let prefix = format!("INV/{}/", invoice_date.year());
        invoice_number.trim().is_empty() || invoice_number.starts_with(&prefix)
    }

    fn should_generate_purchase_internal_reference(
        internal_reference: &str,
        invoice_date: time::Date,
    ) -> bool {
        let prefix = format!("PI/{}/", invoice_date.year());
        internal_reference.trim().is_empty() || internal_reference.starts_with(&prefix)
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
            None => {
                return Err(AppError::NotFound {
                    resource: "SalesInvoice".to_string(),
                    id: id.to_string(),
                })
            }
        };

        let lines_rows = sqlx::query(
            r#"
            SELECT id, sales_invoice_id, item_id, description, quantity, unit_price, discount_amount, tax_type_id, tax_rate, tax_amount, line_total, account_id, sort_order
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
                item_id: r.get("item_id"),
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

    async fn find_sales_by_company(
        &self,
        company_id: Uuid,
        page: u32,
        per_page: u32,
    ) -> Result<Vec<SalesInvoice>, AppError> {
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
            SELECT id, sales_invoice_id, item_id, description, quantity, unit_price, discount_amount, tax_type_id, tax_rate, tax_amount, line_total, account_id, sort_order
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
                        item_id: lr.get("item_id"),
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

    async fn save_sales(&self, invoice: &SalesInvoice) -> Result<SalesInvoice, AppError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

        let mut stored_invoice = invoice.clone();

        if Self::should_generate_sales_invoice_number(
            &stored_invoice.invoice_number,
            stored_invoice.invoice_date,
        ) {
            let lock_key = format!(
                "sales_invoice_number:{}:{}",
                stored_invoice.company_id,
                stored_invoice.invoice_date.year()
            );
            sqlx::query(
                r#"
                SELECT pg_advisory_xact_lock(hashtext($1), hashtext($1 || ':sequence'))
                "#,
            )
            .bind(&lock_key)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

            let row = sqlx::query(
                r#"
                SELECT COALESCE(
                    MAX((split_part(invoice_number, '/', 3))::int),
                    0
                ) AS max_sequence
                FROM sales_invoices
                WHERE company_id = $1
                  AND invoice_number LIKE $2
                  AND split_part(invoice_number, '/', 1) = 'INV'
                  AND split_part(invoice_number, '/', 2) = $3
                  AND split_part(invoice_number, '/', 3) ~ '^[0-9]+$'
                "#,
            )
            .bind(stored_invoice.company_id)
            .bind(format!("INV/{}/%", stored_invoice.invoice_date.year()))
            .bind(stored_invoice.invoice_date.year().to_string())
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

            let next_sequence: i64 = row.get("max_sequence");
            stored_invoice.invoice_number = format!(
                "INV/{}/{:03}",
                stored_invoice.invoice_date.year(),
                next_sequence + 1
            );
        }

        sqlx::query(
            r#"
            INSERT INTO sales_invoices (
                id, company_id, branch_id, invoice_number, customer_id, invoice_date, due_date,
                subtotal, tax_amount, total_amount, status, notes, journal_entry_id, created_by, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11::document_status, $12, $13, $14, $15, $16)
            "#,
        )
        .bind(stored_invoice.id)
        .bind(stored_invoice.company_id)
        .bind(stored_invoice.branch_id)
        .bind(&stored_invoice.invoice_number)
        .bind(stored_invoice.customer_id)
        .bind(stored_invoice.invoice_date)
        .bind(stored_invoice.due_date)
        .bind(stored_invoice.subtotal)
        .bind(stored_invoice.tax_amount)
        .bind(stored_invoice.total_amount)
        .bind(stored_invoice.status.clone())
        .bind(&stored_invoice.notes)
        .bind(stored_invoice.journal_entry_id)
        .bind(stored_invoice.created_by)
        .bind(stored_invoice.created_at)
        .bind(stored_invoice.updated_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        for line in &stored_invoice.lines {
            sqlx::query(
                r#"
                INSERT INTO sales_invoice_lines (
                    id, sales_invoice_id, description, quantity, unit_price, discount_amount,
                    tax_type_id, tax_rate, tax_amount, line_total, account_id, sort_order, item_id
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
                "#,
            )
            .bind(line.id)
            .bind(stored_invoice.id)
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
            .bind(line.item_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        }

        tx.commit()
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        Ok(stored_invoice)
    }

    async fn update_sales(&self, invoice: &SalesInvoice) -> Result<(), AppError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

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
                    tax_type_id, tax_rate, tax_amount, line_total, account_id, sort_order, item_id
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
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
            .bind(line.item_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        }

        tx.commit()
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
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

    async fn find_purchase_by_id(&self, id: Uuid) -> Result<PurchaseInvoice, AppError> {
        let invoice_row = sqlx::query(
            r#"
            SELECT id, company_id, branch_id, supplier_invoice_number, internal_reference, supplier_id, invoice_date, due_date, subtotal, tax_amount, total_amount, status, ai_confidence, uploaded_document_id, journal_entry_id, notes, created_by, created_at, updated_at
            FROM purchase_invoices
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let invoice_row = match invoice_row {
            Some(r) => r,
            None => {
                return Err(AppError::NotFound {
                    resource: "PurchaseInvoice".to_string(),
                    id: id.to_string(),
                })
            }
        };

        let lines_rows = sqlx::query(
            r#"
            SELECT id, purchase_invoice_id, item_id, description, quantity, unit_price, discount_amount, tax_type_id, tax_rate, tax_amount, line_total, account_id, sort_order
            FROM purchase_invoice_lines
            WHERE purchase_invoice_id = $1
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
                item_id: r.get("item_id"),
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

        Ok(PurchaseInvoice {
            id: invoice_row.get("id"),
            company_id: invoice_row.get("company_id"),
            branch_id: invoice_row.get("branch_id"),
            supplier_invoice_number: invoice_row.get("supplier_invoice_number"),
            internal_reference: invoice_row.get("internal_reference"),
            supplier_id: invoice_row.get("supplier_id"),
            invoice_date: invoice_row.get("invoice_date"),
            due_date: invoice_row.get("due_date"),
            lines,
            subtotal: invoice_row.get("subtotal"),
            tax_amount: invoice_row.get("tax_amount"),
            total_amount: invoice_row.get("total_amount"),
            status,
            ai_confidence: invoice_row.get("ai_confidence"),
            uploaded_document_id: invoice_row.get("uploaded_document_id"),
            journal_entry_id: invoice_row.get("journal_entry_id"),
            notes: invoice_row.get("notes"),
            created_by: invoice_row.get("created_by"),
            created_at: invoice_row.get("created_at"),
            updated_at: invoice_row.get("updated_at"),
        })
    }

    async fn find_purchase_by_company(
        &self,
        company_id: Uuid,
        page: u32,
        per_page: u32,
    ) -> Result<Vec<PurchaseInvoice>, AppError> {
        let limit = per_page as i64;
        let offset = ((page.max(1) - 1) * per_page) as i64;

        let invoice_rows = sqlx::query(
            r#"
            SELECT id, company_id, branch_id, supplier_invoice_number, internal_reference, supplier_id, invoice_date, due_date, subtotal, tax_amount, total_amount, status, ai_confidence, uploaded_document_id, journal_entry_id, notes, created_by, created_at, updated_at
            FROM purchase_invoices
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
            SELECT id, purchase_invoice_id, item_id, description, quantity, unit_price, discount_amount, tax_type_id, tax_rate, tax_amount, line_total, account_id, sort_order
            FROM purchase_invoice_lines
            WHERE purchase_invoice_id = ANY($1)
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
                let purchase_invoice_id: Uuid = lr.get("purchase_invoice_id");
                if purchase_invoice_id == id {
                    invoice_lines.push(InvoiceLine {
                        id: lr.get("id"),
                        item_id: lr.get("item_id"),
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

            invoices.push(PurchaseInvoice {
                id,
                company_id: r.get("company_id"),
                branch_id: r.get("branch_id"),
                supplier_invoice_number: r.get("supplier_invoice_number"),
                internal_reference: r.get("internal_reference"),
                supplier_id: r.get("supplier_id"),
                invoice_date: r.get("invoice_date"),
                due_date: r.get("due_date"),
                lines: invoice_lines,
                subtotal: r.get("subtotal"),
                tax_amount: r.get("tax_amount"),
                total_amount: r.get("total_amount"),
                status,
                ai_confidence: r.get("ai_confidence"),
                uploaded_document_id: r.get("uploaded_document_id"),
                journal_entry_id: r.get("journal_entry_id"),
                notes: r.get("notes"),
                created_by: r.get("created_by"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
            });
        }

        Ok(invoices)
    }

    async fn count_purchase_by_company(&self, company_id: Uuid) -> Result<u64, AppError> {
        let row = sqlx::query(
            r#"
            SELECT COUNT(*) as count
            FROM purchase_invoices
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

    async fn save_purchase(&self, invoice: &PurchaseInvoice) -> Result<PurchaseInvoice, AppError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

        let mut stored_invoice = invoice.clone();

        if Self::should_generate_purchase_internal_reference(
            &stored_invoice.internal_reference,
            stored_invoice.invoice_date,
        ) {
            let lock_key = format!(
                "purchase_invoice_reference:{}:{}",
                stored_invoice.company_id,
                stored_invoice.invoice_date.year()
            );
            sqlx::query(
                r#"
                SELECT pg_advisory_xact_lock(hashtext($1), hashtext($1 || ':sequence'))
                "#,
            )
            .bind(&lock_key)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

            let row = sqlx::query(
                r#"
                SELECT COALESCE(
                    MAX((split_part(internal_reference, '/', 3))::int),
                    0
                ) AS max_sequence
                FROM purchase_invoices
                WHERE company_id = $1
                  AND internal_reference LIKE $2
                  AND split_part(internal_reference, '/', 1) = 'PI'
                  AND split_part(internal_reference, '/', 2) = $3
                  AND split_part(internal_reference, '/', 3) ~ '^[0-9]+$'
                "#,
            )
            .bind(stored_invoice.company_id)
            .bind(format!("PI/{}/%", stored_invoice.invoice_date.year()))
            .bind(stored_invoice.invoice_date.year().to_string())
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

            let next_sequence: i64 = row.get("max_sequence");
            stored_invoice.internal_reference = format!(
                "PI/{}/{:03}",
                stored_invoice.invoice_date.year(),
                next_sequence + 1
            );
        }

        sqlx::query(
            r#"
            INSERT INTO purchase_invoices (
                id, company_id, branch_id, supplier_invoice_number, internal_reference, supplier_id, invoice_date, due_date,
                subtotal, tax_amount, total_amount, status, ai_confidence, uploaded_document_id, journal_entry_id, notes, created_by, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12::document_status, $13, $14, $15, $16, $17, $18, $19)
            "#,
        )
        .bind(stored_invoice.id)
        .bind(stored_invoice.company_id)
        .bind(stored_invoice.branch_id)
        .bind(&stored_invoice.supplier_invoice_number)
        .bind(&stored_invoice.internal_reference)
        .bind(stored_invoice.supplier_id)
        .bind(stored_invoice.invoice_date)
        .bind(stored_invoice.due_date)
        .bind(stored_invoice.subtotal)
        .bind(stored_invoice.tax_amount)
        .bind(stored_invoice.total_amount)
        .bind(stored_invoice.status.clone())
        .bind(stored_invoice.ai_confidence)
        .bind(stored_invoice.uploaded_document_id)
        .bind(stored_invoice.journal_entry_id)
        .bind(&stored_invoice.notes)
        .bind(stored_invoice.created_by)
        .bind(stored_invoice.created_at)
        .bind(stored_invoice.updated_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        for line in &stored_invoice.lines {
            sqlx::query(
                r#"
                INSERT INTO purchase_invoice_lines (
                    id, purchase_invoice_id, description, quantity, unit_price, discount_amount,
                    tax_type_id, tax_rate, tax_amount, line_total, account_id, sort_order, item_id
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
                "#,
            )
            .bind(line.id)
            .bind(stored_invoice.id)
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
            .bind(line.item_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        }

        tx.commit()
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        Ok(stored_invoice)
    }

    async fn update_purchase(&self, invoice: &PurchaseInvoice) -> Result<(), AppError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

        sqlx::query(
            r#"
            UPDATE purchase_invoices
            SET branch_id = $2,
                supplier_invoice_number = $3,
                internal_reference = $4,
                supplier_id = $5,
                invoice_date = $6,
                due_date = $7,
                subtotal = $8,
                tax_amount = $9,
                total_amount = $10,
                status = $11::document_status,
                ai_confidence = $12,
                uploaded_document_id = $13,
                journal_entry_id = $14,
                notes = $15,
                updated_at = $16
            WHERE id = $1
            "#,
        )
        .bind(invoice.id)
        .bind(invoice.branch_id)
        .bind(&invoice.supplier_invoice_number)
        .bind(&invoice.internal_reference)
        .bind(invoice.supplier_id)
        .bind(invoice.invoice_date)
        .bind(invoice.due_date)
        .bind(invoice.subtotal)
        .bind(invoice.tax_amount)
        .bind(invoice.total_amount)
        .bind(invoice.status.clone())
        .bind(invoice.ai_confidence)
        .bind(invoice.uploaded_document_id)
        .bind(invoice.journal_entry_id)
        .bind(&invoice.notes)
        .bind(invoice.updated_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        sqlx::query(
            r#"
            DELETE FROM purchase_invoice_lines
            WHERE purchase_invoice_id = $1
            "#,
        )
        .bind(invoice.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        for line in &invoice.lines {
            sqlx::query(
                r#"
                INSERT INTO purchase_invoice_lines (
                    id, purchase_invoice_id, description, quantity, unit_price, discount_amount,
                    tax_type_id, tax_rate, tax_amount, line_total, account_id, sort_order, item_id
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
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
            .bind(line.item_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        }

        tx.commit()
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        Ok(())
    }

    async fn delete_purchase(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            r#"
            DELETE FROM purchase_invoices
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;
        Ok(())
    }

    async fn find_duplicate_purchase(
        &self,
        company_id: Uuid,
        supplier_id: Uuid,
        invoice_number: &str,
    ) -> Result<bool, AppError> {
        let row = sqlx::query(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM purchase_invoices
                WHERE company_id = $1
                  AND supplier_id = $2
                  AND supplier_invoice_number = $3
            ) AS exists
            "#,
        )
        .bind(company_id)
        .bind(supplier_id)
        .bind(invoice_number)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(row.get::<bool, _>("exists"))
    }
}
