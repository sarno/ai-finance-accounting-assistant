use axum::{
    middleware,
    routing::{get, post, put},
    Router,
};
use std::sync::Arc;
use std::time::Duration;
use tower_http::{
    cors::{Any, CorsLayer},
    request_id::{MakeRequestUuid, SetRequestIdLayer},
    services::ServeDir,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::{
    handlers::{approvals, auth, health, invoices, items, journals, master_data, reports, upload, payments, ai, users},
    middleware::{auth_middleware, rate_limit, idempotency, audit},
    state::AppState,
};

/// Build the complete Axum router with all middleware and routes.
pub fn build(state: AppState) -> Router {
    let storage_base_path = state.config.storage_base_path.clone();
    let shared_state = Arc::new(state);

    // ─── Public routes (no auth required) ────────────────────────────────────
    let public_routes = Router::new()
        .route("/health", get(health::health_check))
        .route("/health/db", get(health::db_health))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/refresh", post(auth::refresh_token))
        .route("/api/auth/logout", post(auth::logout))
        .nest_service("/uploads", ServeDir::new(storage_base_path));

    // ─── Protected routes (JWT required) ─────────────────────────────────────
    let protected_routes = Router::new()
        // Upload & Documents
        .route("/api/upload", post(upload::upload_file))
        .route("/api/documents", get(upload::list_documents))
        .route("/api/documents/:id", get(upload::get_document))
        // Approvals
        .route("/api/approvals", get(approvals::list_pending_approvals))
        .route("/api/approvals/:id", get(approvals::get_approval))
        .route(
            "/api/approvals/:id/approve",
            post(approvals::approve_request),
        )
        .route("/api/approvals/:id/reject", post(approvals::reject_request))
        // Journals
        .route("/api/journals", get(journals::list_journals))
        .route("/api/journals/draft", post(journals::create_draft))
        .route(
            "/api/journals/:id",
            get(journals::get_journal)
                .put(journals::update_journal)
                .delete(journals::delete_journal),
        )
        .route("/api/journals/:id/submit", post(journals::submit_approval))
        .route("/api/journals/:id/post", post(journals::post_journal))
        .route("/api/journals/:id/approve", post(journals::approve_journal))
        // Sales Invoices
        .route("/api/sales-invoices", get(invoices::list_sales_invoices))
        .route(
            "/api/sales-invoices/draft",
            post(invoices::create_sales_draft),
        )
        .route(
            "/api/sales-invoices/:id",
            get(invoices::get_sales_invoice)
                .put(invoices::update_sales_invoice)
                .delete(invoices::delete_sales_invoice),
        )
        .route(
            "/api/sales-invoices/:id/submit",
            post(invoices::submit_approval),
        )
        // Purchase Invoices
        .route("/api/purchase-invoices", get(invoices::list_purchase_invoices))
        .route(
            "/api/purchase-invoices/draft",
            post(invoices::create_purchase_draft),
        )
        .route(
            "/api/purchase-invoices/from-document",
            post(invoices::create_purchase_from_document),
        )
        .route(
            "/api/purchase-invoices/:id",
            get(invoices::get_purchase_invoice)
                .put(invoices::update_purchase_invoice)
                .delete(invoices::delete_purchase_invoice),
        )
        .route(
            "/api/purchase-invoices/:id/submit",
            post(invoices::submit_purchase_approval),
        )
        // Payments
        .route("/api/payments", get(payments::list_payments))
        .route("/api/payments/draft", post(payments::create_payment_draft))
        .route(
            "/api/payments/:id",
            get(payments::get_payment)
                .put(payments::update_payment)
                .delete(payments::delete_payment),
        )
        .route(
            "/api/payments/:id/submit",
            post(payments::submit_payment_approval),
        )
        // Reports
        .route("/api/reports/cash-position", get(reports::cash_position))
        .route("/api/reports/profit-loss", get(reports::profit_loss))
        .route("/api/reports/accounts-receivable", get(reports::accounts_receivable))
        .route("/api/reports/accounts-payable", get(reports::accounts_payable))
        .route("/api/reports/trial-balance", get(reports::trial_balance))
        .route("/api/reports/general-ledger", get(reports::general_ledger))
        .route("/api/reports/tax-summary", get(reports::tax_summary))
        // Companies
        .route(
            "/api/companies",
            post(master_data::create_company).get(master_data::list_companies),
        )
        .route(
            "/api/companies/:id",
            get(master_data::get_company).put(master_data::update_company),
        )
        // Accounts (COA)
        .route("/api/accounts", post(master_data::create_account))
        .route(
            "/api/accounts/:id",
            get(master_data::get_account).put(master_data::update_account),
        )
        .route(
            "/api/companies/:company_id/accounts",
            get(master_data::list_accounts),
        )
        // Customers
        .route("/api/customers", post(master_data::create_customer))
        .route(
            "/api/customers/:id",
            get(master_data::get_customer).put(master_data::update_customer),
        )
        .route(
            "/api/companies/:company_id/customers",
            get(master_data::list_customers),
        )
        // Suppliers
        .route("/api/suppliers", post(master_data::create_supplier))
        .route(
            "/api/suppliers/:id",
            get(master_data::get_supplier).put(master_data::update_supplier),
        )
        .route(
            "/api/companies/:company_id/suppliers",
            get(master_data::list_suppliers),
        )
        // Bank Accounts
        .route("/api/bank-accounts", post(master_data::create_bank_account))
        .route(
            "/api/bank-accounts/:id",
            get(master_data::get_bank_account).put(master_data::update_bank_account),
        )
        .route(
            "/api/companies/:company_id/bank-accounts",
            get(master_data::list_bank_accounts),
        )
        // Tax Types
        .route("/api/tax-types", post(master_data::create_tax_type))
        .route(
            "/api/tax-types/:id",
            get(master_data::get_tax_type).put(master_data::update_tax_type),
        )
        .route(
            "/api/companies/:company_id/tax-types",
            get(master_data::list_tax_types),
        )
        // Tax Records & Summary
        .route(
            "/api/companies/:company_id/tax-records",
            get(master_data::list_tax_records),
        )
        .route(
            "/api/companies/:company_id/tax-summary",
            get(master_data::get_tax_summary),
        )
        // Tax Calendar
        .route(
            "/api/companies/:company_id/tax-calendar",
            get(master_data::list_tax_calendar),
        )
        .route(
            "/api/tax-calendar",
            post(master_data::create_tax_calendar_entry),
        )
        .route(
            "/api/tax-calendar/:id/status",
            put(master_data::update_tax_calendar_status),
        )
        // Branches
        .route("/api/branches", post(master_data::create_branch))
        .route(
            "/api/branches/:id",
            get(master_data::get_branch).put(master_data::update_branch),
        )
        .route(
            "/api/companies/:company_id/branches",
            get(master_data::list_branches),
        )
        // Item Categories
        .route("/api/item-categories", post(items::create_category))
        .route(
            "/api/item-categories/:id",
            get(items::get_category)
                .put(items::update_category)
                .delete(items::delete_category),
        )
        .route(
            "/api/companies/:company_id/item-categories",
            get(items::list_categories),
        )
        // Items
        .route("/api/items", post(items::create_item))
        .route(
            "/api/items/:id",
            get(items::get_item)
                .put(items::update_item)
                .delete(items::delete_item),
        )
        .route("/api/companies/:company_id/items", get(items::list_items))
        // User & Role Management
        .route(
            "/api/users",
            get(users::list_users).post(users::create_user),
        )
        .route(
            "/api/users/:id",
            put(users::update_user).delete(users::delete_user),
        )
        .layer(middleware::from_fn_with_state(
            shared_state.clone(),
            audit::audit_middleware,
        ))
        .layer(middleware::from_fn_with_state(
            shared_state.clone(),
            idempotency::idempotency_middleware,
        ))
        .layer(middleware::from_fn_with_state(
            shared_state.clone(),
            auth_middleware::require_auth,
        ));

    // ─── AI & OpenClaw routes (Bearer AI_SERVICE_TOKEN required) ──────────────
    let ai_routes = Router::new()
        .route("/api/ai/tools/query-report", post(ai::query_report))
        .route("/api/ai/tools/create-draft-invoice", post(ai::create_draft_invoice))
        .route("/api/ai/tools/create-draft-payment", post(ai::create_draft_payment))
        .route("/api/ai/tools/create-draft-journal", post(ai::create_draft_journal))
        .route("/api/ai/tools/submit-approval-command", post(ai::submit_approval_command));

    // ─── Global middleware stack ──────────────────────────────────────────────
    public_routes
        .merge(protected_routes)
        .merge(ai_routes)
        .with_state(shared_state)
        .layer(
            tower::ServiceBuilder::new()
                .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
                .layer(TraceLayer::new_for_http())
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
                .layer(middleware::from_fn(rate_limit::rate_limit_middleware))
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_headers(Any)
                        .allow_methods(Any),
                ),
        )
}
