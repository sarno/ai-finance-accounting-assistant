use sqlx::PgPool;
use std::time::Duration;

/// Scheduled report generation job — runs weekly.
pub async fn run(pool: PgPool) -> anyhow::Result<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(604800));
    loop {
        interval.tick().await;
        tracing::info!("Running scheduled report generation");
        // TODO: generate and cache weekly summary reports
    }
}
