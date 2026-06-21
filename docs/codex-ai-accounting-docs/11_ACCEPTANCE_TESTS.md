# Acceptance Tests

## 1. Accounting tests

### Test: balanced journal can be posted

Given a draft journal with total debit equal total credit
When user with permission posts the journal
Then status becomes Posted
And posting date is set
And audit log is created

### Test: unbalanced journal cannot be posted

Given a draft journal with debit not equal credit
When user tries to post
Then API returns `JOURNAL_NOT_BALANCED`
And status remains Draft

### Test: posted journal cannot be edited

Given a Posted journal
When user tries to update journal lines
Then API returns `POSTED_DOCUMENT_IMMUTABLE`

## 2. Sales invoice tests

### Test: sales invoice creates AR journal

Given approved sales invoice with subtotal and PPN
When posted
Then journal lines are created:

- Debit AR total invoice
- Credit revenue subtotal
- Credit PPN output tax

## 3. Purchase invoice tests

### Test: purchase invoice creates AP journal

Given approved purchase invoice with subtotal and PPN
When posted
Then journal lines are created:

- Debit expense/inventory subtotal
- Debit PPN input tax
- Credit AP total invoice

### Test: duplicate supplier invoice detected

Given existing purchase invoice with same supplier and invoice number
When user creates another draft
Then system warns duplicate or rejects based on policy

## 4. Payment tests

### Test: payment received reduces AR

Given posted sales invoice with outstanding amount
When payment received is posted
Then AR outstanding decreases
And cash/bank balance increases

### Test: partial payment works

Given invoice total Rp10.000.000
When payment Rp4.000.000 is posted
Then outstanding balance is Rp6.000.000

## 5. Tax tests

### Test: PPN uses effective tax config

Given tax type effective for transaction date
When invoice is created
Then tax calculation uses matching config rate

### Test: tax summary only uses posted records

Given draft and posted invoices
When tax summary is generated
Then draft invoice tax is excluded

## 6. Approval tests

### Test: approval required

Given invoice in Draft
When user tries to post directly
Then API returns `APPROVAL_REQUIRED`

### Test: authorized manager can approve

Given approval request waiting approval
When FinanceManager approves
Then approval status becomes Approved
And audit log is created

### Test: unauthorized user cannot approve

Given approval request
When AccountingStaff without approval permission approves
Then API returns 403

## 7. AI/OpenClaw tests

### Test: AI query report uses backend

Given user asks cash position
When OpenClaw calls query-report endpoint
Then backend returns cash numbers
And ai_tool_call is logged

### Test: AI cannot post transaction

Given AI service calls post endpoint without user approval
Then API returns `APPROVAL_REQUIRED`

### Test: unmapped chat user blocked

Given external chat user not mapped to active internal user
When query report called
Then API returns 403

## 8. Frontend tests

### Test: invoice automation page validates fields

Given missing invoice number
When user submits draft
Then UI shows validation error

### Test: approval page shows journal preview

Given approval request for invoice
When user opens approval detail
Then document summary, attachment, journal lines, and tax records are visible

## 9. Performance acceptance

MVP target:

- Dashboard loads under 3 seconds for common datasets.
- Report query under 5 seconds for 1 year transaction data.
- Chat response under 10 seconds excluding AI provider latency.
