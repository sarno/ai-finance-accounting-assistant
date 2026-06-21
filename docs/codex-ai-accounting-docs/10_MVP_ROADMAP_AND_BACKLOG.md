# MVP Roadmap and Backlog

## Phase 0 — Project setup

Deliverables:

- Monorepo structure.
- Rust Cargo workspace with Axum API crate.
- Vue.js app.
- Docker Compose.
- PostgreSQL connection (SQLx PgPool).
- SQLx migrations baseline.
- CI test command (`cargo test`).

Acceptance:

- `docker compose up` runs API, web, and database.
- API health check works.

## Phase 1 — Accounting foundation

Deliverables:

- Company setup.
- Auth + RBAC.
- Chart of Accounts.
- Customer/supplier.
- Bank/cash accounts.
- Journal entry draft/post.
- Balanced journal validation.

Acceptance:

- User can create COA.
- User can create balanced journal draft.
- User cannot post unbalanced journal.
- Posted journal appears in ledger.

## Phase 2 — Sales and purchase

Deliverables:

- Sales invoice draft.
- Purchase invoice draft.
- Invoice lines.
- Tax line calculation from config.
- Submit approval.
- Approval/rejection.
- Posting to journal.

Acceptance:

- Sales invoice creates AR journal.
- Purchase invoice creates AP journal.
- Approval required before posting.

## Phase 3 — Payments

Deliverables:

- Payment received.
- Payment paid.
- Allocation to invoice.
- Partial payment.
- AR/AP balance update through journal.

Acceptance:

- Payment received reduces AR.
- Payment paid reduces AP.
- Partial payment shows outstanding balance.

## Phase 4 — Tax module

Deliverables:

- Tax type config with effective dates.
- PPN Masukan/Keluaran records.
- PPh withholding records.
- Tax summary report.
- Tax calendar and reminders.
- Missing tax document alerts.

Acceptance:

- Tax summary uses posted transactions.
- Rates come from database config.
- Due dates can be managed.

## Phase 5 — Reports

Deliverables:

- Cash position.
- AR aging.
- AP aging.
- Profit/loss.
- Trial balance.
- General ledger.
- Tax summary.

Acceptance:

- Owner can see core finance reports.
- Report numbers reconcile with journals.

## Phase 6 — AI and OpenClaw integration

Deliverables:

- OpenClaw agent prompt.
- Backend AI tool endpoints.
- Query report from chat.
- Create draft invoice from chat.
- Approval command from chat.
- AI tool call audit log.

Acceptance:

- Owner can ask cash via chat.
- AI creates draft only.
- Approval command checks permission.

## Phase 7 — OCR/document automation

Deliverables:

- Upload invoice document.
- OCR extraction.
- AI field extraction.
- Confidence score.
- Validation checklist.
- Draft purchase invoice from extraction.

Acceptance:

- Finance can upload invoice.
- AI suggests draft and journal.
- User can correct before approval.

## Phase 8 — Hardening

Deliverables:

- Backup job.
- Logging dashboard.
- Rate limit.
- Security review.
- Audit export.
- Production deployment guide.

Acceptance:

- Restore backup tested.
- Sensitive endpoints require permission.
- Audit log covers critical actions.

## MVP priority backlog

### Must have

- Auth/RBAC
- COA
- Journal engine
- Sales invoice
- Purchase invoice
- Payment
- Approval
- Posting
- Basic reports
- Tax config + tax summary
- OpenClaw report query
- AI draft transaction

### Should have

- OCR invoice extraction
- Chat approval
- Tax calendar
- AR/AP aging
- Dashboard charts

### Could have

- Bank reconciliation
- Inventory
- Payroll/PPh 21 detailed payroll
- Direct integration with official tax service/PJAP
- Multi-company
