# OpenClaw and AI Workflows

## 1. Role of OpenClaw

OpenClaw is used as chat/AI automation gateway. It connects WhatsApp/Telegram channels to an AI assistant and backend tools.

OpenClaw should not contain accounting business logic. It must call Rust (Axum) APIs.

## 2. Recommended OpenClaw folder

```text
/apps/openclaw
  agents/
    finance-assistant.md
    accounting-automation.md
  tools/
    backend-tools.json
  workflows/
    cash-query.yaml
    invoice-draft.yaml
    approval-command.yaml
  prompts/
    system-prompt.md
    safety-rules.md
```

## 3. AI system prompt draft

```text
You are AI Finance & Accounting Assistant.
You help business owners and finance teams query financial reports and create draft accounting documents.

Rules:
1. Never invent financial numbers.
2. Always get numbers from backend tools.
3. Never post a transaction without explicit approval.
4. For invoice, journal, payment, and tax transaction creation, create DRAFT only.
5. If required fields are missing, ask a short clarification.
6. If user requests tax advice, provide operational summary and recommend finance/tax review.
7. Use Indonesian unless user asks another language.
8. Keep answers concise and structured.
```

## 4. Tool list

### query_report

Purpose: get financial report data.

Parameters:

```json
{
  "intent": "CashPosition | AccountsReceivable | AccountsPayable | ProfitLoss | TaxSummary | Cashflow",
  "period": "YYYY-MM",
  "asOf": "YYYY-MM-DD"
}
```

### create_draft_sales_invoice

Purpose: create draft sales invoice.

### create_draft_purchase_invoice

Purpose: create draft purchase invoice from chat/OCR data.

### create_draft_payment

Purpose: create draft payment received/paid.

### create_draft_journal

Purpose: create draft manual journal suggestion.

### submit_approval_command

Purpose: process owner/manager approval command from chat.

### get_pending_approvals

Purpose: show approval list to authorized user.

## 5. Workflow: owner asks cash

User:

```text
Berapa kas hari ini?
```

AI steps:

1. Identify intent: `CashPosition`.
2. Call `query_report` with `asOf=today`.
3. Format answer:

```text
Posisi kas per 2 Juni 2026:

Total kas: Rp xxx

Rincian:
- BCA Operasional: Rp xxx
- Mandiri: Rp xxx
- Kas kecil: Rp xxx

Catatan: data berdasarkan transaksi yang sudah posted.
```

## 6. Workflow: owner asks tax

User:

```text
Pajak bulan ini berapa?
```

AI steps:

1. Identify intent: `TaxSummary`.
2. Call backend tax summary.
3. Return tax amounts and alerts.
4. Include disclaimer: based on recorded transactions and tax config.

## 7. Workflow: create draft purchase invoice from chat

User:

```text
Catat invoice dari CV Maju Bersama sebesar Rp2.608.500 termasuk PPN, jatuh tempo 30 Juni.
```

AI steps:

1. Extract supplier, total amount, tax mention, due date.
2. Check missing fields: invoice date, invoice number, expense account if needed.
3. Call backend create draft endpoint.
4. Backend validates supplier, tax config, journal suggestion.
5. AI responds with draft summary and approval instruction.

Response:

```text
Saya buatkan draft purchase invoice:

Supplier: CV Maju Bersama
Total: Rp2.608.500
Jatuh tempo: 30 Juni 2026
Status: Draft

Draft belum diposting. Silakan review di dashboard atau ketik "ajukan approval".
```

## 8. Workflow: invoice OCR

Input: PDF/image invoice via chat or dashboard.

Steps:

```text
Document upload
  -> backend stores original file
  -> OCR extraction
  -> AI extracts structured fields
  -> backend validates
  -> draft purchase invoice created
  -> approval request generated
```

Validation checklist:

- supplier exists or suggested new supplier;
- invoice number not duplicate for same supplier;
- subtotal + tax = total;
- tax rate matches active config;
- due date valid;
- attachment stored.

## 9. Workflow: approval from chat

User:

```text
Setujui invoice INV-2026-001
```

AI steps:

1. Verify user identity and role through backend.
2. Get pending approval by document number.
3. Confirm if multiple matches.
4. Call `submit_approval_command`.
5. Backend approves and logs action.
6. If policy allows auto-post after approval, backend posts; otherwise user must explicitly post.

## 10. Guardrails

AI must refuse or redirect when:

- user is not authorized;
- user asks to bypass approval;
- user asks to modify posted journal directly;
- user asks for tax manipulation or fraudulent reporting;
- user asks for unverifiable financial numbers.

## 11. Response style

Use Indonesian. Be concise.

Example report response:

```text
Ringkasan kas per 2 Juni 2026:

Total kas: Rp1.250.000.000

Rincian:
- BCA Operasional: Rp900.000.000
- Mandiri: Rp300.000.000
- Kas kecil: Rp50.000.000

Status: data dari transaksi posted.
```

Example draft response:

```text
Draft berhasil dibuat, belum diposting.

No draft: PINV-DRAFT-00012
Supplier: CV Maju Bersama
Total: Rp2.608.500
Jurnal disarankan sudah balance.

Langkah berikutnya: review dan approval.
```
