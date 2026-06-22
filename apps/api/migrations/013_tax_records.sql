-- Migration 013: Create Tax Records and Tax Calendar Tables

CREATE TABLE tax_records (
    id                     UUID PRIMARY KEY,
    company_id             UUID NOT NULL REFERENCES companies(id),
    tax_type_id            UUID NOT NULL REFERENCES tax_types(id),
    source_document_type   TEXT NOT NULL, -- 'sales_invoice', 'purchase_invoice', etc.
    source_document_id     UUID NOT NULL,
    tax_period             DATE NOT NULL,
    tax_base_amount        NUMERIC(18, 2) NOT NULL,
    tax_rate               NUMERIC(9, 6) NOT NULL,
    tax_amount             NUMERIC(18, 2) NOT NULL,
    status                 TEXT NOT NULL, -- 'drafted', 'validated', 'reported', 'paid', 'archived', 'not_required', 'required'
    counterparty_name      TEXT,
    counterparty_npwp      TEXT,
    created_at             TIMESTAMPTZ NOT NULL,
    updated_at             TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_tax_records_company_period ON tax_records(company_id, tax_period);
CREATE INDEX idx_tax_records_source_document ON tax_records(source_document_id);

CREATE TABLE tax_calendar (
    id                     UUID PRIMARY KEY,
    company_id             UUID NOT NULL REFERENCES companies(id),
    tax_type_id            UUID NOT NULL REFERENCES tax_types(id),
    tax_period             DATE NOT NULL,
    payment_due_date       DATE NOT NULL,
    filing_due_date        DATE NOT NULL,
    payment_status         TEXT NOT NULL DEFAULT 'unpaid', -- 'unpaid', 'paid'
    filing_status          TEXT NOT NULL DEFAULT 'unfiled', -- 'unfiled', 'filed'
    reminder_sent_at       TIMESTAMPTZ,
    created_at             TIMESTAMPTZ NOT NULL,
    updated_at             TIMESTAMPTZ NOT NULL,
    UNIQUE(company_id, tax_type_id, tax_period)
);

CREATE INDEX idx_tax_calendar_company_period ON tax_calendar(company_id, tax_period);
