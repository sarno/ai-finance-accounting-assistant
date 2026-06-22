-- Migration 014: Seed Tax Module Data
-- This seeds a VAT Input tax type, mock tax records for sales/purchase invoices, and compliance calendar entries.

-- 1. Insert default VAT Input Tax Type (PPN11-IN) if not exists
INSERT INTO tax_types (id, company_id, code, name, category, default_rate, payable_account_id, effective_from, effective_to, is_active)
VALUES (
    'e0000000-0000-0000-0000-000000000002',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    'PPN11-IN',
    'VAT Input 11%',
    'vat_input',
    0.1100,
    'a0000000-0000-0000-0000-000000000005', -- Linked to VAT Payable Account
    '2022-04-01',
    NULL,
    TRUE
) ON CONFLICT (company_id, code, effective_from) DO NOTHING;

-- 2. Seed Mock Tax Records (PPN Keluaran & PPN Masukan) for June 2026
INSERT INTO tax_records (id, company_id, tax_type_id, source_document_type, source_document_id, tax_period, tax_base_amount, tax_rate, tax_amount, status, counterparty_name, counterparty_npwp, created_at, updated_at)
VALUES
    (
        'f1000000-0000-0000-0000-000000000001',
        'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
        'e0000000-0000-0000-0000-000000000001', -- VAT Output
        'SalesInvoice',
        '550e8400-e29b-41d4-a716-446655440001',
        '2026-06-01',
        50000000.00,
        0.11,
        5500000.00,
        'validated',
        'PT Merdeka Jaya',
        '01.222.333.4-012.000',
        '2026-06-10 09:00:00+07',
        '2026-06-10 09:00:00+07'
    ),
    (
        'f1000000-0000-0000-0000-000000000002',
        'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
        'e0000000-0000-0000-0000-000000000001', -- VAT Output
        'SalesInvoice',
        '550e8400-e29b-41d4-a716-446655440002',
        '2026-06-01',
        30000000.00,
        0.11,
        3300000.00,
        'paid',
        'CV Bintang Timur',
        '02.333.444.5-015.000',
        '2026-06-12 11:30:00+07',
        '2026-06-12 11:30:00+07'
    ),
    (
        'f1000000-0000-0000-0000-000000000003',
        'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
        'e0000000-0000-0000-0000-000000000002', -- VAT Input
        'PurchaseInvoice',
        'd0000000-0000-0000-0000-000000000003',
        '2026-06-01',
        30000000.00,
        0.11,
        3300000.00,
        'validated',
        'PT Global Tech Utama',
        '01.555.444.3-012.000',
        '2026-06-08 14:00:00+07',
        '2026-06-08 14:00:00+07'
    ),
    (
        'f1000000-0000-0000-0000-000000000004',
        'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
        'e0000000-0000-0000-0000-000000000002', -- VAT Input
        'PurchaseInvoice',
        'd0000000-0000-0000-0000-000000000004',
        '2026-06-01',
        10000000.00,
        0.11,
        1100000.00,
        'validated',
        'CV Alat Tulis Kantor Jaya',
        '02.444.333.2-015.000',
        '2026-06-15 09:30:00+07',
        '2026-06-15 09:30:00+07'
    )
ON CONFLICT (id) DO NOTHING;

-- 3. Seed Mock Tax Compliance Calendar Entries
INSERT INTO tax_calendar (id, company_id, tax_type_id, tax_period, payment_due_date, filing_due_date, payment_status, filing_status, reminder_sent_at, created_at, updated_at)
VALUES
    -- Current Period (June 2026) compliance calendar (Due in July 2026)
    (
        'c1000000-0000-0000-0000-000000000001',
        'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
        'e0000000-0000-0000-0000-000000000001', -- VAT Output
        '2026-06-01',
        '2026-07-15',
        '2026-07-20',
        'unpaid',
        'unfiled',
        NULL,
        '2026-06-01 00:00:00+07',
        '2026-06-01 00:00:00+07'
    ),
    (
        'c1000000-0000-0000-0000-000000000002',
        'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
        'e0000000-0000-0000-0000-000000000002', -- VAT Input
        '2026-06-01',
        '2026-07-15',
        '2026-07-20',
        'unpaid',
        'unfiled',
        NULL,
        '2026-06-01 00:00:00+07',
        '2026-06-01 00:00:00+07'
    ),
    -- Overdue Period (May 2026) compliance calendar (Due in June 2026)
    (
        'c1000000-0000-0000-0000-000000000003',
        'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
        'e0000000-0000-0000-0000-000000000001', -- VAT Output
        '2026-05-01',
        '2026-06-15',
        '2026-06-20',
        'unpaid',
        'unfiled',
        NULL,
        '2026-05-01 00:00:00+07',
        '2026-05-01 00:00:00+07'
    )
ON CONFLICT (id) DO NOTHING;
