use sqlx::PgPool;
use std::time::Duration;

/// Run overdue AR/AP alert job every hour.
pub async fn run(pool: PgPool) -> anyhow::Result<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(3600));
    loop {
        interval.tick().await;
        tracing::info!("Running overdue alerts job");
        // TODO: query overdue invoices, send notifications
    }
}
