<template>
  <MainLayout>
    <div class="page-header">
      <div>
        <h1>Purchase Invoices</h1>
        <p class="page-title-desc">Create and manage supplier bills with tax calculation and approval flow.</p>
      </div>
      <button class="btn btn-primary" @click="openCreateModal">+ Create Purchase Invoice</button>
    </div>

    <div v-if="successMsg" class="alert alert-success">{{ successMsg }}</div>
    <div v-if="errorMsg" class="alert alert-danger">{{ errorMsg }}</div>

    <div class="card" style="margin-bottom: 24px; padding: 16px;">
      <div style="display: flex; gap: 16px; flex-wrap: wrap; align-items: center;">
        <div style="flex: 1; min-width: 220px;">
          <input
            v-model="searchTerm"
            type="text"
            class="form-input"
            placeholder="Search supplier invoice, internal reference, notes..."
          />
        </div>
        <div style="width: 180px;">
          <select v-model="selectedStatus" class="form-select">
            <option value="">All Statuses</option>
            <option value="draft">Draft</option>
            <option value="waiting_approval">Waiting Approval</option>
            <option value="posted">Posted</option>
            <option value="rejected">Rejected</option>
          </select>
        </div>
        <div>
          <button class="btn btn-secondary" @click="resetFilters">Reset</button>
        </div>
      </div>
    </div>

    <div class="table-container" style="overflow: visible !important;">
      <div v-if="loading && filteredPurchases.length === 0" class="loading-state">
        Loading purchase invoices...
      </div>
      <div v-else-if="filteredPurchases.length === 0" class="empty-state">
        <span v-if="searchTerm || selectedStatus">No purchase invoices match your filters.</span>
        <span v-else>No purchase invoices found. Click "+ Create Purchase Invoice" to start.</span>
      </div>
      <table v-else>
        <thead>
          <tr>
            <th>Invoice Date</th>
            <th>Supplier Invoice #</th>
            <th>Internal Ref</th>
            <th>Supplier</th>
            <th style="text-align: right;">Subtotal</th>
            <th style="text-align: right;">Tax Amount</th>
            <th style="text-align: right;">Total Amount</th>
            <th>Status</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="invoice in filteredPurchases" :key="invoice.id">
            <td>{{ formatDate(invoice.invoiceDate) }}</td>
            <td style="font-weight: 600; color: var(--accent-primary);">{{ invoice.supplierInvoiceNumber }}</td>
            <td style="font-family: monospace;">{{ invoice.internalReference }}</td>
            <td>{{ getSupplierName(invoice.supplierId) }}</td>
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
                <button class="btn-actions-trigger" @click="toggleRowDropdown(invoice.id, $event)">
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

    <div v-if="showDetailModal && selectedInvoice" class="modal-overlay" @click.self="closeDetailModal">
      <div class="modal-content modal-lg-custom">
        <div class="modal-header">
          <h2>Purchase Invoice Details</h2>
          <button class="modal-close" @click="closeDetailModal">&times;</button>
        </div>
        <div class="modal-body">
          <div class="meta-card">
            <div>
              <p class="meta-label">Supplier Invoice #</p>
              <p class="meta-val-highlight">{{ selectedInvoice.supplierInvoiceNumber }}</p>
            </div>
            <div>
              <p class="meta-label">Internal Ref</p>
              <p class="meta-val">{{ selectedInvoice.internalReference }}</p>
            </div>
            <div>
              <p class="meta-label">Supplier</p>
              <p class="meta-val">{{ getSupplierName(selectedInvoice.supplierId) }}</p>
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
              <p class="meta-label">Status</p>
              <p><span :class="['badge', getStatusBadgeClass(selectedInvoice.status)]">{{ formatStatus(selectedInvoice.status) }}</span></p>
            </div>
          </div>

          <div v-if="selectedInvoice.notes" class="notes-box" style="margin-top: 20px;">
            <p class="notes-label">Notes</p>
            <p>{{ selectedInvoice.notes }}</p>
          </div>

          <div style="margin-top: 24px;">
            <h3>Line Items</h3>
            <div class="table-container" style="margin-top: 8px;">
              <table>
                <thead>
                  <tr>
                    <th>Item</th>
                    <th>Description</th>
                    <th>Expense Account</th>
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
                    <td>{{ getItemLabel(line.itemId) || '-' }}</td>
                    <td>{{ line.description }}</td>
                    <td>{{ getAccountName(line.accountId) }}</td>
                    <td style="text-align: right;">{{ formatQuantity(line.quantity) }}</td>
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

          <div class="totals-summary-card">
            <div class="totals-row"><span>Subtotal:</span><span class="totals-val">{{ formatCurrency(selectedInvoice.subtotal) }}</span></div>
            <div class="totals-row"><span>Tax Amount:</span><span class="totals-val">{{ formatCurrency(selectedInvoice.taxAmount) }}</span></div>
            <div class="totals-row grand-total"><span>Total Amount:</span><span>{{ formatCurrency(selectedInvoice.totalAmount) }}</span></div>
          </div>
        </div>
        <div class="modal-footer">
          <button type="button" class="btn btn-secondary" @click="closeDetailModal">Close</button>
          <button
            v-if="selectedInvoice.status === 'draft' || selectedInvoice.status === 'rejected'"
            type="button"
            class="btn btn-secondary"
            style="background: #e2e8f0; color: var(--text-primary);"
            @click="editFromDetail"
          >
            ✏️ Edit Draft
          </button>
          <button
            v-if="selectedInvoice.status === 'draft' || selectedInvoice.status === 'rejected'"
            type="button"
            class="btn btn-danger"
            @click="deleteFromDetail"
          >
            🗑️ Delete Draft
          </button>
          <button
            v-if="selectedInvoice.status === 'draft' || selectedInvoice.status === 'rejected'"
            type="button"
            class="btn btn-primary"
            style="background-color: var(--info-bg); color: var(--info); border: 1px solid rgba(59, 130, 246, 0.15);"
            @click="submitApprovalFromDetail"
          >
            Submit for Approval
          </button>
        </div>
      </div>
    </div>

    <div v-if="showFormModal" class="modal-overlay" @click.self="closeFormModal">
      <div class="modal-content modal-lg-custom">
        <div class="modal-header">
          <h2>{{ isEdit ? 'Edit Draft Purchase Invoice' : 'Create Purchase Invoice Draft' }}</h2>
          <button class="modal-close" @click="closeFormModal">&times;</button>
        </div>
        <div class="modal-body">
          <form @submit.prevent="saveDraft">
            <div class="form-grid-3">
              <div class="form-group">
                <label class="form-label">Branch *</label>
                <select v-model="form.branchId" class="form-select" required>
                  <option value="">Select Branch</option>
                  <option v-for="branch in branches" :key="branch.id" :value="branch.id">{{ branch.name }} ({{ branch.code }})</option>
                </select>
              </div>
              <div class="form-group">
                <label class="form-label">Supplier Invoice # *</label>
                <input v-model="form.supplierInvoiceNumber" type="text" class="form-input" required />
              </div>
              <div class="form-group">
                <label class="form-label">Internal Reference *</label>
                <input v-model="form.internalReference" type="text" class="form-input" :readonly="!isEdit" required />
                <small v-if="!isEdit" class="form-help-text">Auto-generated from purchase year and sequence.</small>
              </div>
            </div>

            <div class="form-grid-2" style="margin-top: 16px;">
              <div class="form-group">
                <label class="form-label">Supplier *</label>
                <SearchableDropdown
                  v-model="form.supplierId"
                  :options="suppliers"
                  placeholder="Search and select supplier..."
                  no-results-text="No suppliers found"
                  :required="true"
                  container-class="custom-select-search-container"
                  :get-option-key="(supplier) => supplier.id"
                  :get-option-label="(supplier) => supplier.name"
                  :get-option-search-text="(supplier) => supplier.name"
                />
              </div>
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
              <textarea v-model="form.notes" class="form-textarea" rows="2" placeholder="Internal notes or supplier remarks..."></textarea>
            </div>

            <div style="margin-top: 24px;">
              <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                <h3>Invoice Lines</h3>
                <button type="button" class="btn btn-secondary btn-sm" @click="addLine">+ Add Item Line</button>
              </div>

              <div class="invoice-lines-list">
                <div v-if="form.lines.length === 0" class="empty-state invoice-lines-empty">
                  No lines added. Click "+ Add Item Line" to add purchase items.
                </div>

                <div v-for="(line, index) in form.lines" :key="index" class="invoice-line-card">
                  <div class="invoice-line-header">
                    <div>
                      <p class="invoice-line-title">Line {{ index + 1 }}</p>
                      <p class="invoice-line-subtitle">{{ line.itemId ? getItemLabel(line.itemId) : 'Select an item to start' }}</p>
                    </div>
                    <div class="invoice-line-header-actions">
                      <div class="invoice-line-total">
                        <span class="invoice-line-total-label">Line Total</span>
                        <span class="invoice-line-total-value">{{ formatCurrency(getLineTotal(line)) }}</span>
                      </div>
                      <button type="button" class="invoice-line-toggle" @click="toggleLineExpanded(line)">
                        {{ line.isExpanded ? 'Collapse' : 'Expand' }}
                      </button>
                    </div>
                  </div>

                  <div v-show="line.isExpanded" class="invoice-line-body">
                    <div class="invoice-line-top">
                      <div class="invoice-line-field">
                        <label class="invoice-line-label">Item *</label>
                        <SearchableDropdown
                          v-model="line.itemId"
                          :options="items"
                          placeholder="Search and select item..."
                          no-results-text="No items found"
                          size="sm"
                          container-class="custom-select-search-container"
                          :get-option-key="(item) => item.id"
                          :get-option-label="(item) => `${item.code} - ${item.name}`"
                          :get-option-search-text="(item) => `${item.code} ${item.name} ${item.description || ''}`"
                          @select="selectLineItem(line, $event)"
                        />
                      </div>

                      <div class="invoice-line-field">
                        <label class="invoice-line-label">Description *</label>
                        <input v-model="line.description" type="text" class="form-input form-input-sm" required />
                      </div>

                      <div class="invoice-line-field">
                        <label class="invoice-line-label">Expense Account *</label>
                        <select v-model="line.accountId" class="form-select form-select-sm" required>
                          <option value="">Select Account</option>
                          <option v-for="account in expenseAccounts" :key="account.id" :value="account.id">
                            {{ account.code }} - {{ account.name }}
                          </option>
                        </select>
                      </div>
                    </div>

                    <div class="invoice-line-bottom">
                      <div class="invoice-line-field">
                        <label class="invoice-line-label">Quantity *</label>
                        <input v-model.number="line.quantity" type="text" inputmode="decimal" class="form-input form-input-sm quantity-control-input" required />
                      </div>
                      <div class="invoice-line-field">
                        <label class="invoice-line-label">Unit Price *</label>
                        <input v-model.number="line.unitPrice" type="number" step="any" min="0" class="form-input form-input-sm numeric-input" required />
                      </div>
                      <div class="invoice-line-field">
                        <label class="invoice-line-label">Discount</label>
                        <input v-model.number="line.discountAmount" type="number" step="any" min="0" class="form-input form-input-sm numeric-input" />
                      </div>
                      <div class="invoice-line-field">
                        <label class="invoice-line-label">Tax Rate</label>
                        <select v-model="line.taxTypeId" class="form-select form-select-sm">
                          <option value="">No Tax (0%)</option>
                          <option v-for="tax in taxTypes" :key="tax.id" :value="tax.id">
                            {{ tax.name }} ({{ (Number(tax.defaultRate) * 100).toFixed(0) }}%)
                          </option>
                        </select>
                      </div>
                      <div class="invoice-line-field invoice-line-field-actions">
                        <label class="invoice-line-label">&nbsp;</label>
                        <button type="button" class="btn-delete-row" @click="removeLine(index)">&times;</button>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <div class="totals-summary-card">
              <div class="totals-row"><span>Subtotal:</span><span class="totals-val">{{ formatCurrency(computedSubtotal) }}</span></div>
              <div class="totals-row"><span>Tax Amount:</span><span class="totals-val">{{ formatCurrency(computedTaxAmount) }}</span></div>
              <div class="totals-row grand-total"><span>Total Amount:</span><span>{{ formatCurrency(computedTotalAmount) }}</span></div>
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

    <ConfirmDialog
      :isOpen="showConfirmDelete"
      title="Delete Purchase Draft"
      message="Are you sure you want to delete this draft purchase invoice? This action is permanent and cannot be undone."
      confirmText="Yes, Delete Draft"
      cancelText="Cancel"
      :isDanger="true"
      @confirm="handleDelete"
      @cancel="showConfirmDelete = false"
    />
  </MainLayout>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import MainLayout from '@/components/MainLayout.vue'
import ConfirmDialog from '@/components/ConfirmDialog.vue'
import SearchableDropdown from '@/components/SearchableDropdown.vue'
import { useAuthStore } from '@/stores/auth.store'
import { invoiceApi, type PaginatedResponse } from '@/api/invoices.api'
import { accountApi, branchApi, itemApi, supplierApi, taxTypeApi } from '@/api/master-data.api'
import type { Account, Branch, Item, Supplier, TaxType } from '@/types/master-data.types'
import type { CreatePurchaseInvoiceRequest, PurchaseInvoice } from '@/types/invoice.types'

const authStore = useAuthStore()

const purchases = ref<PurchaseInvoice[]>([])
const loading = ref(false)
const successMsg = ref<string | null>(null)
const errorMsg = ref<string | null>(null)
const activeDropdownId = ref<string | null>(null)
const showDetailModal = ref(false)
const showFormModal = ref(false)
const showConfirmDelete = ref(false)
const selectedInvoice = ref<PurchaseInvoice | null>(null)
const targetDeleteId = ref<string | null>(null)
const isEdit = ref(false)
const targetEditId = ref<string | null>(null)
const submitting = ref(false)
const searchTerm = ref('')
const selectedStatus = ref('')

const suppliers = ref<Supplier[]>([])
const taxTypes = ref<TaxType[]>([])
const branches = ref<Branch[]>([])
const accounts = ref<Account[]>([])
const items = ref<Item[]>([])

const form = ref({
  branchId: '',
  supplierInvoiceNumber: '',
  internalReference: '',
  supplierId: '',
  invoiceDate: new Date().toISOString().substring(0, 10),
  dueDate: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString().substring(0, 10),
  notes: '',
  lines: [] as Array<{
    itemId: string
    isExpanded: boolean
    description: string
    quantity: number
    unitPrice: number
    discountAmount: number
    taxTypeId: string
    accountId: string
  }>,
})

const supplierMap = computed(() => new Map(suppliers.value.map(s => [s.id, s])))
const accountMap = computed(() => new Map(accounts.value.map(a => [a.id, a])))
const itemMap = computed(() => new Map(items.value.map(i => [i.id, i])))
const expenseAccounts = computed(() => accounts.value.filter(a => a.accountType === 'Expense' || a.accountType === 'Asset'))

const filteredPurchases = computed(() => {
  return purchases.value.filter(invoice => {
    if (searchTerm.value) {
      const q = searchTerm.value.toLowerCase()
      const matches = [
        invoice.supplierInvoiceNumber,
        invoice.internalReference,
        getSupplierName(invoice.supplierId),
        invoice.notes || '',
      ].some(value => value.toLowerCase().includes(q))
      if (!matches) return false
    }
    if (selectedStatus.value && invoice.status.toLowerCase() !== selectedStatus.value.toLowerCase()) {
      return false
    }
    return true
  })
})

const computedSubtotal = computed(() => form.value.lines.reduce((sum, line) => sum + (line.quantity * line.unitPrice - line.discountAmount), 0))
const computedTaxAmount = computed(() => form.value.lines.reduce((sum, line) => {
  const net = line.quantity * line.unitPrice - line.discountAmount
  return sum + (net * getTaxRate(line.taxTypeId))
}, 0))
const computedTotalAmount = computed(() => computedSubtotal.value + computedTaxAmount.value)

onMounted(async () => {
  window.addEventListener('click', closeRowActions)
  await loadMasterData()
  await fetchPurchases()
})

onBeforeUnmount(() => {
  window.removeEventListener('click', closeRowActions)
})

watch(
  () => form.value.invoiceDate,
  (newDate) => {
    if (!showFormModal.value || isEdit.value) return
    form.value.internalReference = generateInternalReference(newDate)
  },
)

async function loadMasterData() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return
  const [sRes, tRes, bRes, aRes, iRes] = await Promise.all([
    supplierApi.listByCompany(companyId),
    taxTypeApi.listByCompany(companyId),
    branchApi.listByCompany(companyId),
    accountApi.listByCompany(companyId),
    itemApi.listByCompany(companyId),
  ])
  suppliers.value = sRes
  taxTypes.value = tRes
  branches.value = bRes
  accounts.value = aRes
  items.value = iRes
}

async function fetchPurchases() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return
  loading.value = true
  try {
    const result: PaginatedResponse<PurchaseInvoice> = await invoiceApi.listPurchases({ companyId, page: 1, perPage: 100 })
    purchases.value = result.data
  } catch (err: any) {
    errorMsg.value = err.message || 'Failed to fetch purchase invoices.'
  } finally {
    loading.value = false
  }
}

function getSupplierName(id: string) {
  return supplierMap.value.get(id)?.name || id
}

function getAccountName(id: string) {
  const account = accountMap.value.get(id)
  return account ? `${account.code} - ${account.name}` : id
}

function getItemLabel(id?: string) {
  if (!id) return ''
  const item = itemMap.value.get(id)
  return item ? `${item.code} - ${item.name}` : id
}

function generateInternalReference(dateValue: string) {
  const year = new Date(dateValue || new Date().toISOString()).getFullYear()
  const prefix = `PI/${year}/`
  const nextSequence = purchases.value.reduce((max, invoice) => {
    if (!invoice.internalReference.startsWith(prefix)) return max
    const parsed = Number.parseInt(invoice.internalReference.slice(prefix.length), 10)
    return Number.isFinite(parsed) ? Math.max(max, parsed) : max
  }, 0) + 1
  return `${prefix}${String(nextSequence).padStart(3, '0')}`
}

function openCreateModal() {
  isEdit.value = false
  targetEditId.value = null
  form.value = {
    branchId: branches.value[0]?.id || '',
    supplierInvoiceNumber: '',
    internalReference: '',
    supplierId: '',
    invoiceDate: new Date().toISOString().substring(0, 10),
    dueDate: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString().substring(0, 10),
    notes: '',
    lines: [],
  }
  form.value.internalReference = generateInternalReference(form.value.invoiceDate)
  addLine()
  showFormModal.value = true
}

function openEditModal(invoice: PurchaseInvoice) {
  isEdit.value = true
  targetEditId.value = invoice.id
  form.value = {
    branchId: invoice.branchId || '',
    supplierInvoiceNumber: invoice.supplierInvoiceNumber,
    internalReference: invoice.internalReference,
    supplierId: invoice.supplierId,
    invoiceDate: new Date(invoice.invoiceDate).toISOString().substring(0, 10),
    dueDate: new Date(invoice.dueDate).toISOString().substring(0, 10),
    notes: invoice.notes || '',
    lines: invoice.lines.map(line => ({
      itemId: line.itemId || '',
      isExpanded: true,
      description: line.description,
      quantity: Number(line.quantity),
      unitPrice: Number(line.unitPrice),
      discountAmount: Number(line.discountAmount),
      taxTypeId: line.taxTypeId || '',
      accountId: line.accountId,
    })),
  }
  showFormModal.value = true
}

function closeFormModal() {
  showFormModal.value = false
}

function addLine() {
  const defaultAccount = expenseAccounts.value[0]
  form.value.lines.push({
    itemId: '',
    isExpanded: true,
    description: '',
    quantity: 1,
    unitPrice: 0,
    discountAmount: 0,
    taxTypeId: taxTypes.value[0]?.id || '',
    accountId: defaultAccount?.id || '',
  })
}

function selectLineItem(line: { itemId: string; description: string; unitPrice: number; taxTypeId: string; accountId: string }, item: Item) {
  line.itemId = item.id
  line.description = item.name
  line.unitPrice = Number(item.unitPrice)
  if (item.taxTypeId) line.taxTypeId = item.taxTypeId
  if (item.purchaseAccountId) line.accountId = item.purchaseAccountId
}

function toggleLineExpanded(line: { isExpanded?: boolean }) {
  line.isExpanded = !line.isExpanded
}

function removeLine(index: number) {
  form.value.lines.splice(index, 1)
}

function getTaxRate(taxTypeId: string) {
  if (!taxTypeId) return 0
  const tax = taxTypes.value.find(t => t.id === taxTypeId)
  return tax ? Number(tax.defaultRate) : 0
}

function getLineTotal(line: { quantity: number; unitPrice: number; discountAmount: number; taxTypeId: string }) {
  const net = line.quantity * line.unitPrice - line.discountAmount
  return net + (net * getTaxRate(line.taxTypeId))
}

function formatCurrency(value: number) {
  return new Intl.NumberFormat('id-ID', { style: 'currency', currency: 'IDR', minimumFractionDigits: 0 }).format(value)
}

function formatQuantity(value: number | string | null | undefined) {
  if (value === null || value === undefined || value === '') return '-'
  const numeric = typeof value === 'string' ? Number(value) : value
  if (!Number.isFinite(numeric)) return '-'
  const rounded = Number(numeric.toFixed(4))
  return Number.isInteger(rounded) ? `${rounded}` : `${rounded}`
}

function formatDate(dateStr: string) {
  if (!dateStr) return '-'
  return new Date(dateStr).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' })
}

function getStatusBadgeClass(status: string) {
  const s = status.toLowerCase()
  if (s === 'draft') return 'badge-secondary'
  if (s === 'waiting_approval') return 'badge-warning'
  if (s === 'posted') return 'badge-success'
  if (s === 'rejected') return 'badge-danger'
  return 'badge-secondary'
}

function formatStatus(status: string) {
  return status.replace('_', ' ').toUpperCase()
}

function toggleRowDropdown(id: string, event?: Event) {
  event?.stopPropagation()
  activeDropdownId.value = activeDropdownId.value === id ? null : id
}

function closeRowActions() {
  activeDropdownId.value = null
}

function viewDetails(invoice: PurchaseInvoice) {
  selectedInvoice.value = invoice
  showDetailModal.value = true
}

function closeDetailModal() {
  showDetailModal.value = false
  selectedInvoice.value = null
}

function editFromDetail() {
  if (!selectedInvoice.value) return
  openEditModal(selectedInvoice.value)
  closeDetailModal()
}

function deleteFromDetail() {
  if (!selectedInvoice.value) return
  confirmDelete(selectedInvoice.value.id)
  closeDetailModal()
}

async function submitApprovalFromDetail() {
  if (!selectedInvoice.value) return
  await submitApproval(selectedInvoice.value.id)
  closeDetailModal()
}

async function submitApproval(id: string) {
  errorMsg.value = null
  successMsg.value = null
  try {
    await invoiceApi.submitPurchaseApproval(id)
    successMsg.value = 'Purchase invoice submitted for approval successfully.'
    await fetchPurchases()
  } catch (err: any) {
    errorMsg.value = err.response?.data?.message || err.message || 'Failed to submit purchase invoice for approval.'
  }
}

function confirmDelete(id: string) {
  targetDeleteId.value = id
  showConfirmDelete.value = true
}

async function handleDelete() {
  if (!targetDeleteId.value) return
  try {
    await invoiceApi.deletePurchaseDraft(targetDeleteId.value)
    successMsg.value = 'Purchase invoice draft deleted.'
    await fetchPurchases()
  } catch (err: any) {
    errorMsg.value = err.message || 'Failed to delete purchase invoice.'
  } finally {
    showConfirmDelete.value = false
    targetDeleteId.value = null
  }
}

function validateDraftForm() {
  if (!form.value.branchId) return 'Branch is required.'
  if (!form.value.supplierInvoiceNumber.trim()) return 'Supplier invoice number is required.'
  if (!form.value.internalReference.trim()) return 'Internal reference is required.'
  if (!form.value.supplierId) return 'Supplier is required.'
  if (form.value.lines.length === 0) return 'Add at least one purchase invoice line.'
  for (const [index, line] of form.value.lines.entries()) {
    if (!line.description.trim()) return `Line ${index + 1}: description is required.`
    if (!line.accountId) return `Line ${index + 1}: expense account is required.`
    if (!Number.isFinite(line.quantity) || line.quantity <= 0) return `Line ${index + 1}: quantity must be greater than 0.`
    if (!Number.isFinite(line.unitPrice) || line.unitPrice < 0) return `Line ${index + 1}: unit price must be 0 or greater.`
    if (!Number.isFinite(line.discountAmount) || line.discountAmount < 0) return `Line ${index + 1}: discount must be 0 or greater.`
  }
  return null
}

async function saveDraft() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return

  const validationError = validateDraftForm()
  if (validationError) {
    errorMsg.value = validationError
    return
  }

  submitting.value = true
  errorMsg.value = null
  successMsg.value = null

  const payload: CreatePurchaseInvoiceRequest = {
    companyId,
    branchId: form.value.branchId || undefined,
    supplierInvoiceNumber: form.value.supplierInvoiceNumber,
    internalReference: form.value.internalReference,
    supplierId: form.value.supplierId,
    invoiceDate: form.value.invoiceDate,
    dueDate: form.value.dueDate,
    notes: form.value.notes || undefined,
    lines: form.value.lines.map((line, idx) => ({
      itemId: line.itemId || undefined,
      description: line.description,
      quantity: Number(line.quantity),
      unitPrice: Number(line.unitPrice),
      discountAmount: Number(line.discountAmount) || 0,
      taxTypeId: line.taxTypeId || undefined,
      accountId: line.accountId,
      sortOrder: idx,
    })),
  }

  try {
    if (isEdit.value && targetEditId.value) {
      await invoiceApi.updatePurchaseDraft(targetEditId.value, payload)
      successMsg.value = 'Purchase invoice draft updated successfully.'
    } else {
      await invoiceApi.createPurchaseDraft(payload)
      successMsg.value = 'Purchase invoice draft created.'
    }
    showFormModal.value = false
    await fetchPurchases()
  } catch (err: any) {
    errorMsg.value = err.response?.data?.message || err.message || 'Failed to save purchase invoice draft.'
  } finally {
    submitting.value = false
  }
}

function resetFilters() {
  searchTerm.value = ''
  selectedStatus.value = ''
}
</script>

<style scoped>
.modal-lg-custom {
  max-width: 1180px;
  width: min(1180px, calc(100vw - 32px));
}

.table-container {
  overflow: visible !important;
}

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
  width: 180px;
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

.notes-box {
  background-color: var(--bg-tertiary);
  border-left: 4px solid var(--accent-primary);
  padding: 12px 16px;
  border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
}

.notes-label {
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
  color: var(--text-secondary);
  margin-bottom: 4px;
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

.invoice-lines-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.invoice-lines-empty {
  margin-top: 4px;
}

.invoice-line-card {
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  background: var(--bg-secondary);
  box-shadow: var(--shadow-sm);
  padding: 16px;
}

.invoice-line-header {
  display: flex;
  justify-content: space-between;
  gap: 16px;
  align-items: flex-start;
  margin-bottom: 14px;
}

.invoice-line-header-actions {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 0 0 auto;
}

.invoice-line-title {
  margin: 0;
  font-size: 0.82rem;
  font-weight: 800;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--accent-primary);
}

.invoice-line-subtitle {
  margin: 6px 0 0 0;
  font-size: 0.92rem;
  color: var(--text-secondary);
  line-height: 1.4;
}

.invoice-line-total {
  min-width: 170px;
  padding: 10px 12px;
  border-radius: var(--radius-md);
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  text-align: right;
}

.invoice-line-total-label {
  display: block;
  font-size: 0.72rem;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--text-secondary);
  margin-bottom: 4px;
}

.invoice-line-total-value {
  font-size: 1rem;
  font-weight: 800;
  color: var(--text-primary);
}

.invoice-line-toggle {
  border: 1px solid var(--border-color);
  background: white;
  color: var(--text-primary);
  border-radius: var(--radius-md);
  padding: 8px 12px;
  font-size: 0.8rem;
  font-weight: 700;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.invoice-line-toggle:hover {
  border-color: var(--accent-primary);
  color: var(--accent-primary);
}

.invoice-line-body {
  border-top: 1px solid var(--border-color);
  padding-top: 14px;
}

.invoice-line-top,
.invoice-line-bottom {
  display: grid;
  gap: 14px;
}

.invoice-line-top {
  grid-template-columns: minmax(220px, 1.2fr) minmax(260px, 1.8fr) minmax(220px, 1fr);
}

.invoice-line-bottom {
  grid-template-columns: minmax(110px, 0.7fr) minmax(140px, 0.9fr) minmax(120px, 0.7fr) minmax(160px, 1fr) 56px;
  margin-top: 12px;
  align-items: end;
}

.invoice-line-field {
  min-width: 0;
}

.invoice-line-label {
  display: block;
  margin-bottom: 6px;
  font-size: 0.75rem;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.04em;
  color: var(--text-secondary);
}

.invoice-line-field-actions {
  display: flex;
  justify-content: center;
}

.quantity-control-input {
  width: 100%;
  text-align: center;
  padding-left: 12px;
  padding-right: 12px;
  box-sizing: border-box;
}

.numeric-input {
  width: 100%;
  text-align: right;
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

.form-help-text {
  display: block;
  margin-top: 6px;
  font-size: 0.78rem;
  color: var(--text-secondary);
}

@media (max-width: 1100px) {
  .invoice-line-header {
    flex-direction: column;
  }

  .invoice-line-header-actions {
    width: 100%;
    justify-content: space-between;
  }

  .invoice-line-total {
    text-align: left;
    min-width: 0;
    width: 100%;
  }

  .invoice-line-top {
    grid-template-columns: 1fr;
  }

  .invoice-line-bottom {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  .invoice-line-field-actions {
    justify-content: flex-end;
  }
}

@media (max-width: 700px) {
  .invoice-line-bottom {
    grid-template-columns: 1fr;
  }
}
</style>
