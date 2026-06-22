use sqlx::{PgPool, Row};
use std::time::Duration;
use uuid::Uuid;
use time::OffsetDateTime;

/// Run tax due date reminder job every day.
pub async fn run(pool: PgPool) -> anyhow::Result<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(86400));
    loop {
        interval.tick().await;
        tracing::info!("Running tax reminders job");

        // 1. Get all companies
        let companies: Vec<Uuid> = match sqlx::query_scalar("SELECT id FROM companies")
            .fetch_all(&pool)
            .await 
        {
            Ok(ids) => ids,
            Err(e) => {
                tracing::error!("Failed to fetch companies for tax reminders: {:?}", e);
                continue;
            }
        };

        let today = time::OffsetDateTime::now_utc().date();
        let alert_threshold = today + time::Duration::days(7); // reminder 7 days before due date

        for company_id in companies {
            // 2. Find upcoming tax calendar entries for this company
            let rows = match sqlx::query(
                r#"
                SELECT c.id, c.tax_period, c.payment_due_date, c.filing_due_date, t.code, t.name
                FROM tax_calendar c
                JOIN tax_types t ON c.tax_type_id = t.id
                WHERE c.company_id = $1
                  AND (c.payment_due_date <= $2 OR c.filing_due_date <= $2)
                  AND (c.payment_status = 'unpaid' OR c.filing_status = 'unfiled')
                  AND c.reminder_sent_at IS NULL
                "#
            )
            .bind(company_id)
            .bind(alert_threshold)
            .fetch_all(&pool)
            .await
            {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!("Failed to fetch tax calendar for company {}: {:?}", company_id, e);
                    continue;
                }
            };

            for row in rows {
                let id: Uuid = row.get("id");
                let tax_period: time::Date = row.get("tax_period");
                let payment_due_date: time::Date = row.get("payment_due_date");
                let filing_due_date: time::Date = row.get("filing_due_date");
                let tax_code: String = row.get("code");
                let tax_name: String = row.get("name");

                tracing::warn!(
                    "TAX REMINDER: Tax {} ({}) for period {} is due soon! Payment due: {}, Filing due: {}",
                    tax_name, tax_code, tax_period, payment_due_date, filing_due_date
                );

                // Update reminder_sent_at
                let now = OffsetDateTime::now_utc();
                let _ = sqlx::query("UPDATE tax_calendar SET reminder_sent_at = $1, updated_at = $1 WHERE id = $2")
                    .bind(now)
                    .bind(id)
                    .execute(&pool)
                    .await;
            }
        }
    }
}
