use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use finance_assistant_domain::{
    entities::{
        account::Account,
        bank_account::BankAccount,
        branch::Branch,
        company::Company,
        customer::Customer,
        supplier::Supplier,
        tax::{TaxCategory, TaxType},
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

use finance_assistant_domain::entities::tax::TaxRecord;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxRecordResponse {
    pub id: Uuid,
    pub company_id: Uuid,
    pub tax_type_id: Uuid,
    pub source_document_type: String,
    pub source_document_id: Uuid,
    pub tax_period: String,
    pub tax_base_amount: Decimal,
    pub tax_rate: Decimal,
    pub tax_amount: Decimal,
    pub status: String,
    pub counterparty_name: Option<String>,
    pub counterparty_npwp: Option<String>,
    #[serde(with = "crate::dto::datetime_format")]
    pub created_at: time::OffsetDateTime,
}

impl From<TaxRecord> for TaxRecordResponse {
    fn from(r: TaxRecord) -> Self {
        Self {
            id: r.id,
            company_id: r.company_id,
            tax_type_id: r.tax_type_id,
            source_document_type: r.source_document_type,
            source_document_id: r.source_document_id,
            tax_period: r.tax_period.to_string(),
            tax_base_amount: r.tax_base_amount,
            tax_rate: r.tax_rate,
            tax_amount: r.tax_amount,
            status: r.status.as_str().to_string(),
            counterparty_name: r.counterparty_name,
            counterparty_npwp: r.counterparty_npwp,
            created_at: r.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxSummaryResponse {
    pub total_vat_output: Decimal,
    pub total_vat_input: Decimal,
    pub net_tax_due: Decimal,
    pub records: Vec<TaxRecordResponse>,
}

use finance_assistant_domain::entities::tax::TaxCalendarEntry;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxCalendarResponse {
    pub id: Uuid,
    pub company_id: Uuid,
    pub tax_type_id: Uuid,
    pub tax_period: String,
    pub payment_due_date: String,
    pub filing_due_date: String,
    pub payment_status: String,
    pub filing_status: String,
    pub reminder_sent_at: Option<time::OffsetDateTime>,
    #[serde(with = "crate::dto::datetime_format")]
    pub created_at: time::OffsetDateTime,
}

impl From<TaxCalendarEntry> for TaxCalendarResponse {
    fn from(e: TaxCalendarEntry) -> Self {
        Self {
            id: e.id,
            company_id: e.company_id,
            tax_type_id: e.tax_type_id,
            tax_period: e.tax_period.to_string(),
            payment_due_date: e.payment_due_date.to_string(),
            filing_due_date: e.filing_due_date.to_string(),
            payment_status: e.payment_status.as_str().to_string(),
            filing_status: e.filing_status.as_str().to_string(),
            reminder_sent_at: e.reminder_sent_at,
            created_at: e.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaxCalendarRequest {
    pub company_id: Uuid,
    pub tax_type_id: Uuid,
    pub tax_period: time::Date,
    pub payment_due_date: time::Date,
    pub filing_due_date: time::Date,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaxCalendarStatusRequest {
    pub payment_status: Option<String>,
    pub filing_status: Option<String>,
}


