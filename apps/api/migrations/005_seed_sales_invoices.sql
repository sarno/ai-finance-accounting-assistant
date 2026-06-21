-- Migration 005: Seed Customers and Sales Invoices
-- This seeds initial customers, draft invoices, pending approval requests, and posted invoices with matching ledger entries.

-- 1. Insert default Customers
INSERT INTO customers (id, company_id, name, tax_number, email, phone, address, is_active)
VALUES 
    (
        'c0000000-0000-0000-0000-000000000001',
        'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
        'PT Digital Nusantara',
        '01.222.333.4-012.000',
        'finance@digitalnusantara.co.id',
        '+62-21-555-0321',
        'Graha Sudirman Lt. 12, Jakarta',
        TRUE
    ),
    (
        'c0000000-0000-0000-0000-000000000002',
        'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
        'PT Mega Retail Indo',
        '02.555.666.7-015.000',
        'billing@megaretail.co.id',
        '+62-21-555-0876',
        'Mega Mall CBD Lt. 3, Jakarta',
        TRUE
    )
ON CONFLICT (id) DO NOTHING;

-- 2. DRAFT Invoice (INV/2026/06/001)
INSERT INTO sales_invoices (id, company_id, branch_id, invoice_number, customer_id, invoice_date, due_date, subtotal, tax_amount, total_amount, status, notes, journal_entry_id, created_by)
VALUES (
    'f0000000-0000-0000-0000-000000000001',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    '550e8400-e29b-41d4-a716-446655440000', -- HO Branch
    'INV/2026/06/001',
    'c0000000-0000-0000-0000-000000000001', -- PT Digital Nusantara
    '2026-06-15',
    '2026-07-15',
    50000000.00,
    5500000.00,
    55500000.00,
    'draft'::document_status,
    'Draft invoice for June enterprise software license deployment.',
    NULL,
    'f47ac10b-58cc-4372-a567-0e02b2c3d479' -- Chief Accountant
) ON CONFLICT (id) DO NOTHING;

INSERT INTO sales_invoice_lines (id, sales_invoice_id, description, quantity, unit_price, discount_amount, tax_type_id, tax_rate, tax_amount, line_total, account_id, sort_order)
VALUES (
    'e1000000-0000-0000-0000-000000000001',
    'f0000000-0000-0000-0000-000000000001',
    'Enterprise ERP Platform Customization License',
    1.0000,
    50000000.00,
    0.00,
    'e0000000-0000-0000-0000-000000000001', -- VAT 11%
    0.1100,
    5500000.00,
    55500000.00,
    'a0000000-0000-0000-0000-000000000007', -- Sales Revenue Account
    0
) ON CONFLICT (id) DO NOTHING;

-- 3. WAITING APPROVAL Invoice (INV/2026/06/002)
INSERT INTO sales_invoices (id, company_id, branch_id, invoice_number, customer_id, invoice_date, due_date, subtotal, tax_amount, total_amount, status, notes, journal_entry_id, created_by)
VALUES (
    'f0000000-0000-0000-0000-000000000002',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    '550e8400-e29b-41d4-a716-446655440000',
    'INV/2026/06/002',
    'c0000000-0000-0000-0000-000000000002', -- PT Mega Retail Indo
    '2026-06-18',
    '2026-07-18',
    20000000.00,
    2200000.00,
    22200000.00,
    'waiting_approval'::document_status,
    'Waiting approval for professional IT consulting services.',
    NULL,
    'f47ac10b-58cc-4372-a567-0e02b2c3d479'
) ON CONFLICT (id) DO NOTHING;

INSERT INTO sales_invoice_lines (id, sales_invoice_id, description, quantity, unit_price, discount_amount, tax_type_id, tax_rate, tax_amount, line_total, account_id, sort_order)
VALUES (
    'e1000000-0000-0000-0000-000000000002',
    'f0000000-0000-0000-0000-000000000002',
    'Consulting Services - Architecture Design (10 Days)',
    10.0000,
    2000000.00,
    0.00,
    'e0000000-0000-0000-0000-000000000001', -- VAT 11%
    0.1100,
    2200000.00,
    22200000.00,
    'a0000000-0000-0000-0000-000000000007',
    0
) ON CONFLICT (id) DO NOTHING;

-- Associated Approval Request for INV/2026/06/002
INSERT INTO approval_requests (id, company_id, document_type, document_id, status, requested_by, created_at, updated_at)
VALUES (
    'a0000000-0000-0000-0000-000000000201',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    'sales_invoice',
    'f0000000-0000-0000-0000-000000000002',
    'pending',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    '2026-06-18 10:30:00+00',
    '2026-06-18 10:30:00+00'
) ON CONFLICT (id) DO NOTHING;

-- 4. POSTED Invoice (INV/2026/06/003)
-- Balanced Journal Entry
INSERT INTO journal_entries (id, company_id, branch_id, reference_number, description, transaction_date, status, source, source_document_id, created_by, posted_by, posted_at, created_at, updated_at)
VALUES (
    'b0000000-0000-0000-0000-000000000005',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    '550e8400-e29b-41d4-a716-446655440000',
    'INV/2026/06/003',
    'Auto-posting sales invoice INV/2026/06/003',
    '2026-06-10',
    'posted'::document_status,
    'sales_invoice',
    'f0000000-0000-0000-0000-000000000003',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    '2026-06-10 11:00:00+00',
    '2026-06-10 10:45:00+00',
    '2026-06-10 11:00:00+00'
) ON CONFLICT (id) DO NOTHING;

-- Journal lines (balanced)
INSERT INTO journal_lines (id, journal_entry_id, account_id, debit, credit, description, sort_order)
VALUES 
    -- Credit Sales Revenue (subtotal)
    ('c0000000-0000-0000-0000-000000000011', 'b0000000-0000-0000-0000-000000000005', 'a0000000-0000-0000-0000-000000000007', 0.00, 100000000.00, 'Cloud Infrastructure Setup Support', 0),
    -- Credit VAT Payable (tax amount)
    ('c0000000-0000-0000-0000-000000000012', 'b0000000-0000-0000-0000-000000000005', 'a0000000-0000-0000-0000-000000000005', 0.00, 11000000.00, 'Tax for: Cloud Infrastructure Setup Support', 1),
    -- Debit Accounts Receivable (total amount)
    ('c0000000-0000-0000-0000-000000000013', 'b0000000-0000-0000-0000-000000000005', 'a0000000-0000-0000-0000-000000000003', 111000000.00, 0.00, 'Receivable for invoice INV/2026/06/003', 2)
ON CONFLICT (id) DO NOTHING;

-- Sales Invoice itself
INSERT INTO sales_invoices (id, company_id, branch_id, invoice_number, customer_id, invoice_date, due_date, subtotal, tax_amount, total_amount, status, notes, journal_entry_id, created_by)
VALUES (
    'f0000000-0000-0000-0000-000000000003',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    '550e8400-e29b-41d4-a716-446655440000',
    'INV/2026/06/003',
    'c0000000-0000-0000-0000-000000000001', -- PT Digital Nusantara
    '2026-06-10',
    '2026-07-10',
    100000000.00,
    11000000.00,
    111000000.00,
    'posted'::document_status,
    'Posted invoice for cloud migration services.',
    'b0000000-0000-0000-0000-000000000005', -- Linked journal entry
    'f47ac10b-58cc-4372-a567-0e02b2c3d479'
) ON CONFLICT (id) DO NOTHING;

INSERT INTO sales_invoice_lines (id, sales_invoice_id, description, quantity, unit_price, discount_amount, tax_type_id, tax_rate, tax_amount, line_total, account_id, sort_order)
VALUES (
    'e1000000-0000-0000-0000-000000000003',
    'f0000000-0000-0000-0000-000000000003',
    'Cloud Infrastructure Setup Support',
    1.0000,
    100000000.00,
    0.00,
    'e0000000-0000-0000-0000-000000000001', -- VAT 11%
    0.1100,
    11000000.00,
    111000000.00,
    'a0000000-0000-0000-0000-000000000007',
    0
) ON CONFLICT (id) DO NOTHING;

-- Associated Approval Request for INV/2026/06/003
INSERT INTO approval_requests (id, company_id, document_type, document_id, status, requested_by, reviewed_by, reviewed_at, comment, created_at, updated_at)
VALUES (
    'a0000000-0000-0000-0000-000000000202',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    'sales_invoice',
    'f0000000-0000-0000-0000-000000000003',
    'approved',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    '2026-06-10 11:00:00+00',
    'Invoice verified and approved',
    '2026-06-10 10:45:00+00',
    '2026-06-10 11:00:00+00'
) ON CONFLICT (id) DO NOTHING;
