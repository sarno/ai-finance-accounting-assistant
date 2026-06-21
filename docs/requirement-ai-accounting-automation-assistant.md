# Requirement Teknologi: AI Accounting Automation Assistant

## 1. Ringkasan Solusi

**AI Accounting Automation Assistant** adalah sistem accounting custom yang dilengkapi AI assistant untuk membantu owner dan tim finance dalam membaca kondisi keuangan perusahaan, membuat draft transaksi, memproses invoice, membuat jurnal, mencatat pembayaran, memantau pajak, serta menghasilkan laporan keuangan melalui dashboard web dan chat WhatsApp/Telegram.

Sistem ini tidak hanya chatbot, tetapi terdiri dari:

1. Sistem accounting core.
2. Dashboard admin finance.
3. Database keuangan terstruktur.
4. AI assistant untuk query dan automation.
5. OpenClaw sebagai gateway chat dan agent automation.
6. Approval workflow sebelum transaksi diposting.
7. Audit log untuk keamanan dan kepatuhan.

---

## 2. Tujuan Sistem

Tujuan utama sistem:

- Owner dapat bertanya kondisi keuangan kapan saja melalui WhatsApp/Telegram.
- Finance dapat mencatat invoice, pembayaran, jurnal, hutang, piutang, kas, bank, biaya, dan pajak secara terstruktur.
- AI dapat membantu membuat draft transaksi, tetapi tidak langsung posting tanpa validasi.
- Semua transaksi tercatat ke sistem accounting berbasis double-entry.
- Laporan keuangan bisa dihasilkan otomatis dari jurnal.
- Pajak seperti PPN, PPh, pajak keluaran, pajak masukan, dan kewajiban pajak dapat dimonitor.
- Data lebih aman, rapi, dan tidak bergantung penuh pada file Excel manual.

---

## 3. Stack Teknologi

| Kebutuhan | Teknologi |
|---|---|
| Chat / AI Gateway | OpenClaw |
| Backend API | Rust (Axum Web Framework) |
| Frontend Dashboard | Vue.js |
| State Management | Pinia |
| Database | PostgreSQL |
| Database Driver | SQLx (async, compile-time checked) |
| Background Job | Tokio async tasks / background worker (Rust) |
| Cache / Queue | Redis |
| File Storage | Synology NAS / Object Storage |
| OCR Invoice | Tesseract / Google Vision / Azure Document Intelligence |
| AI Model | OpenAI API / model lain |
| Authentication | JWT Authentication |
| Authorization | Role Based Access Control |
| Deployment | Docker / Docker Compose |
| Reverse Proxy | Nginx / Caddy |
| Logging | `tracing` + `tracing-subscriber` (Rust) |
| Monitoring | Grafana / Prometheus |
| Backup | PostgreSQL Backup + Synology Backup |

---

## 4. Arsitektur Sistem

```text
WhatsApp / Telegram
        ↓
OpenClaw
        ↓
Rust (Axum) API
        ↓
PostgreSQL Database
        ↓
Vue.js + Pinia Dashboard
```

Arsitektur lebih lengkap:

```text
User / Owner / Finance
        ↓
WhatsApp / Telegram / Web Dashboard
        ↓
OpenClaw / Vue Frontend
        ↓
Rust (Axum) Backend API
        ↓
Business Logic Accounting
        ↓
PostgreSQL
        ↓
Reports / Audit Logs / Approval / Tax Records
```

---

## 5. Prinsip Desain Penting

### 5.1 AI Tidak Boleh Langsung Posting Transaksi

AI hanya boleh membuat **draft transaksi**.

```text
AI membuat draft
      ↓
Finance review
      ↓
Owner / Admin approve
      ↓
Sistem posting transaksi
```

### 5.2 Accounting Harus Double-Entry

Semua transaksi final harus menghasilkan jurnal debit dan kredit yang seimbang.

Contoh:

```text
Dr. Piutang Usaha      Rp11.100.000
Cr. Penjualan          Rp10.000.000
Cr. PPN Keluaran       Rp1.100.000
```

### 5.3 Backend Menjadi Sumber Kebenaran

AI tidak boleh menjadi sumber angka utama. Semua angka laporan harus dihitung dari database.

### 5.4 Semua Aktivitas Harus Tercatat

Setiap input, edit, approval, posting, dan query AI harus masuk ke audit log.

---

## 6. Role Pengguna

Minimal role yang disarankan:

| Role | Hak Akses |
|---|---|
| Owner | Akses semua laporan, approve transaksi penting, bertanya ke AI |
| Finance Admin | Input dan review transaksi, upload invoice, rekonsiliasi |
| Accounting | Kelola COA, jurnal, pajak, laporan keuangan |
| Manager | Akses laporan terbatas sesuai divisi |
| Auditor | Read-only + akses audit log |
| System Admin | Kelola user, konfigurasi sistem, backup |

---

## 7. Modul Sistem Accounting Core

### 7.1 Modul Master Data

Fitur:

- Company profile.
- Branch / cabang.
- Department / divisi.
- Chart of Accounts / COA.
- Customer.
- Supplier.
- Bank account.
- Cash account.
- Product / service.
- Tax code.
- Payment terms.
- Currency.
- Fiscal period.

### 7.2 Modul Chart of Accounts

COA minimal:

- Aset.
- Kas dan bank.
- Piutang usaha.
- Persediaan, jika dibutuhkan.
- Aset tetap, jika dibutuhkan.
- Hutang usaha.
- Hutang pajak.
- Modal.
- Pendapatan.
- Harga pokok penjualan, jika dibutuhkan.
- Beban operasional.
- Beban pajak.

Fitur COA:

- Kode akun.
- Nama akun.
- Tipe akun.
- Parent account.
- Status aktif/nonaktif.
- Mapping akun default untuk invoice, pembayaran, pajak, dan jurnal otomatis.

### 7.3 Modul Sales Invoice

Fitur:

- Buat invoice penjualan.
- Draft invoice dari AI.
- Nomor invoice otomatis.
- Customer.
- Tanggal invoice.
- Due date.
- Item produk/jasa.
- Diskon.
- PPN keluaran.
- PPh dipotong customer, jika ada.
- Status invoice: Draft, Waiting Approval, Approved, Posted, Paid, Partial Paid, Cancelled.
- Posting otomatis ke jurnal.

Contoh jurnal sales invoice:

```text
Dr. Piutang Usaha
Cr. Penjualan
Cr. PPN Keluaran
```

### 7.4 Modul Purchase Invoice

Fitur:

- Input invoice pembelian.
- Upload invoice PDF/foto.
- OCR invoice.
- Draft invoice dari AI.
- Supplier.
- Tanggal invoice.
- Due date.
- Item pembelian.
- Beban / persediaan.
- PPN masukan.
- PPh yang dipotong, jika ada.
- Status pembayaran.
- Posting otomatis ke jurnal.

Contoh jurnal purchase invoice:

```text
Dr. Beban / Persediaan
Dr. PPN Masukan
Cr. Hutang Usaha
```

### 7.5 Modul Payment Received

Fitur:

- Catat pembayaran dari customer.
- Matching ke sales invoice.
- Pembayaran sebagian.
- Biaya admin bank.
- Selisih pembayaran.
- Bukti transfer.
- Posting jurnal otomatis.

Contoh jurnal:

```text
Dr. Bank
Cr. Piutang Usaha
```

### 7.6 Modul Payment Paid

Fitur:

- Catat pembayaran ke supplier.
- Matching ke purchase invoice.
- Pembayaran sebagian.
- Biaya admin bank.
- Bukti transfer.
- Posting jurnal otomatis.

Contoh jurnal:

```text
Dr. Hutang Usaha
Cr. Bank
```

### 7.7 Modul Journal Entry

Fitur:

- Jurnal umum manual.
- Jurnal otomatis dari invoice dan payment.
- Jurnal penyesuaian.
- Jurnal reversal.
- Validasi debit = kredit.
- Approval jurnal.
- Lock period setelah periode ditutup.

### 7.8 Modul Cash & Bank

Fitur:

- Daftar rekening bank.
- Kas kecil.
- Mutasi kas/bank.
- Rekonsiliasi bank.
- Import mutasi bank dari Excel/CSV.
- Matching pembayaran dengan invoice.
- Saldo kas real-time.

### 7.9 Modul Expense

Fitur:

- Input biaya operasional.
- Kategori biaya.
- Department / cost center.
- Upload bukti biaya.
- PPN masukan jika ada.
- PPh jika ada.
- Approval expense.
- Posting jurnal otomatis.

### 7.10 Modul Accounts Receivable

Fitur:

- Daftar piutang.
- Aging piutang.
- Piutang jatuh tempo.
- Customer overdue.
- Reminder penagihan.
- Analisis piutang bermasalah.

### 7.11 Modul Accounts Payable

Fitur:

- Daftar hutang.
- Aging hutang.
- Hutang jatuh tempo.
- Supplier payable.
- Reminder pembayaran.
- Proyeksi kebutuhan kas.

---

## 8. Modul Pajak

Karena sistem accounting dikembangkan sendiri, modul pajak perlu dirancang sejak awal agar transaksi tidak perlu dibongkar ulang di kemudian hari.

### 8.1 Master Tax Code

Fitur:

- Kode pajak.
- Nama pajak.
- Jenis pajak.
- Tarif pajak.
- Akun pajak terkait.
- Berlaku mulai tanggal.
- Status aktif/nonaktif.

Contoh tax code:

| Kode | Nama | Tarif | Akun |
|---|---:|---:|---|
| VAT_OUT_11 | PPN Keluaran 11% | 11% | PPN Keluaran |
| VAT_IN_11 | PPN Masukan 11% | 11% | PPN Masukan |
| PPH23 | PPh 23 | 2% | PPh Dipotong |
| PPH21 | PPh 21 | sesuai aturan | Hutang PPh 21 |
| PPH25 | PPh 25 | sesuai aturan | Hutang PPh 25 |
| PPH_FINAL | PPh Final | sesuai aturan | Hutang PPh Final |

### 8.2 PPN Keluaran

Digunakan saat penjualan kena PPN.

Fitur:

- Hitung PPN keluaran dari sales invoice.
- Simpan nomor faktur pajak, jika ada.
- Status faktur pajak.
- Masa pajak.
- Ekspor data PPN keluaran.
- Rekap per masa pajak.

Contoh jurnal:

```text
Dr. Piutang Usaha
Cr. Penjualan
Cr. PPN Keluaran
```

### 8.3 PPN Masukan

Digunakan saat pembelian kena PPN.

Fitur:

- Hitung PPN masukan dari purchase invoice.
- Simpan nomor faktur pajak supplier.
- Validasi NPWP/NIK supplier, jika dibutuhkan.
- Status dapat dikreditkan/tidak.
- Masa pajak.
- Rekap PPN masukan.

Contoh jurnal:

```text
Dr. Beban / Persediaan
Dr. PPN Masukan
Cr. Hutang Usaha
```

### 8.4 PPh Dipotong Customer

Digunakan ketika customer memotong PPh atas pembayaran ke perusahaan.

Fitur:

- Catat PPh dipotong pada sales invoice/payment.
- Simpan bukti potong.
- Matching bukti potong dengan invoice.
- Rekap PPh dipotong sebagai kredit pajak.

Contoh jurnal saat pembayaran diterima dengan potongan PPh:

```text
Dr. Bank
Dr. PPh Dipotong Dimuka
Cr. Piutang Usaha
```

### 8.5 PPh yang Dipotong Perusahaan

Digunakan ketika perusahaan memotong PPh atas pembayaran ke supplier/vendor.

Fitur:

- Hitung PPh 21/23/4(2)/final sesuai tax code.
- Catat hutang pajak.
- Simpan bukti potong.
- Status setor pajak.
- Tanggal setor.
- Nomor NTPN, jika ada.

Contoh jurnal:

```text
Dr. Hutang Usaha
Cr. Bank
Cr. Hutang PPh 23
```

### 8.6 Tax Payable & Settlement

Fitur:

- Hitung saldo hutang pajak.
- Rekap pajak per masa pajak.
- Catat pembayaran pajak.
- Simpan bukti setor.
- Simpan NTPN.
- Status pajak: Draft, Calculated, Waiting Payment, Paid, Reported.

Contoh jurnal pembayaran pajak:

```text
Dr. Hutang Pajak
Cr. Bank
```

### 8.7 Laporan Pajak

Minimal laporan:

- Rekap PPN keluaran.
- Rekap PPN masukan.
- Selisih PPN kurang/lebih bayar.
- Rekap PPh 21.
- Rekap PPh 23.
- Rekap PPh final.
- Daftar hutang pajak.
- Daftar pembayaran pajak.
- Daftar bukti potong.
- Pajak per masa pajak.

---

## 9. Modul AI Accounting Assistant

### 9.1 Query Keuangan

Owner dapat bertanya:

- Berapa saldo kas hari ini?
- Berapa total piutang jatuh tempo?
- Berapa hutang minggu ini?
- Berapa laba bulan berjalan?
- Beban operasional terbesar bulan ini apa?
- Berapa PPN keluaran bulan ini?
- Berapa PPN masukan bulan ini?
- Berapa estimasi pajak yang harus dibayar?
- Customer mana yang paling telat bayar?
- Supplier mana yang tagihannya jatuh tempo minggu ini?

### 9.2 Draft Transaksi dari Chat

Contoh:

```text
User:
Catat invoice pembelian dari PT ABC sebesar Rp11.100.000 termasuk PPN, jatuh tempo 30 hari.

AI:
Saya buatkan draft purchase invoice:
Supplier: PT ABC
DPP: Rp10.000.000
PPN Masukan: Rp1.100.000
Total: Rp11.100.000
Due date: 30 hari

Status: Waiting Review
```

### 9.3 Draft Jurnal dari Chat

AI dapat membantu membuat jurnal, tetapi backend harus validasi debit dan kredit.

Contoh:

```text
User:
Buat jurnal biaya internet Rp500.000 dibayar dari BCA.

AI Draft:
Dr. Beban Internet Rp500.000
Cr. Bank BCA Rp500.000
```

### 9.4 Draft Pajak dari Chat

Contoh:

```text
User:
Catat pembayaran PPh 23 bulan Mei sebesar Rp2.500.000 dari rekening BCA.

AI Draft:
Dr. Hutang PPh 23 Rp2.500.000
Cr. Bank BCA Rp2.500.000
```

### 9.5 AI Rules

AI wajib mematuhi aturan:

- Tidak boleh posting final tanpa approval.
- Tidak boleh membuat akun COA baru tanpa izin.
- Tidak boleh menghapus transaksi.
- Tidak boleh mengubah transaksi posted tanpa reversal.
- Harus menyebutkan sumber data saat menjawab laporan.
- Jika data tidak lengkap, AI harus menjawab bahwa data belum tersedia.

---

## 10. OpenClaw Integration

OpenClaw digunakan sebagai:

- Gateway WhatsApp.
- Gateway Telegram.
- AI agent orchestrator.
- Automation runner.
- Message router.
- Approval interaction layer.

### 10.1 Alur Chat Query

```text
Owner bertanya via WhatsApp/Telegram
        ↓
OpenClaw menerima pesan
        ↓
OpenClaw kirim request ke Rust (Axum) API
        ↓
Backend ambil data dari PostgreSQL
        ↓
Backend kirim hasil ke OpenClaw
        ↓
OpenClaw jawab ke owner
```

### 10.2 Alur Draft Transaction

```text
User kirim instruksi transaksi
        ↓
OpenClaw parsing intent
        ↓
Backend membuat draft transaksi
        ↓
Draft muncul di dashboard
        ↓
Finance review
        ↓
Owner approve via dashboard/chat
        ↓
Backend posting jurnal
```

### 10.3 Alur Upload Invoice

```text
User upload invoice PDF/foto
        ↓
OpenClaw menerima file
        ↓
Backend menyimpan file ke storage
        ↓
OCR membaca dokumen
        ↓
AI ekstrak data invoice
        ↓
Backend validasi vendor, nominal, pajak
        ↓
Draft invoice dibuat
        ↓
Finance review dan approve
```

---

## 11. Frontend Dashboard Vue.js + Pinia

### 11.1 Halaman Utama

- Dashboard ringkasan keuangan.
- Kas dan bank.
- Piutang.
- Hutang.
- Laba rugi.
- Pajak bulan berjalan.
- Approval pending.
- Alert penting.

### 11.2 Halaman Accounting

- COA.
- Journal entries.
- Sales invoice.
- Purchase invoice.
- Payment received.
- Payment paid.
- Expense.
- Bank reconciliation.

### 11.3 Halaman Pajak

- Tax code.
- PPN keluaran.
- PPN masukan.
- PPh dipotong.
- Hutang pajak.
- Pembayaran pajak.
- Rekap masa pajak.
- Upload bukti potong.
- Upload bukti setor pajak.

### 11.4 Halaman AI Review

- Draft dari AI.
- Hasil OCR.
- Koreksi field invoice.
- Confidence score.
- Riwayat chat.
- Riwayat tool call.

### 11.5 Halaman Approval

- Daftar transaksi menunggu approval.
- Detail jurnal yang akan diposting.
- Approve.
- Reject.
- Request revision.
- Catatan reviewer.

---

## 12. Backend Rust (Axum)

### 12.1 Layer Backend

Struktur disarankan:

```text
API Layer (Axum handlers, middleware, DTOs)
Application Service Layer
Domain Layer
Infrastructure Layer (SQLx, storage, external API clients)
Worker Layer (Tokio async background tasks)
```

### 12.2 Modul Backend

- Auth service.
- User service.
- Accounting service.
- Invoice service.
- Payment service.
- Journal service.
- Tax service.
- Report service.
- AI orchestration service.
- OCR service.
- Approval service.
- Audit log service.
- Notification service.

### 12.3 API Endpoint Contoh

```text
POST /api/auth/login
GET  /api/dashboard/summary

GET  /api/accounts
POST /api/accounts

POST /api/sales-invoices/draft
POST /api/sales-invoices/{id}/approve
POST /api/sales-invoices/{id}/post

POST /api/purchase-invoices/draft
POST /api/purchase-invoices/{id}/approve
POST /api/purchase-invoices/{id}/post

POST /api/payments/received/draft
POST /api/payments/paid/draft

POST /api/journals/draft
POST /api/journals/{id}/approve
POST /api/journals/{id}/post

GET  /api/reports/cash-position
GET  /api/reports/accounts-receivable
GET  /api/reports/accounts-payable
GET  /api/reports/profit-loss
GET  /api/reports/balance-sheet
GET  /api/reports/cashflow

GET  /api/tax/vat-output
GET  /api/tax/vat-input
GET  /api/tax/withholding
GET  /api/tax/payable
POST /api/tax/payments

POST /api/ai/query
POST /api/ai/draft-transaction
POST /api/ai/upload-invoice
```

---

## 13. Database PostgreSQL

### 13.1 Tabel Master

```text
companies
branches
departments
users
roles
user_roles
customers
suppliers
chart_of_accounts
bank_accounts
products
tax_codes
fiscal_periods
```

### 13.2 Tabel Transaksi

```text
sales_invoices
sales_invoice_lines
purchase_invoices
purchase_invoice_lines
payments
payment_allocations
expenses
journal_entries
journal_lines
bank_transactions
bank_reconciliations
```

### 13.3 Tabel Pajak

```text
tax_codes
tax_transactions
vat_outputs
vat_inputs
withholding_taxes
tax_payables
tax_payments
tax_documents
tax_periods
```

### 13.4 Tabel AI dan Audit

```text
ai_conversations
ai_messages
ai_tool_calls
ai_draft_transactions
uploaded_documents
ocr_results
approval_requests
approval_histories
audit_logs
system_settings
```

### 13.5 Struktur Journal Entry

Tabel `journal_entries`:

```text
id
company_id
transaction_date
posting_date
source_module
source_id
journal_number
description
status
created_by
approved_by
posted_by
created_at
approved_at
posted_at
```

Tabel `journal_lines`:

```text
id
journal_entry_id
account_id
debit
credit
department_id
customer_id
supplier_id
tax_code_id
description
```

Validasi wajib:

```text
Total debit = total credit
```

---

## 14. Reporting

### 14.1 Laporan Keuangan

- Neraca.
- Laba rugi.
- Arus kas.
- Buku besar.
- Trial balance.
- Jurnal umum.
- Aging piutang.
- Aging hutang.
- Mutasi kas dan bank.

### 14.2 Laporan Pajak

- PPN keluaran.
- PPN masukan.
- Rekap PPN per masa pajak.
- Selisih PPN kurang/lebih bayar.
- PPh 21.
- PPh 23.
- PPh final.
- Hutang pajak.
- Pembayaran pajak.
- Daftar bukti potong.
- Daftar bukti setor.

### 14.3 Laporan AI

- Pertanyaan owner.
- Jawaban AI.
- Draft transaksi dari AI.
- Draft yang disetujui.
- Draft yang ditolak.
- Error AI/OCR.
- Confidence score OCR.

---

## 15. Security Requirement

Minimal keamanan:

- HTTPS wajib.
- JWT authentication.
- Refresh token.
- Role Based Access Control.
- Whitelist nomor WhatsApp/Telegram.
- Audit log tidak boleh dihapus user biasa.
- Approval untuk semua transaksi penting.
- Password hashing.
- Rate limit API.
- Backup otomatis.
- Enkripsi environment variable.
- File invoice tidak boleh public.
- Log akses dokumen.
- Lock period accounting.
- Reversal, bukan edit langsung, untuk transaksi posted.

---

## 16. Backup dan Recovery

Requirement:

- Backup database harian.
- Backup file invoice dan bukti transaksi.
- Retensi backup minimal 30 hari.
- Backup bulanan disimpan terpisah.
- Restore test berkala.
- Export laporan ke Excel/PDF.

---

## 17. Roadmap Pengembangan

### Tahap 0 — Discovery dan Audit Data

Tujuan:

- Memahami proses accounting client saat ini.
- Mengumpulkan semua file Excel.
- Menganalisis struktur data.
- Menentukan COA awal.
- Menentukan kebutuhan pajak.
- Menentukan alur approval.

Output:

- Dokumen proses bisnis.
- Mapping Excel ke database.
- Draft COA.
- Daftar laporan yang dibutuhkan.
- Scope MVP.

---

### Tahap 1 — Foundation Sistem Accounting

Fitur:

- Setup Rust (Axum) backend.
- Setup PostgreSQL.
- Setup Vue.js + Pinia.
- Auth dan role.
- Company profile.
- COA.
- Customer.
- Supplier.
- Bank account.
- Fiscal period.

Output:

- Dashboard admin dasar.
- Master data siap digunakan.
- Struktur database accounting tersedia.

---

### Tahap 2 — Core Double-Entry Accounting

Fitur:

- Journal entry.
- Journal lines.
- Validasi debit kredit.
- Posting transaksi.
- Reversal journal.
- Lock period.
- Buku besar.
- Trial balance.

Output:

- Sistem sudah bisa mencatat transaksi berbasis jurnal.
- Laporan dasar mulai bisa dihasilkan.

---

### Tahap 3 — Sales, Purchase, Payment

Fitur:

- Sales invoice.
- Purchase invoice.
- Payment received.
- Payment paid.
- Expense.
- Piutang.
- Hutang.
- Status invoice.
- Matching pembayaran.

Output:

- Proses invoice, hutang, piutang, dan pembayaran berjalan.
- Jurnal otomatis dari invoice dan payment.

---

### Tahap 4 — Modul Pajak

Fitur:

- Master tax code.
- PPN keluaran.
- PPN masukan.
- PPh dipotong customer.
- PPh yang dipotong perusahaan.
- Hutang pajak.
- Pembayaran pajak.
- Bukti potong.
- Bukti setor.
- Rekap masa pajak.

Output:

- Pajak tercatat dalam transaksi accounting.
- Laporan pajak dasar tersedia.
- Pajak bisa ditanyakan melalui AI.

---

### Tahap 5 — Reporting Dashboard

Fitur:

- Dashboard kas.
- Neraca.
- Laba rugi.
- Cashflow.
- Aging piutang.
- Aging hutang.
- Laporan pajak.
- Export Excel/PDF.

Output:

- Owner dan finance dapat melihat laporan dari dashboard.

---

### Tahap 6 — OpenClaw + AI Query Assistant

Fitur:

- Integrasi OpenClaw.
- Integrasi Telegram/WhatsApp.
- Query laporan via chat.
- Tanya kas, hutang, piutang, laba, beban, pajak.
- Laporan harian otomatis.
- Alert piutang/hutang jatuh tempo.

Output:

- Owner dapat bertanya kondisi keuangan lewat chat.

---

### Tahap 7 — AI Draft Transaction

Fitur:

- AI membuat draft sales invoice.
- AI membuat draft purchase invoice.
- AI membuat draft payment.
- AI membuat draft journal.
- AI membuat draft transaksi pajak.
- Approval via dashboard.
- Approval via chat.

Output:

- AI membantu input transaksi, tetapi tetap melalui approval.

---

### Tahap 8 — OCR Invoice dan Document Automation

Fitur:

- Upload invoice PDF/foto.
- OCR dokumen.
- Ekstraksi vendor/customer.
- Ekstraksi tanggal invoice.
- Ekstraksi DPP, PPN, PPh, total.
- Deteksi duplikasi invoice.
- Confidence score.
- Review hasil OCR.

Output:

- Input invoice lebih cepat dan semi-otomatis.

---

### Tahap 9 — Rekonsiliasi Bank dan Automation Lanjutan

Fitur:

- Import mutasi bank.
- Matching otomatis pembayaran dengan invoice.
- Deteksi transaksi belum tercatat.
- Saran jurnal otomatis.
- Alert cashflow.
- Forecast kas.

Output:

- Sistem mulai mendekati AI CFO Assistant yang proaktif.

---

### Tahap 10 — Hardening dan Production

Fitur:

- Security audit.
- Backup automation.
- Monitoring.
- Error alert.
- Performance optimization.
- User training.
- SOP operasional.

Output:

- Sistem siap digunakan operasional penuh.

---

## 18. MVP yang Disarankan

Agar proyek tidak terlalu besar di awal, MVP sebaiknya mencakup:

1. Master data.
2. COA.
3. Journal entry.
4. Sales invoice.
5. Purchase invoice.
6. Payment received.
7. Payment paid.
8. PPN masukan dan keluaran.
9. Hutang dan piutang.
10. Dashboard kas, hutang, piutang, laba rugi sederhana.
11. OpenClaw untuk tanya laporan via Telegram.
12. AI membuat draft transaksi sederhana.
13. Approval workflow.
14. Audit log.

WhatsApp dan OCR invoice bisa masuk setelah MVP stabil.

---

## 19. Batasan Penting

- Sistem ini bukan pengganti konsultan pajak.
- Perhitungan pajak harus divalidasi oleh finance/accounting/tax officer.
- AI tidak boleh diberi akses untuk menghapus transaksi.
- AI tidak boleh posting final tanpa approval.
- Perubahan pada transaksi posted harus memakai reversal journal.
- Format laporan pajak resmi perlu disesuaikan dengan regulasi yang berlaku.

---

## 20. Kesimpulan

Dengan stack **OpenClaw + Rust (Axum) + Vue.js + Pinia + PostgreSQL**, sistem dapat dibangun sebagai solusi lengkap:

```text
Custom Accounting System
        +
AI Accounting Automation Assistant
        +
WhatsApp/Telegram Financial Query
        +
Tax Recording Module
        +
Approval & Audit Trail
```

Prioritas utama bukan membuat AI yang bebas melakukan pencatatan, tetapi membuat sistem accounting yang benar terlebih dahulu, lalu AI digunakan untuk mempercepat input, membaca dokumen, membuat draft, memberikan analisis, dan menjawab pertanyaan owner secara cepat dan aman.
