# System Architecture

## 1. High-level architecture

```text
WhatsApp / Telegram
        |
        v
OpenClaw Gateway
        |
        v
Rust (Axum) REST API
        |
        v
PostgreSQL
        |
        v
Vue.js + Pinia Admin Dashboard
```

Additional services:

```text
Synology NAS / Object Storage  -> invoice attachments, imported Excel files
Background Worker             -> scheduled sync, reminders, reports
Redis                         -> cache / Hangfire storage optional
Reverse Proxy                 -> HTTPS, routing
```

## 2. Component responsibility

### OpenClaw

Responsibilities:

- receive messages from WhatsApp/Telegram;
- route messages to AI agent;
- call backend APIs as tools;
- send responses back to user;
- handle approval commands from chat.

OpenClaw is **not** responsible for final accounting logic.

### Rust (Axum) API

Responsibilities:

- authentication and authorization;
- accounting business rules;
- tax calculation/configuration;
- draft transaction creation;
- approval workflow;
- posting engine;
- report APIs;
- audit log;
- AI tool endpoint validation.

### Vue.js + Pinia Dashboard

Responsibilities:

- master data management;
- transaction forms;
- document upload;
- review AI extraction results;
- approval UI;
- financial reports;
- tax report views;
- settings.

### PostgreSQL

Responsibilities:

- source of truth for accounting records;
- audit log storage;
- AI interaction history;
- configuration storage.

### Synology NAS / Storage

Responsibilities:

- store uploaded invoice PDFs/images;
- store imported Excel files;
- optionally retain original evidence documents.

## 3. Backend internal architecture

```text
Api Layer (Axum)
  Handlers (route functions)
  Auth Middleware (JWT extractor)
  Webhook Endpoints
  DTOs (serde Serialize/Deserialize structs)

Application Layer
  Services
  Commands
  Queries
  Validators
  Report Builders

Domain Layer
  Entities
  Value Objects
  Domain Rules
  Accounting Posting Rules

Infrastructure Layer
  SQLx repositories (async queries)
  Storage Provider
  OpenAI/OCR Clients (reqwest)
  Email/Notification
  Background Jobs (Tokio tasks / cron)
```

## 4. Recommended backend projects (Rust workspace)

```text
src/
  finance_assistant_api/      # Axum handlers, routing, middleware
  finance_assistant_app/      # Application services, commands, queries
  finance_assistant_domain/   # Domain entities, value objects, rules
  finance_assistant_infra/    # SQLx repos, storage, external clients
  finance_assistant_worker/   # Background Tokio tasks and cron jobs

tests/
  unit_tests/
  integration_tests/
```

## 5. Data flow: ask report via chat

```text
Owner -> WhatsApp/Telegram -> OpenClaw -> AI Agent
AI Agent -> POST /api/ai/query or direct report endpoint
Backend -> query PostgreSQL
Backend -> return structured result
AI Agent -> format answer in Indonesian
OpenClaw -> owner
```

Important:

- AI formats answer, but numbers come from backend.
- Backend returns JSON with calculated report values.

## 6. Data flow: invoice automation

```text
Finance uploads invoice
        |
        v
OpenClaw or Web Dashboard receives document
        |
        v
Backend stores file
        |
        v
OCR / document extraction
        |
        v
AI extracts invoice fields
        |
        v
Backend validates fields, vendor, tax, duplicate invoice
        |
        v
Draft purchase invoice created
        |
        v
Approval request generated
        |
        v
Reviewer approves
        |
        v
Posting engine creates journal
```

## 7. Deployment topology MVP

```text
Single VPS / server
  - nginx/caddy
  - api container
  - web container
  - postgres container or managed postgres
  - openclaw container/process
  - redis container optional
  - worker container
  - mounted storage to Synology or object storage
```

## 8. Environment separation

Use separate environments:

- local
- staging
- production

Each environment must have separate:

- database;
- API keys;
- OpenClaw channel config;
- storage bucket/path;
- JWT secrets.

## 9. Key architecture decisions

### ADR-001: AI creates drafts only

AI output is untrusted until validated and approved.

### ADR-002: Journal is the source of financial reports

Financial reports should be generated from posted journal entries, not from invoice tables alone.

### ADR-003: Tax rules configurable

Tax rates and tax mapping must be stored in database with effective dates.

### ADR-004: Immutable posted documents

Posted accounting documents cannot be directly edited.

### ADR-005: OpenClaw is gateway, not accounting brain

Business logic remains in the Rust backend.
