use sqlx::PgPool;
use std::time::Duration;

/// Run tax due date reminder job every day.
pub async fn run(pool: PgPool) -> anyhow::Result<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(86400));
    loop {
        interval.tick().await;
        tracing::info!("Running tax reminders job");
        // TODO: check upcoming tax due dates, send notifications
    }
}
