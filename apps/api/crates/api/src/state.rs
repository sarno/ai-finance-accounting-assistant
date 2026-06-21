use sqlx::PgPool;
use std::sync::Arc;

use finance_assistant_app::services::{
    approval_service::ApprovalService, auth_service::AuthService, invoice_service::InvoiceService,
    item_service::ItemService, journal_service::JournalService,
    master_data_service::MasterDataService,
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
