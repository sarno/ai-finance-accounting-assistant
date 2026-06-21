-- Migration 006: Item Categories and Items for Invoice Lines

-- 1. Create item_categories table
CREATE TABLE item_categories (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id      UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    description     TEXT,
    is_active       BOOLEAN NOT NULL DEFAULT TRUE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_company_category_name UNIQUE(company_id, name)
);

-- 2. Create items table
CREATE TABLE items (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id          UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    category_id         UUID REFERENCES item_categories(id) ON DELETE SET NULL,
    code                TEXT NOT NULL,
    name                TEXT NOT NULL,
    description         TEXT,
    unit_price          NUMERIC(20, 2) NOT NULL DEFAULT 0,
    sale_account_id     UUID REFERENCES chart_of_accounts(id) ON DELETE SET NULL,
    purchase_account_id UUID REFERENCES chart_of_accounts(id) ON DELETE SET NULL,
    tax_type_id         UUID REFERENCES tax_types(id) ON DELETE SET NULL,
    is_active           BOOLEAN NOT NULL DEFAULT TRUE,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_company_item_code UNIQUE(company_id, code)
);

-- Indexes for performance
CREATE INDEX idx_item_categories_company ON item_categories(company_id);
CREATE INDEX idx_items_company ON items(company_id);
CREATE INDEX idx_items_category ON items(category_id);

-- 3. Seed default Item Categories
INSERT INTO item_categories (id, company_id, name, description)
VALUES 
    (
        'c1000000-0000-0000-0000-000000000001',
        'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
        'Services',
        'Professional IT services, consulting, setup support, and training.'
    ),
    (
        'c1000000-0000-0000-0000-000000000002',
        'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
        'Software Licenses',
        'Enterprise SaaS licenses, cloud platform products, and custom software builds.'
    )
ON CONFLICT (id) DO NOTHING;

-- 4. Seed default Items
INSERT INTO items (id, company_id, category_id, code, name, description, unit_price, sale_account_id, purchase_account_id, tax_type_id)
VALUES
    (
        'd1000000-0000-0000-0000-000000000001',
        'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
        'c1000000-0000-0000-0000-000000000001', -- Services category
        'SRV-CONS',
        'Professional IT Consulting',
        'High-level software architecture design, custom requirements analysis, and system consulting.',
        2000000.00,
        'a0000000-0000-0000-0000-000000000007', -- Sales Revenue Account
        'a0000000-0000-0000-0000-000000000008', -- Office Expenses Account
        'e0000000-0000-0000-0000-000000000001'  -- VAT 11%
    ),
    (
        'd1000000-0000-0000-0000-000000000002',
        'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
        'c1000000-0000-0000-0000-000000000001',
        'SRV-MIGR',
        'Cloud Infrastructure Migration',
        'Complete workload assessment, target landing zone configuration, and migration execution to AWS/GCP.',
        50000000.00,
        'a0000000-0000-0000-0000-000000000007',
        'a0000000-0000-0000-0000-000000000008',
        'e0000000-0000-0000-0000-000000000001'
    ),
    (
        'd1000000-0000-0000-0000-000000000003',
        'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
        'c1000000-0000-0000-0000-000000000002', -- Licenses category
        'LIC-ERP-ENT',
        'ERP Platform Enterprise License',
        'Enterprise customization license for PT Solusi Codex Indonesian ERP system deployment (Per Workspace/Year).',
        100000000.00,
        'a0000000-0000-0000-0000-000000000007',
        'a0000000-0000-0000-0000-000000000008',
        'e0000000-0000-0000-0000-000000000001'
    )
ON CONFLICT (id) DO NOTHING;
