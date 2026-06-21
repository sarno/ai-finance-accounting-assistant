use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::{
    cors::{Any, CorsLayer},
    request_id::{MakeRequestUuid, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use std::time::Duration;

use crate::{
    handlers::{auth, health, journals, reports, master_data, approvals},
    middleware::auth_middleware,
    state::AppState,
};

/// Build the complete Axum router with all middleware and routes.
pub fn build(state: AppState) -> Router {
    let shared_state = Arc::new(state);

    // ─── Public routes (no auth required) ────────────────────────────────────
    let public_routes = Router::new()
        .route("/health",    get(health::health_check))
        .route("/health/db", get(health::db_health))
        .route("/api/auth/login",   post(auth::login))
        .route("/api/auth/refresh", post(auth::refresh_token));

    // ─── Protected routes (JWT required) ─────────────────────────────────────
    let protected_routes = Router::new()
        // Approvals
        .route("/api/approvals",             get(approvals::list_pending_approvals))
        .route("/api/approvals/:id",         get(approvals::get_approval))
        .route("/api/approvals/:id/approve", post(approvals::approve_request))
        .route("/api/approvals/:id/reject",  post(approvals::reject_request))
        // Journals
        .route("/api/journals",              get(journals::list_journals))
        .route("/api/journals/draft",        post(journals::create_draft))
        .route("/api/journals/:id",          get(journals::get_journal))
        .route("/api/journals/:id/submit",   post(journals::submit_approval))
        .route("/api/journals/:id/post",     post(journals::post_journal))
        .route("/api/journals/:id/approve",  post(journals::approve_journal))
        // Reports
        .route("/api/reports/cash-position",    get(reports::cash_position))
        .route("/api/reports/profit-loss",      get(reports::profit_loss))
        // Companies
        .route("/api/companies",             post(master_data::create_company).get(master_data::list_companies))
        .route("/api/companies/:id",         get(master_data::get_company).put(master_data::update_company))
        // Accounts (COA)
        .route("/api/accounts",              post(master_data::create_account))
        .route("/api/accounts/:id",          get(master_data::get_account).put(master_data::update_account))
        .route("/api/companies/:company_id/accounts", get(master_data::list_accounts))
        // Customers
        .route("/api/customers",             post(master_data::create_customer))
        .route("/api/customers/:id",         get(master_data::get_customer).put(master_data::update_customer))
        .route("/api/companies/:company_id/customers", get(master_data::list_customers))
        // Suppliers
        .route("/api/suppliers",             post(master_data::create_supplier))
        .route("/api/suppliers/:id",         get(master_data::get_supplier).put(master_data::update_supplier))
        .route("/api/companies/:company_id/suppliers", get(master_data::list_suppliers))
        // Bank Accounts
        .route("/api/bank-accounts",         post(master_data::create_bank_account))
        .route("/api/bank-accounts/:id",     get(master_data::get_bank_account).put(master_data::update_bank_account))
        .route("/api/companies/:company_id/bank-accounts", get(master_data::list_bank_accounts))
        // Tax Types
        .route("/api/tax-types",             post(master_data::create_tax_type))
        .route("/api/tax-types/:id",         get(master_data::get_tax_type).put(master_data::update_tax_type))
        .route("/api/companies/:company_id/tax-types", get(master_data::list_tax_types))
        // Branches
        .route("/api/branches",             post(master_data::create_branch))
        .route("/api/branches/:id",         get(master_data::get_branch).put(master_data::update_branch))
        .route("/api/companies/:company_id/branches", get(master_data::list_branches))
        .layer(middleware::from_fn_with_state(shared_state.clone(), auth_middleware::require_auth));


    // ─── Global middleware stack ──────────────────────────────────────────────
    public_routes
        .merge(protected_routes)
        .with_state(shared_state)
        .layer(
            tower::ServiceBuilder::new()
                .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
                .layer(TraceLayer::new_for_http())
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_headers(Any)
                        .allow_methods(Any),
                ),
        )
}
