# AI Finance & Accounting Assistant

Sistem accounting custom berbasis AI untuk manajemen keuangan perusahaan.

## Tech Stack

| Layer | Teknologi |
|---|---|
| Backend API | Rust + Axum |
| Frontend | Vue.js 3 + Pinia + TypeScript |
| Database | PostgreSQL |
| DB Driver | SQLx (async, compile-time checked) |
| Background Jobs | Tokio async worker |
| Cache / Queue | Redis |
| AI Gateway | OpenClaw |
| Deployment | Docker Compose |

## Struktur Monorepo

```
/
├── apps/
│   ├── api/                  # Rust backend (Axum)
│   │   ├── crates/
│   │   │   ├── api/          # HTTP handlers, routing, middleware
│   │   │   ├── application/  # Services, commands, queries
│   │   │   ├── domain/       # Entities, value objects, domain rules
│   │   │   ├── infrastructure/ # SQLx repos, storage, external clients
│   │   │   └── worker/       # Background Tokio tasks
│   │   └── migrations/       # SQL migration files
│   └── web/                  # Vue.js 3 frontend
│       ├── src/
│       │   ├── api/          # Typed API client
│       │   ├── components/   # Reusable components
│       │   ├── composables/  # Vue composables
│       │   ├── layouts/      # Page layouts
│       │   ├── pages/        # Route pages
│       │   ├── stores/       # Pinia stores
│       │   ├── types/        # TypeScript types
│       │   └── utils/        # Utility functions
│       └── public/
├── infra/                    # Docker, nginx config
├── docs/                     # Product & technical docs
└── tests/                    # E2E / integration tests
```

## Quick Start (Development)

```bash
# 1. Copy environment
cp .env.example .env

# 2. Start infrastructure
docker compose -f infra/docker-compose.dev.yml up -d postgres redis

# 3. Run DB migrations
cd apps/api && cargo sqlx migrate run

# 4. Start Rust API
cargo run -p finance_assistant_api

# 5. Start Vue frontend (separate terminal)
cd apps/web && npm install && npm run dev
```

## Prinsip Arsitektur

- AI hanya membuat **draft transaksi** — tidak boleh langsung posting.
- Semua angka laporan berasal dari database, bukan dari AI.
- Setiap aksi penting dicatat di audit log.
- Tax rules configurable (stored in DB with effective dates).
- Posting requires approval workflow.
# ai-finance-accounting-assistant
