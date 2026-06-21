# Frontend Specification — Vue.js 3 + Pinia

## 1. Tech stack

- Vue.js 3
- Vite
- Pinia
- Vue Router
- TypeScript recommended
- Axios/fetch API client
- UI library optional: PrimeVue, Naive UI, Vuetify, or custom Tailwind

## 2. App structure

```text
src/
  api/
    client.ts
    auth.api.ts
    reports.api.ts
    invoices.api.ts
    approvals.api.ts
  components/
  layouts/
  pages/
    dashboard/
    reports/
    sales/
    purchases/
    payments/
    journals/
    approvals/
    tax/
    settings/
  stores/
    auth.store.ts
    company.store.ts
    coa.store.ts
    invoice.store.ts
    approval.store.ts
    report.store.ts
  router/
  types/
  utils/
```

## 3. Main navigation

Sidebar menu:

- Dashboard
- Cashflow
- Sales
- Purchases
- Transactions
- Receivables
- Payables
- Expenses
- Journals
- Tax
- Reports
- Approvals
- AI Inbox
- Settings

## 4. Dashboard page

Widgets:

- Cash position
- Accounts receivable
- Accounts payable
- Profit/loss current month
- Tax payable current period
- Pending approvals
- Overdue AR
- Upcoming tax deadlines

## 5. Invoice automation page

Route:

```text
/purchases/invoice-automation
```

Features:

- Upload invoice PDF/image.
- Show document preview.
- Show AI extracted fields with confidence score.
- Allow manual correction.
- Show validation checklist.
- Show suggested journal entries.
- Submit for approval.

UI sections:

```text
Left: document preview
Middle: extracted fields + validation
Bottom: suggested journal table
Right: approval workflow
```

## 6. Approval page

Route:

```text
/approvals
```

Features:

- List pending approvals.
- Filter by document type/status/date.
- Detail view with document, attachment, journal, tax records.
- Approve/reject/revise.

## 7. Tax page

Route:

```text
/tax
```

Tabs:

- Summary
- PPN
- PPh
- Tax Calendar
- Tax Documents
- Settings

Summary cards:

- PPN Keluaran
- PPN Masukan
- PPN Terutang/lebih bayar
- PPh 21
- PPh 23
- PPh 25
- Upcoming deadlines
- Missing documents

## 8. Reports page

Route:

```text
/reports
```

Report tabs:

- Laba Rugi
- Neraca
- Arus Kas
- Buku Besar
- Trial Balance
- Piutang
- Hutang
- Pajak

## 9. Pinia store examples

### auth.store.ts

State:

```ts
user
accessToken
refreshToken
roles
permissions
```

Actions:

```ts
login()
logout()
refreshToken()
loadCurrentUser()
```

### approval.store.ts

State:

```ts
pendingApprovals
selectedApproval
loading
filters
```

Actions:

```ts
fetchApprovals()
fetchApprovalDetail(id)
approve(id, comment)
reject(id, reason)
```

## 10. Form validation

Use shared validation rules:

- Required fields.
- Amount must be greater than zero.
- Invoice date cannot be empty.
- Due date must not be before invoice date.
- Journal debit total must equal credit total.
- Account must be active.
- Tax type must be valid for transaction date.

## 11. UI status labels

Use consistent status labels:

```text
Draft = Abu-abu
WaitingApproval = Kuning
Approved = Biru
Posted = Hijau
Rejected = Merah
Cancelled = Abu gelap
```

## 12. UX principle

- Always show whether transaction is draft or posted.
- Always show if data came from AI/OCR.
- Always allow finance staff to edit draft before approval.
- Always show audit history on document detail.
