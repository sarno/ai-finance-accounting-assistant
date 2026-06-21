# Codex Task Breakdown

Use this file to implement the project incrementally.

## Task 001 — Create monorepo structure

Create:

```text
/apps/api      (Rust backend, Cargo workspace)
/apps/web      (Vue.js frontend)
/apps/openclaw (OpenClaw config and agent prompts)
/docs
/infra
/tests
```

Add README, Cargo.toml workspace manifest, and solution files.

## Task 002 — Bootstrap Rust workspace

Create Cargo workspace with crates:

```text
finance_assistant_api      (Axum handlers, routing, middleware)
finance_assistant_app      (Application services, commands, queries)
finance_assistant_domain   (Domain entities, value objects, domain rules)
finance_assistant_infra    (SQLx repos, storage, external API clients)
finance_assistant_worker   (Background Tokio tasks and scheduled jobs)
```

Add references according to clean architecture (domain has no external deps).

## Task 003 — Add PostgreSQL SQLx setup

- Add `sqlx` with PostgreSQL and `tokio` runtime features.
- Create async `PgPool` connection pool via `sqlx::postgres::PgPoolOptions`.
- Configure snake_case column naming (matches Postgres default).
- Add initial migration file in `migrations/` directory.
- Add health check endpoint that pings the database pool.

## Task 004 — Implement auth and RBAC

- Users, roles, permissions.
- JWT login.
- Refresh token.
- Authorization policy.

## Task 005 — Implement master data

- Company
- COA
- Customers
- Suppliers
- Bank accounts
- Tax types

Include CRUD APIs and Vue pages.

## Task 006 — Implement journal engine

- Journal entry entity.
- Journal line entity.
- Draft journal endpoint.
- Validate balanced debit/credit.
- Posting endpoint.
- Immutable posted journals.
- Unit tests.

## Task 007 — Implement approval workflow

- Approval request.
- Approval actions.
- Submit approval.
- Approve/reject.
- Permission checks.
- Audit log.

## Task 008 — Implement sales invoice

- Sales invoice draft.
- Sales invoice lines.
- Tax calculation from config.
- Submit approval.
- Posting to AR/revenue/tax journal.
- Tests.

## Task 009 — Implement purchase invoice

- Purchase invoice draft.
- Purchase invoice lines.
- Tax calculation.
- Duplicate detection.
- Submit approval.
- Posting to expense/AP/tax journal.
- Tests.

## Task 010 — Implement payments

- Payment received.
- Payment paid.
- Allocation to invoice.
- Partial payments.
- Posting journal.
- Tests.

## Task 011 — Implement tax module

- Tax records.
- Tax summary report.
- Tax calendar.
- Missing document alerts.
- Effective-date tax config.

## Task 012 — Implement reports

- Cash position.
- Accounts receivable aging.
- Accounts payable aging.
- Profit/loss.
- Trial balance.
- General ledger.
- Tax summary.

## Task 013 — Build Vue dashboard shell

- Login page.
- Auth store.
- Main layout.
- Sidebar.
- Dashboard cards.
- API client.

## Task 014 — Build transaction pages

- Sales invoices.
- Purchase invoices.
- Payments.
- Journals.
- Approval page.
- Tax page.

## Task 015 — Implement OpenClaw tool endpoints

Backend endpoints:

```text
POST /api/ai/tools/query-report
POST /api/ai/tools/create-draft-invoice
POST /api/ai/tools/create-draft-payment
POST /api/ai/tools/create-draft-journal
POST /api/ai/tools/submit-approval-command
```

Add service token auth and audit log.

## Task 016 — Add OpenClaw agent config

Create:

- system prompt;
- tool definitions;
- report query workflow;
- draft invoice workflow;
- approval command workflow.

## Task 017 — Add document upload and OCR pipeline

- Uploaded documents table.
- Storage provider.
- Upload API.
- Background OCR job.
- AI extraction result.
- Validation checklist.

## Task 018 — Build invoice automation UI

- Document preview.
- Extracted fields panel.
- Validation checklist.
- Suggested journal table.
- Approval workflow panel.

## Task 019 — Add security hardening

- Rate limiting.
- Idempotency middleware.
- Audit log middleware.
- Sensitive data masking.
- Backup job.

## Task 020 — Production deployment

- Dockerfile API (multi-stage Rust build: `cargo build --release`).
- Dockerfile Web.
- Docker Compose.
- Reverse proxy config.
- Environment template.
- Deployment checklist.

## Recommended implementation order

```text
001 -> 002 -> 003 -> 004 -> 005 -> 006 -> 007 -> 008 -> 009 -> 010 -> 011 -> 012 -> 013 -> 014 -> 015 -> 016 -> 017 -> 018 -> 019 -> 020
```
