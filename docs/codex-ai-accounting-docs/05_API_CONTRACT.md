# API Contract Draft — Rust (Axum) Web API

Base path:

```text
/api
```

All protected endpoints require JWT bearer token.

OpenClaw tool endpoints require (validated by Rust middleware):

```text
Authorization: Bearer <service_token>
X-Idempotency-Key: <uuid>
```

## 1. Auth

### POST /auth/login

Request:

```json
{
  "email": "owner@example.com",
  "password": "secret"
}
```

Response:

```json
{
  "accessToken": "...",
  "refreshToken": "...",
  "user": {
    "id": "uuid",
    "fullName": "Owner",
    "roles": ["Owner"]
  }
}
```

### POST /auth/refresh

### POST /auth/logout

## 2. Master data

### GET /companies/current

### GET /chart-of-accounts

### POST /chart-of-accounts

### PUT /chart-of-accounts/{id}

### GET /customers

### POST /customers

### GET /suppliers

### POST /suppliers

### GET /bank-accounts

### POST /bank-accounts

## 3. Tax configuration

### GET /tax-types

### POST /tax-types

Request:

```json
{
  "code": "PPN_OUTPUT",
  "name": "PPN Keluaran",
  "category": "VAT_OUTPUT",
  "defaultRate": 0.11,
  "effectiveFrom": "2026-01-01",
  "payableAccountId": "uuid"
}
```

### PUT /tax-types/{id}

### GET /tax-summary?period=2026-06

Response:

```json
{
  "period": "2026-06",
  "vatOutput": 125000000,
  "vatInput": 85000000,
  "netVatPayable": 40000000,
  "withholdingTaxPayable": 45300000,
  "warnings": [
    {
      "type": "MissingDocument",
      "message": "12 transaksi belum memiliki lampiran pajak."
    }
  ]
}
```

## 4. Sales invoices

### GET /sales-invoices

Filters:

```text
status, customerId, dateFrom, dateTo, q
```

### POST /sales-invoices/draft

Request:

```json
{
  "customerId": "uuid",
  "invoiceDate": "2026-06-02",
  "dueDate": "2026-07-02",
  "lines": [
    {
      "description": "Jasa konsultasi",
      "quantity": 1,
      "unitPrice": 10000000,
      "taxTypeId": "uuid",
      "revenueAccountId": "uuid"
    }
  ],
  "notes": "Created by AI"
}
```

### POST /sales-invoices/{id}/submit-approval

### POST /sales-invoices/{id}/post

Posting response:

```json
{
  "documentId": "uuid",
  "journalEntryId": "uuid",
  "status": "Posted"
}
```

## 5. Purchase invoices

### POST /purchase-invoices/draft

### POST /purchase-invoices/from-document

Request:

```json
{
  "uploadedDocumentId": "uuid",
  "supplierId": "uuid",
  "extractedFields": {
    "invoiceNo": "INV-2026-001",
    "invoiceDate": "2026-06-01",
    "dueDate": "2026-06-30",
    "subtotal": 2350000,
    "taxAmount": 258500,
    "totalAmount": 2608500
  },
  "lines": [
    {
      "description": "Alat tulis kantor",
      "quantity": 10,
      "unitPrice": 50000,
      "expenseAccountId": "uuid",
      "taxTypeId": "uuid"
    }
  ],
  "aiConfidence": 98.7
}
```

### POST /purchase-invoices/{id}/submit-approval

### POST /purchase-invoices/{id}/post

## 6. Payments

### POST /payments/draft

Request:

```json
{
  "paymentType": "PaymentPaid",
  "counterpartyType": "Supplier",
  "counterpartyId": "uuid",
  "paymentDate": "2026-06-02",
  "bankAccountId": "uuid",
  "amount": 2608500,
  "allocations": [
    {
      "documentType": "PurchaseInvoice",
      "documentId": "uuid",
      "amount": 2608500
    }
  ]
}
```

### POST /payments/{id}/submit-approval

### POST /payments/{id}/post

## 7. Journals

### POST /journals/draft

Request:

```json
{
  "transactionDate": "2026-06-02",
  "description": "Adjustment journal",
  "lines": [
    {
      "accountId": "uuid",
      "debit": 100000,
      "credit": 0,
      "description": "Debit line"
    },
    {
      "accountId": "uuid",
      "debit": 0,
      "credit": 100000,
      "description": "Credit line"
    }
  ]
}
```

### POST /journals/{id}/submit-approval

### POST /journals/{id}/post

## 8. Approval

### GET /approvals

### GET /approvals/{id}

### POST /approvals/{id}/approve

Request:

```json
{
  "comment": "Data sudah sesuai."
}
```

### POST /approvals/{id}/reject

Request:

```json
{
  "reason": "Nominal pajak perlu dicek ulang."
}
```

## 9. Reports

### GET /reports/cash-position?asOf=2026-06-02

Response:

```json
{
  "asOf": "2026-06-02",
  "totalCash": 1250000000,
  "accounts": [
    {
      "bankName": "BCA",
      "accountName": "Bank BCA Operasional",
      "balance": 900000000
    }
  ]
}
```

### GET /reports/accounts-receivable?asOf=2026-06-02

### GET /reports/accounts-payable?asOf=2026-06-02

### GET /reports/profit-loss?period=2026-06

### GET /reports/cashflow?periodFrom=2026-01&periodTo=2026-06

### GET /reports/tax-summary?period=2026-06

## 10. AI tool endpoints

These endpoints are used by OpenClaw/AI agent.

### POST /ai/tools/query-report

Request:

```json
{
  "userExternalId": "telegram:123",
  "intent": "CashPosition",
  "parameters": {
    "asOf": "2026-06-02"
  }
}
```

### POST /ai/tools/create-draft-invoice

### POST /ai/tools/create-draft-payment

### POST /ai/tools/create-draft-journal

### POST /ai/tools/submit-approval-command

Request:

```json
{
  "userExternalId": "whatsapp:+62812...",
  "approvalRequestId": "uuid",
  "action": "Approve",
  "comment": "Setujui"
}
```

## 11. Error format

Use consistent error response:

```json
{
  "errorCode": "JOURNAL_NOT_BALANCED",
  "message": "Total debit and credit must be equal.",
  "details": {
    "totalDebit": 100000,
    "totalCredit": 90000
  }
}
```
