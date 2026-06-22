-- Migration 012: Add attachment_url to payments
ALTER TABLE payments ADD COLUMN attachment_url TEXT;
