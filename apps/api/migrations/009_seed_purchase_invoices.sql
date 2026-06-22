-- Migration 009: Seed Suppliers and Purchase Invoices
-- This seeds initial suppliers, draft purchase invoices, pending approval requests, and posted purchase invoices with matching ledger entries.

-- 1. Insert default Suppliers
INSERT INTO suppliers (id, company_id, name, tax_number, email, phone, address, is_active)
VALUES 
    (
        '88888888-8888-8888-8888-888888888881',
        'd3b07384-d113-4a1e-a4b5-12cf2f2f754c', -- PT Solusi Codex Indonesia
        'PT Global Tech Utama',
        '01.555.444.3-012.000',
        'billing@globaltech.co.id',
        '+62-21-555-9000',
        'Sudirman Plaza Lt. 5, Jakarta',
        TRUE
    ),
    (
        '88888888-8888-8888-8888-888888888882',
        'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
        'CV Alat Tulis Kantor Jaya',
        '02.444.333.2-015.000',
        'sales@atkjaya.co.id',
        '+62-21-555-1212',
        'Ruko Mangga Dua Square Block B-4, Jakarta',
        TRUE
    )
ON CONFLICT (id) DO NOTHING;

-- 2. DRAFT Purchase Invoice (PINV/2026/06/001)
INSERT INTO purchase_invoices (id, company_id, branch_id, supplier_invoice_number, internal_reference, supplier_id, invoice_date, due_date, subtotal, tax_amount, total_amount, status, notes, journal_entry_id, created_by)
VALUES (
    'd0000000-0000-0000-0000-000000000001',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    '550e8400-e29b-41d4-a716-446655440000', -- HO Branch
    'TAX-2026-9988',
    'PINV/2026/06/001',
    '88888888-8888-8888-8888-888888888881', -- PT Global Tech Utama
    '2026-06-12',
    '2026-07-12',
    15000000.00,
    1650000.00,
    16650000.00,
    'draft'::document_status,
    'Draft purchase invoice for server hosting lease.',
    NULL,
    'f47ac10b-58cc-4372-a567-0e02b2c3d479' -- Chief Accountant
) ON CONFLICT (id) DO NOTHING;

INSERT INTO purchase_invoice_lines (id, purchase_invoice_id, item_id, description, quantity, unit_price, discount_amount, tax_type_id, tax_rate, tax_amount, line_total, account_id, sort_order)
VALUES (
    'e2000000-0000-0000-0000-000000000001',
    'd0000000-0000-0000-0000-000000000001',
    NULL,
    'Dedicated Server Hosting Lease - June 2026',
    1.0000,
    15000000.00,
    0.00,
    'e0000000-0000-0000-0000-000000000001', -- VAT 11%
    0.1100,
    1650000.00,
    16650000.00,
    'a0000000-0000-0000-0000-000000000008', -- Office Expenses Account
    0
) ON CONFLICT (id) DO NOTHING;

-- 3. WAITING APPROVAL Purchase Invoice (PINV/2026/06/002)
INSERT INTO purchase_invoices (id, company_id, branch_id, supplier_invoice_number, internal_reference, supplier_id, invoice_date, due_date, subtotal, tax_amount, total_amount, status, notes, journal_entry_id, created_by)
VALUES (
    'd0000000-0000-0000-0000-000000000002',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    '550e8400-e29b-41d4-a716-446655440000',
    'INV-ATK-443',
    'PINV/2026/06/002',
    '88888888-8888-8888-8888-888888888882', -- CV Alat Tulis Kantor Jaya
    '2026-06-15',
    '2026-07-15',
    2500000.00,
    275000.00,
    2775000.00,
    'waiting_approval'::document_status,
    'Purchase of monthly office stationery supplies.',
    NULL,
    'f47ac10b-58cc-4372-a567-0e02b2c3d479'
) ON CONFLICT (id) DO NOTHING;

INSERT INTO purchase_invoice_lines (id, purchase_invoice_id, item_id, description, quantity, unit_price, discount_amount, tax_type_id, tax_rate, tax_amount, line_total, account_id, sort_order)
VALUES (
    'e2000000-0000-0000-0000-000000000002',
    'd0000000-0000-0000-0000-000000000002',
    NULL,
    'Office Stationery Package A',
    5.0000,
    500000.00,
    0.00,
    'e0000000-0000-0000-0000-000000000001', -- VAT 11%
    0.1100,
    275000.00,
    2775000.00,
    'a0000000-0000-0000-0000-000000000008',
    0
) ON CONFLICT (id) DO NOTHING;

-- Associated Approval Request for PINV/2026/06/002
INSERT INTO approval_requests (id, company_id, document_type, document_id, status, requested_by, created_at, updated_at)
VALUES (
    'a0000000-0000-0000-0000-000000000301',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    'purchase_invoice',
    'd0000000-0000-0000-0000-000000000002',
    'pending',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    '2026-06-15 09:00:00+00',
    '2026-06-15 09:00:00+00'
) ON CONFLICT (id) DO NOTHING;

-- 4. POSTED Purchase Invoice (PINV/2026/06/003)
-- Balanced Journal Entry
INSERT INTO journal_entries (id, company_id, branch_id, reference_number, description, transaction_date, status, source, source_document_id, created_by, posted_by, posted_at, created_at, updated_at)
VALUES (
    'b0000000-0000-0000-0000-000000000009',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    '550e8400-e29b-41d4-a716-446655440000',
    'PINV/2026/06/003',
    'Auto-posting purchase invoice PINV/2026/06/003',
    '2026-06-08',
    'posted'::document_status,
    'purchase_invoice',
    'd0000000-0000-0000-0000-000000000003',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    '2026-06-08 14:00:00+00',
    '2026-06-08 13:45:00+00',
    '2026-06-08 14:00:00+00'
) ON CONFLICT (id) DO NOTHING;

-- Journal lines (balanced)
INSERT INTO journal_lines (id, journal_entry_id, account_id, debit, credit, description, sort_order)
VALUES 
    -- Debit Expense (subtotal)
    ('c0000000-0000-0000-0000-000000000091', 'b0000000-0000-0000-0000-000000000009', 'a0000000-0000-0000-0000-000000000008', 30000000.00, 0.00, 'Consulting Services - IT Security Audit', 0),
    -- Debit VAT Input (tax amount)
    ('c0000000-0000-0000-0000-000000000092', 'b0000000-0000-0000-0000-000000000009', 'a0000000-0000-0000-0000-000000000005', 3300000.00, 0.00, 'Tax for: Consulting Services - IT Security Audit', 1),
    -- Credit Accounts Payable (total amount)
    ('c0000000-0000-0000-0000-000000000093', 'b0000000-0000-0000-0000-000000000009', 'a0000000-0000-0000-0000-000000000004', 0.00, 33300000.00, 'Payable for purchase invoice PINV/2026/06/003', 2)
ON CONFLICT (id) DO NOTHING;

-- Purchase Invoice itself
INSERT INTO purchase_invoices (id, company_id, branch_id, supplier_invoice_number, internal_reference, supplier_id, invoice_date, due_date, subtotal, tax_amount, total_amount, status, notes, journal_entry_id, created_by)
VALUES (
    'd0000000-0000-0000-0000-000000000003',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    '550e8400-e29b-41d4-a716-446655440000',
    'INV-2026-HQ-001',
    'PINV/2026/06/003',
    '88888888-8888-8888-8888-888888888881', -- PT Global Tech Utama
    '2026-06-08',
    '2026-07-08',
    30000000.00,
    3300000.00,
    33300000.00,
    'posted'::document_status,
    'Posted purchase invoice for consulting services.',
    'b0000000-0000-0000-0000-000000000009', -- Linked journal entry
    'f47ac10b-58cc-4372-a567-0e02b2c3d479'
) ON CONFLICT (id) DO NOTHING;

INSERT INTO purchase_invoice_lines (id, purchase_invoice_id, item_id, description, quantity, unit_price, discount_amount, tax_type_id, tax_rate, tax_amount, line_total, account_id, sort_order)
VALUES (
    'e2000000-0000-0000-0000-000000000003',
    'd0000000-0000-0000-0000-000000000003',
    NULL,
    'Consulting Services - IT Security Audit',
    1.0000,
    30000000.00,
    0.00,
    'e0000000-0000-0000-0000-000000000001', -- VAT 11%
    0.1100,
    3300000.00,
    33300000.00,
    'a0000000-0000-0000-0000-000000000008',
    0
) ON CONFLICT (id) DO NOTHING;

-- Associated Approval Request for PINV/2026/06/003
INSERT INTO approval_requests (id, company_id, document_type, document_id, status, requested_by, reviewed_by, reviewed_at, comment, created_at, updated_at)
VALUES (
    'a0000000-0000-0000-0000-000000000302',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    'purchase_invoice',
    'd0000000-0000-0000-0000-000000000003',
    'approved',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    '2026-06-08 14:00:00+00',
    'Approved audit invoice',
    '2026-06-08 13:45:00+00',
    '2026-06-08 14:00:00+00'
) ON CONFLICT (id) DO NOTHING;
