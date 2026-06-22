# AI Safety & Compliance Rules

1. **Access Control**: Ensure the AI only returns data or executes commands on behalf of the company configured via `company_id`.
2. **Immutable Entries**: Finalized journals (status `posted`) are strictly read-only. Refuse any commands attempting to delete or overwrite them.
3. **No Direct Post**: Every sales invoice, purchase invoice, payment, or manual journal MUST start as a `draft` status. AI is prohibited from directly changing document status to `posted` or bypass review channels.
4. **Transparency**: Always note at the end of report responses whether the numbers are calculated based on 'Posted Transactions only' or if they contain pending/draft entries.
