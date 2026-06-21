use std::sync::Arc;

use dotenvy::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod config;
mod router;
mod middleware;
mod handlers;
mod state;
mod errors;

use config::AppConfig;
use finance_assistant_infra::{
    db,
    repositories::{
        journal_repository::PgJournalRepository,
        audit_log_repository::PgAuditLogRepository,
        user_repository::PgUserRepository,
        company_repository::PgCompanyRepository,
        branch_repository::PgBranchRepository,
        account_repository::PgAccountRepository,
        customer_repository::PgCustomerRepository,
        supplier_repository::PgSupplierRepository,
        bank_account_repository::PgBankAccountRepository,
        tax_repository::PgTaxRepository,
        approval_repository::PgApprovalRepository,
        invoice_repository::PgInvoiceRepository,
    },
};
use finance_assistant_app::services::{
    journal_service::JournalService,
    auth_service::AuthService,
    master_data_service::MasterDataService,
    approval_service::ApprovalService,
    invoice_service::InvoiceService,
};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ─── Load environment ─────────────────────────────────────────────────────
    dotenv().ok();

    // ─── Tracing / logging setup ──────────────────────────────────────────────
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    // ─── Config ───────────────────────────────────────────────────────────────
    let cfg = AppConfig::from_env()?;

    // ─── Database pool ────────────────────────────────────────────────────────
    let pool = db::create_pool(&cfg.database_url).await?;
    sqlx::migrate!("../../migrations").run(&pool).await?;

    tracing::info!("Database connected and migrations applied");

    // ─── Build application state (dependency injection) ───────────────────────
    let journal_repo = Arc::new(PgJournalRepository::new(pool.clone()));
    let audit_repo   = Arc::new(PgAuditLogRepository::new(pool.clone()));
    let user_repo    = Arc::new(PgUserRepository::new(pool.clone()));
    let company_repo = Arc::new(PgCompanyRepository::new(pool.clone()));
    let branch_repo = Arc::new(PgBranchRepository::new(pool.clone()));
    let account_repo = Arc::new(PgAccountRepository::new(pool.clone()));
    let customer_repo = Arc::new(PgCustomerRepository::new(pool.clone()));
    let supplier_repo = Arc::new(PgSupplierRepository::new(pool.clone()));
    let bank_account_repo = Arc::new(PgBankAccountRepository::new(pool.clone()));
    let tax_repo     = Arc::new(PgTaxRepository::new(pool.clone()));
    let approval_repo = Arc::new(PgApprovalRepository::new(pool.clone()));
    let invoice_repo = Arc::new(PgInvoiceRepository::new(pool.clone()));

    let journal_svc  = Arc::new(JournalService::new(journal_repo.clone(), audit_repo.clone()));
    let auth_svc     = Arc::new(AuthService::new(
        user_repo.clone(),
        cfg.jwt_secret.clone(),
        cfg.jwt_access_minutes,
        cfg.jwt_refresh_days,
    ));
    let master_data_svc = Arc::new(MasterDataService::new(
        company_repo,
        branch_repo,
        account_repo.clone(),
        customer_repo,
        supplier_repo,
        bank_account_repo,
        tax_repo.clone(),
    ));
    let invoice_svc = Arc::new(InvoiceService::new(
        invoice_repo.clone(),
        tax_repo.clone(),
        journal_repo.clone(),
    ));
    let approval_svc = Arc::new(ApprovalService::new(
        approval_repo,
        journal_repo.clone(),
        audit_repo,
        user_repo.clone(),
        invoice_repo.clone(),
        account_repo.clone(),
        tax_repo.clone(),
    ));

    let app_state = state::AppState {
        config: cfg.clone(),
        db_pool: pool,
        journal_service: journal_svc,
        auth_service: auth_svc,
        master_data_service: master_data_svc,
        approval_service: approval_svc,
        invoice_service: invoice_svc,
    };


    // ─── Build Axum router ────────────────────────────────────────────────────
    let app = router::build(app_state);

    // ─── Start server ─────────────────────────────────────────────────────────
    let addr = format!("0.0.0.0:{}", cfg.api_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Server listening on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
