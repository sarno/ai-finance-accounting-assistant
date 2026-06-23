use dotenvy::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod jobs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = finance_assistant_infra::db::create_pool(&database_url).await?;

    tracing::info!("Worker started — running background jobs");

    // Spawn all periodic background jobs concurrently
    let overdue_alerts = tokio::spawn(jobs::overdue_alerts::run(pool.clone()));
    let tax_reminders = tokio::spawn(jobs::tax_reminders::run(pool.clone()));
    let report_scheduler = tokio::spawn(jobs::report_scheduler::run(pool.clone()));
    let backup_job = tokio::spawn(jobs::backup_job::run(pool.clone()));

    tokio::try_join!(overdue_alerts, tax_reminders, report_scheduler, backup_job)?;

    Ok(())
}
