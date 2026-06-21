# Security and Audit Requirements

## 1. Security principles

- Least privilege.
- AI output is untrusted.
- Backend validates all accounting logic.
- No direct database mutation from OpenClaw.
- No direct posting without approval.
- All sensitive actions are logged.

## 2. Authentication

Use JWT:

- short-lived access token;
- refresh token;
- refresh token rotation;
- revoke on logout/password change.

## 3. Authorization

Use RBAC.

Default roles:

```text
Owner
FinanceManager
AccountingStaff
TaxStaff
Auditor
Admin
AIService
```

## 4. Permission examples

```text
reports.read.all
invoice.create.draft
invoice.submit_approval
invoice.approve
invoice.post
journal.create.draft
journal.approve
journal.post
tax.read
tax.configure
user.manage
ai.tool.call
```

## 5. OpenClaw security

OpenClaw calls backend through service token.

Requirements:

- dedicated `AIService` role;
- API key or JWT client credential;
- IP allowlist if possible;
- request signing optional but recommended;
- idempotency key for mutation endpoints;
- rate limits;
- audit every tool call.

## 6. Chat identity mapping

Map external chat identity to internal user:

```text
channel: whatsapp | telegram
external_user_id: phone number / telegram id
user_id: internal UUID
status: active/inactive
```

Only mapped and active users can use financial chat.

## 7. Sensitive data handling

- Do not expose financial report to unauthorized phone/chat ID.
- Mask NPWP/NIK in UI unless user has permission.
- Store attachment in restricted storage path.
- Use HTTPS only.
- Use encrypted database backups.

## 8. Audit log events

Must log:

- login/logout;
- master data create/update/delete;
- draft transaction create/update;
- approval submit/approve/reject;
- posting;
- reversal;
- tax config change;
- AI tool call;
- report query through AI;
- document upload/download.

## 9. Audit log fields

```text
actor_user_id
actor_type: User | AIService | System
entity_type
entity_id
action
before_json
after_json
ip_address
user_agent
created_at
```

## 10. AI safety guardrails

Backend must reject AI requests when:

- caller is not authenticated as AIService;
- mapped user has no permission;
- transaction is invalid;
- journal is not balanced;
- tax type not active/effective;
- approval is missing;
- document already posted;
- idempotency key duplicated incorrectly.

## 11. Immutable posting

Posted documents:

- cannot be updated through normal update endpoints;
- require reversal or adjustment;
- must preserve original journal lines.

## 12. Backup and retention

Minimum:

- daily database backup;
- weekly full backup;
- retain at least 30 days for MVP;
- monthly archive optional;
- test restore process.

## 13. Compliance note

This system helps accounting and tax administration. Final tax compliance must be reviewed by authorized finance/tax staff and aligned with current regulations.
