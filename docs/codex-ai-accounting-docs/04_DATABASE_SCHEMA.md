# Database Schema Draft — PostgreSQL

## 1. Naming conventions

- Tables: plural, snake_case.
- Columns: snake_case.
- Primary key: `id uuid`.
- Timestamps: `created_at`, `updated_at`, `deleted_at`.
- Money: `numeric(18,2)` or `numeric(20,4)` if needed.
- Use `currency_code varchar(3)` default `IDR`.

## 2. Core tables

### companies

```sql
create table companies (
  id uuid primary key,
  name text not null,
  legal_name text,
  npwp text,
  address text,
  base_currency varchar(3) not null default 'IDR',
  fiscal_year_start_month int not null default 1,
  created_at timestamptz not null,
  updated_at timestamptz not null
);
```

### users

```sql
create table users (
  id uuid primary key,
  company_id uuid not null references companies(id),
  full_name text not null,
  email text not null,
  phone text,
  password_hash text,
  is_active boolean not null default true,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  unique(company_id, email)
);
```

### roles and user_roles

```sql
create table roles (
  id uuid primary key,
  company_id uuid references companies(id),
  name text not null,
  description text,
  created_at timestamptz not null
);

create table user_roles (
  user_id uuid not null references users(id),
  role_id uuid not null references roles(id),
  primary key(user_id, role_id)
);
```

### chart_of_accounts

```sql
create table chart_of_accounts (
  id uuid primary key,
  company_id uuid not null references companies(id),
  code text not null,
  name text not null,
  account_type text not null,
  parent_id uuid references chart_of_accounts(id),
  normal_balance text not null,
  is_cash_account boolean not null default false,
  is_bank_account boolean not null default false,
  is_active boolean not null default true,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  unique(company_id, code)
);
```

Allowed `account_type`:

```text
Asset, Liability, Equity, Revenue, COGS, Expense, OtherIncome, OtherExpense
```

### customers

```sql
create table customers (
  id uuid primary key,
  company_id uuid not null references companies(id),
  code text,
  name text not null,
  npwp text,
  email text,
  phone text,
  billing_address text,
  is_active boolean not null default true,
  created_at timestamptz not null,
  updated_at timestamptz not null
);
```

### suppliers

```sql
create table suppliers (
  id uuid primary key,
  company_id uuid not null references companies(id),
  code text,
  name text not null,
  npwp text,
  email text,
  phone text,
  address text,
  is_active boolean not null default true,
  created_at timestamptz not null,
  updated_at timestamptz not null
);
```

### bank_accounts

```sql
create table bank_accounts (
  id uuid primary key,
  company_id uuid not null references companies(id),
  account_id uuid not null references chart_of_accounts(id),
  bank_name text,
  account_number text,
  account_holder text,
  currency_code varchar(3) not null default 'IDR',
  is_active boolean not null default true,
  created_at timestamptz not null,
  updated_at timestamptz not null
);
```

## 3. Tax tables

### tax_types

```sql
create table tax_types (
  id uuid primary key,
  company_id uuid not null references companies(id),
  code text not null,
  name text not null,
  category text not null,
  default_rate numeric(9,6),
  effective_from date not null,
  effective_to date,
  payable_account_id uuid references chart_of_accounts(id),
  receivable_account_id uuid references chart_of_accounts(id),
  expense_account_id uuid references chart_of_accounts(id),
  is_active boolean not null default true,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  unique(company_id, code, effective_from)
);
```

### tax_records

```sql
create table tax_records (
  id uuid primary key,
  company_id uuid not null references companies(id),
  tax_type_id uuid not null references tax_types(id),
  source_document_type text not null,
  source_document_id uuid not null,
  tax_period date not null,
  tax_base_amount numeric(18,2) not null,
  tax_rate numeric(9,6) not null,
  tax_amount numeric(18,2) not null,
  status text not null,
  counterparty_name text,
  counterparty_npwp text,
  created_at timestamptz not null,
  updated_at timestamptz not null
);
```

## 4. Journal tables

### journal_entries

```sql
create table journal_entries (
  id uuid primary key,
  company_id uuid not null references companies(id),
  journal_no text not null,
  transaction_date date not null,
  posting_date timestamptz,
  source_document_type text,
  source_document_id uuid,
  description text,
  status text not null,
  created_by uuid references users(id),
  posted_by uuid references users(id),
  created_at timestamptz not null,
  updated_at timestamptz not null,
  unique(company_id, journal_no)
);
```

### journal_lines

```sql
create table journal_lines (
  id uuid primary key,
  journal_entry_id uuid not null references journal_entries(id),
  account_id uuid not null references chart_of_accounts(id),
  description text,
  debit numeric(18,2) not null default 0,
  credit numeric(18,2) not null default 0,
  currency_code varchar(3) not null default 'IDR',
  line_no int not null
);
```

Constraint to add in code/db:

```text
Debit and credit cannot both be positive.
Debit and credit cannot both be zero.
Journal total debit must equal total credit before posting.
```

## 5. Sales tables

### sales_invoices

```sql
create table sales_invoices (
  id uuid primary key,
  company_id uuid not null references companies(id),
  customer_id uuid not null references customers(id),
  invoice_no text not null,
  invoice_date date not null,
  due_date date,
  subtotal numeric(18,2) not null,
  tax_amount numeric(18,2) not null default 0,
  total_amount numeric(18,2) not null,
  paid_amount numeric(18,2) not null default 0,
  status text not null,
  created_by uuid references users(id),
  created_at timestamptz not null,
  updated_at timestamptz not null,
  unique(company_id, invoice_no)
);
```

### sales_invoice_lines

```sql
create table sales_invoice_lines (
  id uuid primary key,
  sales_invoice_id uuid not null references sales_invoices(id),
  description text not null,
  quantity numeric(18,4) not null,
  unit_price numeric(18,2) not null,
  tax_type_id uuid references tax_types(id),
  tax_amount numeric(18,2) not null default 0,
  line_total numeric(18,2) not null,
  revenue_account_id uuid references chart_of_accounts(id),
  line_no int not null
);
```

## 6. Purchase tables

### purchase_invoices

```sql
create table purchase_invoices (
  id uuid primary key,
  company_id uuid not null references companies(id),
  supplier_id uuid not null references suppliers(id),
  invoice_no text not null,
  invoice_date date not null,
  due_date date,
  subtotal numeric(18,2) not null,
  tax_amount numeric(18,2) not null default 0,
  total_amount numeric(18,2) not null,
  paid_amount numeric(18,2) not null default 0,
  status text not null,
  source_document_id uuid,
  ai_confidence numeric(5,2),
  created_by uuid references users(id),
  created_at timestamptz not null,
  updated_at timestamptz not null
);
```

### purchase_invoice_lines

```sql
create table purchase_invoice_lines (
  id uuid primary key,
  purchase_invoice_id uuid not null references purchase_invoices(id),
  description text not null,
  quantity numeric(18,4) not null,
  unit_price numeric(18,2) not null,
  tax_type_id uuid references tax_types(id),
  tax_amount numeric(18,2) not null default 0,
  line_total numeric(18,2) not null,
  expense_account_id uuid references chart_of_accounts(id),
  line_no int not null
);
```

## 7. Payment tables

```sql
create table payments (
  id uuid primary key,
  company_id uuid not null references companies(id),
  payment_no text not null,
  payment_type text not null,
  counterparty_type text not null,
  counterparty_id uuid,
  payment_date date not null,
  bank_account_id uuid not null references bank_accounts(id),
  amount numeric(18,2) not null,
  status text not null,
  description text,
  created_by uuid references users(id),
  created_at timestamptz not null,
  updated_at timestamptz not null,
  unique(company_id, payment_no)
);

create table payment_allocations (
  id uuid primary key,
  payment_id uuid not null references payments(id),
  document_type text not null,
  document_id uuid not null,
  amount numeric(18,2) not null
);
```

## 8. Approval tables

```sql
create table approval_requests (
  id uuid primary key,
  company_id uuid not null references companies(id),
  document_type text not null,
  document_id uuid not null,
  status text not null,
  requested_by uuid references users(id),
  requested_at timestamptz not null,
  approved_by uuid references users(id),
  approved_at timestamptz,
  rejected_by uuid references users(id),
  rejected_at timestamptz,
  rejection_reason text
);

create table approval_actions (
  id uuid primary key,
  approval_request_id uuid not null references approval_requests(id),
  actor_user_id uuid references users(id),
  action text not null,
  comment text,
  created_at timestamptz not null
);
```

## 9. Document and AI tables

```sql
create table uploaded_documents (
  id uuid primary key,
  company_id uuid not null references companies(id),
  original_file_name text not null,
  storage_path text not null,
  mime_type text,
  size_bytes bigint,
  document_type text,
  uploaded_by uuid references users(id),
  uploaded_at timestamptz not null
);

create table ai_conversations (
  id uuid primary key,
  company_id uuid not null references companies(id),
  channel text not null,
  external_user_id text,
  user_id uuid references users(id),
  started_at timestamptz not null,
  last_message_at timestamptz
);

create table ai_messages (
  id uuid primary key,
  conversation_id uuid not null references ai_conversations(id),
  role text not null,
  content text not null,
  created_at timestamptz not null
);

create table ai_tool_calls (
  id uuid primary key,
  conversation_id uuid references ai_conversations(id),
  tool_name text not null,
  request_json jsonb not null,
  response_json jsonb,
  status text not null,
  created_at timestamptz not null
);
```

## 10. Audit logs

```sql
create table audit_logs (
  id uuid primary key,
  company_id uuid not null references companies(id),
  actor_user_id uuid references users(id),
  actor_type text not null,
  action text not null,
  entity_type text not null,
  entity_id uuid,
  before_json jsonb,
  after_json jsonb,
  ip_address text,
  user_agent text,
  created_at timestamptz not null
);
```

## 11. Index suggestions

```sql
create index idx_journal_entries_company_date on journal_entries(company_id, transaction_date);
create index idx_journal_lines_account on journal_lines(account_id);
create index idx_sales_invoices_company_status on sales_invoices(company_id, status);
create index idx_purchase_invoices_company_status on purchase_invoices(company_id, status);
create index idx_tax_records_company_period on tax_records(company_id, tax_period);
create index idx_audit_logs_entity on audit_logs(entity_type, entity_id);
```
