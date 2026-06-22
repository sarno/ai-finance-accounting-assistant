use std::sync::Arc;
use time::OffsetDateTime;
use uuid::Uuid;

use finance_assistant_domain::{
    entities::{
        account::Account,
        bank_account::BankAccount,
        branch::Branch,
        company::Company,
        customer::Customer,
        supplier::Supplier,
        tax::{TaxCategory, TaxType, TaxCalendarEntry},
    },
    value_objects::{AccountCode, AccountType},
};

use crate::{
    dto::master_data::*,
    errors::AppError,
    ports::{
        account_repository::AccountRepository, bank_account_repository::BankAccountRepository,
        branch_repository::BranchRepository, company_repository::CompanyRepository,
        customer_repository::CustomerRepository, supplier_repository::SupplierRepository,
        tax_repository::TaxRepository, tax_repository::TaxRecordRepository,
        tax_repository::TaxCalendarRepository,
    },
};

pub struct MasterDataService {
    company_repo: Arc<dyn CompanyRepository>,
    branch_repo: Arc<dyn BranchRepository>,
    account_repo: Arc<dyn AccountRepository>,
    customer_repo: Arc<dyn CustomerRepository>,
    supplier_repo: Arc<dyn SupplierRepository>,
    bank_account_repo: Arc<dyn BankAccountRepository>,
    tax_repo: Arc<dyn TaxRepository>,
    tax_record_repo: Arc<dyn TaxRecordRepository>,
    tax_calendar_repo: Arc<dyn TaxCalendarRepository>,
}

impl MasterDataService {
    pub fn new(
        company_repo: Arc<dyn CompanyRepository>,
        branch_repo: Arc<dyn BranchRepository>,
        account_repo: Arc<dyn AccountRepository>,
        customer_repo: Arc<dyn CustomerRepository>,
        supplier_repo: Arc<dyn SupplierRepository>,
        bank_account_repo: Arc<dyn BankAccountRepository>,
        tax_repo: Arc<dyn TaxRepository>,
        tax_record_repo: Arc<dyn TaxRecordRepository>,
        tax_calendar_repo: Arc<dyn TaxCalendarRepository>,
    ) -> Self {
        Self {
            company_repo,
            branch_repo,
            account_repo,
            customer_repo,
            supplier_repo,
            bank_account_repo,
            tax_repo,
            tax_record_repo,
            tax_calendar_repo,
        }
    }

    // ─── Company Service Methods ─────────────────────────────────────────────
    pub async fn create_company(
        &self,
        req: CreateCompanyRequest,
    ) -> Result<CompanyResponse, AppError> {
        let company = Company::new(req.name, req.tax_number, req.address, req.currency);
        self.company_repo.save(&company).await?;
        Ok(CompanyResponse::from(company))
    }

    pub async fn get_company(&self, id: Uuid) -> Result<CompanyResponse, AppError> {
        let company = self.company_repo.find_by_id(id).await?;
        Ok(CompanyResponse::from(company))
    }

    pub async fn update_company(
        &self,
        id: Uuid,
        req: UpdateCompanyRequest,
    ) -> Result<CompanyResponse, AppError> {
        let mut company = self.company_repo.find_by_id(id).await?;
        company.name = req.name;
        company.tax_number = req.tax_number;
        company.address = req.address;
        company.currency = req.currency;
        company.is_active = req.is_active;
        company.updated_at = OffsetDateTime::now_utc();

        self.company_repo.update(&company).await?;
        Ok(CompanyResponse::from(company))
    }

    pub async fn list_companies(&self) -> Result<Vec<CompanyResponse>, AppError> {
        let companies = self.company_repo.find_all().await?;
        Ok(companies.into_iter().map(CompanyResponse::from).collect())
    }

    // ─── Branch Service Methods ─────────────────────────────────────────────
    pub async fn create_branch(
        &self,
        req: CreateBranchRequest,
    ) -> Result<BranchResponse, AppError> {
        let now = OffsetDateTime::now_utc();
        let branch = Branch {
            id: Uuid::new_v4(),
            company_id: req.company_id,
            code: req.code,
            name: req.name,
            address: req.address,
            phone: req.phone,
            is_active: true,
            created_at: now,
            updated_at: now,
        };
        self.branch_repo.save(&branch).await?;
        Ok(BranchResponse::from(branch))
    }

    pub async fn get_branch(&self, id: Uuid) -> Result<BranchResponse, AppError> {
        let branch = self.branch_repo.find_by_id(id).await?;
        Ok(BranchResponse::from(branch))
    }

    pub async fn update_branch(
        &self,
        id: Uuid,
        req: UpdateBranchRequest,
    ) -> Result<BranchResponse, AppError> {
        let mut branch = self.branch_repo.find_by_id(id).await?;
        branch.code = req.code;
        branch.name = req.name;
        branch.address = req.address;
        branch.phone = req.phone;
        branch.is_active = req.is_active;
        branch.updated_at = OffsetDateTime::now_utc();

        self.branch_repo.update(&branch).await?;
        Ok(BranchResponse::from(branch))
    }

    pub async fn list_branches(&self, company_id: Uuid) -> Result<Vec<BranchResponse>, AppError> {
        let branches = self.branch_repo.find_by_company(company_id).await?;
        Ok(branches.into_iter().map(BranchResponse::from).collect())
    }

    // ─── Account (COA) Service Methods ───────────────────────────────────────
    pub async fn create_account(
        &self,
        req: CreateAccountRequest,
    ) -> Result<AccountResponse, AppError> {
        // Validate account type
        let account_type = match req.account_type.to_lowercase().as_str() {
            "asset" => AccountType::Asset,
            "liability" => AccountType::Liability,
            "equity" => AccountType::Equity,
            "revenue" => AccountType::Revenue,
            "expense" => AccountType::Expense,
            _ => {
                return Err(AppError::Validation {
                    message: "Invalid account type".to_string(),
                })
            }
        };

        // Check uniqueness of code
        let code = AccountCode::new(req.code);
        if let Some(_) = self
            .account_repo
            .find_by_code(req.company_id, &code)
            .await?
        {
            return Err(AppError::Conflict {
                message: format!("Account code '{}' already exists in company", code),
            });
        }

        let account = Account::new(req.company_id, code, req.name, account_type, req.parent_id);
        self.account_repo.save(&account).await?;
        Ok(AccountResponse::from(account))
    }

    pub async fn get_account(&self, id: Uuid) -> Result<AccountResponse, AppError> {
        let account = self.account_repo.find_by_id(id).await?;
        Ok(AccountResponse::from(account))
    }

    pub async fn update_account(
        &self,
        id: Uuid,
        req: UpdateAccountRequest,
    ) -> Result<AccountResponse, AppError> {
        let mut account = self.account_repo.find_by_id(id).await?;

        let account_type = match req.account_type.to_lowercase().as_str() {
            "asset" => AccountType::Asset,
            "liability" => AccountType::Liability,
            "equity" => AccountType::Equity,
            "revenue" => AccountType::Revenue,
            "expense" => AccountType::Expense,
            _ => {
                return Err(AppError::Validation {
                    message: "Invalid account type".to_string(),
                })
            }
        };

        let new_code = AccountCode::new(req.code);
        if new_code != account.code {
            if let Some(_) = self
                .account_repo
                .find_by_code(account.company_id, &new_code)
                .await?
            {
                return Err(AppError::Conflict {
                    message: format!("Account code '{}' already exists in company", new_code),
                });
            }
            account.code = new_code;
        }

        account.name = req.name;
        account.account_type = account_type;
        account.parent_id = req.parent_id;
        account.is_active = req.is_active;
        account.updated_at = OffsetDateTime::now_utc();

        self.account_repo.update(&account).await?;
        Ok(AccountResponse::from(account))
    }

    pub async fn list_accounts(&self, company_id: Uuid) -> Result<Vec<AccountResponse>, AppError> {
        let accounts = self.account_repo.find_all_by_company(company_id).await?;
        Ok(accounts.into_iter().map(AccountResponse::from).collect())
    }

    // ─── Customer Service Methods ────────────────────────────────────────────
    pub async fn create_customer(
        &self,
        req: CreateCustomerRequest,
    ) -> Result<CustomerResponse, AppError> {
        let customer = Customer::new(
            req.company_id,
            req.name,
            req.tax_number,
            req.email,
            req.phone,
            req.address,
        );
        self.customer_repo.save(&customer).await?;
        Ok(CustomerResponse::from(customer))
    }

    pub async fn get_customer(&self, id: Uuid) -> Result<CustomerResponse, AppError> {
        let customer = self.customer_repo.find_by_id(id).await?;
        Ok(CustomerResponse::from(customer))
    }

    pub async fn update_customer(
        &self,
        id: Uuid,
        req: UpdateCustomerRequest,
    ) -> Result<CustomerResponse, AppError> {
        let mut customer = self.customer_repo.find_by_id(id).await?;
        customer.name = req.name;
        customer.tax_number = req.tax_number;
        customer.email = req.email;
        customer.phone = req.phone;
        customer.address = req.address;
        customer.is_active = req.is_active;
        customer.updated_at = OffsetDateTime::now_utc();

        self.customer_repo.update(&customer).await?;
        Ok(CustomerResponse::from(customer))
    }

    pub async fn list_customers(
        &self,
        company_id: Uuid,
    ) -> Result<Vec<CustomerResponse>, AppError> {
        let customers = self.customer_repo.find_all_by_company(company_id).await?;
        Ok(customers.into_iter().map(CustomerResponse::from).collect())
    }

    // ─── Supplier Service Methods ────────────────────────────────────────────
    pub async fn create_supplier(
        &self,
        req: CreateSupplierRequest,
    ) -> Result<SupplierResponse, AppError> {
        let supplier = Supplier::new(
            req.company_id,
            req.name,
            req.tax_number,
            req.email,
            req.phone,
            req.address,
        );
        self.supplier_repo.save(&supplier).await?;
        Ok(SupplierResponse::from(supplier))
    }

    pub async fn get_supplier(&self, id: Uuid) -> Result<SupplierResponse, AppError> {
        let supplier = self.supplier_repo.find_by_id(id).await?;
        Ok(SupplierResponse::from(supplier))
    }

    pub async fn update_supplier(
        &self,
        id: Uuid,
        req: UpdateSupplierRequest,
    ) -> Result<SupplierResponse, AppError> {
        let mut supplier = self.supplier_repo.find_by_id(id).await?;
        supplier.name = req.name;
        supplier.tax_number = req.tax_number;
        supplier.email = req.email;
        supplier.phone = req.phone;
        supplier.address = req.address;
        supplier.is_active = req.is_active;
        supplier.updated_at = OffsetDateTime::now_utc();

        self.supplier_repo.update(&supplier).await?;
        Ok(SupplierResponse::from(supplier))
    }

    pub async fn list_suppliers(
        &self,
        company_id: Uuid,
    ) -> Result<Vec<SupplierResponse>, AppError> {
        let suppliers = self.supplier_repo.find_all_by_company(company_id).await?;
        Ok(suppliers.into_iter().map(SupplierResponse::from).collect())
    }

    // ─── BankAccount Service Methods ─────────────────────────────────────────
    pub async fn create_bank_account(
        &self,
        req: CreateBankAccountRequest,
    ) -> Result<BankAccountResponse, AppError> {
        let bank_account = BankAccount::new(
            req.company_id,
            req.account_id,
            req.bank_name,
            req.account_number,
            req.account_name,
            req.currency,
        );
        self.bank_account_repo.save(&bank_account).await?;
        Ok(BankAccountResponse::from(bank_account))
    }

    pub async fn get_bank_account(&self, id: Uuid) -> Result<BankAccountResponse, AppError> {
        let bank_account = self.bank_account_repo.find_by_id(id).await?;
        Ok(BankAccountResponse::from(bank_account))
    }

    pub async fn update_bank_account(
        &self,
        id: Uuid,
        req: UpdateBankAccountRequest,
    ) -> Result<BankAccountResponse, AppError> {
        let mut bank_account = self.bank_account_repo.find_by_id(id).await?;
        bank_account.account_id = req.account_id;
        bank_account.bank_name = req.bank_name;
        bank_account.account_number = req.account_number;
        bank_account.account_name = req.account_name;
        bank_account.currency = req.currency;
        bank_account.is_active = req.is_active;
        bank_account.updated_at = OffsetDateTime::now_utc();

        self.bank_account_repo.update(&bank_account).await?;
        Ok(BankAccountResponse::from(bank_account))
    }

    pub async fn list_bank_accounts(
        &self,
        company_id: Uuid,
    ) -> Result<Vec<BankAccountResponse>, AppError> {
        let bank_accounts = self
            .bank_account_repo
            .find_all_by_company(company_id)
            .await?;
        Ok(bank_accounts
            .into_iter()
            .map(BankAccountResponse::from)
            .collect())
    }

    // ─── TaxType Service Methods ─────────────────────────────────────────────
    pub async fn create_tax_type(
        &self,
        req: CreateTaxTypeRequest,
    ) -> Result<TaxTypeResponse, AppError> {
        let category = match req.category.to_lowercase().as_str() {
            "vat_output" | "vatoutput" => TaxCategory::VatOutput,
            "vat_input" | "vatinput" => TaxCategory::VatInput,
            "withholding_pph21" | "withholdingpph21" => TaxCategory::WithholdingPph21,
            "withholding_pph23" | "withholdingpph23" => TaxCategory::WithholdingPph23,
            "withholding_pph25" | "withholdingpph25" => TaxCategory::WithholdingPph25,
            "withholding_pph_final" | "withholdingpphfinal" => TaxCategory::WithholdingPphFinal,
            _ => {
                return Err(AppError::Validation {
                    message: "Invalid tax category".to_string(),
                })
            }
        };

        let now = OffsetDateTime::now_utc();
        let tax_type = TaxType {
            id: Uuid::new_v4(),
            company_id: req.company_id,
            code: req.code,
            name: req.name,
            category,
            default_rate: req.default_rate,
            payable_account_id: req.payable_account_id,
            effective_from: req.effective_from,
            effective_to: req.effective_to,
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        self.tax_repo.save(&tax_type).await?;
        Ok(TaxTypeResponse::from(tax_type))
    }

    pub async fn get_tax_type(&self, id: Uuid) -> Result<TaxTypeResponse, AppError> {
        let tax_type = self.tax_repo.find_by_id(id).await?;
        Ok(TaxTypeResponse::from(tax_type))
    }

    pub async fn update_tax_type(
        &self,
        id: Uuid,
        req: UpdateTaxTypeRequest,
    ) -> Result<TaxTypeResponse, AppError> {
        let mut tax_type = self.tax_repo.find_by_id(id).await?;

        let category = match req.category.to_lowercase().as_str() {
            "vat_output" | "vatoutput" => TaxCategory::VatOutput,
            "vat_input" | "vatinput" => TaxCategory::VatInput,
            "withholding_pph21" | "withholdingpph21" => TaxCategory::WithholdingPph21,
            "withholding_pph23" | "withholdingpph23" => TaxCategory::WithholdingPph23,
            "withholding_pph25" | "withholdingpph25" => TaxCategory::WithholdingPph25,
            "withholding_pph_final" | "withholdingpphfinal" => TaxCategory::WithholdingPphFinal,
            _ => {
                return Err(AppError::Validation {
                    message: "Invalid tax category".to_string(),
                })
            }
        };

        tax_type.name = req.name;
        tax_type.category = category;
        tax_type.default_rate = req.default_rate;
        tax_type.payable_account_id = req.payable_account_id;
        tax_type.effective_from = req.effective_from;
        tax_type.effective_to = req.effective_to;
        tax_type.is_active = req.is_active;
        tax_type.updated_at = OffsetDateTime::now_utc();

        self.tax_repo.update(&tax_type).await?;
        Ok(TaxTypeResponse::from(tax_type))
    }

    pub async fn list_tax_types(&self, company_id: Uuid) -> Result<Vec<TaxTypeResponse>, AppError> {
        let tax_types = self.tax_repo.find_all_by_company(company_id).await?;
        Ok(tax_types.into_iter().map(TaxTypeResponse::from).collect())
    }

    pub async fn list_tax_records(
        &self,
        company_id: Uuid,
        page: u32,
        per_page: u32,
    ) -> Result<Vec<TaxRecordResponse>, AppError> {
        let list = self
            .tax_record_repo
            .find_all_by_company(company_id, page, per_page)
            .await?;
        Ok(list.into_iter().map(TaxRecordResponse::from).collect())
    }

    pub async fn count_tax_records(&self, company_id: Uuid) -> Result<u64, AppError> {
        self.tax_record_repo.count_by_company(company_id).await
    }

    pub async fn get_tax_summary(
        &self,
        company_id: Uuid,
        start_date: time::Date,
        end_date: time::Date,
    ) -> Result<TaxSummaryResponse, AppError> {
        let records = self
            .tax_record_repo
            .get_summary(company_id, start_date, end_date)
            .await?;

        let tax_types = self.tax_repo.find_all_by_company(company_id).await?;

        let mut total_vat_output = rust_decimal::Decimal::ZERO;
        let mut total_vat_input = rust_decimal::Decimal::ZERO;
        let mut summary_records = Vec::new();

        for r in records {
            if let Some(tax_type) = tax_types.iter().find(|t| t.id == r.tax_type_id) {
                match tax_type.category {
                    TaxCategory::VatOutput => {
                        total_vat_output += r.tax_amount;
                    }
                    TaxCategory::VatInput => {
                        total_vat_input += r.tax_amount;
                    }
                    _ => {}
                }
            }
            summary_records.push(TaxRecordResponse::from(r));
        }

        let net_tax_due = total_vat_output - total_vat_input;

        Ok(TaxSummaryResponse {
            total_vat_output,
            total_vat_input,
            net_tax_due,
            records: summary_records,
        })
    }

    pub async fn list_tax_calendar(&self, company_id: Uuid) -> Result<Vec<TaxCalendarResponse>, AppError> {
        let list = self.tax_calendar_repo.find_all_by_company(company_id).await?;
        Ok(list.into_iter().map(TaxCalendarResponse::from).collect())
    }

    pub async fn create_tax_calendar_entry(
        &self,
        req: CreateTaxCalendarRequest,
    ) -> Result<TaxCalendarResponse, AppError> {
        let now = OffsetDateTime::now_utc();
        let entry = TaxCalendarEntry {
            id: Uuid::new_v4(),
            company_id: req.company_id,
            tax_type_id: req.tax_type_id,
            tax_period: req.tax_period,
            payment_due_date: req.payment_due_date,
            filing_due_date: req.filing_due_date,
            payment_status: finance_assistant_domain::entities::tax::TaxPaymentStatus::Unpaid,
            filing_status: finance_assistant_domain::entities::tax::TaxFilingStatus::Unfiled,
            reminder_sent_at: None,
            created_at: now,
            updated_at: now,
        };

        self.tax_calendar_repo.save(&entry).await?;
        Ok(TaxCalendarResponse::from(entry))
    }

    pub async fn update_tax_calendar_status(
        &self,
        id: Uuid,
        req: UpdateTaxCalendarStatusRequest,
    ) -> Result<TaxCalendarResponse, AppError> {
        let mut entry = self.tax_calendar_repo.find_by_id(id).await?;

        if let Some(p_status) = req.payment_status {
            entry.payment_status = finance_assistant_domain::entities::tax::TaxPaymentStatus::from_str(&p_status);
        }
        if let Some(f_status) = req.filing_status {
            entry.filing_status = finance_assistant_domain::entities::tax::TaxFilingStatus::from_str(&f_status);
        }

        entry.updated_at = OffsetDateTime::now_utc();
        self.tax_calendar_repo.update(&entry).await?;

        Ok(TaxCalendarResponse::from(entry))
    }
}
