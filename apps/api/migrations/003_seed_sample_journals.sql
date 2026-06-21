-- Migration 003: Seed sample journals and approval requests

-- 1. Share Capital Deposit (Posted)
-- Journal Entry
INSERT INTO journal_entries (id, company_id, reference_number, description, transaction_date, status, source, created_by, posted_by, posted_at, created_at, updated_at)
VALUES (
    'b0000000-0000-0000-0000-000000000001',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    'JV/2026/001',
    'Initial share capital injection from shareholders',
    '2026-06-01',
    'posted'::document_status,
    'manual',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    '2026-06-01 09:00:00+00',
    '2026-06-01 08:30:00+00',
    '2026-06-01 09:00:00+00'
) ON CONFLICT (id) DO NOTHING;

-- Lines (Debit Bank BCA, Credit Common Stock)
INSERT INTO journal_lines (id, journal_entry_id, account_id, debit, credit, description, sort_order)
VALUES 
    ('c0000000-0000-0000-0000-000000000001', 'b0000000-0000-0000-0000-000000000001', 'a0000000-0000-0000-0000-000000000002', 500000000.00, 0.00, 'Bank deposit share capital', 0),
    ('c0000000-0000-0000-0000-000000000002', 'b0000000-0000-0000-0000-000000000001', 'a0000000-0000-0000-0000-000000000006', 0.00, 500000000.00, 'Common stock issue', 1)
ON CONFLICT (id) DO NOTHING;

-- 2. Office Rent Prepayment (Draft)
-- Journal Entry
INSERT INTO journal_entries (id, company_id, reference_number, description, transaction_date, status, source, created_by, created_at, updated_at)
VALUES (
    'b0000000-0000-0000-0000-000000000002',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    'JV/2026/002',
    'Prepayment for Sudirman office rent (1 Year)',
    '2026-06-18',
    'draft'::document_status,
    'manual',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    '2026-06-18 10:00:00+00',
    '2026-06-18 10:00:00+00'
) ON CONFLICT (id) DO NOTHING;

-- Lines (Debit Office Expenses, Credit Bank BCA)
INSERT INTO journal_lines (id, journal_entry_id, account_id, debit, credit, description, sort_order)
VALUES 
    ('c0000000-0000-0000-0000-000000000003', 'b0000000-0000-0000-0000-000000000002', 'a0000000-0000-0000-0000-000000000008', 24000000.00, 0.00, 'Office expense prepaid rent', 0),
    ('c0000000-0000-0000-0000-000000000004', 'b0000000-0000-0000-0000-000000000002', 'a0000000-0000-0000-0000-000000000002', 0.00, 24000000.00, 'Rent payment via BCA', 1)
ON CONFLICT (id) DO NOTHING;

-- 3. Server hosting & office internet (Waiting Approval)
-- Journal Entry
INSERT INTO journal_entries (id, company_id, reference_number, description, transaction_date, status, source, created_by, created_at, updated_at)
VALUES (
    'b0000000-0000-0000-0000-000000000003',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    'JV/2026/003',
    'Server hosting and office internet prepayment',
    '2026-06-19',
    'waiting_approval'::document_status,
    'manual',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    '2026-06-19 11:30:00+00',
    '2026-06-19 11:30:00+00'
) ON CONFLICT (id) DO NOTHING;

-- Lines (Debit Office Expenses, Credit Bank BCA)
INSERT INTO journal_lines (id, journal_entry_id, account_id, debit, credit, description, sort_order)
VALUES 
    ('c0000000-0000-0000-0000-000000000005', 'b0000000-0000-0000-0000-000000000003', 'a0000000-0000-0000-0000-000000000008', 5000000.00, 0.00, 'Hosting expense', 0),
    ('c0000000-0000-0000-0000-000000000006', 'b0000000-0000-0000-0000-000000000003', 'a0000000-0000-0000-0000-000000000002', 0.00, 5000000.00, 'BCA Bank payment', 1)
ON CONFLICT (id) DO NOTHING;

-- Approval Request
INSERT INTO approval_requests (id, company_id, document_type, document_id, status, requested_by, created_at, updated_at)
VALUES (
    'a0000000-0000-0000-0000-000000000101',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    'journal_entry',
    'b0000000-0000-0000-0000-000000000003',
    'pending',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    '2026-06-19 11:30:00+00',
    '2026-06-19 11:30:00+00'
) ON CONFLICT (id) DO NOTHING;

-- 4. Developer laptop purchase (Waiting Approval)
-- Journal Entry
INSERT INTO journal_entries (id, company_id, reference_number, description, transaction_date, status, source, created_by, created_at, updated_at)
VALUES (
    'b0000000-0000-0000-0000-000000000004',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    'JV/2026/004',
    'Purchase of development laptop for engineers',
    '2026-06-20',
    'waiting_approval'::document_status,
    'manual',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    '2026-06-20 14:00:00+00',
    '2026-06-20 14:00:00+00'
) ON CONFLICT (id) DO NOTHING;

-- Lines (Debit Office Expenses, Credit Accounts Payable)
INSERT INTO journal_lines (id, journal_entry_id, account_id, debit, credit, description, sort_order)
VALUES 
    ('c0000000-0000-0000-0000-000000000007', 'b0000000-0000-0000-0000-000000000004', 'a0000000-0000-0000-0000-000000000008', 15000000.00, 0.00, 'Laptop equipment expense', 0),
    ('c0000000-0000-0000-0000-000000000008', 'b0000000-0000-0000-0000-000000000004', 'a0000000-0000-0000-0000-000000000004', 0.00, 15000000.00, 'Accounts payable Laptop Store', 1)
ON CONFLICT (id) DO NOTHING;

-- Approval Request
INSERT INTO approval_requests (id, company_id, document_type, document_id, status, requested_by, created_at, updated_at)
VALUES (
    'a0000000-0000-0000-0000-000000000102',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    'journal_entry',
    'b0000000-0000-0000-0000-000000000004',
    'pending',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    '2026-06-20 14:00:00+00',
    '2026-06-20 14:00:00+00'
) ON CONFLICT (id) DO NOTHING;
