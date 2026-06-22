<template>
  <MainLayout>
    <div class="page-header">
      <div>
        <h1>Payments</h1>
        <p class="page-title-desc">Record, allocate, and process customer receipts and supplier payments.</p>
      </div>
      <button class="btn btn-primary" @click="openCreateModal">+ Record Payment</button>
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
            placeholder="Search by reference, notes..."
            @input="applyFilters"
          />
        </div>
        <div style="width: 200px;">
          <select v-model="selectedType" class="form-select" @change="applyFilters">
            <option value="">All Payment Types</option>
            <option value="payment_received">Payment Received (Customer)</option>
            <option value="payment_paid">Payment Paid (Supplier)</option>
          </select>
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

    <!-- Payments Table -->
    <div class="table-container" style="overflow: visible !important;">
      <div v-if="loading && payments.length === 0" class="loading-state">
        Loading payments...
      </div>
      <div v-else-if="filteredPayments.length === 0" class="empty-state">
        <span v-if="searchTerm || selectedType || selectedStatus">No payments match your filters.</span>
        <span v-else>No payments found. Click "+ Record Payment" to start.</span>
      </div>
      <table v-else>
        <thead>
          <tr>
            <th>Payment Date</th>
            <th>Reference Number</th>
            <th>Type</th>
            <th>Counterparty</th>
            <th>Bank Account</th>
            <th style="text-align: right;">Amount</th>
            <th>Status</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="payment in paginatedPayments" :key="payment.id">
            <td>{{ formatDate(payment.paymentDate) }}</td>
            <td style="font-weight: 600; color: var(--accent-primary);">
              {{ payment.referenceNumber || 'DRAFT' }}
            </td>
            <td>
              <span :class="['badge', payment.paymentType === 'payment_received' ? 'badge-success' : 'badge-info']">
                {{ payment.paymentType === 'payment_received' ? 'Received' : 'Paid' }}
              </span>
            </td>
            <td>{{ getCounterpartyName(payment) }}</td>
            <td>{{ getBankAccountName(payment.bankAccountId) }}</td>
            <td style="text-align: right; font-weight: 600;">{{ formatCurrency(payment.amount) }}</td>
            <td>
              <span :class="['badge', getStatusBadgeClass(payment.status)]">
                {{ formatStatus(payment.status) }}
              </span>
            </td>
            <td style="position: relative; width: 80px; text-align: center;">
              <div class="row-actions-container">
                <button class="btn-actions-trigger" @click="toggleRowDropdown(payment.id, $event)">
                  <svg class="actions-trigger-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <circle cx="12" cy="12" r="1.5" />
                    <circle cx="19" cy="12" r="1.5" />
                    <circle cx="5" cy="12" r="1.5" />
                  </svg>
                </button>

                <transition name="dropdown-fade">
                  <div v-if="activeDropdownId === payment.id" class="row-actions-dropdown">
                    <button class="dropdown-item" @click="viewDetails(payment)">
                      👁️ View Details
                    </button>
                    <button
                      v-if="payment.status === 'draft' || payment.status === 'rejected'"
                      class="dropdown-item"
                      @click="openEditModal(payment)"
                    >
                      ✏️ Edit Draft
                    </button>
                    <button
                      v-if="payment.status === 'draft' || payment.status === 'rejected'"
                      class="dropdown-item"
                      @click="submitApproval(payment.id)"
                    >
                      📤 Submit Approval
                    </button>
                    <div v-if="payment.status === 'draft' || payment.status === 'rejected'" class="dropdown-divider"></div>
                    <button
                      v-if="payment.status === 'draft' || payment.status === 'rejected'"
                      class="dropdown-item text-danger"
                      @click="confirmDelete(payment.id)"
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
    <div class="pagination-footer" v-if="filteredPayments.length > 0">
      <div class="pagination-info">
        Showing {{ paginationStart }} to {{ paginationEnd }} of {{ filteredPayments.length }} entries
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
          <h2>{{ isEdit ? 'Edit Draft Payment' : 'Record Payment Draft' }}</h2>
          <button class="modal-close" @click="closeFormModal">&times;</button>
        </div>
        <div class="modal-body">
          <form @submit.prevent="saveDraft">
            <div class="form-grid-3">
              <div class="form-group">
                <label class="form-label">Payment Type *</label>
                <select v-model="form.paymentType" class="form-select" :disabled="isEdit" required>
                  <option value="payment_received">Payment Received (Customer)</option>
                  <option value="payment_paid">Payment Paid (Supplier)</option>
                </select>
              </div>
              
              <div class="form-group" v-if="form.paymentType === 'payment_received'">
                <label class="form-label">Customer *</label>
                <SearchableDropdown
                  key="customer-dropdown"
                  v-model="form.counterpartyId"
                  :options="customers"
                  placeholder="Search and select customer..."
                  no-results-text="No customers found"
                  :required="true"
                  container-class="custom-select-search-container"
                  :get-option-key="(c) => c.id"
                  :get-option-label="(c) => c.name"
                  :get-option-search-text="(c) => c.name"
                  @select="handleCounterpartyChange"
                />
              </div>
              <div class="form-group" v-else>
                <label class="form-label">Supplier *</label>
                <SearchableDropdown
                  key="supplier-dropdown"
                  v-model="form.counterpartyId"
                  :options="suppliers"
                  placeholder="Search and select supplier..."
                  no-results-text="No suppliers found"
                  :required="true"
                  container-class="custom-select-search-container"
                  :get-option-key="(s) => s.id"
                  :get-option-label="(s) => s.name"
                  :get-option-search-text="(s) => s.name"
                  @select="handleCounterpartyChange"
                />
              </div>

              <div class="form-group">
                <label class="form-label">Bank Account *</label>
                <select v-model="form.bankAccountId" class="form-select" required>
                  <option value="">Select Bank Account</option>
                  <option v-for="ba in bankAccounts" :key="ba.id" :value="ba.id">
                    {{ ba.bankName }} - {{ ba.accountName }} ({{ ba.currency }})
                  </option>
                </select>
              </div>
            </div>

            <div class="form-grid-2" style="margin-top: 16px;">
              <div class="form-group">
                <label class="form-label">Payment Date *</label>
                <input v-model="form.paymentDate" type="date" class="form-input" required />
              </div>
              <div class="form-group">
                <label class="form-label">Payment Amount *</label>
                <input
                  v-model.number="form.amount"
                  type="number"
                  step="0.01"
                  min="0.01"
                  class="form-input"
                  placeholder="0.00"
                  required
                />
              </div>
            </div>

            <div class="form-group" style="margin-top: 16px;">
              <label class="form-label">Notes</label>
              <textarea v-model="form.notes" class="form-textarea" rows="2" placeholder="Record reference notes, check numbers, etc..."></textarea>
            </div>

            <div class="form-group" style="margin-top: 16px;">
              <label class="form-label">Receipt Attachment (Image or PDF)</label>
              <div class="attachment-upload-container">
                <input
                  type="file"
                  ref="fileInput"
                  style="display: none;"
                  accept="image/*,application/pdf"
                  @change="handleFileUpload"
                />
                <div
                  class="attachment-dropzone"
                  :class="{ 'has-file': form.attachmentUrl, 'has-error': uploadError, 'has-success': uploadSuccess }"
                  @click="triggerFileInput"
                >
                  <div v-if="uploadingAttachment" class="uploading-spinner">
                    <span>🔄</span> Uploading attachment...
                  </div>
                  <div v-else-if="form.attachmentUrl" class="file-uploaded-view">
                    <span>✅</span>
                    <span class="file-name">{{ getAttachmentFilename(form.attachmentUrl) }}</span>
                    <button type="button" class="btn-remove-file" @click.stop="removeAttachment">&times;</button>
                  </div>
                  <div v-else-if="uploadError" class="file-error-view">
                    <span>❌ Upload Failed:</span>
                    <span class="file-name" :title="uploadError">{{ uploadError }}</span>
                    <button type="button" class="btn-remove-file" @click.stop="clearUploadError">&times;</button>
                  </div>
                  <div v-else class="upload-prompt">
                    <span>📁</span> Click to choose image/PDF here
                  </div>
                </div>
                <div v-if="uploadSuccess" class="attachment-success-msg" style="color: #10b981; font-size: 0.85rem; display: flex; align-items: center; gap: 4px; margin-top: 4px;">
                  <span>✨</span> Attachment uploaded successfully!
                </div>
              </div>
            </div>

            <!-- Invoices Allocations -->
            <div style="margin-top: 24px;">
              <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;">
                <h3>Invoice Allocations</h3>
                <span class="badge" :class="remainingToAllocate === 0 ? 'badge-success' : 'badge-secondary'">
                  Total Allocated: {{ formatCurrency(totalAllocatedAmount) }} / Left: {{ formatCurrency(remainingToAllocate) }}
                </span>
              </div>

              <div class="table-container" style="max-height: 250px; overflow-y: auto;">
                <div v-if="!form.counterpartyId" class="empty-state">
                  Select a counterparty to see outstanding invoices.
                </div>
                <div v-else-if="localAllocations.length === 0" class="empty-state">
                  No outstanding posted invoices found for this counterparty.
                </div>
                <table v-else>
                  <thead>
                    <tr>
                      <th>Invoice Date</th>
                      <th>Invoice Number</th>
                      <th style="text-align: right;">Total Amount</th>
                      <th style="text-align: right;">Remaining Balance</th>
                      <th style="text-align: right; width: 180px;">Allocated Amount</th>
                      <th style="text-align: center; width: 120px;">Actions</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr v-for="(item, index) in localAllocations" :key="item.invoiceId">
                      <td>{{ formatDateOnly(item.invoiceDate) }}</td>
                      <td style="font-weight: 500;">{{ item.invoiceNumber }}</td>
                      <td style="text-align: right;">{{ formatCurrency(item.totalAmount) }}</td>
                      <td style="text-align: right; font-weight: 600; color: var(--accent-primary);">
                        {{ formatCurrency(item.remainingBalance) }}
                      </td>
                      <td style="text-align: right;">
                        <input
                          v-model.number="item.allocatedAmount"
                          type="number"
                          step="0.01"
                          min="0"
                          :max="item.remainingBalance + (isEdit ? getExistingAllocated(item.invoiceId) : 0)"
                          class="form-input"
                          style="width: 100%; text-align: right; padding: 6px 10px; font-size: 0.9rem;"
                          placeholder="0.00"
                        />
                      </td>
                      <td style="text-align: center;">
                        <div style="display: flex; gap: 4px; justify-content: center;">
                          <button
                            type="button"
                            class="btn btn-secondary btn-xs"
                            :disabled="remainingToAllocate <= 0 && item.allocatedAmount === 0"
                            @click="allocateFull(index)"
                          >
                            Full
                          </button>
                          <button
                            type="button"
                            class="btn btn-secondary btn-xs text-danger"
                            :disabled="item.allocatedAmount === 0"
                            @click="clearAllocation(index)"
                          >
                            Clear
                          </button>
                        </div>
                      </td>
                    </tr>
                  </tbody>
                </table>
              </div>
              <div v-if="totalAllocatedAmount > form.amount" class="alert alert-danger" style="margin-top: 10px; padding: 8px 12px; font-size: 0.8rem;">
                ⚠️ Warning: Total allocated amount ({{ formatCurrency(totalAllocatedAmount) }}) exceeds payment amount ({{ formatCurrency(form.amount) }}).
              </div>
            </div>

            <div class="modal-footer" style="margin-top: 24px; padding-bottom: 0;">
              <button type="button" class="btn btn-secondary" @click="closeFormModal">Cancel</button>
              <button type="submit" class="btn btn-primary" :disabled="loading || totalAllocatedAmount > form.amount">
                Save Draft
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>

    <!-- Detail Modal -->
    <div v-if="showDetailModal && selectedPayment" class="modal-overlay" @click.self="closeDetailModal">
      <div class="modal-content modal-lg-custom">
        <div class="modal-header">
          <div style="display: flex; align-items: center; gap: 12px;">
            <h2>Payment Details</h2>
            <span :class="['badge', getStatusBadgeClass(selectedPayment.status)]">
              {{ formatStatus(selectedPayment.status) }}
            </span>
          </div>
          <button class="modal-close" @click="closeDetailModal">&times;</button>
        </div>
        <div class="modal-body">
          <div class="meta-card" style="margin-bottom: 24px;">
            <div>
              <div class="meta-label">Reference Number</div>
              <div class="meta-val-highlight">{{ selectedPayment.referenceNumber || 'DRAFT' }}</div>
            </div>
            <div>
              <div class="meta-label">Payment Date</div>
              <div class="meta-val">{{ formatDate(selectedPayment.paymentDate) }}</div>
            </div>
            <div>
              <div class="meta-label">Type</div>
              <div class="meta-val">
                <span :class="['badge', selectedPayment.paymentType === 'payment_received' ? 'badge-success' : 'badge-info']">
                  {{ selectedPayment.paymentType === 'payment_received' ? 'Payment Received (Customer)' : 'Payment Paid (Supplier)' }}
                </span>
              </div>
            </div>
            <div>
              <div class="meta-label">Counterparty</div>
              <div class="meta-val">{{ getCounterpartyName(selectedPayment) }}</div>
            </div>
            <div>
              <div class="meta-label">Bank Account</div>
              <div class="meta-val">{{ getBankAccountName(selectedPayment.bankAccountId) }}</div>
            </div>
            <div>
              <div class="meta-label">Amount</div>
              <div class="meta-val-highlight" style="color: #10b981;">
                {{ formatCurrency(selectedPayment.amount) }}
              </div>
            </div>
          </div>

          <div v-if="selectedPayment.notes || selectedPayment.attachmentUrl" class="notes-attachment-section" style="margin-bottom: 24px; display: flex; flex-wrap: wrap; gap: 20px;">
            <div v-if="selectedPayment.notes" class="notes-box" style="flex: 1; min-width: 300px; margin-top: 0;">
              <p class="notes-label" style="font-size: 0.75rem; text-transform: uppercase; font-weight: 600; color: var(--text-secondary); margin-bottom: 6px;">Notes</p>
              <p style="margin: 0; font-size: 0.9rem; white-space: pre-wrap;">{{ selectedPayment.notes }}</p>
            </div>
            <div v-if="selectedPayment.attachmentUrl" class="notes-box" style="flex: 1; min-width: 300px; margin-top: 0;">
              <p class="notes-label" style="font-size: 0.75rem; text-transform: uppercase; font-weight: 600; color: var(--text-secondary); margin-bottom: 6px;">Attachment</p>
              <div class="attachment-preview-box">
                <a :href="selectedPayment.attachmentUrl" target="_blank" class="attachment-link" style="color: var(--accent-primary); display: inline-flex; align-items: center; gap: 6px; text-decoration: none; font-weight: 500;">
                  <template v-if="isPdf(selectedPayment.attachmentUrl)">
                    <span>📄</span> View PDF Document
                  </template>
                  <template v-else>
                    <span>🖼️</span> View Attached Image
                  </template>
                </a>
                <div v-if="!isPdf(selectedPayment.attachmentUrl)" class="attachment-image-thumb-container" style="margin-top: 8px;">
                  <img :src="selectedPayment.attachmentUrl" alt="Attachment" class="attachment-image-thumb" style="max-width: 100%; max-height: 120px; border-radius: var(--radius-sm); border: 1px solid var(--border-color); object-fit: contain; cursor: pointer;" />
                </div>
              </div>
            </div>
          </div>

          <!-- Allocations -->
          <div style="margin-top: 24px;">
            <h3>Allocations Summary</h3>
            <div class="table-container">
              <div v-if="selectedPayment.allocations.length === 0" class="empty-state">
                No allocations recorded for this payment.
              </div>
              <table v-else>
                <thead>
                  <tr>
                    <th>Invoice Type</th>
                    <th>Invoice Ref/ID</th>
                    <th style="text-align: right;">Allocated Amount</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="alloc in selectedPayment.allocations" :key="alloc.id">
                    <td>
                      <span class="badge badge-secondary">
                        {{ alloc.documentType === 'sales_invoice' ? 'Sales Invoice' : 'Purchase Invoice' }}
                      </span>
                    </td>
                    <td style="font-weight: 500;">
                      {{ getInvoiceNumberById(alloc.documentId, alloc.documentType) }}
                    </td>
                    <td style="text-align: right; font-weight: 600; color: var(--accent-primary);">
                      {{ formatCurrency(alloc.allocatedAmount) }}
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>

          <!-- Journal Entry Reference -->
          <div v-if="selectedPayment.journalEntryId" style="margin-top: 24px;">
            <h3>Linked Journal Entry</h3>
            <p style="font-size: 0.9rem; color: var(--text-secondary);">
              This payment has been posted to ledger. Linked Journal ID:
              <router-link :to="`/journals`" class="btn btn-secondary btn-xs" style="margin-left: 8px;">
                🔗 View in Journals
              </router-link>
            </p>
          </div>
        </div>
        <div class="modal-footer">
          <button type="button" class="btn btn-secondary" @click="closeDetailModal">Close</button>
          <button
            v-if="selectedPayment.status === 'draft' || selectedPayment.status === 'rejected'"
            type="button"
            class="btn btn-primary"
            @click="openEditModal(selectedPayment); closeDetailModal()"
          >
            ✏️ Edit Draft
          </button>
          <button
            v-if="selectedPayment.status === 'draft' || selectedPayment.status === 'rejected'"
            type="button"
            class="btn btn-primary"
            style="background: #10b981; border-color: #10b981;"
            @click="submitApproval(selectedPayment.id); closeDetailModal()"
          >
            📤 Submit Approval
          </button>
        </div>
      </div>
    </div>

    <!-- Confirm Delete Modal -->
    <ConfirmDialog
      :is-open="showConfirmDelete"
      title="Delete Draft Payment"
      message="Are you sure you want to delete this payment draft? This action cannot be undone."
      confirm-text="Delete Draft"
      cancel-text="Cancel"
      :is-danger="true"
      @confirm="deleteDraft"
      @cancel="showConfirmDelete = false"
    />
  </MainLayout>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, computed, watch } from 'vue'
import MainLayout from '@/components/MainLayout.vue'
import ConfirmDialog from '@/components/ConfirmDialog.vue'
import SearchableDropdown from '@/components/SearchableDropdown.vue'
import { paymentApi } from '@/api/payments.api'
import { invoiceApi } from '@/api/invoices.api'
import { customerApi, supplierApi, bankAccountApi } from '@/api/master-data.api'
import { useAuthStore } from '@/stores/auth.store'
import type { Payment, CreatePaymentRequest } from '@/types/payment.types'
import type { Customer, Supplier, BankAccount } from '@/types/master-data.types'
import type { SalesInvoice, PurchaseInvoice } from '@/types/invoice.types'

const authStore = useAuthStore()

// State lists
const payments = ref<Payment[]>([])
const customers = ref<Customer[]>([])
const suppliers = ref<Supplier[]>([])
const bankAccounts = ref<BankAccount[]>([])
const salesInvoices = ref<SalesInvoice[]>([])
const purchaseInvoices = ref<PurchaseInvoice[]>([])

// UI loading/messaging states
const loading = ref(false)
const successMsg = ref<string | null>(null)
const errorMsg = ref<string | null>(null)

// Selection & Modal states
const activeDropdownId = ref<string | null>(null)
const showDetailModal = ref(false)
const selectedPayment = ref<Payment | null>(null)
const showFormModal = ref(false)
const isEdit = ref(false)

// Confirm dialog state
const showConfirmDelete = ref(false)
const paymentToDeleteId = ref<string | null>(null)

// Filtering & Search
const searchTerm = ref('')
const selectedType = ref('')
const selectedStatus = ref('')
const currentPage = ref(1)
const perPage = ref(10)

// Form structure
const form = ref<{
  id?: string
  companyId: string
  paymentType: 'payment_received' | 'payment_paid'
  counterpartyType: 'customer' | 'supplier'
  counterpartyId: string
  paymentDate: string
  bankAccountId: string
  amount: number
  notes: string
  attachmentUrl?: string
}>({
  companyId: '',
  paymentType: 'payment_received',
  counterpartyType: 'customer',
  counterpartyId: '',
  paymentDate: new Date().toISOString().split('T')[0],
  bankAccountId: '',
  amount: 0,
  notes: '',
  attachmentUrl: '',
})

// UI-local allocation input mapping
const localAllocations = ref<Array<{
  invoiceId: string
  invoiceNumber: string
  invoiceDate: string
  totalAmount: number
  remainingBalance: number
  allocatedAmount: number
}>>([])

// Master lookup maps
const customersMap = computed(() => new Map(customers.value.map(c => [c.id, c])))
const suppliersMap = computed(() => new Map(suppliers.value.map(s => [s.id, s])))
const bankAccountsMap = computed(() => new Map(bankAccounts.value.map(b => [b.id, b])))

onMounted(async () => {
  document.addEventListener('click', handleOutsideClick)
  const companyId = authStore.currentUser?.companyId
  if (companyId) {
    loading.value = true
    try {
      await Promise.all([
        fetchPayments(),
        fetchCustomers(),
        fetchSuppliers(),
        fetchBankAccounts(),
        fetchInvoices()
      ])
    } catch (err: any) {
      errorMsg.value = err.message || 'Failed to load master lists.'
    } finally {
      loading.value = false
    }
  }
})

onBeforeUnmount(() => {
  document.removeEventListener('click', handleOutsideClick)
})

watch(() => form.value.paymentType, (newType) => {
  form.value.counterpartyType = newType === 'payment_received' ? 'customer' : 'supplier'
  form.value.counterpartyId = ''
  localAllocations.value = []
})

// Fetch methods
async function fetchPayments() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return
  const res = await paymentApi.list({ companyId, page: 1, perPage: 1000 })
  payments.value = res.data
}

async function fetchCustomers() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return
  customers.value = await customerApi.listByCompany(companyId)
}

async function fetchSuppliers() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return
  suppliers.value = await supplierApi.listByCompany(companyId)
}

async function fetchBankAccounts() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return
  bankAccounts.value = await bankAccountApi.listByCompany(companyId)
}

async function fetchInvoices() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return
  const [salesRes, purchaseRes] = await Promise.all([
    invoiceApi.listSales({ companyId, page: 1, perPage: 1000 }),
    invoiceApi.listPurchases({ companyId, page: 1, perPage: 1000 })
  ])
  salesInvoices.value = salesRes.data
  purchaseInvoices.value = purchaseRes.data
}

// Outstanding allocation helpers
function getInvoiceAllocatedTotal(invoiceId: string, currentPaymentId?: string) {
  let total = 0
  for (const payment of payments.value) {
    if (currentPaymentId && payment.id === currentPaymentId) continue
    if (payment.status === 'draft' || payment.status === 'rejected') continue
    
    for (const alloc of payment.allocations) {
      if (alloc.documentId === invoiceId) {
        total += Number(alloc.allocatedAmount)
      }
    }
  }
  return total
}

function getExistingAllocated(invoiceId: string): number {
  if (!isEdit.value || !selectedPayment.value) return 0
  const existing = selectedPayment.value.allocations.find(a => a.documentId === invoiceId)
  return existing ? Number(existing.allocatedAmount) : 0
}

function handleCounterpartyChange() {
  if (!form.value.counterpartyId) {
    localAllocations.value = []
    return
  }

  let relevantInvoices: Array<SalesInvoice | PurchaseInvoice> = []
  if (form.value.paymentType === 'payment_received') {
    relevantInvoices = salesInvoices.value.filter(
      inv => inv.customerId === form.value.counterpartyId && inv.status === 'posted'
    )
  } else {
    relevantInvoices = purchaseInvoices.value.filter(
      inv => inv.supplierId === form.value.counterpartyId && inv.status === 'posted'
    )
  }

  localAllocations.value = relevantInvoices.map(inv => {
    const currentPaymentId = isEdit.value ? form.value.id : undefined
    const allocatedOther = getInvoiceAllocatedTotal(inv.id, currentPaymentId)
    const remainingBalance = Number(inv.totalAmount) - allocatedOther

    let existingAllocated = 0
    if (isEdit.value && selectedPayment.value) {
      const existing = selectedPayment.value.allocations.find(a => a.documentId === inv.id)
      if (existing) {
        existingAllocated = Number(existing.allocatedAmount)
      }
    }

    return {
      invoiceId: inv.id,
      invoiceNumber: 'invoiceNumber' in inv ? inv.invoiceNumber : inv.supplierInvoiceNumber,
      invoiceDate: inv.invoiceDate,
      totalAmount: Number(inv.totalAmount),
      remainingBalance,
      allocatedAmount: existingAllocated
    }
  }).filter(item => item.remainingBalance > 0.01 || item.allocatedAmount > 0)
}

// Allocation calculations
const totalAllocatedAmount = computed(() => {
  return localAllocations.value.reduce((sum, item) => sum + (Number(item.allocatedAmount) || 0), 0)
})

const remainingToAllocate = computed(() => {
  return Math.max(0, Number(form.value.amount) - totalAllocatedAmount.value)
})

function allocateFull(index: number) {
  const item = localAllocations.value[index]
  const currentMax = item.remainingBalance + (isEdit.value ? getExistingAllocated(item.invoiceId) : 0)
  const maxCanAllocate = Math.min(currentMax, remainingToAllocate.value + item.allocatedAmount)
  item.allocatedAmount = Number(maxCanAllocate.toFixed(2))
}

function clearAllocation(index: number) {
  localAllocations.value[index].allocatedAmount = 0
}

// Dropdown click handlers
function toggleRowDropdown(id: string, event: Event) {
  event.stopPropagation()
  if (activeDropdownId.value === id) {
    activeDropdownId.value = null
  } else {
    activeDropdownId.value = id
  }
}

function handleOutsideClick() {
  activeDropdownId.value = null
}

// Dialog flows
function openCreateModal() {
  isEdit.value = false
  resetForm()
  uploadError.value = null
  uploadSuccess.value = false
  showFormModal.value = true
}

function openEditModal(payment: Payment) {
  isEdit.value = true
  selectedPayment.value = payment
  
  form.value = {
    id: payment.id,
    companyId: payment.companyId,
    paymentType: payment.paymentType,
    counterpartyType: payment.counterpartyType,
    counterpartyId: payment.counterpartyId,
    paymentDate: payment.paymentDate,
    bankAccountId: payment.bankAccountId,
    amount: Number(payment.amount),
    notes: payment.notes || '',
    attachmentUrl: payment.attachmentUrl || ''
  }
  
  handleCounterpartyChange()
  showFormModal.value = true
}

function closeFormModal() {
  showFormModal.value = false
  uploadError.value = null
  uploadSuccess.value = false
  resetForm()
}

function viewDetails(payment: Payment) {
  selectedPayment.value = payment
  showDetailModal.value = true
}

function closeDetailModal() {
  showDetailModal.value = false
  selectedPayment.value = null
}

// CRUD actions
async function saveDraft() {
  loading.value = true
  errorMsg.value = null
  successMsg.value = null

  if (totalAllocatedAmount.value > form.value.amount) {
    errorMsg.value = 'Total allocated amount cannot exceed payment amount.'
    loading.value = false
    return
  }

  const companyId = authStore.currentUser?.companyId
  if (!companyId) return

  const allocationsPayload = localAllocations.value
    .filter(a => Number(a.allocatedAmount) > 0)
    .map(a => ({
      documentType: form.value.paymentType === 'payment_received' ? ('sales_invoice' as const) : ('purchase_invoice' as const),
      documentId: a.invoiceId,
      allocatedAmount: Number(a.allocatedAmount)
    }))

  const payload: CreatePaymentRequest = {
    companyId,
    paymentType: form.value.paymentType,
    counterpartyType: form.value.paymentType === 'payment_received' ? 'customer' : 'supplier',
    counterpartyId: form.value.counterpartyId,
    paymentDate: form.value.paymentDate,
    bankAccountId: form.value.bankAccountId,
    amount: form.value.amount,
    notes: form.value.notes || undefined,
    attachmentUrl: form.value.attachmentUrl || undefined,
    allocations: allocationsPayload
  }

  try {
    if (isEdit.value && form.value.id) {
      await paymentApi.updateDraft(form.value.id, payload)
      successMsg.value = 'Payment draft updated successfully.'
    } else {
      await paymentApi.createDraft(payload)
      successMsg.value = 'Payment draft recorded successfully.'
    }
    showFormModal.value = false
    await fetchPayments()
    await fetchInvoices()
  } catch (err: any) {
    errorMsg.value = err.response?.data?.message || err.message || 'Failed to save payment draft.'
  } finally {
    loading.value = false
  }
}

function confirmDelete(id: string) {
  paymentToDeleteId.value = id
  showConfirmDelete.value = true
}

async function deleteDraft() {
  if (!paymentToDeleteId.value) return
  loading.value = true
  errorMsg.value = null
  successMsg.value = null
  
  try {
    await paymentApi.deleteDraft(paymentToDeleteId.value)
    successMsg.value = 'Payment draft deleted successfully.'
    showConfirmDelete.value = false
    paymentToDeleteId.value = null
    await fetchPayments()
    await fetchInvoices()
  } catch (err: any) {
    errorMsg.value = err.message || 'Failed to delete payment draft.'
  } finally {
    loading.value = false
  }
}

async function submitApproval(id: string) {
  loading.value = true
  errorMsg.value = null
  successMsg.value = null
  
  try {
    await paymentApi.submitApproval(id)
    successMsg.value = 'Payment draft submitted for approval.'
    await fetchPayments()
  } catch (err: any) {
    errorMsg.value = err.message || 'Failed to submit payment approval.'
  } finally {
    loading.value = false
  }
}

// Resetting Form fields
function resetForm() {
  const companyId = authStore.currentUser?.companyId || ''
  form.value = {
    companyId,
    paymentType: 'payment_received',
    counterpartyType: 'customer',
    counterpartyId: '',
    paymentDate: new Date().toISOString().split('T')[0],
    bankAccountId: '',
    amount: 0,
    notes: '',
    attachmentUrl: '',
  }
  localAllocations.value = []
}

// Attachment states & helpers
const uploadingAttachment = ref(false)
const fileInput = ref<HTMLInputElement | null>(null)
const uploadError = ref<string | null>(null)
const uploadSuccess = ref(false)

function triggerFileInput() {
  uploadError.value = null
  fileInput.value?.click()
}

function getAttachmentFilename(url: string) {
  if (!url) return ''
  const parts = url.split('/')
  return parts[parts.length - 1]
}

const isPdf = (url: string | undefined): boolean => {
  if (!url) return false
  return url.toLowerCase().endsWith('.pdf')
}

async function handleFileUpload(event: Event) {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (!file) return

  uploadingAttachment.value = true
  uploadError.value = null
  uploadSuccess.value = false
  try {
    const url = await invoiceApi.uploadAttachment(file)
    form.value.attachmentUrl = url
    uploadSuccess.value = true
    setTimeout(() => {
      uploadSuccess.value = false
    }, 5000)
  } catch (err: any) {
    uploadError.value = err.response?.data?.message || err.message || 'Failed to upload attachment.'
  } finally {
    uploadingAttachment.value = false
    if (fileInput.value) {
      fileInput.value.value = ''
    }
  }
}

function removeAttachment() {
  form.value.attachmentUrl = ''
  uploadSuccess.value = false
  uploadError.value = null
}

function clearUploadError() {
  uploadError.value = null
}

// Paging & Search filters
const filteredPayments = computed(() => {
  return payments.value.filter(p => {
    // Search term
    if (searchTerm.value) {
      const term = searchTerm.value.toLowerCase()
      const refMatch = p.referenceNumber?.toLowerCase().includes(term)
      const notesMatch = p.notes?.toLowerCase().includes(term)
      if (!refMatch && !notesMatch) return false
    }

    // Type filter
    if (selectedType.value && p.paymentType !== selectedType.value) {
      return false
    }

    // Status filter
    if (selectedStatus.value && p.status !== selectedStatus.value) {
      return false
    }

    return true
  })
})

const paginatedPayments = computed(() => {
  const start = (currentPage.value - 1) * perPage.value
  const end = start + perPage.value
  return filteredPayments.value.slice(start, end)
})

const totalPages = computed(() => {
  return Math.ceil(filteredPayments.value.length / perPage.value) || 1
})

const paginationStart = computed(() => {
  if (filteredPayments.value.length === 0) return 0
  return (currentPage.value - 1) * perPage.value + 1
})

const paginationEnd = computed(() => {
  const val = currentPage.value * perPage.value
  return val > filteredPayments.value.length ? filteredPayments.value.length : val
})

const visiblePages = computed(() => {
  const pages: number[] = []
  const maxPages = 5
  let start = Math.max(1, currentPage.value - Math.floor(maxPages / 2))
  let end = Math.min(totalPages.value, start + maxPages - 1)
  
  if (end - start + 1 < maxPages) {
    start = Math.max(1, end - maxPages + 1)
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

function applyFilters() {
  currentPage.value = 1
}

function resetFilters() {
  searchTerm.value = ''
  selectedType.value = ''
  selectedStatus.value = ''
  currentPage.value = 1
}

// Lookup names
function getCounterpartyName(payment: Payment): string {
  if (payment.paymentType === 'payment_received') {
    const cust = customersMap.value.get(payment.counterpartyId)
    return cust ? cust.name : payment.counterpartyId
  } else {
    const supp = suppliersMap.value.get(payment.counterpartyId)
    return supp ? supp.name : payment.counterpartyId
  }
}

function getBankAccountName(id: string): string {
  const ba = bankAccountsMap.value.get(id)
  return ba ? `${ba.bankName} - ${ba.accountName}` : id
}

function getInvoiceNumberById(id: string, type: 'sales_invoice' | 'purchase_invoice'): string {
  if (type === 'sales_invoice') {
    const inv = salesInvoices.value.find(si => si.id === id)
    return inv ? inv.invoiceNumber : id
  } else {
    const inv = purchaseInvoices.value.find(pi => pi.id === id)
    return inv ? inv.supplierInvoiceNumber : id
  }
}

// Formatting helpers
function formatDate(dateStr: string): string {
  if (!dateStr) return '-'
  return new Date(dateStr).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}

function formatDateOnly(dateStr: string): string {
  if (!dateStr) return '-'
  return new Date(dateStr).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}

function formatStatus(status: string): string {
  return status.replace('_', ' ')
}

function getStatusBadgeClass(status: string): string {
  switch (status) {
    case 'draft': return 'badge-secondary'
    case 'waiting_review':
    case 'waiting_approval': return 'badge-warning'
    case 'approved': return 'badge-info'
    case 'posted': return 'badge-success'
    case 'rejected': return 'badge-danger'
    default: return 'badge-secondary'
  }
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
  max-width: 850px;
}

.text-right-input {
  text-align: right;
}

.totals-row td {
  border-top: 2px solid var(--border-color);
  background-color: rgba(241, 245, 249, 0.5);
}

.btn-xs {
  padding: 4px 8px;
  font-size: 0.75rem;
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

.dropdown-fade-enter-active,
.dropdown-fade-leave-active {
  transition: all 0.15s ease-out;
}
.dropdown-fade-enter-from,
.dropdown-fade-leave-to {
  opacity: 0;
  transform: scale(0.95) translateY(-5px);
}
.attachment-upload-container {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.attachment-dropzone {
  border: 2px dashed var(--border-color);
  border-radius: var(--radius-md);
  padding: 16px;
  text-align: center;
  cursor: pointer;
  transition: all var(--transition-fast);
  background-color: rgba(255, 255, 255, 0.02);
}

.attachment-dropzone:hover {
  border-color: var(--accent-primary);
  background-color: rgba(255, 255, 255, 0.04);
}

.attachment-dropzone.has-file {
  border-style: solid;
  border-color: #10b981;
  background-color: rgba(16, 185, 129, 0.05);
}

.upload-prompt, .file-uploaded-view, .uploading-spinner {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  color: var(--text-secondary);
}

.file-uploaded-view {
  color: var(--text-primary);
  font-weight: 500;
}

.file-name {
  max-width: 300px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.btn-remove-file {
  background: none;
  border: none;
  color: #ef4444;
  font-size: 1.2rem;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0 4px;
}

.btn-remove-file:hover {
  color: #f87171;
}

.attachment-preview-box {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
}

.attachment-dropzone.has-error {
  border-color: #ef4444;
  background-color: rgba(239, 68, 68, 0.05);
}

.file-error-view {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  color: #ef4444;
  font-weight: 500;
}
</style>
