import { computed } from 'vue'
import type { DocumentStatus } from '@/types/journal.types'

const STATUS_LABELS: Record<DocumentStatus, string> = {
  draft: 'Draft',
  waiting_review: 'Waiting Review',
  waiting_approval: 'Waiting Approval',
  approved: 'Approved',
  posted: 'Posted',
  rejected: 'Rejected',
  cancelled: 'Cancelled',
}

const STATUS_COLORS: Record<DocumentStatus, string> = {
  draft: 'var(--color-status-draft)',
  waiting_review: 'var(--color-status-waiting)',
  waiting_approval: 'var(--color-status-waiting)',
  approved: 'var(--color-status-approved)',
  posted: 'var(--color-status-posted)',
  rejected: 'var(--color-status-rejected)',
  cancelled: 'var(--color-status-cancelled)',
}

/**
 * Composable for document status display helpers.
 */
export function useDocumentStatus(status: DocumentStatus) {
  const label = computed(() => STATUS_LABELS[status] ?? status)
  const color = computed(() => STATUS_COLORS[status] ?? 'gray')
  const isEditable = computed(() => status === 'draft' || status === 'waiting_review')
  const isPosted    = computed(() => status === 'posted')
  const isPending   = computed(() =>
    status === 'waiting_review' || status === 'waiting_approval'
  )

  return { label, color, isEditable, isPosted, isPending }
}
