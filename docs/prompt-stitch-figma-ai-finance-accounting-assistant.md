# Prompt UI/UX untuk Stitch / Figma AI

## Project

**AI Finance & Accounting Assistant**

Aplikasi ini adalah sistem asisten keuangan dan akuntansi berbasis AI untuk owner usaha, tim finance, dan tim accounting. Tujuan utamanya adalah membantu owner usaha memahami kondisi keuangan perusahaan secara cepat tanpa menunggu laporan manual, sekaligus membantu tim accounting mengotomasi input invoice, draft jurnal, pembayaran, pencatatan pajak, approval, dan laporan keuangan.

---

## Prompt Utama

Design a modern, clean, and easy-to-use web application UI/UX for a SaaS product called **AI Finance & Accounting Assistant**.

The application is built for Indonesian business owners, finance teams, and accounting staff. The main goal is to help business owners understand their company’s financial condition quickly without waiting for manual reports, while also helping accounting teams automate invoice input, journal drafts, payments, tax records, approvals, and financial reporting.

Use a modern SaaS design style commonly used by today’s top application designers: clean layout, rounded cards, soft shadows, spacious white space, responsive dashboard, minimal but professional icons, clear typography, and simple navigation. The design should feel premium, trustworthy, modern, and easy to understand for non-technical business owners.

---

## Brand Personality

- Professional
- Secure
- Intelligent
- Easy to use
- Finance-focused
- AI-powered
- Business owner friendly

---

## Color Direction

- Primary: dark navy / deep blue for trust and finance
- Secondary: teal / green for growth, cashflow, and positive indicators
- Neutral: white, light gray, slate gray
- Accent: orange/yellow for warnings
- Red for risks, overdue items, rejected status, or danger state
- Use subtle gradients only where appropriate, not too flashy

---

## Suggested Color Palette

```text
Primary Navy: #0B172A
Secondary Navy: #10243F
Teal Accent: #14B8A6
Green Success: #22C55E
Yellow Warning: #F59E0B
Red Danger: #EF4444
Background: #F8FAFC
Card Background: #FFFFFF
Text Primary: #0F172A
Text Secondary: #64748B
Border: #E2E8F0
```

---

## Typography

- Use modern sans-serif typography
- Large readable numbers for financial KPIs
- Clear hierarchy between titles, labels, and values
- Make the UI readable for business owners, not only accountants
- Avoid dense accounting-only terminology where possible

---

## Main Layout

Create a desktop web dashboard layout with:

- Left sidebar navigation
- Top header
- Main content area
- Responsive dashboard cards
- Clean tables
- Modern filter components
- Mobile chat assistant concept

Also include a mobile chat screen concept for WhatsApp/Telegram-style financial assistant interaction.

---

## Sidebar Menu

- Dashboard
- Cashflow
- Transactions
- Receivables
- Payables
- Invoices
- Payments
- Journal Entries
- Tax
- Reports
- Approvals
- AI Assistant
- Settings

---

## Dashboard Page

Create a main dashboard for business owners with these KPI cards:

- Cash / Kas
- Receivables / Piutang Usaha
- Payables / Hutang Usaha
- Monthly Revenue / Pendapatan Bulan Ini
- Net Profit / Laba Bersih
- Operating Expenses / Beban Operasional
- Tax Payable / Pajak Terutang
- Pending Approvals / Menunggu Persetujuan

### Dashboard Components

- Cashflow line chart
- Revenue vs expense chart
- Receivables aging summary
- Payables due soon summary
- Expense by category donut chart
- AI insights card
- Recent transactions table
- Recent approvals table
- Warning cards for overdue receivables, low cash balance, unpaid taxes, and pending documents

---

## AI Assistant Page

Design a chat-style interface where the owner can ask financial questions in natural Indonesian language.

### Example User Questions

- “Berapa posisi kas hari ini?”
- “Piutang yang jatuh tempo siapa saja?”
- “Berapa laba bersih bulan ini?”
- “Beban operasional terbesar bulan ini apa?”
- “Pajak yang harus dibayar bulan ini berapa?”
- “Apakah cashflow kita aman untuk 3 bulan ke depan?”

### AI Response Style

- Short and structured
- Shows financial numbers clearly
- Uses small KPI cards inside chat
- Includes quick action buttons such as:
  - Lihat Detail
  - Tampilkan Grafik
  - Download Laporan
  - Buat Reminder
  - Kirim ke Owner

---

## Invoice Automation Page

Create a page for uploading and processing invoices using AI/OCR.

### Layout

- Invoice PDF/image preview on the left
- Extracted invoice data on the right
- Confidence score for each extracted field
- Validation status
- Suggested accounting journal entry
- Approval workflow timeline

### Extracted Fields

- Vendor
- Invoice number
- Invoice date
- Due date
- Subtotal
- PPN
- PPh if applicable
- Total invoice
- Payment terms
- Bank account

### Validation Checks

- Duplicate invoice detection
- Vendor validation
- NPWP validation
- Tax calculation validation
- Total amount validation
- Missing document warning

### Suggested Journal Example

```text
Debit: Beban Operasional
Debit: PPN Masukan
Credit: Hutang Usaha
```

### Approval Flow

- Draft
- Review
- Approved
- Posted
- Rejected

---

## Approvals Page

Design an approval center for owner/finance manager.

### Include

- Pending invoice approvals
- Pending journal approvals
- Pending payment approvals
- Pending tax approvals
- Approval detail drawer/modal
- Approve button
- Reject button
- Request revision button
- Comment box
- Audit trail section

---

## Reports Page

Create a modern financial reports page with tabs:

- Laba Rugi
- Neraca
- Arus Kas
- Piutang
- Hutang
- Pajak
- Expense Analysis
- Cashflow Forecast

### Each Report Should Have

- Period filter
- Export PDF/Excel button
- Summary cards
- Chart visualization
- Table detail
- AI summary insight

---

## Tax Page

Design a tax management dashboard for Indonesian business context.

### Include

- PPN Keluaran
- PPN Masukan
- Pajak Terutang
- PPh 21
- PPh 23
- PPh 25
- Tax due date calendar
- Compliance warning
- Missing document alerts
- Tax report status
- Reminder card

---

## Transactions Page

Design a clean transaction list page.

### Include

- Search bar
- Date filter
- Account filter
- Transaction type filter
- Status filter

### Table Columns

- Date
- Transaction number
- Type
- Customer/Vendor
- Amount
- Status
- Created by
- Approval status

Add a detail panel when a transaction is selected.

---

## Journal Entries Page

Design a professional accounting journal page.

### Include

- Journal entry list
- Create journal button
- Journal detail form
- Debit and credit table
- Auto-balance indicator
- AI suggested journal badge
- Posting status
- Audit log

---

## Mobile Experience

Create a mobile-friendly chat assistant screen that looks like WhatsApp/Telegram but branded as **AI Finance Assistant**.

### Mobile Chat Should Show

- Owner asking questions
- AI answering with financial summary cards
- Quick action buttons
- Secure access notice
- Simple input field

---

## UX Principles

- The owner should understand the dashboard in less than 10 seconds
- Use plain business language, not overly technical accounting terms
- Important numbers must be large and clear
- Use color indicators for increase, decrease, warning, and risk
- Avoid clutter
- Prioritize dashboard clarity and approval safety
- Every AI-generated transaction must clearly show **Draft** before approval
- Do not make AI appear to post accounting data without human approval
- Keep workflows simple and obvious
- Make all approval actions visible and traceable

---

## Screens to Generate

1. Login screen
2. Main owner dashboard
3. AI chat assistant page
4. Invoice automation page
5. Approval center page
6. Financial reports page
7. Tax management page
8. Transaction list page
9. Journal entry page
10. Mobile AI chat assistant screen

---

## Preferred Style References

- Modern fintech SaaS dashboard
- Clean accounting software UI
- Linear-inspired layout
- Stripe-inspired spacing and polish
- Ramp/Brex-inspired fintech clarity
- Xero/QuickBooks-inspired accounting workflow
- Soft rounded cards
- Minimal icons
- Professional finance analytics dashboard
- Dark navy sidebar with light main content
- Responsive and accessible interface

---

## Short Visual Direction

```text
Modern fintech SaaS dashboard, dark navy sidebar, white content area, teal-green accent, clean cards, large financial KPI numbers, AI chat interface, invoice automation workflow, approval-first accounting system, owner-friendly UX.
```

---

## Output Expected

Output the result as a complete UI/UX design system and high-fidelity app screens.

The final design should include:

- Design system
- Color palette
- Typography
- Components
- Desktop dashboard screens
- Mobile AI chat screen
- Empty state examples
- Loading state examples
- Approval workflow UI
- Error and warning states
- Responsive layout direction
