use std::sync::Arc;
use sqlx::PgPool;

use finance_assistant_app::services::{
    journal_service::JournalService,
    auth_service::AuthService,
    master_data_service::MasterDataService,
    approval_service::ApprovalService,
    invoice_service::InvoiceService,
    item_service::ItemService,
};

use crate::config::AppConfig;

/// Shared application state passed to all Axum handlers via `State<Arc<AppState>>`.
#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db_pool: PgPool,
    pub journal_service: Arc<JournalService>,
    pub auth_service: Arc<AuthService>,
    pub master_data_service: Arc<MasterDataService>,
    pub approval_service: Arc<ApprovalService>,
    pub invoice_service: Arc<InvoiceService>,
    pub item_service: Arc<ItemService>,
}

