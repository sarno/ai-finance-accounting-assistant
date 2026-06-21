# Product Requirements — AI Finance & Accounting Assistant

## 1. Product overview

**AI Finance & Accounting Assistant** adalah sistem keuangan dan accounting custom yang membantu owner dan tim finance/accounting untuk:

- melihat kondisi keuangan perusahaan secara cepat;
- melakukan pencatatan transaksi;
- membuat draft invoice, jurnal, pembayaran, dan pajak dengan bantuan AI;
- melakukan approval sebelum posting;
- menghasilkan laporan keuangan dan pajak.

Produk terdiri dari dua interface utama:

1. **Chat interface** via WhatsApp/Telegram melalui OpenClaw.
2. **Admin dashboard** berbasis Vue.js untuk finance, accounting, dan owner.

## 2. Target user

### Owner / Direktur

Kebutuhan:

- Bertanya kondisi kas, hutang, piutang, laba, beban, pajak, dan cashflow kapan saja.
- Mendapat alert penting.
- Memberi approval transaksi penting.

### Finance / Accounting Staff

Kebutuhan:

- Input invoice, pembayaran, jurnal, pajak.
- Review draft yang dibuat AI.
- Koreksi data hasil OCR/AI.
- Posting transaksi setelah approval.

### Finance Manager / Supervisor

Kebutuhan:

- Review dan approve transaksi.
- Melihat laporan periodik.
- Mengecek audit trail.

### System Admin

Kebutuhan:

- Kelola user, role, company, setting pajak, COA, integrasi OpenClaw, backup.

## 3. Core problem

Client saat ini mengolah keuangan dengan Excel dan file disimpan di Synology. Owner sering harus meminta informasi ke staf sehingga akses informasi lambat dan tergantung jam kerja.

## 4. Product goals

- Owner bisa bertanya laporan keuangan via chat.
- Sistem accounting custom menjadi single source of truth.
- AI membantu otomasi draft transaksi, bukan menggantikan kontrol manusia.
- Data transaksi aman, traceable, dan siap audit.
- Tax module membantu ringkasan pajak, jadwal, validasi dokumen, dan pencatatan pajak.

## 5. Non-goals MVP

Untuk MVP awal, sistem **tidak** wajib:

- mengirim laporan resmi langsung ke DJP;
- terhubung langsung ke bank API;
- melakukan full payroll;
- melakukan inventory kompleks;
- menggantikan konsultan pajak.

## 6. Main modules

### 6.1 Master data

- Company profile
- Fiscal year and accounting periods
- Chart of Accounts
- Customer
- Supplier
- Bank/Cash accounts
- Tax types
- Product/service/items

### 6.2 Accounting core

- Journal entry
- Journal lines
- Posting engine
- Reversal journal
- Period closing
- General ledger
- Trial balance

### 6.3 Sales

- Sales invoice
- Sales return/credit note (phase 2)
- Accounts receivable
- Payment received

### 6.4 Purchase

- Purchase invoice / bill
- Purchase return/debit note (phase 2)
- Accounts payable
- Payment paid

### 6.5 Cash and bank

- Cash/bank account
- Cash/bank mutation
- Transfer antar rekening
- Bank reconciliation (phase 2)

### 6.6 Expense

- Expense claim
- Operational expense
- Attachment receipt

### 6.7 Tax

- VAT/PPN input and output tax recording
- PPh withholding records
- Tax due schedule
- Tax summary report
- Tax document validation status
- Tax configuration per company

### 6.8 AI assistant

- Query financial status
- Create draft invoice
- Create draft payment
- Create draft journal
- Extract invoice data from document
- Suggest journal entries
- Explain report numbers
- Notify overdue receivables/payables

### 6.9 Approval

- Approval request
- Approval workflow
- Approve/reject/revise
- Approval history

### 6.10 Audit

- User activity log
- AI conversation log
- AI tool call log
- Data change log
- Posting log

## 7. Key user stories

### Owner asks cash position

As an owner, I want to ask via WhatsApp/Telegram “Berapa posisi kas hari ini?” so that I can get current cash position instantly.

Acceptance:

- Bot returns total cash and per bank/cash account.
- Result comes from backend report API.
- Response includes reporting period/date.

### Finance uploads invoice

As finance staff, I want to upload a vendor invoice so that AI can extract fields and create a draft purchase invoice.

Acceptance:

- System extracts vendor, invoice number, date, due date, subtotal, tax, total.
- User can correct extracted values.
- System suggests journal entry.
- Transaction remains draft until approval.

### Manager approves transaction

As a finance manager, I want to approve a draft transaction so that it can be posted into accounting records.

Acceptance:

- Manager can see original data, suggested journal, attachment, and AI confidence.
- Manager can approve or reject.
- Posting creates balanced journal entry.
- Audit log is created.

### Owner asks tax summary

As owner, I want to ask “Pajak bulan ini berapa?” so that I know current tax payable and upcoming deadlines.

Acceptance:

- Bot returns tax summary from tax report API.
- Tax rates are taken from configurable tax settings.
- Response includes warning if incomplete documents exist.

## 8. Data migration requirement

Initial system must support importing existing Excel data from Synology:

- COA
- Customer/supplier
- Opening balance
- AR/AP aging
- Historical sales/purchase invoices
- Cash/bank balances

Import must support:

- preview before import;
- validation errors;
- duplicate detection;
- import log.

## 9. Critical constraints

- Money type must be decimal with precision.
- Audit log must be append-only.
- Posted transaction must not be edited directly.
- AI output must always be validated by backend.
- Tax rules must be configurable and versioned.
