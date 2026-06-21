-- Migration 004: Implement Multibranch support

-- 1. Create branches table
CREATE TABLE branches (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id      UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    code            TEXT NOT NULL,
    name            TEXT NOT NULL,
    address         TEXT,
    phone           TEXT,
    is_active       BOOLEAN NOT NULL DEFAULT TRUE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_company_branch_code UNIQUE (company_id, code)
);

-- 2. Seed a default Head Office branch for the default company
INSERT INTO branches (id, company_id, code, name, address, phone, is_active)
VALUES (
    '550e8400-e29b-41d4-a716-446655440000',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    'HO',
    'Head Office',
    'Sudirman Central Business District, Jakarta, Indonesia',
    '+62-21-555-0199',
    TRUE
) ON CONFLICT (id) DO NOTHING;

-- 3. Add branch_id columns to existing tables
-- Users
ALTER TABLE users ADD COLUMN branch_id UUID REFERENCES branches(id);
UPDATE users SET branch_id = '550e8400-e29b-41d4-a716-446655440000' WHERE company_id = 'd3b07384-d113-4a1e-a4b5-12cf2f2f754c';

-- Journal Entries
ALTER TABLE journal_entries ADD COLUMN branch_id UUID REFERENCES branches(id);
UPDATE journal_entries SET branch_id = '550e8400-e29b-41d4-a716-446655440000' WHERE company_id = 'd3b07384-d113-4a1e-a4b5-12cf2f2f754c';

-- Sales Invoices
ALTER TABLE sales_invoices ADD COLUMN branch_id UUID REFERENCES branches(id);
UPDATE sales_invoices SET branch_id = '550e8400-e29b-41d4-a716-446655440000' WHERE company_id = 'd3b07384-d113-4a1e-a4b5-12cf2f2f754c';

-- Purchase Invoices
ALTER TABLE purchase_invoices ADD COLUMN branch_id UUID REFERENCES branches(id);
UPDATE purchase_invoices SET branch_id = '550e8400-e29b-41d4-a716-446655440000' WHERE company_id = 'd3b07384-d113-4a1e-a4b5-12cf2f2f754c';

-- Bank Accounts
ALTER TABLE bank_accounts ADD COLUMN branch_id UUID REFERENCES branches(id);
UPDATE bank_accounts SET branch_id = '550e8400-e29b-41d4-a716-446655440000' WHERE company_id = 'd3b07384-d113-4a1e-a4b5-12cf2f2f754c';

-- Create index for quick branch filtering
CREATE INDEX idx_branches_company ON branches(company_id);
CREATE INDEX idx_journal_entries_branch ON journal_entries(branch_id);
