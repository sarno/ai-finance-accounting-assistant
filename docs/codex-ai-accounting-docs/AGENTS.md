# AGENTS.md — Instructions for Codex / Coding Agent

This repository contains a custom **AI Finance & Accounting Assistant** using:

- Rust (Axum Web Framework)
- PostgreSQL
- SQLx (async, compile-time checked queries with Rust)
- Vue.js 3 + Pinia
- OpenClaw as chat/AI automation gateway

## Primary rule

Build a reliable accounting system first. AI must only create **drafts**, call approved backend APIs, and ask for approval before posting.

Never implement direct AI-to-database writes for posted accounting records.

## Project layout target

```text
/apps
  /api              ASP.NET Core backend
  /web              Vue.js frontend
  /openclaw         OpenClaw config, agent prompts, workflows
/docs               Product and technical docs
/infra              Docker, nginx/caddy, deployment scripts
/tests              E2E and integration tests
```

## Backend conventions

- Use clean architecture style:

```text
Api (Axum handlers, middleware, DTOs)
Application (services, commands, queries, validators)
Domain (entities, value objects, domain rules)
Infrastructure (SQLx repositories, storage, external clients, background jobs)
```

- Use PostgreSQL with SQLx migrations (versioned `.sql` files in `migrations/`).
- Use snake_case database table and column names.
- Use `rust_decimal::Decimal` for all money values, never f32/f64.
- Store currency code explicitly, default `IDR`.
- Use UTC timestamps in database (stored as `TIMESTAMPTZ`).
- Use soft delete only for master data; do not delete posted accounting documents.
- Use immutable audit logs.

## Accounting rules

- Every posted transaction must create balanced journal entries.
- Debit total must equal credit total.
- Posted journal entries cannot be edited directly.
- Corrections must be made through reversal or adjustment journal.
- Drafts can be edited before approval.
- Posting requires valid status transition.

## AI rules

- AI can:
  - query reports through read-only APIs;
  - create draft invoice/payment/journal suggestions;
  - explain results;
  - request approval.
- AI cannot:
  - bypass authentication;
  - post final transactions without approval;
  - mutate master tax rules without admin approval;
  - infer missing critical values silently.

## Frontend conventions

- Use Vue 3 Composition API.
- Use Pinia stores per domain module.
- Use typed API client.
- Keep pages modular and reusable.
- Use form validation for accounting documents.
- Show clear status labels: Draft, WaitingApproval, Approved, Posted, Rejected, Cancelled.

## Testing requirements

At minimum implement:

- Unit tests for journal balancing.
- Unit tests for tax calculation config.
- Integration tests for posting transactions.
- API tests for permissions.
- E2E tests for invoice draft -> approval -> posting.

## Security requirements

- Use JWT access token and refresh token.
- Implement RBAC.
- Whitelist OpenClaw webhook/API key.
- Log every AI tool call.
- Validate all AI-generated payloads server-side.
- Add idempotency key for create/post APIs.

## Definition of done

A task is done only when:

1. Code compiles.
2. Tests pass.
3. API contract is documented or updated.
4. Migration is included when database changes.
5. Permission and audit logging are handled for sensitive actions.
6. The feature follows status workflow and accounting invariants.
