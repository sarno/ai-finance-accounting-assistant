-- Migration 002: Seed initial trial data
-- Standard company, default admin user, standard Indonesian COA, and VAT tax type configuration.

-- 1. Insert default Company
INSERT INTO companies (id, name, tax_number, address, currency, is_active)
VALUES (
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    'PT Solusi Codex Indonesia',
    '01.234.567.8-012.000',
    'Sudirman Central Business District, Jakarta, Indonesia',
    'IDR',
    TRUE
) ON CONFLICT (id) DO NOTHING;

-- 2. Insert default User (password: admin123)
INSERT INTO users (id, company_id, email, full_name, password_hash, is_active)
VALUES (
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    'admin@codex.id',
    'Chief Accountant',
    '$argon2id$v=19$m=19456,t=2,p=1$k5DujGWA+QZWPWIgM0L4KQ$GRJHqDI8biepElbOBNx0FrvVsuMBabJF0wHDK9vLUfg',
    TRUE
) ON CONFLICT (id) DO NOTHING;

-- 3. Insert default User Roles
INSERT INTO user_roles (user_id, role)
VALUES 
    ('f47ac10b-58cc-4372-a567-0e02b2c3d479', 'admin'),
    ('f47ac10b-58cc-4372-a567-0e02b2c3d479', 'owner')
ON CONFLICT (user_id, role) DO NOTHING;

-- 4. Insert default Chart of Accounts (COA)
INSERT INTO chart_of_accounts (id, company_id, code, name, account_type, parent_id, is_active)
VALUES
    ('a0000000-0000-0000-0000-000000000001', 'd3b07384-d113-4a1e-a4b5-12cf2f2f754c', '1000', 'Petty Cash', 'Asset', NULL, TRUE),
    ('a0000000-0000-0000-0000-000000000002', 'd3b07384-d113-4a1e-a4b5-12cf2f2f754c', '1100', 'Bank BCA Account', 'Asset', NULL, TRUE),
    ('a0000000-0000-0000-0000-000000000003', 'd3b07384-d113-4a1e-a4b5-12cf2f2f754c', '1200', 'Accounts Receivable', 'Asset', NULL, TRUE),
    ('a0000000-0000-0000-0000-000000000004', 'd3b07384-d113-4a1e-a4b5-12cf2f2f754c', '2100', 'Accounts Payable', 'Liability', NULL, TRUE),
    ('a0000000-0000-0000-0000-000000000005', 'd3b07384-d113-4a1e-a4b5-12cf2f2f754c', '2200', 'VAT Payable', 'Liability', NULL, TRUE),
    ('a0000000-0000-0000-0000-000000000006', 'd3b07384-d113-4a1e-a4b5-12cf2f2f754c', '3000', 'Common Stock', 'Equity', NULL, TRUE),
    ('a0000000-0000-0000-0000-000000000007', 'd3b07384-d113-4a1e-a4b5-12cf2f2f754c', '4000', 'Sales Revenue', 'Revenue', NULL, TRUE),
    ('a0000000-0000-0000-0000-000000000008', 'd3b07384-d113-4a1e-a4b5-12cf2f2f754c', '5000', 'Office Expenses', 'Expense', NULL, TRUE)
ON CONFLICT (company_id, code) DO NOTHING;

-- 5. Insert default Tax Type configuration (PPN 11%)
INSERT INTO tax_types (id, company_id, code, name, category, default_rate, payable_account_id, effective_from, effective_to, is_active)
VALUES (
    'e0000000-0000-0000-0000-000000000001',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    'PPN11',
    'VAT 11%',
    'vat_output',
    0.1100,
    'a0000000-0000-0000-0000-000000000005', -- Linked to VAT Payable Account
    '2022-04-01',
    NULL,
    TRUE
) ON CONFLICT (company_id, code, effective_from) DO NOTHING;
