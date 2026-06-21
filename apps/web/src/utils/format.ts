/**
 * Format a number as Indonesian Rupiah.
 * @example formatIDR(1250000) → "Rp 1.250.000"
 */
export function formatIDR(value: number): string {
  return new Intl.NumberFormat('id-ID', {
    style: 'currency',
    currency: 'IDR',
    minimumFractionDigits: 0,
    maximumFractionDigits: 0,
  }).format(value)
}

/**
 * Format ISO date string to localized date.
 */
export function formatDate(isoDate: string): string {
  return new Intl.DateTimeFormat('id-ID', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  }).format(new Date(isoDate))
}

/**
 * Format ISO datetime string to localized datetime.
 */
export function formatDateTime(isoDate: string): string {
  return new Intl.DateTimeFormat('id-ID', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  }).format(new Date(isoDate))
}

/**
 * Validate that journal lines are balanced (debit === credit).
 */
export function isJournalBalanced(
  lines: { debit: number; credit: number }[]
): boolean {
  const totalDebit  = lines.reduce((s, l) => s + l.debit, 0)
  const totalCredit = lines.reduce((s, l) => s + l.credit, 0)
  return Math.abs(totalDebit - totalCredit) < 0.001
}
