use axum::{
    extract::{Path, State},
    response::Json,
};
use std::sync::Arc;
use uuid::Uuid;

use finance_assistant_app::dto::master_data::*;
use crate::{
    errors::ApiError,
    middleware::auth_middleware::AuthenticatedUser,
    state::AppState,
};

// ─── Companies ────────────────────────────────────────────────────────────────

pub async fn create_company(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Json(req): Json<CreateCompanyRequest>,
) -> Result<Json<CompanyResponse>, ApiError> {
    let res = state.master_data_service.create_company(req).await?;
    Ok(Json(res))
}

pub async fn get_company(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<CompanyResponse>, ApiError> {
    let res = state.master_data_service.get_company(id).await?;
    Ok(Json(res))
}

pub async fn update_company(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateCompanyRequest>,
) -> Result<Json<CompanyResponse>, ApiError> {
    let res = state.master_data_service.update_company(id, req).await?;
    Ok(Json(res))
}

pub async fn list_companies(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
) -> Result<Json<Vec<CompanyResponse>>, ApiError> {
    let res = state.master_data_service.list_companies().await?;
    Ok(Json(res))
}

// ─── Accounts (Chart of Accounts) ────────────────────────────────────────────

pub async fn create_account(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Json(req): Json<CreateAccountRequest>,
) -> Result<Json<AccountResponse>, ApiError> {
    let res = state.master_data_service.create_account(req).await?;
    Ok(Json(res))
}

pub async fn get_account(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<AccountResponse>, ApiError> {
    let res = state.master_data_service.get_account(id).await?;
    Ok(Json(res))
}

pub async fn update_account(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateAccountRequest>,
) -> Result<Json<AccountResponse>, ApiError> {
    let res = state.master_data_service.update_account(id, req).await?;
    Ok(Json(res))
}

pub async fn list_accounts(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(company_id): Path<Uuid>,
) -> Result<Json<Vec<AccountResponse>>, ApiError> {
    let res = state.master_data_service.list_accounts(company_id).await?;
    Ok(Json(res))
}

// ─── Customers ────────────────────────────────────────────────────────────────

pub async fn create_customer(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Json(req): Json<CreateCustomerRequest>,
) -> Result<Json<CustomerResponse>, ApiError> {
    let res = state.master_data_service.create_customer(req).await?;
    Ok(Json(res))
}

pub async fn get_customer(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<CustomerResponse>, ApiError> {
    let res = state.master_data_service.get_customer(id).await?;
    Ok(Json(res))
}

pub async fn update_customer(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateCustomerRequest>,
) -> Result<Json<CustomerResponse>, ApiError> {
    let res = state.master_data_service.update_customer(id, req).await?;
    Ok(Json(res))
}

pub async fn list_customers(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(company_id): Path<Uuid>,
) -> Result<Json<Vec<CustomerResponse>>, ApiError> {
    let res = state.master_data_service.list_customers(company_id).await?;
    Ok(Json(res))
}

// ─── Suppliers ────────────────────────────────────────────────────────────────

pub async fn create_supplier(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Json(req): Json<CreateSupplierRequest>,
) -> Result<Json<SupplierResponse>, ApiError> {
    let res = state.master_data_service.create_supplier(req).await?;
    Ok(Json(res))
}

pub async fn get_supplier(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<SupplierResponse>, ApiError> {
    let res = state.master_data_service.get_supplier(id).await?;
    Ok(Json(res))
}

pub async fn update_supplier(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateSupplierRequest>,
) -> Result<Json<SupplierResponse>, ApiError> {
    let res = state.master_data_service.update_supplier(id, req).await?;
    Ok(Json(res))
}

pub async fn list_suppliers(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(company_id): Path<Uuid>,
) -> Result<Json<Vec<SupplierResponse>>, ApiError> {
    let res = state.master_data_service.list_suppliers(company_id).await?;
    Ok(Json(res))
}

// ─── Bank Accounts ────────────────────────────────────────────────────────────

pub async fn create_bank_account(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Json(req): Json<CreateBankAccountRequest>,
) -> Result<Json<BankAccountResponse>, ApiError> {
    let res = state.master_data_service.create_bank_account(req).await?;
    Ok(Json(res))
}

pub async fn get_bank_account(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<BankAccountResponse>, ApiError> {
    let res = state.master_data_service.get_bank_account(id).await?;
    Ok(Json(res))
}

pub async fn update_bank_account(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateBankAccountRequest>,
) -> Result<Json<BankAccountResponse>, ApiError> {
    let res = state.master_data_service.update_bank_account(id, req).await?;
    Ok(Json(res))
}

pub async fn list_bank_accounts(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(company_id): Path<Uuid>,
) -> Result<Json<Vec<BankAccountResponse>>, ApiError> {
    let res = state.master_data_service.list_bank_accounts(company_id).await?;
    Ok(Json(res))
}

// ─── Tax Types ────────────────────────────────────────────────────────────────

pub async fn create_tax_type(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Json(req): Json<CreateTaxTypeRequest>,
) -> Result<Json<TaxTypeResponse>, ApiError> {
    let res = state.master_data_service.create_tax_type(req).await?;
    Ok(Json(res))
}

pub async fn get_tax_type(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<TaxTypeResponse>, ApiError> {
    let res = state.master_data_service.get_tax_type(id).await?;
    Ok(Json(res))
}

pub async fn update_tax_type(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTaxTypeRequest>,
) -> Result<Json<TaxTypeResponse>, ApiError> {
    let res = state.master_data_service.update_tax_type(id, req).await?;
    Ok(Json(res))
}

pub async fn list_tax_types(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(company_id): Path<Uuid>,
) -> Result<Json<Vec<TaxTypeResponse>>, ApiError> {
    let res = state.master_data_service.list_tax_types(company_id).await?;
    Ok(Json(res))
}

// ─── Branches ─────────────────────────────────────────────────────────────────

pub async fn create_branch(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Json(req): Json<CreateBranchRequest>,
) -> Result<Json<BranchResponse>, ApiError> {
    let res = state.master_data_service.create_branch(req).await?;
    Ok(Json(res))
}

pub async fn get_branch(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<BranchResponse>, ApiError> {
    let res = state.master_data_service.get_branch(id).await?;
    Ok(Json(res))
}

pub async fn update_branch(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateBranchRequest>,
) -> Result<Json<BranchResponse>, ApiError> {
    let res = state.master_data_service.update_branch(id, req).await?;
    Ok(Json(res))
}

pub async fn list_branches(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(company_id): Path<Uuid>,
) -> Result<Json<Vec<BranchResponse>>, ApiError> {
    let res = state.master_data_service.list_branches(company_id).await?;
    Ok(Json(res))
}

