<template>
  <MainLayout>
    <!-- Header Section -->
    <div class="page-header">
      <div>
        <h1>Tax Management & Compliance</h1>
        <p class="page-title-desc">Monitor VAT Output, VAT Input, net tax liabilities, and manage filing calendars.</p>
      </div>
      <div class="header-actions">
        <button class="btn btn-primary" @click="openCreateCalendarModal">
          <span style="font-size: 1.1rem; line-height: 1;">+</span> Add Tax Calendar Entry
        </button>
      </div>
    </div>

    <!-- Alert Notifications -->
    <div v-if="successMsg" class="alert alert-success alert-dismissible">
      <span>{{ successMsg }}</span>
      <button class="alert-close" @click="successMsg = null">&times;</button>
    </div>
    <div v-if="errorMsg" class="alert alert-danger alert-dismissible">
      <span>{{ errorMsg }}</span>
      <button class="alert-close" @click="errorMsg = null">&times;</button>
    </div>

    <!-- Dashboard Summary Section -->
    <div class="summary-grid">
      <!-- VAT Output Card -->
      <div class="summary-card output-card">
        <div class="card-glow"></div>
        <div class="card-content">
          <div class="card-header-row">
            <span class="card-title">VAT Output (PPN Keluaran)</span>
            <span class="card-icon">📤</span>
          </div>
          <h2 class="card-amount">{{ formatIDR(summary.totalVatOutput) }}</h2>
          <div class="card-footer-info">
            <span>Collected from sales invoices</span>
          </div>
        </div>
      </div>

      <!-- VAT Input Card -->
      <div class="summary-card input-card">
        <div class="card-glow"></div>
        <div class="card-content">
          <div class="card-header-row">
            <span class="card-title">VAT Input (PPN Masukan)</span>
            <span class="card-icon">📥</span>
          </div>
          <h2 class="card-amount">{{ formatIDR(summary.totalVatInput) }}</h2>
          <div class="card-footer-info">
            <span>Paid on purchase invoices</span>
          </div>
        </div>
      </div>

      <!-- Net Tax Due Card -->
      <div :class="['summary-card', summary.netTaxDue >= 0 ? 'payable-card' : 'refundable-card']">
        <div class="card-glow"></div>
        <div class="card-content">
          <div class="card-header-row">
            <span class="card-title">
              {{ summary.netTaxDue >= 0 ? 'Net VAT Payable' : 'Net VAT Receivable (Credit)' }}
            </span>
            <span class="card-icon">{{ summary.netTaxDue >= 0 ? '⚖️' : '💰' }}</span>
          </div>
          <h2 class="card-amount">{{ formatIDR(Math.abs(summary.netTaxDue)) }}</h2>
          <div class="card-footer-info">
            <span>{{ summary.netTaxDue >= 0 ? 'Due to tax authority' : 'Available for refund / carryover' }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Period Filter Bar -->
    <div class="card filter-card">
      <form @submit.prevent="applyFilters" class="filter-form">
        <div class="filter-group">
          <label class="form-label">Start Date</label>
          <input v-model="filterStartDate" type="date" class="form-input" required />
        </div>
        <div class="filter-group">
          <label class="form-label">End Date</label>
          <input v-model="filterEndDate" type="date" class="form-input" required />
        </div>
        <div class="filter-actions">
          <button type="submit" class="btn btn-primary" :disabled="loadingSummary">
            {{ loadingSummary ? 'Loading...' : 'Apply Filter' }}
          </button>
          <button type="button" class="btn btn-secondary" @click="resetDateFilters">
            Reset Range
          </button>
        </div>
      </form>
    </div>

    <!-- Navigation Tabs -->
    <div class="tabs-container">
      <button 
        :class="['tab-btn', activeTab === 'records' ? 'active' : '']" 
        @click="activeTab = 'records'"
      >
        📑 VAT Transaction Ledger
      </button>
      <button 
        :class="['tab-btn', activeTab === 'calendar' ? 'active' : '']" 
        @click="activeTab = 'calendar'"
      >
        📅 Compliance Calendar
      </button>
    </div>

    <!-- Tab 1: VAT Transaction Ledger -->
    <div v-if="activeTab === 'records'" class="tab-pane">
      <div class="tab-pane-header">
        <h3>VAT Transaction Records</h3>
        <p class="section-desc">Historical records of posted purchase and sales taxes.</p>
      </div>

      <div class="table-container">
        <div v-if="loadingRecords" class="loading-state">
          Loading tax ledger records...
        </div>
        <div v-else-if="records.length === 0" class="empty-state">
          No tax records found for the selected period.
        </div>
        <table v-else>
          <thead>
            <tr>
              <th>Period</th>
              <th>Document Type</th>
              <th>Tax Category</th>
              <th>Counterparty</th>
              <th class="text-right">Base Amount (DPP)</th>
              <th class="text-right">Rate</th>
              <th class="text-right">Tax Amount (PPN)</th>
              <th>Status</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="record in records" :key="record.id">
              <td style="font-weight: 500;">{{ formatPeriod(record.taxPeriod) }}</td>
              <td>
                <span class="doc-link">
                  {{ formatDocType(record.sourceDocumentType) }}
                </span>
              </td>
              <td>
                <span :class="['badge', record.sourceDocumentType === 'SalesInvoice' ? 'badge-info' : 'badge-secondary']">
                  {{ record.sourceDocumentType === 'SalesInvoice' ? 'VAT Output' : 'VAT Input' }}
                </span>
              </td>
              <td>
                <div class="counterparty-info">
                  <span class="cp-name">{{ record.counterpartyName || '-' }}</span>
                  <span class="cp-npwp" v-if="record.counterpartyNpwp">NPWP: {{ record.counterpartyNpwp }}</span>
                </div>
              </td>
              <td class="text-right font-mono">{{ formatIDR(record.taxBaseAmount) }}</td>
              <td class="text-right">{{ (record.taxRate * 100).toFixed(0) }}%</td>
              <td class="text-right font-mono font-bold">{{ formatIDR(record.taxAmount) }}</td>
              <td>
                <span :class="['badge', getStatusBadgeClass(record.status)]">
                  {{ formatStatusText(record.status) }}
                </span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Pagination controls -->
      <div class="pagination-footer" v-if="totalPages > 1">
        <div class="pagination-info">
          Showing {{ paginationStart }} to {{ paginationEnd }} of {{ totalCount }} records
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
    </div>

    <!-- Tab 2: Compliance Calendar -->
    <div v-else-if="activeTab === 'calendar'" class="tab-pane">
      <div class="tab-pane-header">
        <h3>Tax Compliance Calendar</h3>
        <p class="section-desc">Track and check off monthly payment and filing tasks.</p>
      </div>

      <div class="table-container">
        <div v-if="loadingCalendar" class="loading-state">
          Loading tax calendar...
        </div>
        <div v-else-if="calendarEntries.length === 0" class="empty-state">
          No calendar entries set up. Click "+ Add Tax Calendar Entry" to schedule one.
        </div>
        <table v-else>
          <thead>
            <tr>
              <th>Period</th>
              <th>Tax Type</th>
              <th>Payment Due</th>
              <th>Payment Status</th>
              <th>Filing Due</th>
              <th>Filing Status</th>
              <th style="width: 220px; text-align: center;">Actions</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="entry in calendarEntries" :key="entry.id">
              <td style="font-weight: 600;">{{ formatPeriod(entry.taxPeriod) }}</td>
              <td>
                <span class="tax-type-code">{{ getTaxTypeCode(entry.taxTypeId) }}</span>
                <span class="tax-type-name" style="margin-left: 8px; font-size: 0.8rem; color: var(--text-secondary);">
                  {{ getTaxTypeName(entry.taxTypeId) }}
                </span>
              </td>
              <td :class="['date-cell', isOverdue(entry.paymentDueDate, entry.paymentStatus) ? 'overdue-date' : '']">
                {{ formatDate(entry.paymentDueDate) }}
                <span class="overdue-label" v-if="isOverdue(entry.paymentDueDate, entry.paymentStatus)">Overdue</span>
              </td>
              <td>
                <span :class="['badge', entry.paymentStatus === 'paid' ? 'badge-success' : 'badge-warning']">
                  🔑 {{ entry.paymentStatus === 'paid' ? 'Paid' : 'Unpaid' }}
                </span>
              </td>
              <td :class="['date-cell', isOverdue(entry.filingDueDate, entry.filingStatus) ? 'overdue-date' : '']">
                {{ formatDate(entry.filingDueDate) }}
                <span class="overdue-label" v-if="isOverdue(entry.filingDueDate, entry.filingStatus)">Overdue</span>
              </td>
              <td>
                <span :class="['badge', entry.filingStatus === 'filed' ? 'badge-success' : 'badge-warning']">
                  📂 {{ entry.filingStatus === 'filed' ? 'Filed' : 'Unfiled' }}
                </span>
              </td>
              <td>
                <div class="calendar-actions-cell">
                  <button 
                    v-if="entry.paymentStatus !== 'paid'"
                    class="btn btn-secondary btn-xs btn-success-outline"
                    @click="updateStatus(entry.id, 'paid', undefined)"
                  >
                    💸 Mark Paid
                  </button>
                  <button 
                    v-if="entry.filingStatus !== 'filed'"
                    class="btn btn-secondary btn-xs btn-info-outline"
                    @click="updateStatus(entry.id, undefined, 'filed')"
                  >
                    📝 Mark Filed
                  </button>
                  <span v-if="entry.paymentStatus === 'paid' && entry.filingStatus === 'filed'" class="compliance-complete">
                    ✓ Compliant
                  </span>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Create Calendar Entry Modal -->
    <div v-if="showCreateCalendarModal" class="modal-overlay" @click.self="closeCreateCalendarModal">
      <div class="modal-content">
        <div class="modal-header">
          <h2>Create Tax Calendar Entry</h2>
          <button class="modal-close" @click="closeCreateCalendarModal">&times;</button>
        </div>
        <div class="modal-body">
          <div v-if="modalError" class="alert alert-danger" style="margin-bottom: 16px;">
            {{ modalError }}
          </div>

          <form @submit.prevent="createCalendarEntry">
            <div class="form-group">
              <label class="form-label">Tax Type *</label>
              <select v-model="calendarForm.taxTypeId" class="form-select" required>
                <option value="" disabled>Select Tax Type...</option>
                <option v-for="t in activeTaxTypes" :key="t.id" :value="t.id">
                  {{ t.code }} - {{ t.name }} ({{ t.category }})
                </option>
              </select>
            </div>

            <div class="form-group">
              <label class="form-label">Tax Period *</label>
              <input v-model="calendarForm.taxPeriod" type="date" class="form-input" required />
              <small class="form-help">Select any date in the target month (stored as first day of month).</small>
            </div>

            <div class="form-grid-2">
              <div class="form-group">
                <label class="form-label">Payment Due Date *</label>
                <input v-model="calendarForm.paymentDueDate" type="date" class="form-input" required />
              </div>
              <div class="form-group">
                <label class="form-label">Filing Due Date *</label>
                <input v-model="calendarForm.filingDueDate" type="date" class="form-input" required />
              </div>
            </div>

            <div class="modal-footer" style="padding-top: 16px;">
              <button type="button" class="btn btn-secondary" @click="closeCreateCalendarModal">
                Cancel
              </button>
              <button type="submit" class="btn btn-primary" :disabled="creatingEntry">
                {{ creatingEntry ? 'Creating...' : 'Create Entry' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import MainLayout from '@/components/MainLayout.vue'
import { formatIDR, formatDate } from '@/utils/format'
import { taxRecordApi, taxCalendarApi, taxTypeApi } from '@/api/master-data.api'
import { useAuthStore } from '@/stores/auth.store'
import type { TaxRecord, TaxCalendarEntry, TaxType } from '@/types/master-data.types'

const authStore = useAuthStore()

// Navigation & Layout State
const activeTab = ref<'records' | 'calendar'>('records')
const successMsg = ref<string | null>(null)
const errorMsg = ref<string | null>(null)

// Date Range Filter State
const now = new Date()
const defaultStartDate = new Date(now.getFullYear(), now.getMonth(), 1).toISOString().split('T')[0]
const defaultEndDate = new Date(now.getFullYear(), now.getMonth() + 1, 0).toISOString().split('T')[0]

const filterStartDate = ref(defaultStartDate)
const filterEndDate = ref(defaultEndDate)

// Tax Data States
const loadingSummary = ref(false)
const summary = ref({
  totalVatOutput: 0,
  totalVatInput: 0,
  netTaxDue: 0
})

const loadingRecords = ref(false)
const records = ref<TaxRecord[]>([])
const totalCount = ref(0)
const currentPage = ref(1)
const perPage = ref(15)

const loadingCalendar = ref(false)
const calendarEntries = ref<TaxCalendarEntry[]>([])

const taxTypes = ref<TaxType[]>([])
const activeTaxTypes = computed(() => taxTypes.value.filter(t => t.isActive))

// Create Calendar Entry State
const showCreateCalendarModal = ref(false)
const creatingEntry = ref(false)
const modalError = ref<string | null>(null)
const calendarForm = ref({
  taxTypeId: '',
  taxPeriod: new Date().toISOString().split('T')[0],
  paymentDueDate: new Date(now.getFullYear(), now.getMonth() + 1, 15).toISOString().split('T')[0], // 15th of next month
  filingDueDate: new Date(now.getFullYear(), now.getMonth() + 1, 20).toISOString().split('T')[0] // 20th of next month
})

// Fetch all initial data
async function loadData() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return

  try {
    loadingCalendar.value = true
    // Fetch tax types for dropdowns and name mappings
    taxTypes.value = await taxTypeApi.listByCompany(companyId)
    
    // Fetch summary
    await fetchSummary()
    
    // Fetch records
    await fetchRecords()

    // Fetch calendar
    await fetchCalendar()
  } catch (err: any) {
    errorMsg.value = err.response?.data?.message || 'Failed to load tax dashboard data.'
  } finally {
    loadingCalendar.value = false
  }
}

// Fetch VAT Summary statistics
async function fetchSummary() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return

  try {
    loadingSummary.value = true
    const res = await taxRecordApi.getSummary(companyId, filterStartDate.value, filterEndDate.value)
    summary.value = {
      totalVatOutput: Number(res.totalVatOutput) || 0,
      totalVatInput: Number(res.totalVatInput) || 0,
      netTaxDue: Number(res.netTaxDue) || 0
    }
  } catch (err: any) {
    console.error('Failed to load tax summary', err)
  } finally {
    loadingSummary.value = false
  }
}

// Fetch Paginated Ledger Records
async function fetchRecords() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return

  try {
    loadingRecords.value = true
    const { records: data, totalCount: count } = await taxRecordApi.listByCompany(
      companyId,
      currentPage.value,
      perPage.value
    )
    // Filter records locally or display them based on filter ranges if appropriate, but since
    // getSummary and general lists exist, we keep standard pagination
    records.value = data
    totalCount.value = count
  } catch (err: any) {
    console.error('Failed to load tax records', err)
  } finally {
    loadingRecords.value = false
  }
}

// Fetch Calendar entries
async function fetchCalendar() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return

  try {
    loadingCalendar.value = true
    const data = await taxCalendarApi.listByCompany(companyId)
    // Sort entries by period descending, then payment due date descending
    calendarEntries.value = data.sort((a, b) => {
      const pDiff = new Date(b.taxPeriod).getTime() - new Date(a.taxPeriod).getTime()
      if (pDiff !== 0) return pDiff
      return new Date(b.paymentDueDate).getTime() - new Date(a.paymentDueDate).getTime()
    })
  } catch (err: any) {
    console.error('Failed to load tax calendar', err)
  } finally {
    loadingCalendar.value = false
  }
}

// Event handlers
async function applyFilters() {
  currentPage.value = 1
  await Promise.all([fetchSummary(), fetchRecords()])
}

function resetDateFilters() {
  filterStartDate.value = defaultStartDate
  filterEndDate.value = defaultEndDate
  applyFilters()
}

// Pagination logic
const totalPages = computed(() => Math.ceil(totalCount.value / perPage.value))

const paginationStart = computed(() => {
  if (totalCount.value === 0) return 0
  return (currentPage.value - 1) * perPage.value + 1
})

const paginationEnd = computed(() => {
  return Math.min(currentPage.value * perPage.value, totalCount.value)
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
  if (page < 1 || page > totalPages.value) return
  currentPage.value = page
  fetchRecords()
}

// Tax Calendar Status Updates
async function updateStatus(id: string, paymentStatus?: 'paid', filingStatus?: 'filed') {
  try {
    await taxCalendarApi.updateStatus(id, {
      paymentStatus,
      filingStatus
    })
    successMsg.value = 'Tax calendar status updated successfully.'
    await fetchCalendar()
  } catch (err: any) {
    errorMsg.value = err.response?.data?.message || 'Failed to update calendar status.'
  }
}

// Create Calendar Entry Modal Handlers
function openCreateCalendarModal() {
  modalError.value = null
  calendarForm.value = {
    taxTypeId: activeTaxTypes.value[0]?.id || '',
    taxPeriod: new Date().toISOString().split('T')[0],
    paymentDueDate: new Date(now.getFullYear(), now.getMonth() + 1, 15).toISOString().split('T')[0],
    filingDueDate: new Date(now.getFullYear(), now.getMonth() + 1, 20).toISOString().split('T')[0]
  }
  showCreateCalendarModal.value = true
}

function closeCreateCalendarModal() {
  showCreateCalendarModal.value = false
}

async function createCalendarEntry() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return

  try {
    creatingEntry.value = true
    modalError.value = null
    await taxCalendarApi.create({
      companyId,
      taxTypeId: calendarForm.value.taxTypeId,
      taxPeriod: calendarForm.value.taxPeriod,
      paymentDueDate: calendarForm.value.paymentDueDate,
      filingDueDate: calendarForm.value.filingDueDate
    })
    successMsg.value = 'Tax calendar entry added successfully.'
    closeCreateCalendarModal()
    await fetchCalendar()
  } catch (err: any) {
    modalError.value = err.response?.data?.message || 'Failed to create tax calendar entry.'
  } finally {
    creatingEntry.value = false
  }
}

// Formatting helpers
function formatPeriod(dateStr: string): string {
  if (!dateStr) return '-'
  try {
    const d = new Date(dateStr)
    return new Intl.DateTimeFormat('id-ID', { month: 'long', year: 'numeric' }).format(d)
  } catch {
    return dateStr
  }
}

function formatDocType(type: string): string {
  if (type === 'SalesInvoice') return 'Sales Invoice'
  if (type === 'PurchaseInvoice') return 'Purchase Invoice'
  return type
}

function getStatusBadgeClass(status: string): string {
  switch (status.toLowerCase()) {
    case 'paid':
      return 'badge-success'
    case 'reported':
    case 'validated':
      return 'badge-info'
    case 'required':
    case 'drafted':
      return 'badge-warning'
    case 'not_required':
      return 'badge-secondary'
    default:
      return 'badge-secondary'
  }
}

function formatStatusText(status: string): string {
  return status.replace('_', ' ').replace(/\b\w/g, c => c.toUpperCase())
}

function getTaxTypeCode(taxTypeId: string): string {
  const t = taxTypes.value.find(x => x.id === taxTypeId)
  return t ? t.code : 'TAX'
}

function getTaxTypeName(taxTypeId: string): string {
  const t = taxTypes.value.find(x => x.id === taxTypeId)
  return t ? t.name : 'Unknown Tax Type'
}

function isOverdue(dueDateStr: string, status: string): boolean {
  if (status === 'paid' || status === 'filed') return false
  const due = new Date(dueDateStr)
  const today = new Date()
  today.setHours(0, 0, 0, 0)
  return due < today
}

onMounted(() => {
  loadData()
})
</script>

<style scoped>
/* Page Header styling */
.header-actions {
  display: flex;
  gap: 12px;
}

/* Summary Panels */
.summary-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 24px;
  margin-bottom: 30px;
}

.summary-card {
  position: relative;
  border-radius: var(--radius-lg);
  padding: 28px;
  overflow: hidden;
  box-shadow: var(--shadow-md);
  transition: all var(--transition-normal);
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.summary-card:hover {
  transform: translateY(-4px);
  box-shadow: var(--shadow-lg), 0 0 25px rgba(79, 70, 229, 0.12);
}

/* Card Glow Effect */
.card-glow {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 1;
  opacity: 0.15;
  pointer-events: none;
}

.card-content {
  position: relative;
  z-index: 2;
}

.card-header-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.card-title {
  font-family: var(--font-heading);
  font-size: 0.95rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.card-icon {
  font-size: 1.5rem;
}

.card-amount {
  font-size: 2.2rem;
  font-weight: 700;
  letter-spacing: -0.02em;
  margin-bottom: 10px;
  line-height: 1.1;
  font-family: var(--font-heading);
}

.card-footer-info {
  font-size: 0.8rem;
  opacity: 0.8;
}

/* Custom Card Color Gradients */
.output-card {
  background: linear-gradient(135deg, #4f46e5 0%, #6366f1 100%);
  color: #ffffff;
}
.output-card .card-title, .output-card .card-footer-info {
  color: rgba(255, 255, 255, 0.85);
}

.input-card {
  background: linear-gradient(135deg, #0ea5e9 0%, #2563eb 100%);
  color: #ffffff;
}
.input-card .card-title, .input-card .card-footer-info {
  color: rgba(255, 255, 255, 0.85);
}

.payable-card {
  background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
  color: #ffffff;
}
.payable-card .card-title, .payable-card .card-footer-info {
  color: rgba(255, 255, 255, 0.85);
}

.refundable-card {
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  color: #ffffff;
}
.refundable-card .card-title, .refundable-card .card-footer-info {
  color: rgba(255, 255, 255, 0.85);
}

/* Period Filter Bar Styling */
.filter-card {
  margin-bottom: 30px;
}

.filter-form {
  display: flex;
  flex-wrap: wrap;
  gap: 20px;
  align-items: flex-end;
}

.filter-group {
  flex: 1;
  min-width: 200px;
  margin-bottom: 0;
}

.filter-actions {
  display: flex;
  gap: 12px;
}

/* Interactive Tabs */
.tabs-container {
  display: flex;
  border-bottom: 2px solid var(--border-color);
  margin-bottom: 24px;
  gap: 8px;
}

.tab-btn {
  background: none;
  border: none;
  padding: 12px 24px;
  font-family: var(--font-heading);
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-secondary);
  cursor: pointer;
  border-bottom: 3px solid transparent;
  transition: all var(--transition-fast);
}

.tab-btn:hover {
  color: var(--text-primary);
  background-color: var(--bg-tertiary);
  border-radius: var(--radius-sm) var(--radius-sm) 0 0;
}

.tab-btn.active {
  color: var(--accent-primary);
  border-bottom-color: var(--accent-primary);
}

/* Tab pane */
.tab-pane {
  animation: fadeIn 0.2s ease-out;
}

.tab-pane-header {
  margin-bottom: 16px;
}

.section-desc {
  color: var(--text-secondary);
  font-size: 0.875rem;
}

/* Custom Table additions */
.text-right {
  text-align: right;
}
.font-mono {
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
}
.font-bold {
  font-weight: 700;
}

.doc-link {
  color: var(--accent-primary);
  font-weight: 500;
}

.counterparty-info {
  display: flex;
  flex-direction: column;
}

.cp-name {
  font-weight: 500;
  color: var(--text-primary);
}

.cp-npwp {
  font-size: 0.75rem;
  color: var(--text-muted);
}

/* Overdue logic colors */
.date-cell {
  position: relative;
}

.overdue-date {
  color: var(--danger);
  font-weight: 600;
}

.overdue-label {
  display: inline-block;
  background-color: var(--danger-bg);
  color: var(--danger);
  font-size: 0.65rem;
  padding: 2px 6px;
  border-radius: 4px;
  margin-left: 8px;
  font-weight: 700;
  text-transform: uppercase;
}

/* Compliance status update actions styling */
.calendar-actions-cell {
  display: flex;
  gap: 8px;
  justify-content: center;
  align-items: center;
}

.btn-xs {
  padding: 4px 8px;
  font-size: 0.75rem;
  border-radius: 4px;
}

.btn-success-outline {
  border: 1px solid var(--success);
  color: var(--success);
  background: var(--success-bg);
}

.btn-success-outline:hover {
  background: var(--success);
  color: #fff;
}

.btn-info-outline {
  border: 1px solid var(--info);
  color: var(--info);
  background: var(--info-bg);
}

.btn-info-outline:hover {
  background: var(--info);
  color: #fff;
}

.compliance-complete {
  color: var(--success);
  font-weight: 600;
  font-size: 0.85rem;
}

.tax-type-code {
  font-weight: 600;
  background-color: var(--bg-tertiary);
  padding: 2px 6px;
  border-radius: 4px;
  font-family: var(--font-heading);
}

/* Alert styling with dismiss button */
.alert-dismissible {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-right: 16px;
}

.alert-close {
  background: none;
  border: none;
  font-size: 1.25rem;
  cursor: pointer;
  opacity: 0.5;
  color: inherit;
  transition: opacity var(--transition-fast);
}

.alert-close:hover {
  opacity: 1;
}

/* Form Helper & Grid */
.form-help {
  display: block;
  margin-top: 4px;
  color: var(--text-muted);
  font-size: 0.75rem;
}

.form-grid-2 {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(4px); }
  to { opacity: 1; transform: translateY(0); }
}
</style>
