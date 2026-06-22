-- Migration 010: Add attachment_url to purchase_invoices
ALTER TABLE purchase_invoices ADD COLUMN attachment_url TEXT;
