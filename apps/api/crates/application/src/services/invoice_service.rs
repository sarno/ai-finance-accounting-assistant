use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;
use time::OffsetDateTime;

use finance_assistant_domain::{
    entities::invoice::{SalesInvoice, InvoiceLine},
    value_objects::DocumentStatus,
};

use crate::{
    dto::invoice::{CreateSalesInvoiceRequest, SalesInvoiceResponse},
    errors::AppError,
    ports::{
        invoice_repository::InvoiceRepository,
        tax_repository::TaxRepository,
        journal_repository::JournalRepository,
    },
};

pub struct InvoiceService {
    invoice_repo: Arc<dyn InvoiceRepository>,
    tax_repo: Arc<dyn TaxRepository>,
    journal_repo: Arc<dyn JournalRepository>,
}

impl InvoiceService {
    pub fn new(
        invoice_repo: Arc<dyn InvoiceRepository>,
        tax_repo: Arc<dyn TaxRepository>,
        journal_repo: Arc<dyn JournalRepository>,
    ) -> Self {
        Self {
            invoice_repo,
            tax_repo,
            journal_repo,
        }
    }

    pub async fn create_sales_draft(&self, req: CreateSalesInvoiceRequest, created_by: Uuid) -> Result<SalesInvoiceResponse, AppError> {
        let mut subtotal = Decimal::ZERO;
        let mut tax_amount = Decimal::ZERO;
        let mut lines = Vec::new();

        for line_req in req.lines {
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

        self.invoice_repo.save_sales(&invoice).await?;
        Ok(SalesInvoiceResponse::from(invoice))
    }

    pub async fn update_sales_draft(&self, id: Uuid, req: CreateSalesInvoiceRequest) -> Result<SalesInvoiceResponse, AppError> {
        let mut existing = self.invoice_repo.find_sales_by_id(id).await?;
        existing.ensure_editable()?;

        let mut subtotal = Decimal::ZERO;
        let mut tax_amount = Decimal::ZERO;
        let mut lines = Vec::new();

        for line_req in req.lines {
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

    pub async fn list_sales_invoices(&self, company_id: Uuid, page: u32, per_page: u32) -> Result<Vec<SalesInvoiceResponse>, AppError> {
        let list = self.invoice_repo.find_sales_by_company(company_id, page, per_page).await?;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use finance_assistant_domain::entities::tax::{TaxType, TaxCategory};
    use finance_assistant_domain::entities::invoice::PurchaseInvoice;
    use finance_assistant_domain::entities::journal::JournalEntry;
    use crate::dto::invoice::CreateInvoiceLineRequest;
    use std::str::FromStr;

    struct MockInvoiceRepository {
        sales: Mutex<Vec<SalesInvoice>>,
    }

    #[async_trait::async_trait]
    impl InvoiceRepository for MockInvoiceRepository {
        async fn find_sales_by_id(&self, _id: Uuid) -> Result<SalesInvoice, AppError> {
            Err(AppError::NotFound { resource: "SalesInvoice".to_string(), id: _id.to_string() })
        }
        async fn find_sales_by_company(&self, _company_id: Uuid, _page: u32, _per_page: u32) -> Result<Vec<SalesInvoice>, AppError> {
            Ok(vec![])
        }
        async fn count_sales_by_company(&self, _company_id: Uuid) -> Result<u64, AppError> {
            Ok(0)
        }
        async fn save_sales(&self, invoice: &SalesInvoice) -> Result<(), AppError> {
            self.sales.lock().unwrap().push(invoice.clone());
            Ok(())
        }
        async fn update_sales(&self, _invoice: &SalesInvoice) -> Result<(), AppError> {
            Ok(())
        }
        async fn delete_sales(&self, _id: Uuid) -> Result<(), AppError> {
            Ok(())
        }
        async fn find_purchase_by_id(&self, _id: Uuid) -> Result<PurchaseInvoice, AppError> {
            Err(AppError::NotFound { resource: "PurchaseInvoice".to_string(), id: _id.to_string() })
        }
        async fn save_purchase(&self, _invoice: &PurchaseInvoice) -> Result<(), AppError> {
            Ok(())
        }
        async fn update_purchase(&self, _invoice: &PurchaseInvoice) -> Result<(), AppError> {
            Ok(())
        }
        async fn find_duplicate_purchase(&self, _company_id: Uuid, _supplier_id: Uuid, _invoice_number: &str) -> Result<bool, AppError> {
            Ok(false)
        }
    }

    struct MockTaxRepository;

    #[async_trait::async_trait]
    impl TaxRepository for MockTaxRepository {
        async fn find_by_id(&self, id: Uuid) -> Result<TaxType, AppError> {
            Ok(TaxType {
                id,
                company_id: Uuid::new_v4(),
                code: "VAT10".to_string(),
                name: "VAT 10%".to_string(),
                category: TaxCategory::VatOutput,
                default_rate: Decimal::from_str("0.10").unwrap(),
                payable_account_id: Uuid::new_v4(),
                effective_from: time::Date::from_calendar_date(2020, time::Month::January, 1).unwrap(),
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
            Err(AppError::NotFound { resource: "JournalEntry".to_string(), id: _id.to_string() })
        }
        async fn find_by_company(&self, _company_id: Uuid, _page: u32, _per_page: u32) -> Result<Vec<JournalEntry>, AppError> {
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
        async fn save_lines(&self, _lines: &[finance_assistant_domain::entities::journal::JournalLine]) -> Result<(), AppError> {
            Ok(())
        }
        async fn count_by_company(&self, _company_id: Uuid) -> Result<i64, AppError> {
            Ok(0)
        }
    }

    #[tokio::test]
    async fn test_create_sales_draft_calculations() {
        let invoice_repo = Arc::new(MockInvoiceRepository { sales: Mutex::new(vec![]) });
        let tax_repo = Arc::new(MockTaxRepository);
        let journal_repo = Arc::new(MockJournalRepository);

        let service = InvoiceService::new(invoice_repo.clone(), tax_repo, journal_repo);

        let company_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let account_id = Uuid::new_v4();
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
                    description: "Item 1".to_string(),
                    quantity: Decimal::from(2),
                    unit_price: Decimal::from(100),
                    discount_amount: Some(Decimal::from(10)), // net = 200 - 10 = 190. tax = 19. total = 209
                    tax_type_id: Some(tax_type_id),
                    account_id,
                    sort_order: 1,
                },
                CreateInvoiceLineRequest {
                    description: "Item 2".to_string(),
                    quantity: Decimal::from(1),
                    unit_price: Decimal::from(50),
                    discount_amount: None, // net = 50. tax = 0. total = 50
                    tax_type_id: None,
                    account_id,
                    sort_order: 2,
                }
            ],
        };

        let result = service.create_sales_draft(req, Uuid::new_v4()).await.unwrap();

        assert_eq!(result.subtotal, Decimal::from(240)); // 190 + 50
        assert_eq!(result.tax_amount, Decimal::from(19)); // 19 + 0
        assert_eq!(result.total_amount, Decimal::from(259)); // 240 + 19

        let saved = invoice_repo.sales.lock().unwrap();
        assert_eq!(saved.len(), 1);
        assert_eq!(saved[0].subtotal, Decimal::from(240));
        assert_eq!(saved[0].tax_amount, Decimal::from(19));
        assert_eq!(saved[0].total_amount, Decimal::from(259));
        assert_eq!(saved[0].lines.len(), 2);
        assert_eq!(saved[0].lines[0].tax_rate, Some(Decimal::from_str("0.10").unwrap()));
        assert_eq!(saved[0].lines[0].tax_amount, Decimal::from(19));
        assert_eq!(saved[0].lines[0].line_total, Decimal::from(209));
    }
}
