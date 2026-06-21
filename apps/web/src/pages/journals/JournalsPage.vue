<template>
  <MainLayout>
    <div class="page-header">
      <div>
        <h1>General Journal Entries</h1>
        <p class="page-title-desc">Create, review, and post double-entry general ledger adjustments.</p>
      </div>
      <button class="btn btn-primary" @click="openCreateModal">+ Create Journal Entry</button>
    </div>

    <!-- Notifications -->
    <div v-if="successMsg" class="alert alert-success">{{ successMsg }}</div>
    <div v-if="errorMsg" class="alert alert-danger">{{ errorMsg }}</div>

    <!-- Filters/Search Card -->
    <div class="card" style="margin-bottom: 24px; padding: 16px;">
      <div style="display: flex; gap: 16px; flex-wrap: wrap; align-items: center;">
        <div style="flex: 1; min-width: 200px;">
          <input
            v-model="searchTerm"
            type="text"
            class="form-input"
            placeholder="Search by reference, description..."
            @input="applyFilters"
          />
        </div>
        <div style="width: 180px;">
          <select v-model="selectedStatus" class="form-select" @change="applyFilters">
            <option value="">All Statuses</option>
            <option value="draft">Draft</option>
            <option value="waiting_approval">Waiting Approval</option>
            <option value="approved">Approved</option>
            <option value="posted">Posted</option>
            <option value="rejected">Rejected</option>
          </select>
        </div>
        <div>
          <button class="btn btn-secondary" @click="resetFilters">Reset</button>
        </div>
      </div>
    </div>

    <!-- Journals Table -->
    <div class="table-container">
      <div v-if="journalStore.loading && journalsList.length === 0" class="loading-state">
        Loading journal entries...
      </div>
      <div v-else-if="journalsList.length === 0" class="empty-state">
        No journal entries found. Click "+ Create Journal Entry" to create one.
      </div>
      <table v-else>
        <thead>
          <tr>
            <th>Date</th>
            <th>Reference</th>
            <th>Branch</th>
            <th>Description</th>
            <th>Debit Total</th>
            <th>Credit Total</th>
            <th>Status</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="journal in journalsList" :key="journal.id">
            <td>{{ formatDate(journal.transactionDate) }}</td>
            <td style="font-weight: 600; color: var(--accent-primary);">
              {{ journal.referenceNumber }}
            </td>
            <td>
              <span class="badge badge-secondary" style="font-weight: 600;">
                {{ getBranchCode(journal.branchId) }}
              </span>
            </td>
            <td>{{ journal.description }}</td>
            <td style="font-weight: 600;">{{ formatCurrency(getJournalTotal(journal, 'debit')) }}</td>
            <td style="font-weight: 600;">{{ formatCurrency(getJournalTotal(journal, 'credit')) }}</td>
            <td>
              <span :class="['badge', getStatusBadgeClass(journal.status)]">
                {{ formatStatus(journal.status) }}
              </span>
            </td>
            <td>
              <div style="display: flex; gap: 8px;">
                <button class="btn btn-secondary btn-xs" @click="viewDetails(journal)">View</button>
                <button
                  v-if="journal.status === 'draft'"
                  class="btn btn-secondary btn-xs"
                  @click="submitApproval(journal.id)"
                >
                  Submit
                </button>
                <button
                  v-if="journal.status === 'approved'"
                  class="btn btn-primary btn-xs"
                  @click="postJournal(journal.id)"
                >
                  Post
                </button>
                <!-- Manager/Owner actions direct approve -->
                <button
                  v-if="journal.status === 'waiting_approval' && authStore.canApprove"
                  class="btn btn-primary btn-xs"
                  style="background: #10b981; border-color: #10b981;"
                  @click="approveDirect(journal.id)"
                >
                  Approve
                </button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Create Journal Entry Modal -->
    <div v-if="showCreateModal" class="modal-overlay" @click.self="closeCreateModal">
      <div class="modal-content modal-lg-custom">
        <div class="modal-header">
          <h2>Create General Journal Entry</h2>
          <button class="modal-close" @click="closeCreateModal">&times;</button>
        </div>
        <div class="modal-body">
          <div v-if="modalError" class="alert alert-danger" style="margin-bottom: 16px;">
            {{ modalError }}
          </div>

          <form @submit.prevent="saveDraft">
            <div class="form-grid">
              <div class="form-group">
                <label class="form-label">Transaction Date *</label>
                <input v-model="form.transactionDate" type="date" class="form-input" required />
              </div>
              <div class="form-group">
                <label class="form-label">Reference Number (Optional)</label>
                <input
                  v-model="form.referenceNumber"
                  type="text"
                  class="form-input"
                  placeholder="e.g. JV/2026/001"
                />
              </div>
              <div class="form-group">
                <label class="form-label">Branch *</label>
                <select v-model="form.branchId" class="form-input" required>
                  <option value="" disabled>Select Branch</option>
                  <option v-for="branch in activeBranches" :key="branch.id" :value="branch.id">
                    {{ branch.code }} - {{ branch.name }}
                  </option>
                </select>
              </div>
            </div>

            <div class="form-group">
              <label class="form-label">Description *</label>
              <input
                v-model="form.description"
                type="text"
                class="form-input"
                placeholder="Describe the purpose of this journal entry"
                required
              />
            </div>

            <!-- Journal Lines Section -->
            <div style="margin-top: 24px; margin-bottom: 8px; display: flex; justify-content: space-between; align-items: center;">
              <h3>Journal Lines</h3>
              <button type="button" class="btn btn-secondary btn-sm" @click="addLine">
                + Add Line
              </button>
            </div>

            <div class="table-container" style="margin-top: 8px; max-height: 300px; overflow-y: auto;">
              <table class="lines-table">
                <thead>
                  <tr>
                    <th style="width: 40%;">Account *</th>
                    <th style="width: 25%;">Debit</th>
                    <th style="width: 25%;">Credit</th>
                    <th style="width: 10%; text-align: center;">Action</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="(line, index) in form.lines" :key="index">
                    <td>
                      <select v-model="line.accountId" class="form-select" required>
                        <option value="">Select Account...</option>
                        <option v-for="acc in activeAccounts" :key="acc.id" :value="acc.id">
                          {{ acc.code }} - {{ acc.name }} ({{ acc.accountType }})
                        </option>
                      </select>
                    </td>
                    <td>
                      <div class="input-currency-wrapper">
                        <span class="currency-prefix">Rp</span>
                        <input
                          v-model.number="line.debit"
                          type="number"
                          step="0.01"
                          min="0"
                          class="form-input text-right"
                          @input="onDebitInput(index)"
                        />
                      </div>
                    </td>
                    <td>
                      <div class="input-currency-wrapper">
                        <span class="currency-prefix">Rp</span>
                        <input
                          v-model.number="line.credit"
                          type="number"
                          step="0.01"
                          min="0"
                          class="form-input text-right"
                          @input="onCreditInput(index)"
                        />
                      </div>
                    </td>
                    <td style="text-align: center;">
                      <button
                        type="button"
                        class="btn-delete"
                        title="Remove line"
                        @click="removeLine(index)"
                      >
                        &times;
                      </button>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>

            <!-- Totals and Balance indicator -->
            <div class="totals-summary-bar">
              <div class="summary-col">
                <span class="summary-label">Total Debit</span>
                <span class="summary-value">{{ formatCurrency(totals.debit) }}</span>
              </div>
              <div class="summary-col">
                <span class="summary-label">Total Credit</span>
                <span class="summary-value">{{ formatCurrency(totals.credit) }}</span>
              </div>
              <div class="summary-col">
                <span class="summary-label">Difference</span>
                <span
                  :class="[
                    'summary-value',
                    totals.difference === 0 ? 'text-success' : 'text-danger'
                  ]"
                >
                  {{ formatCurrency(Math.abs(totals.difference)) }}
                  <span v-if="totals.difference !== 0" style="font-size: 0.75rem; font-weight: normal; display: block;">
                    (Must be balanced)
                  </span>
                </span>
              </div>
            </div>
          </form>
        </div>
        <div class="modal-footer">
          <button type="button" class="btn btn-secondary" @click="closeCreateModal">Cancel</button>
          <button
            type="button"
            class="btn btn-primary"
            :disabled="!isValid || saving"
            @click="saveDraft"
          >
            {{ saving ? 'Saving...' : 'Save Draft' }}
          </button>
        </div>
      </div>
    </div>

    <!-- View Journal Entry Details Modal -->
    <div v-if="showDetailModal && selectedJournal" class="modal-overlay" @click.self="closeDetailModal">
      <div class="modal-content modal-lg-custom">
        <div class="modal-header">
          <h2>Journal Entry Details</h2>
          <button class="modal-close" @click="closeDetailModal">&times;</button>
        </div>
        <div class="modal-body">
          <div class="detail-grid">
            <div>
              <p class="detail-label">Reference Number</p>
              <p class="detail-val-highlight">{{ selectedJournal.referenceNumber }}</p>
            </div>
            <div>
              <p class="detail-label">Transaction Date</p>
              <p class="detail-val">{{ formatDate(selectedJournal.transactionDate) }}</p>
            </div>
            <div>
              <p class="detail-label">Status</p>
              <p>
                <span :class="['badge', getStatusBadgeClass(selectedJournal.status)]">
                  {{ formatStatus(selectedJournal.status) }}
                </span>
              </p>
            </div>
            <div>
              <p class="detail-label">Source</p>
              <p class="detail-val" style="text-transform: capitalize;">{{ selectedJournal.source }}</p>
            </div>
            <div>
              <p class="detail-label">Branch</p>
              <p class="detail-val">
                <span class="badge badge-secondary" style="font-weight: 600;">
                  {{ getBranchCode(selectedJournal.branchId) }}
                </span>
              </p>
            </div>
          </div>

          <div style="margin-top: 16px;">
            <p class="detail-label">Description</p>
            <p class="detail-val" style="font-size: 1.05rem;">{{ selectedJournal.description }}</p>
          </div>

          <h3 style="margin-top: 24px; margin-bottom: 12px;">Journal Ledger Lines</h3>
          <div class="table-container">
            <table>
              <thead>
                <tr>
                  <th>Account Code & Name</th>
                  <th style="text-align: right;">Debit</th>
                  <th style="text-align: right;">Credit</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="line in selectedJournal.lines" :key="line.id">
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
                    {{ formatCurrency(getJournalTotal(selectedJournal, 'debit')) }}
                  </td>
                  <td style="text-align: right; font-weight: 700;">
                    {{ formatCurrency(getJournalTotal(selectedJournal, 'credit')) }}
                  </td>
                </tr>
              </tbody>
            </table>
          </div>

          <div class="metadata-section">
            <p>Created by: {{ selectedJournal.createdBy }}</p>
          </div>
        </div>
        <div class="modal-footer">
          <button type="button" class="btn btn-secondary" @click="closeDetailModal">Close</button>
          
          <button
            v-if="selectedJournal.status === 'draft'"
            class="btn btn-primary"
            @click="submitApprovalFromDetail"
          >
            Submit for Approval
          </button>
          
          <button
            v-if="selectedJournal.status === 'approved'"
            class="btn btn-primary"
            @click="postJournalFromDetail"
          >
            Post Journal
          </button>

          <button
            v-if="selectedJournal.status === 'waiting_approval' && authStore.canApprove"
            class="btn btn-primary"
            style="background: #10b981; border-color: #10b981;"
            @click="approveDirectFromDetail"
          >
            Approve Entry
          </button>
        </div>
      </div>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import MainLayout from '@/components/MainLayout.vue'
import { useJournalStore } from '@/stores/journal.store'
import { useAuthStore } from '@/stores/auth.store'
import { accountApi, branchApi } from '@/api/master-data.api'
import type { Account, Branch } from '@/types/master-data.types'
import type { JournalEntry } from '@/types/journal.types'

const journalStore = useJournalStore()
const authStore = useAuthStore()

// State
const searchTerm = ref('')
const selectedStatus = ref('')
const successMsg = ref<string | null>(null)
const errorMsg = ref<string | null>(null)

// COA accounts and Branches
const accounts = ref<Account[]>([])
const activeAccounts = computed(() => accounts.value.filter(a => a.isActive))

const branches = ref<Branch[]>([])
const activeBranches = computed(() => branches.value.filter(b => b.isActive))

// Create modal form state
const showCreateModal = ref(false)
const modalError = ref<string | null>(null)
const saving = ref(false)

interface FormLine {
  accountId: string
  debit: number
  credit: number
  description?: string
}

const form = ref({
  transactionDate: new Date().toISOString().split('T')[0],
  referenceNumber: '',
  description: '',
  branchId: '',
  lines: [] as FormLine[]
})

// Detail modal state
const showDetailModal = ref(false)
const selectedJournal = ref<JournalEntry | null>(null)

// Filtering journals list based on UI selection
const journalsList = computed(() => {
  return journalStore.journals.filter(j => {
    // search filter
    if (searchTerm.value) {
      const q = searchTerm.value.toLowerCase()
      const matchesRef = j.referenceNumber?.toLowerCase().includes(q)
      const matchesDesc = j.description?.toLowerCase().includes(q)
      if (!matchesRef && !matchesDesc) return false
    }
    // status filter
    if (selectedStatus.value && j.status !== selectedStatus.value) {
      return false
    }
    return true
  })
})

// COA accounts map for easy naming resolution
const accountsMap = computed(() => {
  const map = new Map<string, Account>()
  accounts.value.forEach(a => map.set(a.id, a))
  return map
})

function getAccountName(accountId: string): string {
  const acc = accountsMap.value.get(accountId)
  return acc ? `${acc.code} - ${acc.name}` : accountId
}

// Calculate total debit / credit for the new form
const totals = computed(() => {
  let debit = 0
  let credit = 0
  form.value.lines.forEach(l => {
    debit += Number(l.debit) || 0
    credit += Number(l.credit) || 0
  })
  return {
    debit,
    credit,
    difference: parseFloat((debit - credit).toFixed(2))
  }
})

// Check if create form is valid
const isValid = computed(() => {
  if (!form.value.description.trim()) return false
  if (!form.value.transactionDate) return false
  if (form.value.lines.length < 2) return false

  // Total debits must equal credits and be positive
  if (totals.value.difference !== 0) return false
  if (totals.value.debit <= 0) return false

  // Validate each line
  for (const line of form.value.lines) {
    if (!line.accountId) return false
    const deb = Number(line.debit) || 0
    const cred = Number(line.credit) || 0
    // A line must have either debit or credit, not both, and not neither
    if ((deb === 0 && cred === 0) || (deb > 0 && cred > 0)) return false
  }

  return true
})

// Load data on load
onMounted(async () => {
  await fetchJournals()
  await loadAccounts()
  await loadBranches()
})

async function fetchJournals() {
  errorMsg.value = null
  try {
    await journalStore.fetchJournals()
  } catch (err: any) {
    errorMsg.value = err.message || 'Failed to fetch journal entries.'
  }
}

async function loadAccounts() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return
  try {
    accounts.value = await accountApi.listByCompany(companyId)
  } catch (err: any) {
    console.error('Failed to load accounts for COA select', err)
  }
}

async function loadBranches() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return
  try {
    branches.value = await branchApi.listByCompany(companyId)
  } catch (err: any) {
    console.error('Failed to load branches select', err)
  }
}

function getBranchCode(branchId?: string) {
  if (!branchId) return '-'
  const b = branches.value.find(x => x.id === branchId)
  return b ? b.code : '-'
}

function applyFilters() {
  // Filters applied locally via computed property journalsList
}

function resetFilters() {
  searchTerm.value = ''
  selectedStatus.value = ''
}

// Form logic
function openCreateModal() {
  const defaultBranchId = activeBranches.value.length > 0 ? activeBranches.value[0].id : ''
  form.value = {
    transactionDate: new Date().toISOString().split('T')[0],
    referenceNumber: '',
    description: '',
    branchId: defaultBranchId,
    lines: [
      { accountId: '', debit: 0, credit: 0 },
      { accountId: '', debit: 0, credit: 0 }
    ]
  }
  modalError.value = null
  showCreateModal.value = true
}

function closeCreateModal() {
  showCreateModal.value = false
}

function addLine() {
  form.value.lines.push({ accountId: '', debit: 0, credit: 0 })
}

function removeLine(index: number) {
  form.value.lines.splice(index, 1)
}

function onDebitInput(index: number) {
  // If debit is filled, credit must be 0
  const val = Number(form.value.lines[index].debit)
  if (val > 0) {
    form.value.lines[index].credit = 0
  }
}

function onCreditInput(index: number) {
  // If credit is filled, debit must be 0
  const val = Number(form.value.lines[index].credit)
  if (val > 0) {
    form.value.lines[index].debit = 0
  }
}

async function saveDraft() {
  if (!isValid.value) return
  saving.value = true
  modalError.value = null
  
  const companyId = authStore.currentUser?.companyId
  if (!companyId) {
    modalError.value = 'User company profile not found.'
    saving.value = false
    return
  }

  // Format request
  const requestPayload = {
    companyId,
    branchId: form.value.branchId || undefined,
    referenceNumber: form.value.referenceNumber.trim() || undefined,
    description: form.value.description.trim(),
    transactionDate: form.value.transactionDate,
    lines: form.value.lines.map(l => ({
      accountId: l.accountId,
      debit: Number(l.debit) || 0,
      credit: Number(l.credit) || 0,
      description: form.value.description.trim() // standard line description inherits main desc if omitted
    }))
  }

  try {
    await journalStore.createDraft(requestPayload)
    showCreateModal.value = false
    successMsg.value = 'Journal Entry draft saved successfully.'
    // Refresh lists
    await fetchJournals()
  } catch (err: any) {
    modalError.value = err.response?.data?.message || err.message || 'Failed to save journal draft.'
  } finally {
    saving.value = false
  }
}

// Listing actions
async function submitApproval(id: string) {
  errorMsg.value = null
  successMsg.value = null
  try {
    await journalStore.submitForApproval(id)
    successMsg.value = 'Journal entry submitted for approval.'
    await fetchJournals()
  } catch (err: any) {
    errorMsg.value = err.message || 'Failed to submit for approval.'
  }
}

async function postJournal(id: string) {
  errorMsg.value = null
  successMsg.value = null
  try {
    await journalStore.postJournal(id)
    successMsg.value = 'Journal entry posted successfully to ledger.'
    await fetchJournals()
  } catch (err: any) {
    errorMsg.value = err.message || 'Failed to post journal entry.'
  }
}

async function approveDirect(id: string) {
  errorMsg.value = null
  successMsg.value = null
  try {
    // Call approval endpoint
    const { journalApi } = await import('@/api/journals.api')
    await journalApi.approve(id) // direct helper approve endpoint
    successMsg.value = 'Journal entry approved.'
    await fetchJournals()
  } catch (err: any) {
    errorMsg.value = err.message || 'Failed to approve journal entry.'
  }
}

// Detail modal logic
function viewDetails(journal: JournalEntry) {
  selectedJournal.value = journal
  showDetailModal.value = true
}

function closeDetailModal() {
  showDetailModal.value = false
  selectedJournal.value = null
}

async function submitApprovalFromDetail() {
  if (!selectedJournal.value) return
  const id = selectedJournal.value.id
  closeDetailModal()
  await submitApproval(id)
}

async function postJournalFromDetail() {
  if (!selectedJournal.value) return
  const id = selectedJournal.value.id
  closeDetailModal()
  await postJournal(id)
}

async function approveDirectFromDetail() {
  if (!selectedJournal.value) return
  const id = selectedJournal.value.id
  closeDetailModal()
  await approveDirect(id)
}

// Helpers
function getJournalTotal(journal: JournalEntry, type: 'debit' | 'credit'): number {
  return journal.lines.reduce((sum, l) => sum + (type === 'debit' ? Number(l.debit) : Number(l.credit)), 0)
}

function formatDate(dateStr: string): string {
  if (!dateStr) return '-'
  return new Date(dateStr).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}

function formatCurrency(val: number): string {
  return new Intl.NumberFormat('id-ID', {
    style: 'currency',
    currency: 'IDR',
    minimumFractionDigits: 0
  }).format(val)
}

function formatStatus(status: string): string {
  return status.replace('_', ' ')
}

function getStatusBadgeClass(status: string): string {
  switch (status) {
    case 'draft':
      return 'badge-warning'
    case 'waiting_approval':
    case 'waiting_review':
      return 'badge-info'
    case 'approved':
      return 'badge-success'
    case 'posted':
      return 'badge-success' // Or secondary color if customized, success fits indigo posted state
    case 'rejected':
    case 'cancelled':
      return 'badge-danger'
    default:
      return ''
  }
}
</script>

<style scoped>
.form-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
  margin-bottom: 8px;
}

.modal-lg-custom {
  max-width: 800px;
}

.btn-xs {
  padding: 4px 8px;
  font-size: 0.75rem;
}

.btn-sm {
  padding: 6px 12px;
  font-size: 0.8rem;
}

/* Lines editor styling */
.lines-table {
  border-collapse: collapse;
  width: 100%;
}

.lines-table th, .lines-table td {
  padding: 8px 12px;
  border-bottom: 1px solid var(--border-color);
}

.input-currency-wrapper {
  display: flex;
  align-items: center;
  position: relative;
}

.currency-prefix {
  position: absolute;
  left: 10px;
  font-size: 0.8rem;
  color: var(--text-secondary);
}

.input-currency-wrapper .form-input {
  padding-left: 28px;
}

.text-right {
  text-align: right;
}

.btn-delete {
  background: none;
  border: none;
  color: var(--danger);
  font-size: 1.5rem;
  cursor: pointer;
  line-height: 1;
  transition: color var(--transition-fast);
}

.btn-delete:hover {
  color: #dc2626;
}

/* Summary Bar */
.totals-summary-bar {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 16px;
  margin-top: 16px;
  background-color: var(--bg-tertiary);
  padding: 16px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--border-color);
}

.summary-col {
  display: flex;
  flex-direction: column;
}

.summary-label {
  font-size: 0.75rem;
  text-transform: uppercase;
  color: var(--text-secondary);
  font-weight: 600;
  letter-spacing: 0.05em;
}

.summary-value {
  font-family: var(--font-heading);
  font-size: 1.15rem;
  font-weight: 700;
  color: var(--text-primary);
  margin-top: 4px;
}

.text-success {
  color: var(--success) !important;
}

.text-danger {
  color: var(--danger) !important;
}

/* Detail view */
.detail-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 20px;
  background-color: var(--bg-tertiary);
  padding: 20px;
  border-radius: var(--radius-md);
  border: 1px solid var(--border-color);
}

.detail-label {
  font-size: 0.75rem;
  text-transform: uppercase;
  font-weight: 600;
  color: var(--text-secondary);
  margin-bottom: 4px;
}

.detail-val {
  font-size: 0.95rem;
  font-weight: 500;
  color: var(--text-primary);
}

.detail-val-highlight {
  font-size: 1rem;
  font-weight: 700;
  color: var(--accent-primary);
}

.totals-row td {
  border-top: 2px solid var(--border-color);
  background-color: rgba(241, 245, 249, 0.5);
}

.metadata-section {
  margin-top: 24px;
  font-size: 0.8rem;
  color: var(--text-muted);
  text-align: right;
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
  padding: 40px;
  text-align: center;
  color: var(--text-secondary);
  font-size: 0.95rem;
}
</style>
