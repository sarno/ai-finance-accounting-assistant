use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use finance_assistant_app::{errors::AppError, ports::payment_repository::PaymentRepository};
use finance_assistant_domain::{
    entities::payment::{Payment, PaymentAllocation, PaymentType, CounterpartyType, AllocatedDocumentType},
    value_objects::DocumentStatus,
};

pub struct PgPaymentRepository {
    pool: PgPool,
}

impl PgPaymentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PaymentRepository for PgPaymentRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Payment, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, company_id, reference_number, payment_type, counterparty_type, counterparty_id,
                   payment_date, bank_account_id, amount, status, notes, journal_entry_id, attachment_url,
                   created_by, created_at, updated_at
            FROM payments
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
                    resource: "Payment".to_string(),
                    id: id.to_string(),
                })
            }
        };

        // Load allocations
        let allocation_rows = sqlx::query(
            r#"
            SELECT id, payment_id, document_type, document_id, allocated_amount
            FROM payment_allocations
            WHERE payment_id = $1
            "#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let mut allocations = Vec::new();
        for r in allocation_rows {
            let doc_type_str: String = r.get("document_type");
            let document_type = match doc_type_str.as_str() {
                "sales_invoice" => AllocatedDocumentType::SalesInvoice,
                "purchase_invoice" => AllocatedDocumentType::PurchaseInvoice,
                other => return Err(AppError::Internal(anyhow::anyhow!("Unknown allocated document type: {}", other))),
            };
            allocations.push(PaymentAllocation {
                id: r.get("id"),
                payment_id: r.get("payment_id"),
                document_type,
                document_id: r.get("document_id"),
                allocated_amount: r.get("allocated_amount"),
            });
        }

        let payment_type_str: String = row.get("payment_type");
        let payment_type = match payment_type_str.as_str() {
            "payment_received" => PaymentType::PaymentReceived,
            "payment_paid" => PaymentType::PaymentPaid,
            other => return Err(AppError::Internal(anyhow::anyhow!("Unknown payment type: {}", other))),
        };

        let counterparty_type_str: String = row.get("counterparty_type");
        let counterparty_type = match counterparty_type_str.as_str() {
            "customer" => CounterpartyType::Customer,
            "supplier" => CounterpartyType::Supplier,
            other => return Err(AppError::Internal(anyhow::anyhow!("Unknown counterparty type: {}", other))),
        };

        let status: DocumentStatus = row.get("status");

        Ok(Payment {
            id: row.get("id"),
            company_id: row.get("company_id"),
            reference_number: row.get("reference_number"),
            payment_type,
            counterparty_type,
            counterparty_id: row.get("counterparty_id"),
            payment_date: row.get("payment_date"),
            bank_account_id: row.get("bank_account_id"),
            amount: row.get("amount"),
            allocations,
            status,
            notes: row.get("notes"),
            journal_entry_id: row.get("journal_entry_id"),
            attachment_url: row.get("attachment_url"),
            created_by: row.get("created_by"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    async fn find_by_company(&self, company_id: Uuid, page: u32, per_page: u32) -> Result<Vec<Payment>, AppError> {
        let limit = per_page as i64;
        let offset = ((page.max(1) - 1) * per_page) as i64;

        let rows = sqlx::query(
            r#"
            SELECT id, company_id, reference_number, payment_type, counterparty_type, counterparty_id,
                   payment_date, bank_account_id, amount, status, notes, journal_entry_id, attachment_url,
                   created_by, created_at, updated_at
            FROM payments
            WHERE company_id = $1
            ORDER BY payment_date DESC, created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(company_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        if rows.is_empty() {
            return Ok(vec![]);
        }

        let payment_ids: Vec<Uuid> = rows.iter().map(|r| r.get("id")).collect();

        let allocation_rows = sqlx::query(
            r#"
            SELECT id, payment_id, document_type, document_id, allocated_amount
            FROM payment_allocations
            WHERE payment_id = ANY($1)
            "#,
        )
        .bind(&payment_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        let mut allocations_by_payment = std::collections::HashMap::new();
        for r in allocation_rows {
            let payment_id: Uuid = r.get("payment_id");
            let doc_type_str: String = r.get("document_type");
            let document_type = match doc_type_str.as_str() {
                "sales_invoice" => AllocatedDocumentType::SalesInvoice,
                "purchase_invoice" => AllocatedDocumentType::PurchaseInvoice,
                other => return Err(AppError::Internal(anyhow::anyhow!("Unknown allocated document type: {}", other))),
            };
            let allocation = PaymentAllocation {
                id: r.get("id"),
                payment_id,
                document_type,
                document_id: r.get("document_id"),
                allocated_amount: r.get("allocated_amount"),
            };
            allocations_by_payment.entry(payment_id).or_insert_with(Vec::new).push(allocation);
        }

        let mut payments = Vec::new();
        for row in rows {
            let id: Uuid = row.get("id");
            let payment_type_str: String = row.get("payment_type");
            let payment_type = match payment_type_str.as_str() {
                "payment_received" => PaymentType::PaymentReceived,
                "payment_paid" => PaymentType::PaymentPaid,
                other => return Err(AppError::Internal(anyhow::anyhow!("Unknown payment type: {}", other))),
            };

            let counterparty_type_str: String = row.get("counterparty_type");
            let counterparty_type = match counterparty_type_str.as_str() {
                "customer" => CounterpartyType::Customer,
                "supplier" => CounterpartyType::Supplier,
                other => return Err(AppError::Internal(anyhow::anyhow!("Unknown counterparty type: {}", other))),
            };

            let status: DocumentStatus = row.get("status");
            let allocations = allocations_by_payment.remove(&id).unwrap_or_default();

            payments.push(Payment {
                id,
                company_id: row.get("company_id"),
                reference_number: row.get("reference_number"),
                payment_type,
                counterparty_type,
                counterparty_id: row.get("counterparty_id"),
                payment_date: row.get("payment_date"),
                bank_account_id: row.get("bank_account_id"),
                amount: row.get("amount"),
                allocations,
                status,
                notes: row.get("notes"),
                journal_entry_id: row.get("journal_entry_id"),
                attachment_url: row.get("attachment_url"),
                created_by: row.get("created_by"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(payments)
    }

    async fn count_by_company(&self, company_id: Uuid) -> Result<u64, AppError> {
        let row = sqlx::query(
            r#"
            SELECT COUNT(*) as count
            FROM payments
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

    async fn save(&self, payment: &Payment) -> Result<Payment, AppError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

        let mut stored_payment = payment.clone();

        // Generate sequence number if not set or is temp
        if stored_payment.reference_number.is_empty() || stored_payment.reference_number.starts_with("TEMP") {
            let year = stored_payment.payment_date.year();
            let prefix = match stored_payment.payment_type {
                PaymentType::PaymentReceived => format!("PAY/RCV/{}/", year),
                PaymentType::PaymentPaid => format!("PAY/PAY/{}/", year),
            };

            // advisory lock
            let lock_key = format!("payment_reference:{}:{}", stored_payment.company_id, prefix);
            sqlx::query("SELECT pg_advisory_xact_lock(hashtext($1), hashtext($1 || ':seq'))")
                .bind(&lock_key)
                .execute(&mut *tx)
                .await
                .map_err(|e| AppError::Internal(e.into()))?;

            let row = sqlx::query(
                r#"
                SELECT COALESCE(
                    MAX((split_part(reference_number, '/', 4))::bigint),
                    0
                ) AS max_sequence
                FROM payments
                WHERE company_id = $1
                  AND reference_number LIKE $2
                "#,
            )
            .bind(stored_payment.company_id)
            .bind(format!("{}%", prefix))
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

            let next_seq: i64 = row.get("max_sequence");
            stored_payment.reference_number = format!("{}{:03}", prefix, next_seq + 1);
        }

        let payment_type_str = match stored_payment.payment_type {
            PaymentType::PaymentReceived => "payment_received",
            PaymentType::PaymentPaid => "payment_paid",
        };

        let counterparty_type_str = match stored_payment.counterparty_type {
            CounterpartyType::Customer => "customer",
            CounterpartyType::Supplier => "supplier",
        };

        sqlx::query(
            r#"
            INSERT INTO payments (
                id, company_id, reference_number, payment_type, counterparty_type, counterparty_id,
                payment_date, bank_account_id, amount, status, notes, journal_entry_id, attachment_url,
                created_by, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10::document_status, $11, $12, $13, $14, $15, $16)
            "#,
        )
        .bind(stored_payment.id)
        .bind(stored_payment.company_id)
        .bind(&stored_payment.reference_number)
        .bind(payment_type_str)
        .bind(counterparty_type_str)
        .bind(stored_payment.counterparty_id)
        .bind(stored_payment.payment_date)
        .bind(stored_payment.bank_account_id)
        .bind(stored_payment.amount)
        .bind(stored_payment.status.clone())
        .bind(&stored_payment.notes)
        .bind(stored_payment.journal_entry_id)
        .bind(&stored_payment.attachment_url)
        .bind(stored_payment.created_by)
        .bind(stored_payment.created_at)
        .bind(stored_payment.updated_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        // Save allocations
        for alloc in &stored_payment.allocations {
            let doc_type_str = match alloc.document_type {
                AllocatedDocumentType::SalesInvoice => "sales_invoice",
                AllocatedDocumentType::PurchaseInvoice => "purchase_invoice",
            };
            sqlx::query(
                r#"
                INSERT INTO payment_allocations (id, payment_id, document_type, document_id, allocated_amount)
                VALUES ($1, $2, $3, $4, $5)
                "#,
            )
            .bind(alloc.id)
            .bind(stored_payment.id)
            .bind(doc_type_str)
            .bind(alloc.document_id)
            .bind(alloc.allocated_amount)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        }

        tx.commit()
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

        Ok(stored_payment)
    }

    async fn update(&self, payment: &Payment) -> Result<(), AppError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

        let payment_type_str = match payment.payment_type {
            PaymentType::PaymentReceived => "payment_received",
            PaymentType::PaymentPaid => "payment_paid",
        };

        let counterparty_type_str = match payment.counterparty_type {
            CounterpartyType::Customer => "customer",
            CounterpartyType::Supplier => "supplier",
        };

        sqlx::query(
            r#"
            UPDATE payments
            SET reference_number = $2,
                payment_type = $3,
                counterparty_type = $4,
                counterparty_id = $5,
                payment_date = $6,
                bank_account_id = $7,
                amount = $8,
                status = $9::document_status,
                notes = $10,
                journal_entry_id = $11,
                attachment_url = $12,
                updated_at = $13
            WHERE id = $1
            "#,
        )
        .bind(payment.id)
        .bind(&payment.reference_number)
        .bind(payment_type_str)
        .bind(counterparty_type_str)
        .bind(payment.counterparty_id)
        .bind(payment.payment_date)
        .bind(payment.bank_account_id)
        .bind(payment.amount)
        .bind(payment.status.clone())
        .bind(&payment.notes)
        .bind(payment.journal_entry_id)
        .bind(&payment.attachment_url)
        .bind(payment.updated_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        // Clear existing allocations and save new ones
        sqlx::query(
            r#"
            DELETE FROM payment_allocations
            WHERE payment_id = $1
            "#,
        )
        .bind(payment.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        for alloc in &payment.allocations {
            let doc_type_str = match alloc.document_type {
                AllocatedDocumentType::SalesInvoice => "sales_invoice",
                AllocatedDocumentType::PurchaseInvoice => "purchase_invoice",
            };
            sqlx::query(
                r#"
                INSERT INTO payment_allocations (id, payment_id, document_type, document_id, allocated_amount)
                VALUES ($1, $2, $3, $4, $5)
                "#,
            )
            .bind(alloc.id)
            .bind(payment.id)
            .bind(doc_type_str)
            .bind(alloc.document_id)
            .bind(alloc.allocated_amount)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
        }

        tx.commit()
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            r#"
            DELETE FROM payments
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(())
    }
}
