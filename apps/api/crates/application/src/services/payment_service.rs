use std::sync::Arc;
use uuid::Uuid;
use time::OffsetDateTime;

use finance_assistant_domain::{
    entities::payment::{Payment, PaymentAllocation, PaymentType, CounterpartyType, AllocatedDocumentType},
    value_objects::DocumentStatus,
};

use crate::{
    dto::payment::{CreatePaymentRequest, PaymentResponse},
    errors::AppError,
    ports::payment_repository::PaymentRepository,
};

pub struct PaymentService {
    payment_repo: Arc<dyn PaymentRepository>,
}

impl PaymentService {
    pub fn new(payment_repo: Arc<dyn PaymentRepository>) -> Self {
        Self { payment_repo }
    }

    pub async fn create_payment(&self, req: CreatePaymentRequest, user_id: Uuid) -> Result<PaymentResponse, AppError> {
        let payment_id = Uuid::new_v4();
        let payment_type = match req.payment_type.as_str() {
            "payment_received" => PaymentType::PaymentReceived,
            "payment_paid" => PaymentType::PaymentPaid,
            other => return Err(AppError::Validation { message: format!("Invalid payment type: {}", other) }),
        };

        let counterparty_type = match req.counterparty_type.as_str() {
            "customer" => CounterpartyType::Customer,
            "supplier" => CounterpartyType::Supplier,
            other => return Err(AppError::Validation { message: format!("Invalid counterparty type: {}", other) }),
        };

        let mut allocations = Vec::new();
        for alloc_req in req.allocations {
            let document_type = match alloc_req.document_type.as_str() {
                "sales_invoice" => AllocatedDocumentType::SalesInvoice,
                "purchase_invoice" => AllocatedDocumentType::PurchaseInvoice,
                other => return Err(AppError::Validation { message: format!("Invalid document type: {}", other) }),
            };
            allocations.push(PaymentAllocation {
                id: Uuid::new_v4(),
                payment_id,
                document_type,
                document_id: alloc_req.document_id,
                allocated_amount: alloc_req.allocated_amount,
            });
        }

        let now = OffsetDateTime::now_utc();
        let payment = Payment {
            id: payment_id,
            company_id: req.company_id,
            reference_number: String::new(), // Repo will generate one
            payment_type,
            counterparty_type,
            counterparty_id: req.counterparty_id,
            payment_date: req.payment_date,
            bank_account_id: req.bank_account_id,
            amount: req.amount,
            allocations,
            status: DocumentStatus::Draft,
            notes: req.notes,
            journal_entry_id: None,
            attachment_url: req.attachment_url,
            created_by: user_id,
            created_at: now,
            updated_at: now,
        };

        let saved = self.payment_repo.save(&payment).await?;
        Ok(PaymentResponse::from(saved))
    }

    pub async fn get_payment(&self, id: Uuid) -> Result<PaymentResponse, AppError> {
        let payment = self.payment_repo.find_by_id(id).await?;
        Ok(PaymentResponse::from(payment))
    }

    pub async fn list_payments(&self, company_id: Uuid, page: u32, per_page: u32) -> Result<Vec<PaymentResponse>, AppError> {
        let payments = self.payment_repo.find_by_company(company_id, page, per_page).await?;
        Ok(payments.into_iter().map(PaymentResponse::from).collect())
    }

    pub async fn count_payments(&self, company_id: Uuid) -> Result<u64, AppError> {
        self.payment_repo.count_by_company(company_id).await
    }

    pub async fn update_payment(&self, id: Uuid, req: CreatePaymentRequest) -> Result<(), AppError> {
        let mut payment = self.payment_repo.find_by_id(id).await?;
        if payment.status != DocumentStatus::Draft && payment.status != DocumentStatus::Rejected {
            return Err(AppError::Validation {
                message: "Only draft or rejected payments can be modified".to_string(),
            });
        }

        let payment_type = match req.payment_type.as_str() {
            "payment_received" => PaymentType::PaymentReceived,
            "payment_paid" => PaymentType::PaymentPaid,
            other => return Err(AppError::Validation { message: format!("Invalid payment type: {}", other) }),
        };

        let counterparty_type = match req.counterparty_type.as_str() {
            "customer" => CounterpartyType::Customer,
            "supplier" => CounterpartyType::Supplier,
            other => return Err(AppError::Validation { message: format!("Invalid counterparty type: {}", other) }),
        };

        let mut allocations = Vec::new();
        for alloc_req in req.allocations {
            let document_type = match alloc_req.document_type.as_str() {
                "sales_invoice" => AllocatedDocumentType::SalesInvoice,
                "purchase_invoice" => AllocatedDocumentType::PurchaseInvoice,
                other => return Err(AppError::Validation { message: format!("Invalid document type: {}", other) }),
            };
            allocations.push(PaymentAllocation {
                id: Uuid::new_v4(),
                payment_id: id,
                document_type,
                document_id: alloc_req.document_id,
                allocated_amount: alloc_req.allocated_amount,
            });
        }

        payment.payment_type = payment_type;
        payment.counterparty_type = counterparty_type;
        payment.counterparty_id = req.counterparty_id;
        payment.payment_date = req.payment_date;
        payment.bank_account_id = req.bank_account_id;
        payment.amount = req.amount;
        payment.allocations = allocations;
        payment.notes = req.notes;
        payment.attachment_url = req.attachment_url;
        payment.updated_at = OffsetDateTime::now_utc();

        self.payment_repo.update(&payment).await
    }

    pub async fn delete_payment(&self, id: Uuid) -> Result<(), AppError> {
        let payment = self.payment_repo.find_by_id(id).await?;
        if payment.status != DocumentStatus::Draft && payment.status != DocumentStatus::Rejected {
            return Err(AppError::Validation {
                message: "Only draft or rejected payments can be deleted".to_string(),
            });
        }
        self.payment_repo.delete(id).await
    }
}
