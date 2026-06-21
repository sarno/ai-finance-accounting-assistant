<template>
  <MainLayout>
    <div class="page-header">
      <div>
        <h1>Sales Invoices</h1>
        <p class="page-title-desc">Generate, manage, and post customer sales billing and tax invoicing.</p>
      </div>
      <button class="btn btn-primary" @click="openCreateModal">+ Create Sales Invoice</button>
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
            placeholder="Search by invoice number, notes..."
            @input="applyFilters"
          />
        </div>
        <div style="width: 180px;">
          <select v-model="selectedStatus" class="form-select" @change="applyFilters">
            <option value="">All Statuses</option>
            <option value="draft">Draft</option>
            <option value="waiting_approval">Waiting Approval</option>
            <option value="posted">Posted</option>
            <option value="rejected">Rejected</option>
          </select>
        </div>
        <div style="width: 180px;">
          <select v-model="selectedBranch" class="form-select" @change="applyFilters">
            <option value="">All Branches</option>
            <option v-for="branch in branches" :key="branch.id" :value="branch.id">
              {{ branch.name }}
            </option>
          </select>
        </div>
        <div>
          <button class="btn btn-secondary" @click="resetFilters">Reset</button>
        </div>
      </div>
    </div>

    <!-- Invoices Table -->
    <div class="table-container" style="overflow: visible !important;">
      <div v-if="invoiceStore.loading && paginatedInvoices.length === 0" class="loading-state">
        Loading sales invoices...
      </div>
      <div v-else-if="filteredInvoices.length === 0" class="empty-state">
        <span v-if="searchTerm || selectedStatus || selectedBranch">No invoices match your filters.</span>
        <span v-else>No invoices found. Click "+ Create Sales Invoice" to start.</span>
      </div>
      <table v-else>
        <thead>
          <tr>
            <th>Invoice Date</th>
            <th>Invoice Number</th>
            <th>Branch</th>
            <th>Customer</th>
            <th style="text-align: right;">Subtotal</th>
            <th style="text-align: right;">Tax Amount</th>
            <th style="text-align: right;">Total Amount</th>
            <th>Status</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="invoice in paginatedInvoices" :key="invoice.id">
            <td>{{ formatDate(invoice.invoiceDate) }}</td>
            <td style="font-weight: 600; color: var(--accent-primary);">
              {{ invoice.invoiceNumber }}
            </td>
            <td>
              <span class="badge badge-secondary" style="font-weight: 600;">
                {{ getBranchName(invoice.branchId) }}
              </span>
            </td>
            <td>{{ getCustomerName(invoice.customerId) }}</td>
            <td style="text-align: right;">{{ formatCurrency(invoice.subtotal) }}</td>
            <td style="text-align: right;">{{ formatCurrency(invoice.taxAmount) }}</td>
            <td style="text-align: right; font-weight: 600;">{{ formatCurrency(invoice.totalAmount) }}</td>
            <td>
              <span :class="['badge', getStatusBadgeClass(invoice.status)]">
                {{ formatStatus(invoice.status) }}
              </span>
            </td>
            <td style="position: relative; width: 80px; text-align: center;">
              <div class="row-actions-container">
                <button class="btn-actions-trigger" @click.stop="toggleRowDropdown(invoice.id)">
                  <svg class="actions-trigger-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <circle cx="12" cy="12" r="1.5" />
                    <circle cx="19" cy="12" r="1.5" />
                    <circle cx="5" cy="12" r="1.5" />
                  </svg>
                </button>

                <transition name="dropdown-fade">
                  <div v-if="activeDropdownId === invoice.id" class="row-actions-dropdown">
                    <button class="dropdown-item" @click="viewDetails(invoice)">
                      👁️ View Details
                    </button>
                    <button
                      v-if="invoice.status === 'draft' || invoice.status === 'rejected'"
                      class="dropdown-item"
                      @click="openEditModal(invoice)"
                    >
                      ✏️ Edit Draft
                    </button>
                    <button
                      v-if="invoice.status === 'draft' || invoice.status === 'rejected'"
                      class="dropdown-item"
                      @click="submitApproval(invoice.id)"
                    >
                      📤 Submit Approval
                    </button>
                    <div v-if="invoice.status === 'draft' || invoice.status === 'rejected'" class="dropdown-divider"></div>
                    <button
                      v-if="invoice.status === 'draft' || invoice.status === 'rejected'"
                      class="dropdown-item text-danger"
                      @click="confirmDelete(invoice.id)"
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
    <div class="pagination-footer" v-if="filteredInvoices.length > 0">
      <div class="pagination-info">
        Showing {{ paginationStart }} to {{ paginationEnd }} of {{ filteredInvoices.length }} entries
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

    <!-- Create/Edit Modal -->
    <div v-if="showFormModal" class="modal-overlay" @click.self="closeFormModal">
      <div class="modal-content modal-lg-custom">
        <div class="modal-header">
          <h2>{{ isEdit ? 'Edit Draft Sales Invoice' : 'Create Sales Invoice Draft' }}</h2>
          <button class="modal-close" @click="closeFormModal">&times;</button>
        </div>
        <div class="modal-body">
          <form @submit.prevent="saveDraft">
            <div class="form-grid-3">
              <div class="form-group">
                <label class="form-label">Branch *</label>
                <select v-model="form.branchId" class="form-select" required>
                  <option value="">Select Branch</option>
                  <option v-for="b in branches" :key="b.id" :value="b.id">
                    {{ b.name }} ({{ b.code }})
                  </option>
                </select>
              </div>
              <div class="form-group">
                <label class="form-label">Invoice Number *</label>
                <input v-model="form.invoiceNumber" type="text" class="form-input" placeholder="INV/2026/001" required />
              </div>
              <div class="form-group custom-select-search-container" ref="customerDropdownRef">
                <label class="form-label">Customer *</label>
                <div class="custom-select-search-wrapper">
                  <input
                    type="text"
                    class="form-input"
                    placeholder="Search and select customer..."
                    v-model="customerSearchTerm"
                    @focus="isCustomerDropdownOpen = true"
                    required
                  />
                  <span class="custom-select-arrow" @click="toggleCustomerDropdown">▼</span>

                  <div v-show="isCustomerDropdownOpen" class="custom-select-dropdown">
                    <div
                      v-for="c in filteredFormCustomers"
                      :key="c.id"
                      class="custom-select-option"
                      @click="selectFormCustomer(c)"
                    >
                      {{ c.name }}
                    </div>
                    <div v-if="filteredFormCustomers.length === 0" class="custom-select-no-results">
                      No customers found
                    </div>
                  </div>
                </div>
                <input type="hidden" :value="form.customerId" required name="customerId" />
              </div>
            </div>

            <div class="form-grid-2" style="margin-top: 16px;">
              <div class="form-group">
                <label class="form-label">Invoice Date *</label>
                <input v-model="form.invoiceDate" type="date" class="form-input" required />
              </div>
              <div class="form-group">
                <label class="form-label">Due Date *</label>
                <input v-model="form.dueDate" type="date" class="form-input" required />
              </div>
            </div>

            <div class="form-group" style="margin-top: 16px;">
              <label class="form-label">Notes</label>
              <textarea v-model="form.notes" class="form-textarea" rows="2" placeholder="Internal notes or customer remarks..."></textarea>
            </div>

            <!-- Invoice Lines -->
            <div style="margin-top: 24px;">
              <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                <h3>Invoice Lines</h3>
                <button type="button" class="btn btn-secondary btn-sm" @click="addLine">+ Add Item Line</button>
              </div>

              <div class="table-container" style="overflow: visible !important;">
                <table>
                  <thead>
                    <tr>
                      <th style="width: 30%;">Item Description *</th>
                      <th style="width: 10%; text-align: right;">Quantity *</th>
                      <th style="width: 15%; text-align: right;">Unit Price *</th>
                      <th style="width: 10%; text-align: right;">Discount</th>
                      <th style="width: 15%;">Tax Rate</th>
                      <th style="width: 15%;">Revenue Account *</th>
                      <th style="width: 5%;"></th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr v-for="(line, index) in form.lines" :key="index">
                      <td>
                        <input v-model="line.description" type="text" class="form-input form-input-sm" placeholder="Description of goods/services" required />
                      </td>
                      <td>
                        <input v-model.number="line.quantity" type="number" step="any" min="0.0001" class="form-input form-input-sm" style="text-align: right;" required />
                      </td>
                      <td>
                        <input v-model.number="line.unitPrice" type="number" step="any" min="0" class="form-input form-input-sm" style="text-align: right;" required />
                      </td>
                      <td>
                        <input v-model.number="line.discountAmount" type="number" step="any" min="0" class="form-input form-input-sm" style="text-align: right;" />
                      </td>
                      <td>
                        <select v-model="line.taxTypeId" class="form-select form-select-sm">
                          <option value="">No Tax (0%)</option>
                          <option v-for="t in taxTypes" :key="t.id" :value="t.id">
                            {{ t.name }} ({{ (Number(t.defaultRate) * 100).toFixed(0) }}%)
                          </option>
                        </select>
                      </td>
                      <td>
                        <select v-model="line.accountId" class="form-select form-select-sm" required>
                          <option value="">Select Account</option>
                          <option v-for="a in revenueAccounts" :key="a.id" :value="a.id">
                            {{ a.code }} - {{ a.name }}
                          </option>
                        </select>
                      </td>
                      <td style="text-align: center;">
                        <button type="button" class="btn-delete-row" @click="removeLine(index)">&times;</button>
                      </td>
                    </tr>
                    <tr v-if="form.lines.length === 0">
                      <td colspan="7" class="empty-state" style="padding: 16px;">
                        No lines added. Click "+ Add Item Line" to add invoice items.
                      </td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </div>

            <!-- Totals Summary Card -->
            <div class="totals-summary-card">
              <div class="totals-row">
                <span>Subtotal:</span>
                <span class="totals-val">{{ formatCurrency(computedSubtotal) }}</span>
              </div>
              <div class="totals-row">
                <span>Tax Amount:</span>
                <span class="totals-val">{{ formatCurrency(computedTaxAmount) }}</span>
              </div>
              <div class="totals-row grand-total">
                <span>Total Amount:</span>
                <span>{{ formatCurrency(computedTotalAmount) }}</span>
              </div>
            </div>

            <div class="modal-footer" style="margin-top: 24px; padding: 16px 0 0 0;">
              <button type="button" class="btn btn-secondary" @click="closeFormModal">Cancel</button>
              <button type="submit" class="btn btn-primary" :disabled="submitting || form.lines.length === 0">
                {{ submitting ? 'Saving...' : 'Save Draft' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>

    <!-- Detail Modal -->
    <div v-if="showDetailModal && selectedInvoice" class="modal-overlay" @click.self="closeDetailModal">
      <div class="modal-content modal-lg-custom">
        <div class="modal-header">
          <h2>Sales Invoice Details</h2>
          <button class="modal-close" @click="closeDetailModal">&times;</button>
        </div>
        <div class="modal-body">
          <div class="invoice-badge-header">
            <span :class="['badge badge-lg', getStatusBadgeClass(selectedInvoice.status)]">
              {{ formatStatus(selectedInvoice.status) }}
            </span>
          </div>

          <div class="meta-card" style="margin-top: 16px;">
            <div>
              <p class="meta-label">Invoice Number</p>
              <p class="meta-val-highlight">{{ selectedInvoice.invoiceNumber }}</p>
            </div>
            <div>
              <p class="meta-label">Customer</p>
              <p class="meta-val">{{ getCustomerName(selectedInvoice.customerId) }}</p>
            </div>
            <div>
              <p class="meta-label">Branch</p>
              <p class="meta-val">{{ getBranchName(selectedInvoice.branchId) }}</p>
            </div>
            <div>
              <p class="meta-label">Invoice Date</p>
              <p class="meta-val">{{ formatDate(selectedInvoice.invoiceDate) }}</p>
            </div>
            <div>
              <p class="meta-label">Due Date</p>
              <p class="meta-val">{{ formatDate(selectedInvoice.dueDate) }}</p>
            </div>
            <div>
              <p class="meta-label">Journal Link</p>
              <p class="meta-val">
                <span v-if="selectedInvoice.journalEntryId" style="font-family: monospace; font-size: 0.85rem; color: var(--accent-primary);">
                  {{ selectedInvoice.journalEntryId.substring(0, 8) }} (Posted)
                </span>
                <span v-else class="text-muted">Not Posted</span>
              </p>
            </div>
          </div>

          <div v-if="selectedInvoice.notes" style="margin-top: 20px;" class="notes-box">
            <p style="font-size: 0.75rem; font-weight: 600; text-transform: uppercase; color: var(--text-secondary); margin-bottom: 4px;">Notes / Remarks</p>
            <p style="font-size: 0.9rem; color: var(--text-primary);">{{ selectedInvoice.notes }}</p>
          </div>

          <div style="margin-top: 24px;">
            <h3>Line Items</h3>
            <div class="table-container" style="margin-top: 8px;">
              <table>
                <thead>
                  <tr>
                    <th>Item Description</th>
                    <th>Account</th>
                    <th style="text-align: right;">Quantity</th>
                    <th style="text-align: right;">Unit Price</th>
                    <th style="text-align: right;">Discount</th>
                    <th style="text-align: right;">Tax Rate</th>
                    <th style="text-align: right;">Tax Amount</th>
                    <th style="text-align: right;">Line Total</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="line in selectedInvoice.lines" :key="line.id">
                    <td>{{ line.description }}</td>
                    <td>{{ getAccountName(line.accountId) }}</td>
                    <td style="text-align: right;">{{ line.quantity }}</td>
                    <td style="text-align: right;">{{ formatCurrency(line.unitPrice) }}</td>
                    <td style="text-align: right;">{{ line.discountAmount > 0 ? formatCurrency(line.discountAmount) : '-' }}</td>
                    <td style="text-align: right;">{{ line.taxRate ? `${(Number(line.taxRate) * 100).toFixed(0)}%` : '-' }}</td>
                    <td style="text-align: right;">{{ line.taxAmount > 0 ? formatCurrency(line.taxAmount) : '-' }}</td>
                    <td style="text-align: right; font-weight: 500;">{{ formatCurrency(line.lineTotal) }}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>

          <!-- Total display inside invoice details -->
          <div class="totals-summary-card" style="margin-top: 16px;">
            <div class="totals-row">
              <span>Subtotal:</span>
              <span class="totals-val">{{ formatCurrency(selectedInvoice.subtotal) }}</span>
            </div>
            <div class="totals-row">
              <span>Tax Total:</span>
              <span class="totals-val">{{ formatCurrency(selectedInvoice.taxAmount) }}</span>
            </div>
            <div class="totals-row grand-total">
              <span>Total Amount:</span>
              <span>{{ formatCurrency(selectedInvoice.totalAmount) }}</span>
            </div>
          </div>
        </div>
        <div class="modal-footer">
          <button type="button" class="btn btn-secondary" @click="closeDetailModal">Close</button>
          <button
            v-if="selectedInvoice.status === 'draft' || selectedInvoice.status === 'rejected'"
            type="button"
            class="btn btn-primary"
            @click="submitApproval(selectedInvoice.id); closeDetailModal();"
          >
            Submit for Approval
          </button>
        </div>
      </div>
    </div>

    <!-- Reusable Deletion Confirm Dialog -->
    <ConfirmDialog
      :isOpen="showConfirmDelete"
      title="Delete Invoice Draft"
      message="Are you sure you want to delete this draft sales invoice? This action is permanent and cannot be undone."
      confirmText="Yes, Delete Draft"
      cancelText="Cancel"
      :isDanger="true"
      @confirm="handleDelete"
      @cancel="showConfirmDelete = false"
    />
  </MainLayout>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import MainLayout from '@/components/MainLayout.vue'
import ConfirmDialog from '@/components/ConfirmDialog.vue'
import { useInvoiceStore } from '@/stores/invoice.store'
import { useAuthStore } from '@/stores/auth.store'
import { customerApi, taxTypeApi, branchApi, accountApi, itemApi } from '@/api/master-data.api'
import type { SalesInvoice } from '@/types/invoice.types'
import type { Customer, TaxType, Branch, Account, Item } from '@/types/master-data.types'

const invoiceStore = useInvoiceStore()
const authStore = useAuthStore()

// Master Data Lists
const customers = ref<Customer[]>([])
const taxTypes = ref<TaxType[]>([])
const branches = ref<Branch[]>([])
const accounts = ref<Account[]>([])
const items = ref<Item[]>([])

// Notification messages
const successMsg = ref<string | null>(null)
const errorMsg = ref<string | null>(null)

// Action/Dropdown states
const activeDropdownId = ref<string | null>(null)
const showDetailModal = ref(false)
const selectedInvoice = ref<SalesInvoice | null>(null)

// Search, filtering, and paging
const searchTerm = ref('')
const selectedStatus = ref('')

// Custom Select Search for Customer
const isCustomerDropdownOpen = ref(false)
const customerSearchTerm = ref('')
const customerDropdownRef = ref<HTMLElement | null>(null)

const filteredFormCustomers = computed(() => {
  if (!customerSearchTerm.value) {
    return customers.value
  }
  const q = customerSearchTerm.value.toLowerCase()
  return customers.value.filter(c => c.name.toLowerCase().includes(q))
})

function selectFormCustomer(c: Customer) {
  form.value.customerId = c.id
  customerSearchTerm.value = c.name
  isCustomerDropdownOpen.value = false
}

function toggleCustomerDropdown(event: Event) {
  event.stopPropagation()
  isCustomerDropdownOpen.value = !isCustomerDropdownOpen.value
}
const selectedBranch = ref('')
const currentPage = ref(1)
const perPage = ref(5)

// Draft Edit/Create Form
const showFormModal = ref(false)
const isEdit = ref(false)
const targetEditId = ref<string | null>(null)
const submitting = ref(false)

const form = ref({
  branchId: '',
  invoiceNumber: '',
  customerId: '',
  invoiceDate: new Date().toISOString().substring(0, 10),
  dueDate: new Date().toISOString().substring(0, 10),
  notes: '',
  lines: [] as Array<{
    description: string
    quantity: number
    unitPrice: number
    discountAmount: number
    taxTypeId: string
    accountId: string
  }>
})

// Deletion confirm state
const showConfirmDelete = ref(false)
const targetDeleteId = ref<string | null>(null)

// Watch document click to close row actions dropdowns
onMounted(async () => {
  window.addEventListener('click', closeDropdowns)
  await loadMasterData()
  await fetchInvoices()
})

function closeDropdowns(event: MouseEvent) {
  activeDropdownId.value = null
  
  if (customerDropdownRef.value && !customerDropdownRef.value.contains(event.target as Node)) {
    isCustomerDropdownOpen.value = false
    
    // Restore search term based on current selection
    if (form.value.customerId) {
      const selected = customers.value.find(c => c.id === form.value.customerId)
      if (selected) {
        customerSearchTerm.value = selected.name
      }
    } else {
      customerSearchTerm.value = ''
    }
  }
}

async function loadMasterData() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return
  try {
    const [cRes, tRes, bRes, aRes, iRes] = await Promise.all([
      customerApi.listByCompany(companyId),
      taxTypeApi.listByCompany(companyId),
      branchApi.listByCompany(companyId),
      accountApi.listByCompany(companyId),
      itemApi.listByCompany(companyId)
    ])
    customers.value = cRes
    taxTypes.value = tRes
    branches.value = bRes
    accounts.value = aRes
    items.value = iRes
  } catch (err) {
    console.error('Failed to load master data lists', err)
  }
}

async function fetchInvoices() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return
  try {
    invoiceStore.setFilters({ page: 1, perPage: 100 })
    await invoiceStore.fetchSalesInvoices()
  } catch (err: any) {
    errorMsg.value = err.message || 'Failed to fetch sales invoices.'
  }
}

// Maps and names lookups
const customerMap = computed(() => new Map<string, Customer>(customers.value.map(c => [c.id, c])))
const branchMap = computed(() => new Map<string, Branch>(branches.value.map(b => [b.id, b])))
const accountMap = computed(() => new Map<string, Account>(accounts.value.map(a => [a.id, a])))

function getCustomerName(id: string): string {
  return customerMap.value.get(id)?.name || id
}

function getBranchName(id?: string): string {
  if (!id) return '-'
  return branchMap.value.get(id)?.name || id
}

function getAccountName(id: string): string {
  const acc = accountMap.value.get(id)
  return acc ? `${acc.code} - ${acc.name}` : id
}

const revenueAccounts = computed(() => {
  return accounts.value.filter(a => a.accountType === 'Revenue' || a.accountType === 'Asset')
})

// Filter and search logic
const filteredInvoices = computed(() => {
  return invoiceStore.invoices.filter(i => {
    if (searchTerm.value) {
      const q = searchTerm.value.toLowerCase()
      const matchesNum = i.invoiceNumber.toLowerCase().includes(q)
      const matchesNotes = (i.notes || '').toLowerCase().includes(q)
      const matchesCust = getCustomerName(i.customerId).toLowerCase().includes(q)
      if (!matchesNum && !matchesNotes && !matchesCust) return false
    }
    if (selectedStatus.value && i.status.toLowerCase() !== selectedStatus.value.toLowerCase()) {
      return false
    }
    if (selectedBranch.value && i.branchId !== selectedBranch.value) {
      return false
    }
    return true
  })
})

// Paging computations
const paginatedInvoices = computed(() => {
  const start = (currentPage.value - 1) * perPage.value
  const end = start + perPage.value
  return filteredInvoices.value.slice(start, end)
})

const totalPages = computed(() => Math.ceil(filteredInvoices.value.length / perPage.value))

const paginationStart = computed(() => {
  if (filteredInvoices.value.length === 0) return 0
  return (currentPage.value - 1) * perPage.value + 1
})

const paginationEnd = computed(() => Math.min(currentPage.value * perPage.value, filteredInvoices.value.length))

const visiblePages = computed(() => {
  const pages: number[] = []
  const maxVisible = 5
  let start = Math.max(1, currentPage.value - Math.floor(maxVisible / 2))
  let end = Math.min(totalPages.value, start + maxVisible - 1)
  if (end - start + 1 < maxVisible) start = Math.max(1, end - maxVisible + 1)
  for (let i = start; i <= end; i++) pages.push(i)
  return pages
})

function changePage(page: number) {
  if (page >= 1 && page <= totalPages.value) currentPage.value = page
}

function applyFilters() {
  currentPage.value = 1
}

function resetFilters() {
  searchTerm.value = ''
  selectedStatus.value = ''
  selectedBranch.value = ''
  currentPage.value = 1
}

// Action dropdown logic
function toggleRowDropdown(id: string) {
  if (activeDropdownId.value === id) activeDropdownId.value = null
  else activeDropdownId.value = id
}

// Modal action triggers
function openCreateModal() {
  isEdit.value = false
  targetEditId.value = null
  customerSearchTerm.value = ''
  form.value = {
    branchId: branches.value[0]?.id || '',
    invoiceNumber: `INV/${new Date().getFullYear()}/${String(invoiceStore.invoices.length + 1).padStart(3, '0')}`,
    customerId: '',
    invoiceDate: new Date().toISOString().substring(0, 10),
    dueDate: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString().substring(0, 10), // default Net 30
    notes: '',
    lines: []
  }
  addLine()
  showFormModal.value = true
}

function openEditModal(invoice: SalesInvoice) {
  isEdit.value = true
  targetEditId.value = invoice.id
  
  const customer = customers.value.find(c => c.id === invoice.customerId)
  customerSearchTerm.value = customer ? customer.name : ''
  
  form.value = {
    branchId: invoice.branchId || '',
    invoiceNumber: invoice.invoiceNumber,
    customerId: invoice.customerId,
    invoiceDate: new Date(invoice.invoiceDate).toISOString().substring(0, 10),
    dueDate: new Date(invoice.dueDate).toISOString().substring(0, 10),
    notes: invoice.notes || '',
    lines: invoice.lines.map(line => ({
      itemId: '',
      description: line.description,
      quantity: Number(line.quantity),
      unitPrice: Number(line.unitPrice),
      discountAmount: Number(line.discountAmount),
      taxTypeId: line.taxTypeId || '',
      accountId: line.accountId
    }))
  }
  showFormModal.value = true
}

function closeFormModal() {
  showFormModal.value = false
}

function addLine() {
  const defaultAccount = accounts.value.find(a => a.code === '4000' || a.accountType === 'Revenue')
  form.value.lines.push({
    itemId: '',
    description: '',
    quantity: 1,
    unitPrice: 0,
    discountAmount: 0,
    taxTypeId: taxTypes.value[0]?.id || '',
    accountId: defaultAccount?.id || ''
  })
}

function onItemChange(line: any) {
  if (!line.itemId) return
  const item = items.value.find(i => i.id === line.itemId)
  if (item) {
    line.description = item.name
    line.unitPrice = Number(item.unitPrice)
    if (item.taxTypeId) {
      line.taxTypeId = item.taxTypeId
    }
    if (item.saleAccountId) {
      line.accountId = item.saleAccountId
    }
  }
}

function removeLine(index: number) {
  form.value.lines.splice(index, 1)
}

function getTaxRate(taxTypeId: string): number {
  if (!taxTypeId) return 0
  const tax = taxTypes.value.find(t => t.id === taxTypeId)
  return tax ? Number(tax.defaultRate) : 0
}

// Summary computations
const computedSubtotal = computed(() => {
  return form.value.lines.reduce((sum, line) => sum + (line.quantity * line.unitPrice - line.discountAmount), 0)
})

const computedTaxAmount = computed(() => {
  return form.value.lines.reduce((sum, line) => {
    const net = line.quantity * line.unitPrice - line.discountAmount
    const rate = getTaxRate(line.taxTypeId)
    return sum + (net * rate)
  }, 0)
})

const computedTotalAmount = computed(() => computedSubtotal.value + computedTaxAmount.value)

// Save Sales Invoice draft
async function saveDraft() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return
  submitting.value = true
  errorMsg.value = null
  successMsg.value = null

  const payload = {
    companyId,
    branchId: form.value.branchId || undefined,
    invoiceNumber: form.value.invoiceNumber,
    customerId: form.value.customerId,
    invoiceDate: form.value.invoiceDate,
    dueDate: form.value.dueDate,
    notes: form.value.notes || undefined,
    lines: form.value.lines.map((line, idx) => ({
      description: line.description,
      quantity: Number(line.quantity),
      unitPrice: Number(line.unitPrice),
      discountAmount: Number(line.discountAmount) || 0,
      taxTypeId: line.taxTypeId || undefined,
      accountId: line.accountId,
      sortOrder: idx
    }))
  }

  try {
    if (isEdit.value && targetEditId.value) {
      await invoiceStore.updateSalesDraft(targetEditId.value, payload)
      successMsg.value = 'Sales invoice draft updated successfully.'
    } else {
      await invoiceStore.createSalesDraft(payload)
      successMsg.value = 'Sales invoice draft created.'
    }
    showFormModal.value = false
    await fetchInvoices()
  } catch (err: any) {
    errorMsg.value = err.response?.data?.message || err.message || 'Failed to save sales invoice draft.'
  } finally {
    submitting.value = false
  }
}

// Delete action triggers
function confirmDelete(id: string) {
  targetDeleteId.value = id
  showConfirmDelete.value = true
}

async function handleDelete() {
  if (!targetDeleteId.value) return
  errorMsg.value = null
  successMsg.value = null
  try {
    await invoiceStore.deleteSalesDraft(targetDeleteId.value)
    successMsg.value = 'Sales invoice draft deleted.'
    await fetchInvoices()
  } catch (err: any) {
    errorMsg.value = err.message || 'Failed to delete sales invoice.'
  } finally {
    showConfirmDelete.value = false
    targetDeleteId.value = null
  }
}

// Submit approval
async function submitApproval(id: string) {
  errorMsg.value = null
  successMsg.value = null
  try {
    await invoiceStore.submitSalesApproval(id)
    successMsg.value = 'Invoice submitted for approval successfully.'
    await fetchInvoices()
  } catch (err: any) {
    errorMsg.value = err.message || 'Failed to submit sales invoice for approval.'
  }
}

// View details modal
function viewDetails(invoice: SalesInvoice) {
  selectedInvoice.value = invoice
  showDetailModal.value = true
}

function closeDetailModal() {
  showDetailModal.value = false
  selectedInvoice.value = null
}

// Utility styling methods
function getStatusBadgeClass(status: string): string {
  const s = status.toLowerCase()
  if (s === 'draft') return 'badge-secondary'
  if (s === 'waiting_approval') return 'badge-warning'
  if (s === 'posted') return 'badge-success'
  if (s === 'rejected') return 'badge-danger'
  return 'badge-secondary'
}

function formatStatus(status: string): string {
  return status.replace('_', ' ').toUpperCase()
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
</script>

<style scoped>
.modal-lg-custom {
  max-width: 960px;
}

.form-grid-3 {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
}

.form-grid-2 {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 16px;
}

.form-input-sm, .form-select-sm {
  padding: 6px 8px;
  font-size: 0.8rem;
  border-radius: var(--radius-sm);
}

.btn-delete-row {
  background: none;
  border: none;
  color: var(--danger);
  font-size: 1.5rem;
  cursor: pointer;
  padding: 0 8px;
  line-height: 1;
}

.btn-delete-row:hover {
  color: #b91c1c;
}

.totals-summary-card {
  margin-top: 20px;
  background-color: var(--bg-tertiary);
  padding: 16px 24px;
  border-radius: var(--radius-md);
  border: 1px solid var(--border-color);
  max-width: 320px;
  margin-left: auto;
}

.totals-row {
  display: flex;
  justify-content: space-between;
  margin-bottom: 8px;
  font-size: 0.9rem;
  color: var(--text-secondary);
}

.totals-val {
  font-weight: 600;
  color: var(--text-primary);
}

.grand-total {
  margin-top: 12px;
  border-top: 2px solid var(--border-color);
  padding-top: 12px;
  font-size: 1.1rem;
  font-weight: 700;
  color: var(--accent-primary);
}

.btn-xs {
  padding: 4px 8px;
  font-size: 0.75rem;
}

.invoice-badge-header {
  display: flex;
  justify-content: flex-end;
}

.badge-lg {
  padding: 6px 12px;
  font-size: 0.85rem;
}

.notes-box {
  background-color: var(--bg-tertiary);
  border-left: 4px solid var(--accent-primary);
  padding: 12px 16px;
  border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
}

.meta-card {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 16px;
  background-color: var(--bg-tertiary);
  padding: 16px;
  border-radius: var(--radius-md);
  border: 1px solid var(--border-color);
}

.meta-label {
  font-size: 0.725rem;
  text-transform: uppercase;
  font-weight: 600;
  color: var(--text-secondary);
  margin-bottom: 2px;
}

.meta-val {
  font-size: 0.9rem;
  font-weight: 500;
  color: var(--text-primary);
}

.meta-val-highlight {
  font-size: 0.95rem;
  font-weight: 700;
  color: var(--accent-primary);
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

/* Custom Select Search styles */
.custom-select-search-container {
  position: relative;
}

.custom-select-search-wrapper {
  position: relative;
  display: flex;
  align-items: center;
  width: 100%;
}

.custom-select-search-wrapper .form-input {
  width: 100%;
  padding-right: 32px;
}

.custom-select-arrow {
  position: absolute;
  right: 12px;
  font-size: 0.65rem;
  color: var(--text-secondary);
  cursor: pointer;
  user-select: none;
}

.custom-select-dropdown {
  position: absolute;
  top: calc(100% + 4px);
  left: 0;
  width: 100%;
  max-height: 200px;
  overflow-y: auto;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg);
  z-index: 1000;
}

.custom-select-option {
  padding: 10px 14px;
  font-size: 0.875rem;
  color: var(--text-primary);
  cursor: pointer;
  transition: background-color var(--transition-fast);
  text-align: left;
}

.custom-select-option:hover {
  background-color: var(--bg-tertiary);
  color: var(--accent-primary);
}

.custom-select-no-results {
  padding: 12px 14px;
  font-size: 0.875rem;
  color: var(--text-muted);
  text-align: center;
}
</style>
