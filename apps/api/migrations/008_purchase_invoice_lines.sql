-- Migration 008: Purchase invoice lines

CREATE TABLE purchase_invoice_lines (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    purchase_invoice_id  UUID NOT NULL REFERENCES purchase_invoices(id) ON DELETE CASCADE,
    item_id              UUID REFERENCES items(id) ON DELETE SET NULL,
    description          TEXT NOT NULL,
    quantity             NUMERIC(20, 4) NOT NULL,
    unit_price           NUMERIC(20, 2) NOT NULL,
    discount_amount      NUMERIC(20, 2) NOT NULL DEFAULT 0,
    tax_type_id          UUID REFERENCES tax_types(id),
    tax_rate             NUMERIC(10, 4),
    tax_amount           NUMERIC(20, 2) NOT NULL DEFAULT 0,
    line_total           NUMERIC(20, 2) NOT NULL,
    account_id           UUID NOT NULL REFERENCES chart_of_accounts(id),
    sort_order           INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_purchase_invoice_lines_invoice_id ON purchase_invoice_lines(purchase_invoice_id);
CREATE INDEX idx_purchase_invoice_lines_item_id ON purchase_invoice_lines(item_id);
