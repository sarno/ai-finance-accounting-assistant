# AI Finance & Accounting Assistant — Codex Implementation Docs

Dokumen ini disiapkan untuk membantu Codex / coding agent membangun proyek **AI Finance & Accounting Assistant** menjadi aplikasi nyata.

## Stack yang digunakan

- **Chat/AI Gateway**: OpenClaw
- **Backend**: Rust (Axum Web Framework)
- **Frontend**: Vue.js 3 + Pinia
- **Database**: PostgreSQL
- **Database Driver**: SQLx (async, compile-time checked queries)
- **Background Job**: Tokio task / background worker dengan Rust async runtime
- **Deployment**: Docker Compose
- **Storage Dokumen**: Synology NAS / S3-compatible storage

## Tujuan produk

Membangun sistem accounting custom yang dapat:

1. Mencatat transaksi keuangan perusahaan.
2. Membuat draft invoice, pembayaran, jurnal, dan transaksi pajak dengan bantuan AI.
3. Menyediakan chatbot via WhatsApp/Telegram untuk owner dan finance.
4. Menyediakan dashboard web untuk review, approval, posting, dan laporan.
5. Menjaga keamanan melalui approval workflow, audit log, dan role-based access.

## Urutan membaca dokumen

1. `AGENTS.md` — instruksi kerja untuk Codex/coding agent.
2. `01_PRODUCT_REQUIREMENTS.md` — kebutuhan bisnis dan scope produk.
3. `02_SYSTEM_ARCHITECTURE.md` — arsitektur teknis.
4. `03_ACCOUNTING_AND_TAX_DOMAIN.md` — aturan domain accounting dan pajak.
5. `04_DATABASE_SCHEMA.md` — desain database awal.
6. `05_API_CONTRACT.md` — daftar endpoint backend.
7. `06_OPENCLAW_AI_WORKFLOWS.md` — workflow OpenClaw dan AI agent.
8. `07_FRONTEND_SPEC.md` — requirement UI Vue.js + Pinia.
9. `08_SECURITY_AND_AUDIT.md` — keamanan, audit, permission.
10. `09_DEVOPS_DEPLOYMENT.md` — deployment Docker.
11. `10_MVP_ROADMAP_AND_BACKLOG.md` — roadmap dan backlog.
12. `11_ACCEPTANCE_TESTS.md` — acceptance criteria dan test plan.
13. `12_CODEX_TASKS.md` — task konkret untuk Codex.

## Prinsip utama

- AI **tidak boleh langsung posting transaksi final** tanpa validasi dan approval.
- Semua angka laporan harus berasal dari database, bukan hasil karangan AI.
- Semua aksi penting wajib memiliki audit log.
- Tarif pajak, akun pajak, dan aturan pajak wajib dibuat configurable, bukan hardcoded.
- Sistem accounting core tetap berada di Rust backend, sedangkan OpenClaw hanya gateway/automation layer.

## Output MVP

Target MVP pertama:

- Auth dan role sederhana.
- Master COA, customer, supplier, bank account.
- Sales invoice, purchase invoice, payment, journal entry.
- Draft transaction + approval + posting.
- Basic reports: cash position, AR, AP, profit/loss, tax summary.
- Chatbot OpenClaw untuk query laporan dan membuat draft transaksi.
