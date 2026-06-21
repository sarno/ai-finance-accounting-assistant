// ─── Document Status ──────────────────────────────────────────────────────────

export type DocumentStatus =
  | 'draft'
  | 'waiting_review'
  | 'waiting_approval'
  | 'approved'
  | 'posted'
  | 'rejected'
  | 'cancelled'

// ─── Journal Types ────────────────────────────────────────────────────────────

export interface JournalLine {
  id: string
  accountId: string
  debit: number
  credit: number
  description?: string
}

export interface JournalEntry {
  id: string
  companyId: string
  branchId?: string
  referenceNumber: string
  description: string
  transactionDate: string
  lines: JournalLine[]
  status: DocumentStatus
  source: string
  createdBy: string
  createdAt: string
}

// ─── Request Types ────────────────────────────────────────────────────────────

export interface CreateJournalLineRequest {
  accountId: string
  debit: number
  credit: number
  description?: string
}

export interface CreateJournalDraftRequest {
  description: string
  transactionDate: string
  lines: CreateJournalLineRequest[]
  referenceNumber?: string
  branchId?: string
}

export interface JournalFilters {
  page: number
  perPage: number
  status?: DocumentStatus
  dateFrom?: string
  dateTo?: string
  q?: string
}

// ─── Pagination ───────────────────────────────────────────────────────────────

export interface PaginatedResponse<T> {
  data: T[]
  total: number
  page: number
  perPage: number
}
