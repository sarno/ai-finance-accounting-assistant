# Accounting and Tax Domain Specification

## 1. Accounting foundation

The system uses double-entry accounting.

Every posted transaction must create one journal entry with multiple journal lines.

Rule:

```text
Total Debit = Total Credit
```

If not balanced, posting must fail.

## 2. Chart of Accounts

Minimum account categories:

- Asset
- Liability
- Equity
- Revenue
- Cost of Goods Sold
- Expense
- Other Income
- Other Expense

Recommended account groups:

```text
1000 Assets
  1100 Cash and Bank
  1200 Accounts Receivable
  1300 Inventory
  1400 Prepaid Expense

2000 Liabilities
  2100 Accounts Payable
  2200 Tax Payable
  2300 Accrued Expense

3000 Equity
  3100 Owner Capital
  3200 Retained Earnings

4000 Revenue
  4100 Sales Revenue

5000 Cost of Goods Sold

6000 Expenses
  6100 Salary Expense
  6200 Rent Expense
  6300 Office Expense

7000 Other Income
8000 Other Expense
```

## 3. Document lifecycle

All major transaction documents use status:

```text
Draft
WaitingApproval
Approved
Posted
Rejected
Cancelled
```

Allowed transitions:

```text
Draft -> WaitingApproval
WaitingApproval -> Approved
WaitingApproval -> Rejected
Approved -> Posted
Draft -> Cancelled
Rejected -> Draft
```

Posted documents are immutable. Correction uses reversal/adjustment.

## 4. Sales invoice posting

Example sale on credit with VAT/PPN:

```text
Dr Accounts Receivable      total invoice
Cr Sales Revenue            subtotal
Cr VAT/PPN Output Payable   output tax
```

Payment received:

```text
Dr Cash/Bank                payment amount
Cr Accounts Receivable      payment amount
```

## 5. Purchase invoice posting

Example purchase on credit with VAT/PPN input:

```text
Dr Expense/Inventory        subtotal
Dr VAT/PPN Input            input tax
Cr Accounts Payable         total invoice
```

Payment paid:

```text
Dr Accounts Payable         payment amount
Cr Cash/Bank                payment amount
```

## 6. Expense transaction

Cash expense:

```text
Dr Expense Account          amount before tax
Dr VAT/PPN Input            input tax, if applicable
Cr Cash/Bank                total paid
```

## 7. Manual journal

Manual journal must require:

- transaction date;
- description;
- at least two journal lines;
- balanced debit/credit;
- attachment if required by policy;
- approval before posting.

## 8. Tax module scope

The tax module records and summarizes tax data. It does not replace official tax reporting tools unless integrated in later phases.

Minimum tax types:

- PPN Keluaran / VAT Output
- PPN Masukan / VAT Input
- PPh 21
- PPh 23
- PPh 25
- PPh Final
- Other withholding taxes configurable by admin

## 9. Tax configuration

Tax configuration must be stored in database:

```text
tax_types
  id
  code
  name
  category
  default_rate
  effective_from
  effective_to
  payable_account_id
  receivable_account_id
  expense_account_id
  is_active
```

Important: do not hardcode tax rates in code. Indonesian tax rules may change. Use effective date versions.

## 10. VAT/PPN handling

PPN behavior:

- Sales invoice may generate PPN Keluaran.
- Purchase invoice may generate PPN Masukan.
- Tax summary calculates net VAT payable or overpaid.
- PPN rate must come from tax config.
- Luxury goods and special cases must be supported later by tax rule configuration.

## 11. Withholding tax handling

For PPh withholding:

- Store withholding tax record per transaction.
- Store counterparty NPWP/NIK when available.
- Store tax object / service type.
- Store withholding rate and base amount.
- Generate journal lines based on configuration.

Example supplier payment with PPh 23 withheld:

```text
Dr Accounts Payable         gross payable
Cr Cash/Bank                net paid
Cr PPh 23 Payable           withholding amount
```

## 12. Tax document status

Each tax-related transaction should support:

```text
NotRequired
Required
Drafted
Validated
Reported
Paid
Archived
```

## 13. Tax calendar

System should store due dates:

- tax type;
- period;
- due date;
- payment status;
- report status;
- reminder schedule.

## 14. AI tax limitation

AI can help summarize and detect missing tax documents, but final tax treatment must be configurable and reviewable by finance/tax staff.

AI must not invent tax rates or decide complex tax treatment without approval.

## 15. Closing period

When accounting period is closed:

- no new posting allowed in closed period;
- correction must use next open period or special admin reopen approval;
- closing action must be audited.
