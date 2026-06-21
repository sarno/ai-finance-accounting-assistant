use rust_decimal::Decimal;
use std::sync::Arc;
use time::OffsetDateTime;
use uuid::Uuid;

use finance_assistant_domain::{
    entities::{
        account::Account,
        invoice::{InvoiceLine, PurchaseInvoice, SalesInvoice},
        tax::TaxType,
    },
    value_objects::DocumentStatus,
};

use crate::{
    dto::invoice::{
        CreatePurchaseInvoiceRequest, CreateSalesInvoiceRequest, PurchaseInvoiceResponse,
        SalesInvoiceResponse,
    },
    errors::AppError,
    ports::{
        account_repository::AccountRepository,
        invoice_repository::InvoiceRepository, item_repository::ItemRepository,
        journal_repository::JournalRepository, tax_repository::TaxRepository,
        supplier_repository::SupplierRepository,
    },
};

pub struct InvoiceService {
    invoice_repo: Arc<dyn InvoiceRepository>,
    item_repo: Arc<dyn ItemRepository>,
    supplier_repo: Arc<dyn SupplierRepository>,
    account_repo: Arc<dyn AccountRepository>,
    tax_repo: Arc<dyn TaxRepository>,
    journal_repo: Arc<dyn JournalRepository>,
}

impl InvoiceService {
    pub fn new(
        invoice_repo: Arc<dyn InvoiceRepository>,
        item_repo: Arc<dyn ItemRepository>,
        supplier_repo: Arc<dyn SupplierRepository>,
        account_repo: Arc<dyn AccountRepository>,
        tax_repo: Arc<dyn TaxRepository>,
        journal_repo: Arc<dyn JournalRepository>,
    ) -> Self {
        Self {
            invoice_repo,
            item_repo,
            supplier_repo,
            account_repo,
            tax_repo,
            journal_repo,
        }
    }

    async fn validate_line_item(
        &self,
        company_id: Uuid,
        item_id: Option<Uuid>,
    ) -> Result<Option<Uuid>, AppError> {
        let Some(item_id) = item_id else {
            return Ok(None);
        };

        let item = self.item_repo.find_item_by_id(item_id).await?;
        if item.company_id != company_id {
            return Err(AppError::Validation {
                message: "Selected item does not belong to the invoice company".to_string(),
            });
        }
        if !item.is_active {
            return Err(AppError::Validation {
                message: "Selected item is inactive".to_string(),
            });
        }

        Ok(Some(item_id))
    }

    async fn validate_supplier(&self, company_id: Uuid, supplier_id: Uuid) -> Result<(), AppError> {
        let supplier = self.supplier_repo.find_by_id(supplier_id).await?;
        if supplier.company_id != company_id {
            return Err(AppError::Validation {
                message: "Selected supplier does not belong to the invoice company".to_string(),
            });
        }
        if !supplier.is_active {
            return Err(AppError::Validation {
                message: "Selected supplier is inactive".to_string(),
            });
        }
        Ok(())
    }

    async fn validate_account(&self, company_id: Uuid, account_id: Uuid) -> Result<Account, AppError> {
        let account = self.account_repo.find_by_id(account_id).await?;
        if account.company_id != company_id {
            return Err(AppError::Validation {
                message: "Selected account does not belong to the invoice company".to_string(),
            });
        }
        if !account.is_active {
            return Err(AppError::Validation {
                message: "Selected account is inactive".to_string(),
            });
        }
        Ok(account)
    }

    async fn validate_tax_type(&self, company_id: Uuid, tax_type_id: Uuid) -> Result<TaxType, AppError> {
        let tax = self.tax_repo.find_by_id(tax_type_id).await?;
        if tax.company_id != company_id {
            return Err(AppError::Validation {
                message: "Selected tax type does not belong to the invoice company".to_string(),
            });
        }
        if !tax.is_active {
            return Err(AppError::Validation {
                message: "Selected tax type is inactive".to_string(),
            });
        }
        Ok(tax)
    }

    pub async fn create_sales_draft(
        &self,
        req: CreateSalesInvoiceRequest,
        created_by: Uuid,
    ) -> Result<SalesInvoiceResponse, AppError> {
        let company_id = req.company_id;
        let mut subtotal = Decimal::ZERO;
        let mut tax_amount = Decimal::ZERO;
        let mut lines = Vec::new();

        for line_req in req.lines {
            let item_id = self
                .validate_line_item(company_id, line_req.item_id)
                .await?;
            let (tax_rate, line_tax) = match line_req.tax_type_id {
                Some(tax_id) => {
                    let tax_config = self.tax_repo.find_by_id(tax_id).await?;
                    let rate = tax_config.default_rate;
                    let quantity = line_req.quantity;
                    let unit_price = line_req.unit_price;
                    let discount = line_req.discount_amount.unwrap_or(Decimal::ZERO);
                    let net = (quantity * unit_price) - discount;
                    let tax = (net * rate).round_dp(2);
                    (Some(rate), tax)
                }
                None => (None, Decimal::ZERO),
            };

            let quantity = line_req.quantity;
            let unit_price = line_req.unit_price;
            let discount = line_req.discount_amount.unwrap_or(Decimal::ZERO);
            let net_amount = (quantity * unit_price) - discount;
            let line_total = net_amount + line_tax;

            subtotal += net_amount;
            tax_amount += line_tax;

            lines.push(InvoiceLine {
                id: Uuid::new_v4(),
                item_id,
                description: line_req.description,
                quantity,
                unit_price,
                discount_amount: discount,
                tax_type_id: line_req.tax_type_id,
                tax_rate,
                tax_amount: line_tax,
                line_total,
                account_id: line_req.account_id,
                sort_order: line_req.sort_order,
            });
        }
        let total_amount = subtotal + tax_amount;
        let now = OffsetDateTime::now_utc();

        let invoice = SalesInvoice {
            id: Uuid::new_v4(),
            company_id: req.company_id,
            branch_id: req.branch_id,
            invoice_number: req.invoice_number,
            customer_id: req.customer_id,
            invoice_date: req.invoice_date,
            due_date: req.due_date,
            lines,
            subtotal,
            tax_amount,
            total_amount,
            status: DocumentStatus::Draft,
            notes: req.notes,
            journal_entry_id: None,
            created_by,
            created_at: now,
            updated_at: now,
        };

        let saved = self.invoice_repo.save_sales(&invoice).await?;
        Ok(SalesInvoiceResponse::from(saved))
    }

    pub async fn update_sales_draft(
        &self,
        id: Uuid,
        req: CreateSalesInvoiceRequest,
    ) -> Result<SalesInvoiceResponse, AppError> {
        let mut existing = self.invoice_repo.find_sales_by_id(id).await?;
        existing.ensure_editable()?;

        let company_id = req.company_id;
        let mut subtotal = Decimal::ZERO;
        let mut tax_amount = Decimal::ZERO;
        let mut lines = Vec::new();

        for line_req in req.lines {
            let item_id = self
                .validate_line_item(company_id, line_req.item_id)
                .await?;
            let (tax_rate, line_tax) = match line_req.tax_type_id {
                Some(tax_id) => {
                    let tax_config = self.tax_repo.find_by_id(tax_id).await?;
                    let rate = tax_config.default_rate;
                    let quantity = line_req.quantity;
                    let unit_price = line_req.unit_price;
                    let discount = line_req.discount_amount.unwrap_or(Decimal::ZERO);
                    let net = (quantity * unit_price) - discount;
                    let tax = (net * rate).round_dp(2);
                    (Some(rate), tax)
                }
                None => (None, Decimal::ZERO),
            };

            let quantity = line_req.quantity;
            let unit_price = line_req.unit_price;
            let discount = line_req.discount_amount.unwrap_or(Decimal::ZERO);
            let net_amount = (quantity * unit_price) - discount;
            let line_total = net_amount + line_tax;

            subtotal += net_amount;
            tax_amount += line_tax;

            lines.push(InvoiceLine {
                id: Uuid::new_v4(),
                item_id,
                description: line_req.description,
                quantity,
                unit_price,
                discount_amount: discount,
                tax_type_id: line_req.tax_type_id,
                tax_rate,
                tax_amount: line_tax,
                line_total,
                account_id: line_req.account_id,
                sort_order: line_req.sort_order,
            });
        }
        let total_amount = subtotal + tax_amount;

        existing.branch_id = req.branch_id;
        existing.invoice_number = req.invoice_number;
        existing.customer_id = req.customer_id;
        existing.invoice_date = req.invoice_date;
        existing.due_date = req.due_date;
        existing.lines = lines;
        existing.subtotal = subtotal;
        existing.tax_amount = tax_amount;
        existing.total_amount = total_amount;
        existing.notes = req.notes;
        existing.updated_at = OffsetDateTime::now_utc();

        self.invoice_repo.update_sales(&existing).await?;
        Ok(SalesInvoiceResponse::from(existing))
    }

    pub async fn get_sales_invoice(&self, id: Uuid) -> Result<SalesInvoiceResponse, AppError> {
        let invoice = self.invoice_repo.find_sales_by_id(id).await?;
        Ok(SalesInvoiceResponse::from(invoice))
    }

    pub async fn list_sales_invoices(
        &self,
        company_id: Uuid,
        page: u32,
        per_page: u32,
    ) -> Result<Vec<SalesInvoiceResponse>, AppError> {
        let list = self
            .invoice_repo
            .find_sales_by_company(company_id, page, per_page)
            .await?;
        Ok(list.into_iter().map(SalesInvoiceResponse::from).collect())
    }

    pub async fn count_sales_invoices(&self, company_id: Uuid) -> Result<u64, AppError> {
        self.invoice_repo.count_sales_by_company(company_id).await
    }

    pub async fn delete_sales_invoice(&self, id: Uuid) -> Result<(), AppError> {
        let existing = self.invoice_repo.find_sales_by_id(id).await?;
        existing.ensure_editable()?;
        self.invoice_repo.delete_sales(id).await
    }

    pub async fn create_purchase_draft(
        &self,
        req: CreatePurchaseInvoiceRequest,
        created_by: Uuid,
    ) -> Result<PurchaseInvoiceResponse, AppError> {
        let company_id = req.company_id;
        self.validate_supplier(company_id, req.supplier_id).await?;

        if self
            .invoice_repo
            .find_duplicate_purchase(company_id, req.supplier_id, &req.supplier_invoice_number)
            .await?
        {
            return Err(AppError::Domain(
                finance_assistant_domain::errors::DomainError::DuplicateInvoiceNumber,
            ));
        }

        let mut subtotal = Decimal::ZERO;
        let mut tax_amount = Decimal::ZERO;
        let mut lines = Vec::new();

        for line_req in req.lines {
            let item_id = self.validate_line_item(company_id, line_req.item_id).await?;
            let _account = self.validate_account(company_id, line_req.account_id).await?;
            let (tax_rate, line_tax) = match line_req.tax_type_id {
                Some(tax_id) => {
                    let tax_config = self.validate_tax_type(company_id, tax_id).await?;
                    let rate = tax_config.default_rate;
                    let quantity = line_req.quantity;
                    let unit_price = line_req.unit_price;
                    let discount = line_req.discount_amount.unwrap_or(Decimal::ZERO);
                    let net = (quantity * unit_price) - discount;
                    let tax = (net * rate).round_dp(2);
                    (Some(rate), tax)
                }
                None => (None, Decimal::ZERO),
            };

            let quantity = line_req.quantity;
            let unit_price = line_req.unit_price;
            let discount = line_req.discount_amount.unwrap_or(Decimal::ZERO);
            let net_amount = (quantity * unit_price) - discount;
            let line_total = net_amount + line_tax;

            subtotal += net_amount;
            tax_amount += line_tax;

            lines.push(InvoiceLine {
                id: Uuid::new_v4(),
                item_id,
                description: line_req.description,
                quantity,
                unit_price,
                discount_amount: discount,
                tax_type_id: line_req.tax_type_id,
                tax_rate,
                tax_amount: line_tax,
                line_total,
                account_id: line_req.account_id,
                sort_order: line_req.sort_order,
            });
        }

        let total_amount = subtotal + tax_amount;
        let now = OffsetDateTime::now_utc();

        let invoice = PurchaseInvoice {
            id: Uuid::new_v4(),
            company_id: req.company_id,
            branch_id: req.branch_id,
            supplier_invoice_number: req.supplier_invoice_number,
            internal_reference: req.internal_reference,
            supplier_id: req.supplier_id,
            invoice_date: req.invoice_date,
            due_date: req.due_date,
            lines,
            subtotal,
            tax_amount,
            total_amount,
            status: DocumentStatus::Draft,
            ai_confidence: None,
            uploaded_document_id: None,
            journal_entry_id: None,
            notes: req.notes,
            created_by,
            created_at: now,
            updated_at: now,
        };

        let saved = self.invoice_repo.save_purchase(&invoice).await?;
        Ok(PurchaseInvoiceResponse::from(saved))
    }

    pub async fn update_purchase_draft(
        &self,
        id: Uuid,
        req: CreatePurchaseInvoiceRequest,
    ) -> Result<PurchaseInvoiceResponse, AppError> {
        let mut existing = self.invoice_repo.find_purchase_by_id(id).await?;
        existing.ensure_editable()?;

        let company_id = req.company_id;
        self.validate_supplier(company_id, req.supplier_id).await?;

        if existing.supplier_id != req.supplier_id
            || existing.supplier_invoice_number != req.supplier_invoice_number
        {
            if self
                .invoice_repo
                .find_duplicate_purchase(company_id, req.supplier_id, &req.supplier_invoice_number)
                .await?
            {
                return Err(AppError::Domain(
                    finance_assistant_domain::errors::DomainError::DuplicateInvoiceNumber,
                ));
            }
        }

        let mut subtotal = Decimal::ZERO;
        let mut tax_amount = Decimal::ZERO;
        let mut lines = Vec::new();

        for line_req in req.lines {
            let item_id = self.validate_line_item(company_id, line_req.item_id).await?;
            let _account = self.validate_account(company_id, line_req.account_id).await?;
            let (tax_rate, line_tax) = match line_req.tax_type_id {
                Some(tax_id) => {
                    let tax_config = self.validate_tax_type(company_id, tax_id).await?;
                    let rate = tax_config.default_rate;
                    let quantity = line_req.quantity;
                    let unit_price = line_req.unit_price;
                    let discount = line_req.discount_amount.unwrap_or(Decimal::ZERO);
                    let net = (quantity * unit_price) - discount;
                    let tax = (net * rate).round_dp(2);
                    (Some(rate), tax)
                }
                None => (None, Decimal::ZERO),
            };

            let quantity = line_req.quantity;
            let unit_price = line_req.unit_price;
            let discount = line_req.discount_amount.unwrap_or(Decimal::ZERO);
            let net_amount = (quantity * unit_price) - discount;
            let line_total = net_amount + line_tax;

            subtotal += net_amount;
            tax_amount += line_tax;

            lines.push(InvoiceLine {
                id: Uuid::new_v4(),
                item_id,
                description: line_req.description,
                quantity,
                unit_price,
                discount_amount: discount,
                tax_type_id: line_req.tax_type_id,
                tax_rate,
                tax_amount: line_tax,
                line_total,
                account_id: line_req.account_id,
                sort_order: line_req.sort_order,
            });
        }

        let total_amount = subtotal + tax_amount;

        existing.branch_id = req.branch_id;
        existing.supplier_invoice_number = req.supplier_invoice_number;
        existing.internal_reference = req.internal_reference;
        existing.supplier_id = req.supplier_id;
        existing.invoice_date = req.invoice_date;
        existing.due_date = req.due_date;
        existing.lines = lines;
        existing.subtotal = subtotal;
        existing.tax_amount = tax_amount;
        existing.total_amount = total_amount;
        existing.notes = req.notes;
        existing.updated_at = OffsetDateTime::now_utc();

        self.invoice_repo.update_purchase(&existing).await?;
        Ok(PurchaseInvoiceResponse::from(existing))
    }

    pub async fn get_purchase_invoice(&self, id: Uuid) -> Result<PurchaseInvoiceResponse, AppError> {
        let invoice = self.invoice_repo.find_purchase_by_id(id).await?;
        Ok(PurchaseInvoiceResponse::from(invoice))
    }

    pub async fn list_purchase_invoices(
        &self,
        company_id: Uuid,
        page: u32,
        per_page: u32,
    ) -> Result<Vec<PurchaseInvoiceResponse>, AppError> {
        let list = self
            .invoice_repo
            .find_purchase_by_company(company_id, page, per_page)
            .await?;
        Ok(list.into_iter().map(PurchaseInvoiceResponse::from).collect())
    }

    pub async fn count_purchase_invoices(&self, company_id: Uuid) -> Result<u64, AppError> {
        self.invoice_repo.count_purchase_by_company(company_id).await
    }

    pub async fn delete_purchase_invoice(&self, id: Uuid) -> Result<(), AppError> {
        let existing = self.invoice_repo.find_purchase_by_id(id).await?;
        existing.ensure_editable()?;
        self.invoice_repo.delete_purchase(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::invoice::{CreateInvoiceLineRequest, CreatePurchaseInvoiceLineRequest};
    use finance_assistant_domain::entities::account::Account;
    use finance_assistant_domain::entities::invoice::PurchaseInvoice;
    use finance_assistant_domain::entities::item::{Item, ItemCategory};
    use finance_assistant_domain::entities::journal::JournalEntry;
    use finance_assistant_domain::entities::supplier::Supplier;
    use finance_assistant_domain::entities::tax::{TaxCategory, TaxType};
    use finance_assistant_domain::value_objects::{AccountCode, AccountType};
    use std::str::FromStr;
    use std::sync::Mutex;

    struct MockInvoiceRepository {
        sales: Mutex<Vec<SalesInvoice>>,
        purchase: Mutex<Option<PurchaseInvoice>>,
    }

    struct MockItemRepository {
        item: Mutex<Option<Item>>,
    }

    struct MockSupplierRepository {
        supplier: Mutex<Option<Supplier>>,
    }

    struct MockAccountRepository {
        account: Mutex<Option<Account>>,
    }

    #[async_trait::async_trait]
    impl InvoiceRepository for MockInvoiceRepository {
        async fn find_sales_by_id(&self, _id: Uuid) -> Result<SalesInvoice, AppError> {
            Err(AppError::NotFound {
                resource: "SalesInvoice".to_string(),
                id: _id.to_string(),
            })
        }
        async fn find_sales_by_company(
            &self,
            _company_id: Uuid,
            _page: u32,
            _per_page: u32,
        ) -> Result<Vec<SalesInvoice>, AppError> {
            Ok(vec![])
        }
        async fn count_sales_by_company(&self, _company_id: Uuid) -> Result<u64, AppError> {
            Ok(0)
        }
        async fn save_sales(&self, invoice: &SalesInvoice) -> Result<SalesInvoice, AppError> {
            self.sales.lock().unwrap().push(invoice.clone());
            Ok(invoice.clone())
        }
        async fn update_sales(&self, _invoice: &SalesInvoice) -> Result<(), AppError> {
            Ok(())
        }
        async fn delete_sales(&self, _id: Uuid) -> Result<(), AppError> {
            Ok(())
        }
        async fn find_purchase_by_company(
            &self,
            _company_id: Uuid,
            _page: u32,
            _per_page: u32,
        ) -> Result<Vec<PurchaseInvoice>, AppError> {
            Ok(vec![])
        }
        async fn count_purchase_by_company(&self, _company_id: Uuid) -> Result<u64, AppError> {
            Ok(0)
        }
        async fn save_purchase(&self, invoice: &PurchaseInvoice) -> Result<PurchaseInvoice, AppError> {
            *self.purchase.lock().unwrap() = Some(invoice.clone());
            Ok(invoice.clone())
        }
        async fn update_purchase(&self, _invoice: &PurchaseInvoice) -> Result<(), AppError> {
            Ok(())
        }
        async fn delete_purchase(&self, _id: Uuid) -> Result<(), AppError> {
            Ok(())
        }
        async fn find_purchase_by_id(&self, _id: Uuid) -> Result<PurchaseInvoice, AppError> {
            Err(AppError::NotFound {
                resource: "PurchaseInvoice".to_string(),
                id: _id.to_string(),
            })
        }
        async fn find_duplicate_purchase(
            &self,
            _company_id: Uuid,
            _supplier_id: Uuid,
            _invoice_number: &str,
        ) -> Result<bool, AppError> {
            Ok(false)
        }
    }

    #[async_trait::async_trait]
    impl crate::ports::item_repository::ItemRepository for MockItemRepository {
        async fn find_category_by_id(&self, _id: Uuid) -> Result<ItemCategory, AppError> {
            Err(AppError::NotFound {
                resource: "ItemCategory".to_string(),
                id: _id.to_string(),
            })
        }
        async fn find_categories_by_company(
            &self,
            _company_id: Uuid,
        ) -> Result<Vec<ItemCategory>, AppError> {
            Ok(vec![])
        }
        async fn save_category(&self, _category: &ItemCategory) -> Result<(), AppError> {
            Ok(())
        }
        async fn update_category(&self, _category: &ItemCategory) -> Result<(), AppError> {
            Ok(())
        }
        async fn delete_category(&self, _id: Uuid) -> Result<(), AppError> {
            Ok(())
        }
        async fn find_item_by_id(&self, id: Uuid) -> Result<Item, AppError> {
            self.item
                .lock()
                .unwrap()
                .clone()
                .filter(|item| item.id == id)
                .ok_or(AppError::NotFound {
                    resource: "Item".to_string(),
                    id: id.to_string(),
                })
        }
        async fn find_items_by_company(&self, _company_id: Uuid) -> Result<Vec<Item>, AppError> {
            Ok(vec![])
        }
        async fn save_item(&self, _item: &Item) -> Result<(), AppError> {
            Ok(())
        }
        async fn update_item(&self, _item: &Item) -> Result<(), AppError> {
            Ok(())
        }
        async fn delete_item(&self, _id: Uuid) -> Result<(), AppError> {
            Ok(())
        }
    }

    #[async_trait::async_trait]
    impl crate::ports::supplier_repository::SupplierRepository for MockSupplierRepository {
        async fn find_by_id(&self, id: Uuid) -> Result<Supplier, AppError> {
            self.supplier
                .lock()
                .unwrap()
                .clone()
                .filter(|supplier| supplier.id == id)
                .ok_or(AppError::NotFound {
                    resource: "Supplier".to_string(),
                    id: id.to_string(),
                })
        }
        async fn find_all_by_company(&self, _company_id: Uuid) -> Result<Vec<Supplier>, AppError> {
            Ok(vec![])
        }
        async fn save(&self, _supplier: &Supplier) -> Result<(), AppError> {
            Ok(())
        }
        async fn update(&self, _supplier: &Supplier) -> Result<(), AppError> {
            Ok(())
        }
    }

    #[async_trait::async_trait]
    impl crate::ports::account_repository::AccountRepository for MockAccountRepository {
        async fn find_by_id(&self, id: Uuid) -> Result<Account, AppError> {
            self.account
                .lock()
                .unwrap()
                .clone()
                .filter(|account| account.id == id)
                .ok_or(AppError::NotFound {
                    resource: "Account".to_string(),
                    id: id.to_string(),
                })
        }
        async fn find_by_code(
            &self,
            _company_id: Uuid,
            _code: &AccountCode,
        ) -> Result<Option<Account>, AppError> {
            Ok(None)
        }
        async fn find_all_by_company(&self, _company_id: Uuid) -> Result<Vec<Account>, AppError> {
            Ok(vec![])
        }
        async fn save(&self, _account: &Account) -> Result<(), AppError> {
            Ok(())
        }
        async fn update(&self, _account: &Account) -> Result<(), AppError> {
            Ok(())
        }
    }

    struct MockTaxRepository {
        company_id: Uuid,
    }

    #[async_trait::async_trait]
    impl TaxRepository for MockTaxRepository {
        async fn find_by_id(&self, id: Uuid) -> Result<TaxType, AppError> {
            Ok(TaxType {
                id,
                company_id: self.company_id,
                code: "VAT10".to_string(),
                name: "VAT 10%".to_string(),
                category: TaxCategory::VatOutput,
                default_rate: Decimal::from_str("0.10").unwrap(),
                payable_account_id: Uuid::new_v4(),
                effective_from: time::Date::from_calendar_date(2020, time::Month::January, 1)
                    .unwrap(),
                effective_to: None,
                is_active: true,
                created_at: OffsetDateTime::now_utc(),
                updated_at: OffsetDateTime::now_utc(),
            })
        }
        async fn find_all_by_company(&self, _company_id: Uuid) -> Result<Vec<TaxType>, AppError> {
            Ok(vec![])
        }
        async fn save(&self, _tax_type: &TaxType) -> Result<(), AppError> {
            Ok(())
        }
        async fn update(&self, _tax_type: &TaxType) -> Result<(), AppError> {
            Ok(())
        }
    }

    struct MockJournalRepository;

    #[async_trait::async_trait]
    impl JournalRepository for MockJournalRepository {
        async fn find_by_id(&self, _id: Uuid) -> Result<JournalEntry, AppError> {
            Err(AppError::NotFound {
                resource: "JournalEntry".to_string(),
                id: _id.to_string(),
            })
        }
        async fn find_by_company(
            &self,
            _company_id: Uuid,
            _page: u32,
            _per_page: u32,
        ) -> Result<Vec<JournalEntry>, AppError> {
            Ok(vec![])
        }
        async fn save(&self, _entry: &JournalEntry) -> Result<(), AppError> {
            Ok(())
        }
        async fn update(&self, _entry: &JournalEntry) -> Result<(), AppError> {
            Ok(())
        }
        async fn delete(&self, _id: Uuid) -> Result<(), AppError> {
            Ok(())
        }
        async fn save_lines(
            &self,
            _lines: &[finance_assistant_domain::entities::journal::JournalLine],
        ) -> Result<(), AppError> {
            Ok(())
        }
        async fn count_by_company(&self, _company_id: Uuid) -> Result<i64, AppError> {
            Ok(0)
        }
    }

    #[tokio::test]
    async fn test_create_sales_draft_calculations() {
        let invoice_repo = Arc::new(MockInvoiceRepository {
            sales: Mutex::new(vec![]),
            purchase: Mutex::new(None),
        });
        let company_id = Uuid::new_v4();
        let item_id = Uuid::new_v4();
        let account_id = Uuid::new_v4();
        let item_repo = Arc::new(MockItemRepository {
            item: Mutex::new(Some(Item {
                id: item_id,
                company_id,
                category_id: None,
                code: "SRV-001".to_string(),
                name: "Item 1".to_string(),
                description: Some("Linked item".to_string()),
                unit_price: Decimal::from(100),
                sale_account_id: Some(Uuid::new_v4()),
                purchase_account_id: None,
                tax_type_id: None,
                is_active: true,
                created_at: OffsetDateTime::now_utc(),
                updated_at: OffsetDateTime::now_utc(),
            })),
        });
        let supplier_repo = Arc::new(MockSupplierRepository {
            supplier: Mutex::new(Some(Supplier {
                id: Uuid::new_v4(),
                company_id,
                name: "Supplier".to_string(),
                tax_number: None,
                email: None,
                phone: None,
                address: None,
                is_active: true,
                created_at: OffsetDateTime::now_utc(),
                updated_at: OffsetDateTime::now_utc(),
            })),
        });
        let account_repo = Arc::new(MockAccountRepository {
            account: Mutex::new(Some(Account {
                id: account_id,
                company_id,
                code: AccountCode("4000".to_string()),
                name: "Sales Revenue".to_string(),
                account_type: AccountType::Revenue,
                parent_id: None,
                is_active: true,
                created_at: OffsetDateTime::now_utc(),
                updated_at: OffsetDateTime::now_utc(),
            })),
        });
        let tax_repo = Arc::new(MockTaxRepository { company_id });
        let journal_repo = Arc::new(MockJournalRepository);

        let service = InvoiceService::new(
            invoice_repo.clone(),
            item_repo,
            supplier_repo,
            account_repo,
            tax_repo,
            journal_repo,
        );

        let customer_id = Uuid::new_v4();
        let tax_type_id = Uuid::new_v4();

        let req = CreateSalesInvoiceRequest {
            company_id,
            branch_id: None,
            invoice_number: "INV-2026-001".to_string(),
            customer_id,
            invoice_date: time::Date::from_calendar_date(2026, time::Month::June, 21).unwrap(),
            due_date: time::Date::from_calendar_date(2026, time::Month::July, 21).unwrap(),
            notes: Some("Test note".to_string()),
            lines: vec![
                CreateInvoiceLineRequest {
                    item_id: Some(item_id),
                    description: "Item 1".to_string(),
                    quantity: Decimal::from(2),
                    unit_price: Decimal::from(100),
                    discount_amount: Some(Decimal::from(10)), // net = 200 - 10 = 190. tax = 19. total = 209
                    tax_type_id: Some(tax_type_id),
                    account_id,
                    sort_order: 1,
                },
                CreateInvoiceLineRequest {
                    item_id: Some(item_id),
                    description: "Item 2".to_string(),
                    quantity: Decimal::from(1),
                    unit_price: Decimal::from(50),
                    discount_amount: None, // net = 50. tax = 0. total = 50
                    tax_type_id: None,
                    account_id,
                    sort_order: 2,
                },
            ],
        };

        let result = service
            .create_sales_draft(req, Uuid::new_v4())
            .await
            .unwrap();

        assert_eq!(result.subtotal, Decimal::from(240)); // 190 + 50
        assert_eq!(result.tax_amount, Decimal::from(19)); // 19 + 0
        assert_eq!(result.total_amount, Decimal::from(259)); // 240 + 19

        let saved = invoice_repo.sales.lock().unwrap();
        assert_eq!(saved.len(), 1);
        assert_eq!(saved[0].subtotal, Decimal::from(240));
        assert_eq!(saved[0].tax_amount, Decimal::from(19));
        assert_eq!(saved[0].total_amount, Decimal::from(259));
        assert_eq!(saved[0].lines.len(), 2);
        assert_eq!(saved[0].lines[0].item_id, Some(item_id));
        assert_eq!(saved[0].lines[1].item_id, Some(item_id));
        assert_eq!(
            saved[0].lines[0].tax_rate,
            Some(Decimal::from_str("0.10").unwrap())
        );
        assert_eq!(saved[0].lines[0].tax_amount, Decimal::from(19));
        assert_eq!(saved[0].lines[0].line_total, Decimal::from(209));
    }

    #[tokio::test]
    async fn test_create_purchase_draft_calculations() {
        let invoice_repo = Arc::new(MockInvoiceRepository {
            sales: Mutex::new(vec![]),
            purchase: Mutex::new(None),
        });
        let company_id = Uuid::new_v4();
        let item_id = Uuid::new_v4();
        let account_id = Uuid::new_v4();
        let supplier_id = Uuid::new_v4();
        let tax_type_id = Uuid::new_v4();

        let item_repo = Arc::new(MockItemRepository {
            item: Mutex::new(Some(Item {
                id: item_id,
                company_id,
                category_id: None,
                code: "PUR-001".to_string(),
                name: "Purchase Item".to_string(),
                description: Some("Linked purchase item".to_string()),
                unit_price: Decimal::from(100),
                sale_account_id: None,
                purchase_account_id: Some(account_id),
                tax_type_id: None,
                is_active: true,
                created_at: OffsetDateTime::now_utc(),
                updated_at: OffsetDateTime::now_utc(),
            })),
        });
        let supplier_repo = Arc::new(MockSupplierRepository {
            supplier: Mutex::new(Some(Supplier {
                id: supplier_id,
                company_id,
                name: "Supplier".to_string(),
                tax_number: None,
                email: None,
                phone: None,
                address: None,
                is_active: true,
                created_at: OffsetDateTime::now_utc(),
                updated_at: OffsetDateTime::now_utc(),
            })),
        });
        let account_repo = Arc::new(MockAccountRepository {
            account: Mutex::new(Some(Account {
                id: account_id,
                company_id,
                code: AccountCode("5000".to_string()),
                name: "Expense".to_string(),
                account_type: AccountType::Expense,
                parent_id: None,
                is_active: true,
                created_at: OffsetDateTime::now_utc(),
                updated_at: OffsetDateTime::now_utc(),
            })),
        });
        let tax_repo = Arc::new(MockTaxRepository { company_id });
        let journal_repo = Arc::new(MockJournalRepository);

        let service = InvoiceService::new(
            invoice_repo.clone(),
            item_repo,
            supplier_repo,
            account_repo,
            tax_repo,
            journal_repo,
        );

        let req = CreatePurchaseInvoiceRequest {
            company_id,
            branch_id: None,
            supplier_invoice_number: "SUP-INV-001".to_string(),
            internal_reference: "PI/2026/0001".to_string(),
            supplier_id,
            invoice_date: time::Date::from_calendar_date(2026, time::Month::June, 21).unwrap(),
            due_date: time::Date::from_calendar_date(2026, time::Month::July, 21).unwrap(),
            notes: Some("Purchase note".to_string()),
            lines: vec![CreatePurchaseInvoiceLineRequest {
                item_id: Some(item_id),
                description: "Purchase line".to_string(),
                quantity: Decimal::from(2),
                unit_price: Decimal::from(100),
                discount_amount: Some(Decimal::from(20)),
                tax_type_id: Some(tax_type_id),
                account_id,
                sort_order: 1,
            }],
        };

        let result = service
            .create_purchase_draft(req, Uuid::new_v4())
            .await
            .unwrap();

        assert_eq!(result.subtotal, Decimal::from(180));
        assert_eq!(result.tax_amount, Decimal::from(18));
        assert_eq!(result.total_amount, Decimal::from(198));
        assert_eq!(result.status, "draft");
        assert_eq!(result.lines.len(), 1);
        assert_eq!(result.lines[0].account_id, account_id);
        assert_eq!(result.lines[0].tax_amount, Decimal::from(18));

        let saved = invoice_repo.purchase.lock().unwrap().clone().unwrap();
        assert_eq!(saved.total_amount, Decimal::from(198));
        assert_eq!(saved.lines.len(), 1);
        assert_eq!(saved.supplier_id, supplier_id);
    }
}
