-- Migration 007: Link sales invoice lines to master data items

ALTER TABLE sales_invoice_lines
ADD COLUMN item_id UUID REFERENCES items(id) ON DELETE SET NULL;

CREATE INDEX idx_sales_invoice_lines_item_id ON sales_invoice_lines(item_id);
