-- Migration 001: Initial schema
-- All tables use snake_case naming, UUID primary keys, TIMESTAMPTZ for timestamps.

-- ─── Types ────────────────────────────────────────────────────────────────────
CREATE TYPE document_status AS ENUM (
    'draft',
    'waiting_review',
    'waiting_approval',
    'approved',
    'posted',
    'rejected',
    'cancelled'
);

-- ─── Companies ────────────────────────────────────────────────────────────────
CREATE TABLE companies (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT NOT NULL,
    tax_number  TEXT,
    address     TEXT,
    currency    TEXT NOT NULL DEFAULT 'IDR',
    is_active   BOOLEAN NOT NULL DEFAULT TRUE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ─── Users ────────────────────────────────────────────────────────────────────
CREATE TABLE users (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id      UUID NOT NULL REFERENCES companies(id),
    email           TEXT NOT NULL UNIQUE,
    full_name       TEXT NOT NULL,
    password_hash   TEXT NOT NULL,
    is_active       BOOLEAN NOT NULL DEFAULT TRUE,
    last_login_at   TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE user_roles (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role        TEXT NOT NULL,
    UNIQUE(user_id, role)
);

-- ─── Chart of Accounts ────────────────────────────────────────────────────────
CREATE TABLE chart_of_accounts (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id      UUID NOT NULL REFERENCES companies(id),
    code            TEXT NOT NULL,
    name            TEXT NOT NULL,
    account_type    TEXT NOT NULL, -- Asset, Liability, Equity, Revenue, Expense
    parent_id       UUID REFERENCES chart_of_accounts(id),
    is_active       BOOLEAN NOT NULL DEFAULT TRUE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(company_id, code)
);

-- ─── Customers & Suppliers ────────────────────────────────────────────────────
CREATE TABLE customers (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id  UUID NOT NULL REFERENCES companies(id),
    name        TEXT NOT NULL,
    tax_number  TEXT,
    email       TEXT,
    phone       TEXT,
    address     TEXT,
    is_active   BOOLEAN NOT NULL DEFAULT TRUE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE suppliers (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id  UUID NOT NULL REFERENCES companies(id),
    name        TEXT NOT NULL,
    tax_number  TEXT,
    email       TEXT,
    phone       TEXT,
    address     TEXT,
    is_active   BOOLEAN NOT NULL DEFAULT TRUE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ─── Bank Accounts ────────────────────────────────────────────────────────────
CREATE TABLE bank_accounts (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id      UUID NOT NULL REFERENCES companies(id),
    account_id      UUID NOT NULL REFERENCES chart_of_accounts(id),
    bank_name       TEXT NOT NULL,
    account_number  TEXT NOT NULL,
    account_name    TEXT NOT NULL,
    currency        TEXT NOT NULL DEFAULT 'IDR',
    is_active       BOOLEAN NOT NULL DEFAULT TRUE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ─── Tax Types ────────────────────────────────────────────────────────────────
CREATE TABLE tax_types (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id          UUID NOT NULL REFERENCES companies(id),
    code                TEXT NOT NULL,
    name                TEXT NOT NULL,
    category            TEXT NOT NULL,
    default_rate        NUMERIC(10, 4) NOT NULL,
    payable_account_id  UUID NOT NULL REFERENCES chart_of_accounts(id),
    effective_from      DATE NOT NULL,
    effective_to        DATE,
    is_active           BOOLEAN NOT NULL DEFAULT TRUE,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(company_id, code, effective_from)
);

-- ─── Journal Entries ──────────────────────────────────────────────────────────
CREATE TABLE journal_entries (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id          UUID NOT NULL REFERENCES companies(id),
    reference_number    TEXT NOT NULL,
    description         TEXT NOT NULL,
    transaction_date    DATE NOT NULL,
    status              document_status NOT NULL DEFAULT 'draft',
    source              TEXT NOT NULL DEFAULT 'manual',
    source_document_id  UUID,
    created_by          UUID NOT NULL REFERENCES users(id),
    posted_by           UUID REFERENCES users(id),
    posted_at           TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE journal_lines (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    journal_entry_id    UUID NOT NULL REFERENCES journal_entries(id) ON DELETE CASCADE,
    account_id          UUID NOT NULL REFERENCES chart_of_accounts(id),
    debit               NUMERIC(20, 2) NOT NULL DEFAULT 0,
    credit              NUMERIC(20, 2) NOT NULL DEFAULT 0,
    description         TEXT,
    sort_order          INTEGER NOT NULL DEFAULT 0,
    CONSTRAINT chk_debit_credit CHECK (
        (debit > 0 AND credit = 0) OR (credit > 0 AND debit = 0)
    )
);

-- ─── Sales Invoices ───────────────────────────────────────────────────────────
CREATE TABLE sales_invoices (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id          UUID NOT NULL REFERENCES companies(id),
    invoice_number      TEXT NOT NULL,
    customer_id         UUID NOT NULL REFERENCES customers(id),
    invoice_date        DATE NOT NULL,
    due_date            DATE NOT NULL,
    subtotal            NUMERIC(20, 2) NOT NULL DEFAULT 0,
    tax_amount          NUMERIC(20, 2) NOT NULL DEFAULT 0,
    total_amount        NUMERIC(20, 2) NOT NULL DEFAULT 0,
    status              document_status NOT NULL DEFAULT 'draft',
    notes               TEXT,
    journal_entry_id    UUID REFERENCES journal_entries(id),
    created_by          UUID NOT NULL REFERENCES users(id),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(company_id, invoice_number)
);

CREATE TABLE sales_invoice_lines (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sales_invoice_id    UUID NOT NULL REFERENCES sales_invoices(id) ON DELETE CASCADE,
    description         TEXT NOT NULL,
    quantity            NUMERIC(20, 4) NOT NULL,
    unit_price          NUMERIC(20, 2) NOT NULL,
    discount_amount     NUMERIC(20, 2) NOT NULL DEFAULT 0,
    tax_type_id         UUID REFERENCES tax_types(id),
    tax_rate            NUMERIC(10, 4),
    tax_amount          NUMERIC(20, 2) NOT NULL DEFAULT 0,
    line_total          NUMERIC(20, 2) NOT NULL,
    account_id          UUID NOT NULL REFERENCES chart_of_accounts(id),
    sort_order          INTEGER NOT NULL DEFAULT 0
);

-- ─── Purchase Invoices ────────────────────────────────────────────────────────
CREATE TABLE purchase_invoices (
    id                          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id                  UUID NOT NULL REFERENCES companies(id),
    supplier_invoice_number     TEXT NOT NULL,
    internal_reference          TEXT NOT NULL,
    supplier_id                 UUID NOT NULL REFERENCES suppliers(id),
    invoice_date                DATE NOT NULL,
    due_date                    DATE NOT NULL,
    subtotal                    NUMERIC(20, 2) NOT NULL DEFAULT 0,
    tax_amount                  NUMERIC(20, 2) NOT NULL DEFAULT 0,
    total_amount                NUMERIC(20, 2) NOT NULL DEFAULT 0,
    status                      document_status NOT NULL DEFAULT 'draft',
    ai_confidence               NUMERIC(5, 2),
    uploaded_document_id        UUID,
    journal_entry_id            UUID REFERENCES journal_entries(id),
    notes                       TEXT,
    created_by                  UUID NOT NULL REFERENCES users(id),
    created_at                  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at                  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ─── Payments ─────────────────────────────────────────────────────────────────
CREATE TABLE payments (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id          UUID NOT NULL REFERENCES companies(id),
    reference_number    TEXT NOT NULL,
    payment_type        TEXT NOT NULL, -- payment_received, payment_paid
    counterparty_type   TEXT NOT NULL, -- customer, supplier
    counterparty_id     UUID NOT NULL,
    payment_date        DATE NOT NULL,
    bank_account_id     UUID NOT NULL REFERENCES bank_accounts(id),
    amount              NUMERIC(20, 2) NOT NULL,
    status              document_status NOT NULL DEFAULT 'draft',
    notes               TEXT,
    journal_entry_id    UUID REFERENCES journal_entries(id),
    created_by          UUID NOT NULL REFERENCES users(id),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE payment_allocations (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    payment_id          UUID NOT NULL REFERENCES payments(id) ON DELETE CASCADE,
    document_type       TEXT NOT NULL,
    document_id         UUID NOT NULL,
    allocated_amount    NUMERIC(20, 2) NOT NULL
);

-- ─── Approval Requests ────────────────────────────────────────────────────────
CREATE TABLE approval_requests (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id      UUID NOT NULL REFERENCES companies(id),
    document_type   TEXT NOT NULL,
    document_id     UUID NOT NULL,
    status          TEXT NOT NULL DEFAULT 'pending',
    requested_by    UUID NOT NULL REFERENCES users(id),
    reviewed_by     UUID REFERENCES users(id),
    reviewed_at     TIMESTAMPTZ,
    comment         TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ─── Audit Logs ───────────────────────────────────────────────────────────────
CREATE TABLE audit_logs (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id          UUID NOT NULL REFERENCES companies(id),
    actor_user_id       UUID REFERENCES users(id),
    actor_type          TEXT NOT NULL DEFAULT 'user',
    entity_type         TEXT NOT NULL,
    entity_id           UUID NOT NULL,
    action              TEXT NOT NULL,
    before_snapshot     JSONB,
    after_snapshot      JSONB,
    ip_address          TEXT,
    user_agent          TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
    -- NO updated_at — audit logs are immutable
);

-- ─── Indexes ──────────────────────────────────────────────────────────────────
CREATE INDEX idx_journal_entries_company_date   ON journal_entries(company_id, transaction_date DESC);
CREATE INDEX idx_journal_entries_status         ON journal_entries(status);
CREATE INDEX idx_journal_lines_entry            ON journal_lines(journal_entry_id);
CREATE INDEX idx_sales_invoices_company_date    ON sales_invoices(company_id, invoice_date DESC);
CREATE INDEX idx_sales_invoices_customer        ON sales_invoices(customer_id);
CREATE INDEX idx_purchase_invoices_company_date ON purchase_invoices(company_id, invoice_date DESC);
CREATE INDEX idx_purchase_invoices_supplier     ON purchase_invoices(supplier_id);
CREATE INDEX idx_payments_company_date          ON payments(company_id, payment_date DESC);
CREATE INDEX idx_approval_requests_company      ON approval_requests(company_id, status);
CREATE INDEX idx_audit_logs_entity              ON audit_logs(entity_type, entity_id);
CREATE INDEX idx_audit_logs_company_time        ON audit_logs(company_id, created_at DESC);
