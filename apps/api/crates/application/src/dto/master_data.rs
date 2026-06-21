use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use finance_assistant_domain::{
    entities::{
        company::Company,
        account::Account,
        customer::Customer,
        supplier::Supplier,
        bank_account::BankAccount,
        tax::{TaxType, TaxCategory},
        branch::Branch,
    },
    value_objects::AccountType,
};

// ─── Company DTOs ─────────────────────────────────────────────────────────────
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCompanyRequest {
    pub name: String,
    pub tax_number: Option<String>,
    pub address: Option<String>,
    pub currency: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCompanyRequest {
    pub name: String,
    pub tax_number: Option<String>,
    pub address: Option<String>,
    pub currency: String,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyResponse {
    pub id: Uuid,
    pub name: String,
    pub tax_number: Option<String>,
    pub address: Option<String>,
    pub currency: String,
    pub is_active: bool,
    #[serde(with = "crate::dto::datetime_format")]
    pub created_at: time::OffsetDateTime,
}

impl From<Company> for CompanyResponse {
    fn from(c: Company) -> Self {
        Self {
            id: c.id,
            name: c.name,
            tax_number: c.tax_number,
            address: c.address,
            currency: c.currency,
            is_active: c.is_active,
            created_at: c.created_at,
        }
    }
}

// ─── Account (COA) DTOs ───────────────────────────────────────────────────────
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAccountRequest {
    pub company_id: Uuid,
    pub code: String,
    pub name: String,
    pub account_type: String,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAccountRequest {
    pub code: String,
    pub name: String,
    pub account_type: String,
    pub parent_id: Option<Uuid>,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountResponse {
    pub id: Uuid,
    pub company_id: Uuid,
    pub code: String,
    pub name: String,
    pub account_type: String,
    pub parent_id: Option<Uuid>,
    pub is_active: bool,
    #[serde(with = "crate::dto::datetime_format")]
    pub created_at: time::OffsetDateTime,
}

impl From<Account> for AccountResponse {
    fn from(a: Account) -> Self {
        let type_str = match a.account_type {
            AccountType::Asset => "Asset",
            AccountType::Liability => "Liability",
            AccountType::Equity => "Equity",
            AccountType::Revenue => "Revenue",
            AccountType::Expense => "Expense",
        };
        Self {
            id: a.id,
            company_id: a.company_id,
            code: a.code.to_string(),
            name: a.name,
            account_type: type_str.to_string(),
            parent_id: a.parent_id,
            is_active: a.is_active,
            created_at: a.created_at,
        }
    }
}

// ─── Customer DTOs ────────────────────────────────────────────────────────────
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCustomerRequest {
    pub company_id: Uuid,
    pub name: String,
    pub tax_number: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCustomerRequest {
    pub name: String,
    pub tax_number: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerResponse {
    pub id: Uuid,
    pub company_id: Uuid,
    pub name: String,
    pub tax_number: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub is_active: bool,
    #[serde(with = "crate::dto::datetime_format")]
    pub created_at: time::OffsetDateTime,
}

impl From<Customer> for CustomerResponse {
    fn from(c: Customer) -> Self {
        Self {
            id: c.id,
            company_id: c.company_id,
            name: c.name,
            tax_number: c.tax_number,
            email: c.email,
            phone: c.phone,
            address: c.address,
            is_active: c.is_active,
            created_at: c.created_at,
        }
    }
}

// ─── Supplier DTOs ────────────────────────────────────────────────────────────
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSupplierRequest {
    pub company_id: Uuid,
    pub name: String,
    pub tax_number: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSupplierRequest {
    pub name: String,
    pub tax_number: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SupplierResponse {
    pub id: Uuid,
    pub company_id: Uuid,
    pub name: String,
    pub tax_number: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub is_active: bool,
    #[serde(with = "crate::dto::datetime_format")]
    pub created_at: time::OffsetDateTime,
}

impl From<Supplier> for SupplierResponse {
    fn from(s: Supplier) -> Self {
        Self {
            id: s.id,
            company_id: s.company_id,
            name: s.name,
            tax_number: s.tax_number,
            email: s.email,
            phone: s.phone,
            address: s.address,
            is_active: s.is_active,
            created_at: s.created_at,
        }
    }
}

// ─── BankAccount DTOs ──────────────────────────────────────────────────────────
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBankAccountRequest {
    pub company_id: Uuid,
    pub account_id: Uuid,
    pub bank_name: String,
    pub account_number: String,
    pub account_name: String,
    pub currency: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBankAccountRequest {
    pub account_id: Uuid,
    pub bank_name: String,
    pub account_number: String,
    pub account_name: String,
    pub currency: String,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BankAccountResponse {
    pub id: Uuid,
    pub company_id: Uuid,
    pub account_id: Uuid,
    pub bank_name: String,
    pub account_number: String,
    pub account_name: String,
    pub currency: String,
    pub is_active: bool,
    #[serde(with = "crate::dto::datetime_format")]
    pub created_at: time::OffsetDateTime,
}

impl From<BankAccount> for BankAccountResponse {
    fn from(b: BankAccount) -> Self {
        Self {
            id: b.id,
            company_id: b.company_id,
            account_id: b.account_id,
            bank_name: b.bank_name,
            account_number: b.account_number,
            account_name: b.account_name,
            currency: b.currency,
            is_active: b.is_active,
            created_at: b.created_at,
        }
    }
}

// ─── TaxType DTOs ─────────────────────────────────────────────────────────────
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaxTypeRequest {
    pub company_id: Uuid,
    pub code: String,
    pub name: String,
    pub category: String,
    pub default_rate: Decimal,
    pub payable_account_id: Uuid,
    #[serde(with = "crate::dto::date_format")]
    pub effective_from: time::Date,
    #[serde(with = "crate::dto::option_date_format")]
    pub effective_to: Option<time::Date>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaxTypeRequest {
    pub name: String,
    pub category: String,
    pub default_rate: Decimal,
    pub payable_account_id: Uuid,
    #[serde(with = "crate::dto::date_format")]
    pub effective_from: time::Date,
    #[serde(with = "crate::dto::option_date_format")]
    pub effective_to: Option<time::Date>,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxTypeResponse {
    pub id: Uuid,
    pub company_id: Uuid,
    pub code: String,
    pub name: String,
    pub category: String,
    pub default_rate: Decimal,
    pub payable_account_id: Uuid,
    #[serde(with = "crate::dto::date_format")]
    pub effective_from: time::Date,
    #[serde(with = "crate::dto::option_date_format")]
    pub effective_to: Option<time::Date>,
    pub is_active: bool,
    #[serde(with = "crate::dto::datetime_format")]
    pub created_at: time::OffsetDateTime,
}

impl From<TaxType> for TaxTypeResponse {
    fn from(t: TaxType) -> Self {
        let cat_str = match t.category {
            TaxCategory::VatOutput => "vat_output",
            TaxCategory::VatInput => "vat_input",
            TaxCategory::WithholdingPph21 => "withholding_pph21",
            TaxCategory::WithholdingPph23 => "withholding_pph23",
            TaxCategory::WithholdingPph25 => "withholding_pph25",
            TaxCategory::WithholdingPphFinal => "withholding_pph_final",
        };
        Self {
            id: t.id,
            company_id: t.company_id,
            code: t.code,
            name: t.name,
            category: cat_str.to_string(),
            default_rate: t.default_rate,
            payable_account_id: t.payable_account_id,
            effective_from: t.effective_from,
            effective_to: t.effective_to,
            is_active: t.is_active,
            created_at: t.created_at,
        }
     }
}

// ─── Branch DTOs ──────────────────────────────────────────────────────────────
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBranchRequest {
    pub company_id: Uuid,
    pub code: String,
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBranchRequest {
    pub code: String,
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BranchResponse {
    pub id: Uuid,
    pub company_id: Uuid,
    pub code: String,
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub is_active: bool,
    #[serde(with = "crate::dto::datetime_format")]
    pub created_at: time::OffsetDateTime,
}

impl From<Branch> for BranchResponse {
    fn from(b: Branch) -> Self {
        Self {
            id: b.id,
            company_id: b.company_id,
            code: b.code,
            name: b.name,
            address: b.address,
            phone: b.phone,
            is_active: b.is_active,
            created_at: b.created_at,
        }
    }
}

