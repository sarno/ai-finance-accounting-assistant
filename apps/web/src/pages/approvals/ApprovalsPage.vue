<template>
  <MainLayout>
    <div class="page-header">
      <div>
        <h1>Pending Approvals</h1>
        <p class="page-title-desc">Review and approve transaction postings, journal adjustments, and invoices.</p>
      </div>
    </div>

    <!-- Notifications -->
    <div v-if="successMsg" class="alert alert-success">{{ successMsg }}</div>
    <div v-if="errorMsg" class="alert alert-danger">{{ errorMsg }}</div>

    <!-- Approvals List Table -->
    <div class="table-container">
      <div v-if="approvalStore.loading && approvalStore.pendingApprovals.length === 0" class="loading-state">
        Loading pending approval requests...
      </div>
      <div v-else-if="approvalStore.pendingApprovals.length === 0" class="empty-state">
        No pending approvals found. Excellent!
      </div>
      <table v-else>
        <thead>
          <tr>
            <th>Date Requested</th>
            <th>Requested By</th>
            <th>Document Type</th>
            <th>Document ID</th>
            <th>Status</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="req in approvalStore.pendingApprovals" :key="req.id">
            <td>{{ formatDate(req.createdAt) }}</td>
            <td>{{ req.requestedBy || 'System' }}</td>
            <td>
              <span class="badge badge-info" style="text-transform: capitalize;">
                {{ formatDocType(req.documentType) }}
              </span>
            </td>
            <td style="font-family: monospace; font-size: 0.8rem; color: var(--text-secondary);">
              {{ req.documentId }}
            </td>
            <td>
              <span class="badge badge-warning">
                {{ req.status }}
              </span>
            </td>
            <td>
              <button class="btn btn-primary btn-xs" @click="viewAndProcess(req)">
                Review & Action
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Review & Action Dialog Modal -->
    <div v-if="showProcessModal && activeRequest" class="modal-overlay" @click.self="closeProcessModal">
      <div class="modal-content modal-lg-custom">
        <div class="modal-header">
          <h2>Review Approval Request</h2>
          <button class="modal-close" @click="closeProcessModal">&times;</button>
        </div>
        <div class="modal-body">
          <!-- Request Metadata -->
          <div class="meta-card">
            <div>
              <p class="meta-label">Requested By</p>
              <p class="meta-val">{{ activeRequest.requestedBy || 'System' }}</p>
            </div>
            <div>
              <p class="meta-label">Request Date</p>
              <p class="meta-val">{{ formatDate(activeRequest.createdAt) }}</p>
            </div>
            <div>
              <p class="meta-label">Doc Type</p>
              <p class="meta-val" style="text-transform: capitalize;">{{ formatDocType(activeRequest.documentType) }}</p>
            </div>
          </div>

          <!-- Document Contents Loader -->
          <div style="margin-top: 24px;">
            <h3>Document Details</h3>
            
            <div v-if="loadingDoc" class="loading-state">
              Fetching document details...
            </div>
            <div v-else-if="docLoadError" class="alert alert-danger" style="margin-top: 8px;">
              {{ docLoadError }}
            </div>
            
            <!-- Journal Entry Details Display -->
            <div v-else-if="activeRequest.documentType === 'journal_entry' && journalDetails" class="doc-details-box">
              <div class="detail-grid" style="margin-bottom: 16px;">
                <div>
                  <p class="detail-label">Reference Number</p>
                  <p class="detail-val-highlight">{{ journalDetails.referenceNumber }}</p>
                </div>
                <div>
                  <p class="detail-label">Transaction Date</p>
                  <p class="detail-val">{{ formatDate(journalDetails.transactionDate) }}</p>
                </div>
                <div>
                  <p class="detail-label">Description</p>
                  <p class="detail-val">{{ journalDetails.description }}</p>
                </div>
              </div>

              <h4>Ledger Lines</h4>
              <div class="table-container" style="margin-top: 8px;">
                <table>
                  <thead>
                    <tr>
                      <th>Account Code & Name</th>
                      <th style="text-align: right;">Debit</th>
                      <th style="text-align: right;">Credit</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr v-for="line in journalDetails.lines" :key="line.id">
                      <td>{{ getAccountName(line.accountId) }}</td>
                      <td style="text-align: right; font-weight: 500;">
                        {{ line.debit > 0 ? formatCurrency(line.debit) : '-' }}
                      </td>
                      <td style="text-align: right; font-weight: 500;">
                        {{ line.credit > 0 ? formatCurrency(line.credit) : '-' }}
                      </td>
                    </tr>
                    <tr class="totals-row">
                      <td style="font-weight: 600;">Total</td>
                      <td style="text-align: right; font-weight: 700;">
                        {{ formatCurrency(getJournalTotal(journalDetails, 'debit')) }}
                      </td>
                      <td style="text-align: right; font-weight: 700;">
                        {{ formatCurrency(getJournalTotal(journalDetails, 'credit')) }}
                      </td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </div>

            <!-- Fallback for other doc types -->
            <div v-else class="empty-state" style="margin-top: 8px;">
              Details parser not implemented for document type: {{ activeRequest.documentType }}
            </div>
          </div>

          <!-- Approval Form Comment -->
          <div class="form-group" style="margin-top: 24px;">
            <label class="form-label">Review Comment / Reason *</label>
            <textarea
              v-model="comment"
              class="form-textarea"
              rows="3"
              placeholder="Enter approval details or rejection reasons..."
              required
            ></textarea>
          </div>
        </div>
        <div class="modal-footer">
          <button type="button" class="btn btn-secondary" @click="closeProcessModal">Cancel</button>
          
          <button
            type="button"
            class="btn btn-danger"
            :disabled="submitting || !comment.trim()"
            @click="handleAction('reject')"
          >
            Reject
          </button>
          
          <button
            type="button"
            class="btn btn-primary"
            style="background: #10b981; border-color: #10b981;"
            :disabled="submitting"
            @click="handleAction('approve')"
          >
            Approve
          </button>
        </div>
      </div>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import MainLayout from '@/components/MainLayout.vue'
import { useApprovalStore } from '@/stores/approval.store'
import { useAuthStore } from '@/stores/auth.store'
import { journalApi } from '@/api/journals.api'
import { accountApi } from '@/api/master-data.api'
import type { ApprovalRequest } from '@/types/approval.types'
import type { JournalEntry } from '@/types/journal.types'
import type { Account } from '@/types/master-data.types'

const approvalStore = useApprovalStore()
const authStore = useAuthStore()

// Notifications
const successMsg = ref<string | null>(null)
const errorMsg = ref<string | null>(null)

// Modal states
const showProcessModal = ref(false)
const activeRequest = ref<ApprovalRequest | null>(null)
const comment = ref('')
const submitting = ref(false)

// Loaded details
const loadingDoc = ref(false)
const docLoadError = ref<string | null>(null)
const journalDetails = ref<JournalEntry | null>(null)
const accounts = ref<Account[]>([])

onMounted(async () => {
  await fetchPending()
  await loadAccounts()
})

async function fetchPending() {
  errorMsg.value = null
  try {
    await approvalStore.fetchPending()
  } catch (err: any) {
    errorMsg.value = err.message || 'Failed to fetch pending approval requests.'
  }
}

async function loadAccounts() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return
  try {
    accounts.value = await accountApi.listByCompany(companyId)
  } catch (err: any) {
    console.error('Failed to load accounts map', err)
  }
}

const accountsMap = computed(() => {
  const map = new Map<string, Account>()
  accounts.value.forEach(a => map.set(a.id, a))
  return map
})

function getAccountName(accountId: string): string {
  const acc = accountsMap.value.get(accountId)
  return acc ? `${acc.code} - ${acc.name}` : accountId
}

// Open modal and fetch target details
async function viewAndProcess(req: ApprovalRequest) {
  activeRequest.value = req
  comment.value = ''
  docLoadError.value = null
  journalDetails.value = null
  loadingDoc.value = true
  showProcessModal.value = true

  try {
    if (req.documentType === 'journal_entry') {
      journalDetails.value = await journalApi.getById(req.documentId)
    }
  } catch (err: any) {
    docLoadError.value = `Failed to retrieve target document details: ${err.message}`
  } finally {
    loadingDoc.value = false
  }
}

function closeProcessModal() {
  showProcessModal.value = false
  activeRequest.value = null
  journalDetails.value = null
}

async function handleAction(action: 'approve' | 'reject') {
  if (!activeRequest.value) return
  submitting.value = true
  errorMsg.value = null
  successMsg.value = null

  try {
    if (action === 'approve') {
      await approvalStore.approve(activeRequest.value.id, comment.value)
      successMsg.value = 'Request approved successfully.'
    } else {
      await approvalStore.reject(activeRequest.value.id, comment.value)
      successMsg.value = 'Request rejected.'
    }
    closeProcessModal()
    await fetchPending()
  } catch (err: any) {
    errorMsg.value = err.message || `Failed to ${action} the request.`
  } finally {
    submitting.value = false
  }
}

// Formatting helpers
function getJournalTotal(journal: JournalEntry, type: 'debit' | 'credit'): number {
  return journal.lines.reduce((sum, l) => sum + (type === 'debit' ? Number(l.debit) : Number(l.credit)), 0)
}

function formatDate(dateStr: string): string {
  if (!dateStr) return '-'
  return new Date(dateStr).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

function formatDocType(type: string): string {
  return type.replace('_', ' ')
}

function formatCurrency(val: number): string {
  return new Intl.NumberFormat('id-ID', {
    style: 'currency',
    currency: 'IDR',
    minimumFractionDigits: 0
  }).format(val)
}
</script>

<style scoped>
.modal-lg-custom {
  max-width: 800px;
}

.btn-xs {
  padding: 4px 8px;
  font-size: 0.75rem;
}

/* Metadata Card */
.meta-card {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 16px;
  background-color: var(--bg-tertiary);
  padding: 16px;
  border-radius: var(--radius-md);
  border: 1px solid var(--border-color);
}

.meta-label {
  font-size: 0.75rem;
  text-transform: uppercase;
  font-weight: 600;
  color: var(--text-secondary);
  margin-bottom: 2px;
}

.meta-val {
  font-size: 0.95rem;
  font-weight: 500;
  color: var(--text-primary);
}

.doc-details-box {
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  padding: 16px;
  margin-top: 8px;
}

.detail-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 16px;
}

.detail-label {
  font-size: 0.75rem;
  text-transform: uppercase;
  font-weight: 600;
  color: var(--text-secondary);
  margin-bottom: 2px;
}

.detail-val {
  font-size: 0.9rem;
  color: var(--text-primary);
}

.detail-val-highlight {
  font-size: 0.95rem;
  font-weight: 700;
  color: var(--accent-primary);
}

.totals-row td {
  border-top: 2px solid var(--border-color);
  background-color: rgba(241, 245, 249, 0.5);
}

.form-textarea {
  min-height: 80px;
}

.alert {
  padding: 12px 16px;
  border-radius: var(--radius-sm);
  font-size: 0.875rem;
  margin-bottom: 20px;
}

.alert-success {
  background-color: var(--success-bg);
  color: var(--success);
  border: 1px solid rgba(16, 185, 129, 0.2);
}

.alert-danger {
  background-color: var(--danger-bg);
  color: var(--danger);
  border: 1px solid rgba(239, 68, 68, 0.2);
}

.loading-state, .empty-state {
  padding: 30px;
  text-align: center;
  color: var(--text-secondary);
  font-size: 0.95rem;
}
</style>
