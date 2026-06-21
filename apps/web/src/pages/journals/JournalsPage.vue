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
            <td style="position: relative; width: 80px; text-align: center;">
              <div class="row-actions-container">
                <button class="btn-actions-trigger" @click="toggleRowDropdown(journal.id, $event)">
                  <svg class="actions-trigger-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <circle cx="12" cy="12" r="1.5" />
                    <circle cx="19" cy="12" r="1.5" />
                    <circle cx="5" cy="12" r="1.5" />
                  </svg>
                </button>

                <transition name="dropdown-fade">
                  <div v-if="activeDropdownId === journal.id" class="row-actions-dropdown">
                    <button class="dropdown-item" @click="viewDetails(journal)">
                      👁️ View Details
                    </button>
                    <button
                      v-if="journal.status.toLowerCase() === 'draft'"
                      class="dropdown-item"
                      @click="openEditModal(journal)"
                    >
                      ✏️ Edit Draft
                    </button>
                    <button
                      v-if="journal.status.toLowerCase() === 'draft'"
                      class="dropdown-item"
                      @click="submitApproval(journal.id)"
                    >
                      📤 Submit Approval
                    </button>
                    <button
                      v-if="journal.status.toLowerCase() === 'approved'"
                      class="dropdown-item"
                      @click="postJournal(journal.id)"
                    >
                      ⚙️ Post Ledger
                    </button>
                    <button
                      v-if="journal.status.toLowerCase() === 'waiting_approval' && authStore.canApprove"
                      class="dropdown-item text-success"
                      @click="approveDirect(journal.id)"
                    >
                      ✅ Approve Entry
                    </button>
                    
                    <div v-if="journal.status.toLowerCase() === 'draft'" class="dropdown-divider"></div>
                    
                    <button
                      v-if="journal.status.toLowerCase() === 'draft'"
                      class="dropdown-item text-danger"
                      @click="handleDelete(journal.id)"
                    >
                      🗑️ Delete Draft
                    </button>
                  </div>
                </transition>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Pagination Footer -->
    <div class="pagination-footer" v-if="totalPages > 1">
      <div class="pagination-info">
        Showing {{ paginationStart }} to {{ paginationEnd }} of {{ filteredJournals.length }} entries
      </div>
      <div class="pagination-buttons">
        <button
          class="btn btn-secondary btn-sm"
          :disabled="currentPage === 1"
          @click="changePage(currentPage - 1)"
        >
          Previous
        </button>
        
        <button
          v-for="page in visiblePages"
          :key="page"
          :class="['btn btn-sm', page === currentPage ? 'btn-primary' : 'btn-secondary']"
          @click="changePage(page)"
        >
          {{ page }}
        </button>
        
        <button
          class="btn btn-secondary btn-sm"
          :disabled="currentPage === totalPages"
          @click="changePage(currentPage + 1)"
        >
          Next
        </button>
      </div>
    </div>

    <!-- Create/Edit Journal Entry Modal -->
    <div v-if="showCreateModal" class="modal-overlay" @click.self="closeCreateModal">
      <div class="modal-content modal-lg-custom">
        <div class="modal-header">
          <h2>{{ editingJournalId ? 'Edit General Journal Entry' : 'Create General Journal Entry' }}</h2>
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
            {{ saving ? 'Saving...' : (editingJournalId ? 'Save Changes' : 'Save Draft') }}
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
            v-if="selectedJournal.status.toLowerCase() === 'draft'"
            class="btn btn-secondary"
            style="background: #e2e8f0; color: var(--text-primary);"
            @click="editFromDetail"
          >
            ✏️ Edit Draft
          </button>

          <button
            v-if="selectedJournal.status.toLowerCase() === 'draft'"
            class="btn btn-danger"
            @click="deleteFromDetail"
          >
            🗑️ Delete Draft
          </button>
          
          <button
            v-if="selectedJournal.status.toLowerCase() === 'draft'"
            class="btn btn-primary"
            style="background-color: var(--info-bg); color: var(--info); border: 1px solid rgba(59, 130, 246, 0.15);"
            @click="submitApprovalFromDetail"
          >
            Submit for Approval
          </button>
          
          <button
            v-if="selectedJournal.status.toLowerCase() === 'approved'"
            class="btn btn-primary"
            @click="postJournalFromDetail"
          >
            Post Journal
          </button>

          <button
            v-if="selectedJournal.status.toLowerCase() === 'waiting_approval' && authStore.canApprove"
            class="btn btn-primary"
            style="background: #10b981; border-color: #10b981;"
            @click="approveDirectFromDetail"
          >
            Approve Entry
          </button>
        </div>
      </div>
    </div>

    <!-- Custom Confirmation Dialog Modal -->
    <ConfirmDialog
      :is-open="confirmDialog.isOpen"
      :title="confirmDialog.title"
      :message="confirmDialog.message"
      :confirm-text="confirmDialog.confirmText"
      :cancel-text="confirmDialog.cancelText"
      :is-danger="confirmDialog.isDanger"
      @confirm="handleConfirmDialog(true)"
      @cancel="handleConfirmDialog(false)"
    />
  </MainLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import MainLayout from '@/components/MainLayout.vue'
import ConfirmDialog from '@/components/ConfirmDialog.vue'
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

// Create/Edit modal form state
const showCreateModal = ref(false)
const editingJournalId = ref<string | null>(null)
const modalError = ref<string | null>(null)
const saving = ref(false)

// Row actions dropdown state
const activeDropdownId = ref<string | null>(null)

// Custom Confirmation Dialog State
const confirmDialog = ref({
  isOpen: false,
  title: 'Confirm Action',
  message: 'Are you sure you want to proceed?',
  confirmText: 'Confirm',
  cancelText: 'Cancel',
  isDanger: false,
  resolve: null as ((value: boolean) => void) | null
})

// Pagination State
const currentPage = ref(1)
const perPage = ref(5)

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
const filteredJournals = computed(() => {
  return journalStore.journals.filter(j => {
    // search filter
    if (searchTerm.value) {
      const q = searchTerm.value.toLowerCase()
      const matchesRef = j.referenceNumber?.toLowerCase().includes(q)
      const matchesDesc = j.description?.toLowerCase().includes(q)
      if (!matchesRef && !matchesDesc) return false
    }
    // status filter
    if (selectedStatus.value && j.status.toLowerCase() !== selectedStatus.value.toLowerCase()) {
      return false
    }
    return true
  })
})

// Sliced journals list for the current page
const journalsList = computed(() => {
  const start = (currentPage.value - 1) * perPage.value
  const end = start + perPage.value
  return filteredJournals.value.slice(start, end)
})

// Total pages computation
const totalPages = computed(() => {
  return Math.ceil(filteredJournals.value.length / perPage.value)
})

// Pagination details
const paginationStart = computed(() => {
  if (filteredJournals.value.length === 0) return 0
  return (currentPage.value - 1) * perPage.value + 1
})

const paginationEnd = computed(() => {
  return Math.min(currentPage.value * perPage.value, filteredJournals.value.length)
})

const visiblePages = computed(() => {
  const pages: number[] = []
  const maxVisible = 5
  let start = Math.max(1, currentPage.value - Math.floor(maxVisible / 2))
  let end = Math.min(totalPages.value, start + maxVisible - 1)
  
  if (end - start + 1 < maxVisible) {
    start = Math.max(1, end - maxVisible + 1)
  }
  
  for (let i = start; i <= end; i++) {
    pages.push(i)
  }
  return pages
})

function changePage(page: number) {
  if (page >= 1 && page <= totalPages.value) {
    currentPage.value = page
  }
}

// Dropdown toggle and click outside logic
function toggleRowDropdown(id: string, event: Event) {
  event.stopPropagation()
  if (activeDropdownId.value === id) {
    activeDropdownId.value = null
  } else {
    activeDropdownId.value = id
  }
}

// Close dropdowns when clicking outside
function closeAllDropdowns() {
  activeDropdownId.value = null
}

// Custom Promise-based confirm dialog helper function
function confirmCustom(options: {
  title?: string
  message: string
  confirmText?: string
  cancelText?: string
  isDanger?: boolean
}) {
  return new Promise<boolean>((resolve) => {
    confirmDialog.value = {
      isOpen: true,
      title: options.title || 'Confirm Action',
      message: options.message,
      confirmText: options.confirmText || 'Confirm',
      cancelText: options.cancelText || 'Cancel',
      isDanger: options.isDanger || false,
      resolve
    }
  })
}

function handleConfirmDialog(result: boolean) {
  confirmDialog.value.isOpen = false
  if (confirmDialog.value.resolve) {
    confirmDialog.value.resolve(result)
  }
}

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
  await journalStore.setFilters({ perPage: 1000 }) // Load all journals for client-side pagination
  await fetchJournals()
  await loadAccounts()
  await loadBranches()
  
  // Close dropdowns when clicking outside
  document.addEventListener('click', closeAllDropdowns)
})

onUnmounted(() => {
  document.removeEventListener('click', closeAllDropdowns)
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
  currentPage.value = 1
}

function resetFilters() {
  searchTerm.value = ''
  selectedStatus.value = ''
  currentPage.value = 1
}

// Form logic
function openCreateModal() {
  editingJournalId.value = null
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

// Populate form state for editing an existing journal draft
function openEditModal(journal: JournalEntry) {
  editingJournalId.value = journal.id
  modalError.value = null
  
  // Format transactionDate to YYYY-MM-DD
  const txDate = journal.transactionDate.split('T')[0]
  
  form.value = {
    transactionDate: txDate,
    referenceNumber: journal.referenceNumber || '',
    description: journal.description,
    branchId: journal.branchId || '',
    lines: journal.lines.map(l => ({
      accountId: l.accountId,
      debit: Number(l.debit) || 0,
      credit: Number(l.credit) || 0,
      description: l.description
    }))
  }
  showCreateModal.value = true
}

function closeCreateModal() {
  showCreateModal.value = false
  editingJournalId.value = null
}

// Add/Remove lines in the entries editor
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

  // Format request payload
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
      description: form.value.description.trim() // line description inherits main desc
    }))
  }

  try {
    if (editingJournalId.value) {
      await journalStore.updateDraft(editingJournalId.value, requestPayload)
      successMsg.value = 'Journal Entry draft updated successfully.'
    } else {
      await journalStore.createDraft(requestPayload)
      successMsg.value = 'Journal Entry draft saved successfully.'
    }
    showCreateModal.value = false
    // Refresh list
    await fetchJournals()
  } catch (err: any) {
    modalError.value = err.response?.data?.message || err.message || 'Failed to save journal.'
  } finally {
    saving.value = false
  }
}

async function handleDelete(id: string) {
  const confirmed = await confirmCustom({
    title: 'Delete Journal Draft',
    message: 'Are you sure you want to delete this journal entry draft? This action cannot be undone.',
    confirmText: 'Delete Draft',
    cancelText: 'Cancel',
    isDanger: true
  })
  
  if (!confirmed) return
  
  errorMsg.value = null
  successMsg.value = null
  try {
    await journalStore.deleteDraft(id)
    successMsg.value = 'Journal Entry draft deleted successfully.'
    await fetchJournals()
  } catch (err: any) {
    errorMsg.value = err.message || 'Failed to delete journal entry.'
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
    const { journalApi } = await import('@/api/journals.api')
    await journalApi.approve(id)
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

function editFromDetail() {
  if (!selectedJournal.value) return
  const journal = selectedJournal.value
  closeDetailModal()
  openEditModal(journal)
}

async function deleteFromDetail() {
  if (!selectedJournal.value) return
  const id = selectedJournal.value.id
  closeDetailModal()
  await handleDelete(id)
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

// Format date to local readable format
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
  switch (status.toLowerCase()) {
    case 'draft':
      return 'badge-warning'
    case 'waiting_approval':
    case 'waiting_review':
      return 'badge-info'
    case 'approved':
      return 'badge-success'
    case 'posted':
      return 'badge-success'
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

.btn-edit {
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
}

.btn-edit:hover {
  background-color: var(--border-color);
}

.table-container {
  overflow: visible !important; /* Allow the actions dropdown to bleed outside table boundary without scroll/cropping */
}

/* Row Action Dropdown Styling */
.row-actions-container {
  position: relative;
  display: inline-block;
}

.btn-actions-trigger {
  background: none;
  border: 1px solid var(--border-color);
  border-radius: 50%;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  color: var(--text-secondary);
  transition: all var(--transition-fast);
}

.btn-actions-trigger:hover {
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
  border-color: var(--text-muted);
}

.actions-trigger-icon {
  width: 16px;
  height: 16px;
}

.row-actions-dropdown {
  position: absolute;
  top: calc(100% + 6px);
  right: 0;
  width: 170px;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  box-shadow: var(--shadow-lg), 0 0 0 1px rgba(0, 0, 0, 0.05);
  z-index: 100;
  padding: 6px;
  transform-origin: top right;
}

.row-actions-dropdown .dropdown-item {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  padding: 8px 12px;
  background: none;
  border: none;
  text-align: left;
  font-family: var(--font-body);
  font-size: 0.825rem;
  font-weight: 500;
  color: var(--text-secondary);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.row-actions-dropdown .dropdown-item:hover {
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
}

.row-actions-dropdown .dropdown-item.text-success {
  color: var(--success);
}

.row-actions-dropdown .dropdown-item.text-success:hover {
  background-color: var(--success-bg);
  color: var(--success);
}

.row-actions-dropdown .dropdown-item.text-danger {
  color: var(--danger);
}

.row-actions-dropdown .dropdown-item.text-danger:hover {
  background-color: var(--danger-bg);
  color: var(--danger);
}

.row-actions-dropdown .dropdown-divider {
  height: 1px;
  background-color: var(--border-color);
  margin: 6px 0;
}

/* Pagination Footer */
.pagination-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 20px;
  padding: 16px 24px;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
}

.pagination-info {
  font-size: 0.875rem;
  color: var(--text-secondary);
  font-weight: 500;
}

.pagination-buttons {
  display: flex;
  gap: 6px;
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

/* Transition Animations */
.dropdown-fade-enter-active,
.dropdown-fade-leave-active {
  transition: all var(--transition-fast);
}

.dropdown-fade-enter-from,
.dropdown-fade-leave-to {
  opacity: 0;
  transform: scale(0.95) translateY(-4px);
}
</style>
