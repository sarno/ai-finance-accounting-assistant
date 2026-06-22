-- Migration 011: Seed Bank Accounts, Payments, and Payment Allocations
-- This seeds a default bank account, a posted customer payment (partial) with journal entries, a posted supplier payment (full), and a draft customer payment.

-- 1. Insert default Bank Account
INSERT INTO bank_accounts (id, company_id, branch_id, account_id, bank_name, account_number, account_name, currency, is_active)
VALUES (
    '77777777-7777-7777-7777-777777777777',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c', -- PT Solusi Codex Indonesia
    '550e8400-e29b-41d4-a716-446655440000', -- HO Branch
    'a0000000-0000-0000-0000-000000000002', -- Bank BCA Account (Asset COA)
    'Bank Central Asia (BCA)',
    '123-456-7890',
    'PT Solusi Codex Indonesia Operational',
    'IDR',
    TRUE
) ON CONFLICT (id) DO NOTHING;

-- 2. Customer Payment (Payment Received - PAY/RCV/2026/001)
-- Auto-posting Journal Entry for Customer Payment
INSERT INTO journal_entries (id, company_id, branch_id, reference_number, description, transaction_date, status, source, source_document_id, created_by, posted_by, posted_at, created_at, updated_at)
VALUES (
    'b0000000-0000-0000-0000-000000000010',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    '550e8400-e29b-41d4-a716-446655440000',
    'PAY/RCV/2026/001',
    'Auto-posting payment received PAY/RCV/2026/001',
    '2026-06-12',
    'posted'::document_status,
    'sales_invoice', -- Source mapping
    'f0000000-0000-0000-0000-000000000003', -- Linked Sales Invoice
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    '2026-06-12 10:00:00+00',
    '2026-06-12 09:45:00+00',
    '2026-06-12 10:00:00+00'
) ON CONFLICT (id) DO NOTHING;

-- Journal lines for Customer Payment (Dr Bank BCA, Cr Accounts Receivable)
INSERT INTO journal_lines (id, journal_entry_id, account_id, debit, credit, description, sort_order)
VALUES 
    ('c0000000-0000-0000-0000-000000000101', 'b0000000-0000-0000-0000-000000000010', 'a0000000-0000-0000-0000-000000000002', 50000000.00, 0.00, 'Payment received via bank account PT Solusi Codex Indonesia Operational', 0),
    ('c0000000-0000-0000-0000-000000000102', 'b0000000-0000-0000-0000-000000000010', 'a0000000-0000-0000-0000-000000000003', 0.00, 50000000.00, 'Receivable collection for invoice INV/2026/06/003', 1)
ON CONFLICT (id) DO NOTHING;

-- Customer Payment details
INSERT INTO payments (id, company_id, reference_number, payment_type, counterparty_type, counterparty_id, payment_date, bank_account_id, amount, status, notes, journal_entry_id, created_by, created_at, updated_at)
VALUES (
    '90000000-0000-0000-0000-000000000001',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    'PAY/RCV/2026/001',
    'payment_received',
    'customer',
    'c0000000-0000-0000-0000-000000000001', -- PT Digital Nusantara
    '2026-06-12',
    '77777777-7777-7777-7777-777777777777', -- BCA operational account
    50000000.00,
    'posted'::document_status,
    'Partial bank transfer payment from PT Digital Nusantara for ERP customization.',
    'b0000000-0000-0000-0000-000000000010',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    '2026-06-12 09:45:00+00',
    '2026-06-12 10:00:00+00'
) ON CONFLICT (id) DO NOTHING;

-- Payment Allocation to Sales Invoice INV/2026/06/003
INSERT INTO payment_allocations (id, payment_id, document_type, document_id, allocated_amount)
VALUES (
    '91000000-0000-0000-0000-000000000001',
    '90000000-0000-0000-0000-000000000001',
    'sales_invoice',
    'f0000000-0000-0000-0000-000000000003', -- Sales Invoice INV/2026/06/003
    50000000.00
) ON CONFLICT (id) DO NOTHING;

-- Associated Approval Request for Customer Payment
INSERT INTO approval_requests (id, company_id, document_type, document_id, status, requested_by, reviewed_by, reviewed_at, comment, created_at, updated_at)
VALUES (
    'a0000000-0000-0000-0000-000000000401',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    'payment_received',
    '90000000-0000-0000-0000-000000000001',
    'approved',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    '2026-06-12 10:00:00+00',
    'Receipt confirmed by bank statement.',
    '2026-06-12 09:45:00+00',
    '2026-06-12 10:00:00+00'
) ON CONFLICT (id) DO NOTHING;


-- 3. Supplier Payment (Payment Paid - PAY/PAY/2026/001)
-- Auto-posting Journal Entry for Supplier Payment
INSERT INTO journal_entries (id, company_id, branch_id, reference_number, description, transaction_date, status, source, source_document_id, created_by, posted_by, posted_at, created_at, updated_at)
VALUES (
    'b0000000-0000-0000-0000-000000000011',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    '550e8400-e29b-41d4-a716-446655440000',
    'PAY/PAY/2026/001',
    'Auto-posting payment paid PAY/PAY/2026/001',
    '2026-06-14',
    'posted'::document_status,
    'purchase_invoice',
    'd0000000-0000-0000-0000-000000000003', -- Linked Purchase Invoice
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    '2026-06-14 11:00:00+00',
    '2026-06-14 10:30:00+00',
    '2026-06-14 11:00:00+00'
) ON CONFLICT (id) DO NOTHING;

-- Journal lines for Supplier Payment (Dr Accounts Payable, Cr Bank BCA)
INSERT INTO journal_lines (id, journal_entry_id, account_id, debit, credit, description, sort_order)
VALUES 
    ('c0000000-0000-0000-0000-000000000111', 'b0000000-0000-0000-0000-000000000011', 'a0000000-0000-0000-0000-000000000004', 33300000.00, 0.00, 'Accounts Payable settlement for PT Global Tech Utama', 0),
    ('c0000000-0000-0000-0000-000000000112', 'b0000000-0000-0000-0000-000000000011', 'a0000000-0000-0000-0000-000000000002', 0.00, 33300000.00, 'Payment paid via bank account PT Solusi Codex Indonesia Operational', 1)
ON CONFLICT (id) DO NOTHING;

-- Supplier Payment details
INSERT INTO payments (id, company_id, reference_number, payment_type, counterparty_type, counterparty_id, payment_date, bank_account_id, amount, status, notes, journal_entry_id, created_by, created_at, updated_at)
VALUES (
    '90000000-0000-0000-0000-000000000002',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    'PAY/PAY/2026/001',
    'payment_paid',
    'supplier',
    '88888888-8888-8888-8888-888888888881', -- PT Global Tech Utama
    '2026-06-14',
    '77777777-7777-7777-7777-777777777777', -- BCA operational account
    33300000.00,
    'posted'::document_status,
    'Full settlement payment to PT Global Tech Utama for security audit invoice.',
    'b0000000-0000-0000-0000-000000000011',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    '2026-06-14 10:30:00+00',
    '2026-06-14 11:00:00+00'
) ON CONFLICT (id) DO NOTHING;

-- Payment Allocation to Purchase Invoice PINV/2026/06/003
INSERT INTO payment_allocations (id, payment_id, document_type, document_id, allocated_amount)
VALUES (
    '91000000-0000-0000-0000-000000000002',
    '90000000-0000-0000-0000-000000000002',
    'purchase_invoice',
    'd0000000-0000-0000-0000-000000000003', -- Purchase Invoice PINV/2026/06/003
    33300000.00
) ON CONFLICT (id) DO NOTHING;

-- Associated Approval Request for Supplier Payment
INSERT INTO approval_requests (id, company_id, document_type, document_id, status, requested_by, reviewed_by, reviewed_at, comment, created_at, updated_at)
VALUES (
    'a0000000-0000-0000-0000-000000000402',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    'payment_paid',
    '90000000-0000-0000-0000-000000000002',
    'approved',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    '2026-06-14 11:00:00+00',
    'Disbursement approved.',
    '2026-06-14 10:30:00+00',
    '2026-06-14 11:00:00+00'
) ON CONFLICT (id) DO NOTHING;


-- 4. DRAFT Customer Payment (TEMP-PAY-RCV-001)
INSERT INTO payments (id, company_id, reference_number, payment_type, counterparty_type, counterparty_id, payment_date, bank_account_id, amount, status, notes, journal_entry_id, created_by, created_at, updated_at)
VALUES (
    '90000000-0000-0000-0000-000000000003',
    'd3b07384-d113-4a1e-a4b5-12cf2f2f754c',
    'TEMP-PAY-RCV-001',
    'payment_received',
    'customer',
    'c0000000-0000-0000-0000-000000000002', -- PT Mega Retail Indo
    '2026-06-22',
    '77777777-7777-7777-7777-777777777777',
    10000000.00,
    'draft'::document_status,
    'Draft entry for upcoming cheque clearing from PT Mega Retail.',
    NULL,
    'f47ac10b-58cc-4372-a567-0e02b2c3d479',
    '2026-06-22 08:00:00+00',
    '2026-06-22 08:00:00+00'
) ON CONFLICT (id) DO NOTHING;
