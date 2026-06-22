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

    <!-- Filters/Search Card -->
    <div class="card" style="margin-bottom: 24px; padding: 16px;">
      <div style="display: flex; gap: 16px; flex-wrap: wrap; align-items: center;">
        <div style="flex: 1; min-width: 200px;">
          <input
            v-model="searchTerm"
            type="text"
            class="form-input"
            placeholder="Search by requested by, reference number..."
            @input="applyFilters"
          />
        </div>
        <div style="width: 220px;">
          <select v-model="selectedDocType" class="form-select" @change="applyFilters">
            <option value="">All Document Types</option>
            <option v-for="type in uniqueDocTypes" :key="type" :value="type">
              {{ formatDocType(type) }}
            </option>
          </select>
        </div>
        <div>
          <button class="btn btn-secondary" @click="resetFilters">Reset</button>
        </div>
      </div>
    </div>

    <!-- Approvals List Table -->
    <div class="table-container">
      <div v-if="approvalStore.loading && filteredApprovals.length === 0" class="loading-state">
        Loading pending approval requests...
      </div>
      <div v-else-if="filteredApprovals.length === 0" class="empty-state">
        <span v-if="searchTerm || selectedDocType">No pending approvals matching your filters.</span>
        <span v-else>No pending approvals found. Excellent!</span>
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
          <tr v-for="req in paginatedApprovals" :key="req.id">
            <td>{{ formatDate(req.createdAt) }}</td>
            <td>{{ req.requestedByName || req.requestedBy || 'System' }}</td>
            <td>
              <span class="badge badge-info" style="text-transform: capitalize;">
                {{ formatDocType(req.documentType) }}
              </span>
            </td>
            <td style="font-family: monospace; font-size: 0.85rem; font-weight: 500; color: var(--accent-primary);">
              {{ req.documentReference || req.documentId }}
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

    <!-- Pagination Footer -->
    <div class="pagination-footer" v-if="filteredApprovals.length > 0">
      <div class="pagination-info">
        Showing {{ paginationStart }} to {{ paginationEnd }} of {{ filteredApprovals.length }} entries
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
              <p class="meta-val">{{ activeRequest.requestedByName || activeRequest.requestedBy || 'System' }}</p>
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

            <!-- Sales Invoice Details Display -->
            <div v-else-if="activeRequest.documentType === 'sales_invoice' && invoiceDetails" class="doc-details-box">
              <div class="detail-grid" style="margin-bottom: 16px;">
                <div>
                  <p class="detail-label">Invoice Number</p>
                  <p class="detail-val-highlight">{{ invoiceDetails.invoiceNumber }}</p>
                </div>
                <div>
                  <p class="detail-label">Customer</p>
                  <p class="detail-val">{{ getCustomerName(invoiceDetails.customerId) }}</p>
                </div>
                <div>
                  <p class="detail-label">Invoice Date</p>
                  <p class="detail-val">{{ formatDate(invoiceDetails.invoiceDate) }}</p>
                </div>
                <div>
                  <p class="detail-label">Due Date</p>
                  <p class="detail-val">{{ formatDate(invoiceDetails.dueDate) }}</p>
                </div>
              </div>

              <h4>Invoice Lines</h4>
              <div class="table-container" style="margin-top: 8px;">
                <table>
                  <thead>
                    <tr>
                      <th>Description</th>
                      <th>Account</th>
                      <th style="text-align: right;">Qty</th>
                      <th style="text-align: right;">Unit Price</th>
                      <th style="text-align: right;">Discount</th>
                      <th style="text-align: right;">Tax Amount</th>
                      <th style="text-align: right;">Total</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr v-for="line in invoiceDetails.lines" :key="line.id">
                      <td>{{ line.description }}</td>
                      <td>{{ getAccountName(line.accountId) }}</td>
                      <td style="text-align: right;">{{ line.quantity }}</td>
                      <td style="text-align: right;">{{ formatCurrency(line.unitPrice) }}</td>
                      <td style="text-align: right;">{{ line.discountAmount > 0 ? formatCurrency(line.discountAmount) : '-' }}</td>
                      <td style="text-align: right;">{{ line.taxAmount > 0 ? formatCurrency(line.taxAmount) : '-' }}</td>
                      <td style="text-align: right; font-weight: 500;">{{ formatCurrency(line.lineTotal) }}</td>
                    </tr>
                    <tr class="totals-row">
                      <td colspan="5" style="font-weight: 600;">Subtotal</td>
                      <td colspan="2" style="text-align: right; font-weight: 600;">{{ formatCurrency(invoiceDetails.subtotal) }}</td>
                    </tr>
                    <tr class="totals-row" style="border-top: none;">
                      <td colspan="5" style="font-weight: 600;">Tax Amount</td>
                      <td colspan="2" style="text-align: right; font-weight: 600;">{{ formatCurrency(invoiceDetails.taxAmount) }}</td>
                    </tr>
                    <tr class="totals-row" style="border-top: none;">
                      <td colspan="5" style="font-weight: 700; color: var(--accent-primary);">Grand Total</td>
                      <td colspan="2" style="text-align: right; font-weight: 700; color: var(--accent-primary); font-size: 1.05rem;">
                        {{ formatCurrency(invoiceDetails.totalAmount) }}
                      </td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </div>

            <!-- Purchase Invoice Details Display -->
            <div v-else-if="activeRequest.documentType === 'purchase_invoice' && purchaseInvoiceDetails" class="doc-details-box">
              <div class="detail-grid" style="margin-bottom: 16px;">
                <div>
                  <p class="detail-label">Supplier Invoice Number</p>
                  <p class="detail-val-highlight">{{ purchaseInvoiceDetails.supplierInvoiceNumber }}</p>
                </div>
                <div>
                  <p class="detail-label">Internal Reference</p>
                  <p class="detail-val">{{ purchaseInvoiceDetails.internalReference }}</p>
                </div>
                <div>
                  <p class="detail-label">Supplier</p>
                  <p class="detail-val">{{ getSupplierName(purchaseInvoiceDetails.supplierId) }}</p>
                </div>
                <div>
                  <p class="detail-label">Invoice Date</p>
                  <p class="detail-val">{{ formatDate(purchaseInvoiceDetails.invoiceDate) }}</p>
                </div>
                <div>
                  <p class="detail-label">Due Date</p>
                  <p class="detail-val">{{ formatDate(purchaseInvoiceDetails.dueDate) }}</p>
                </div>
              </div>

              <h4>Invoice Lines</h4>
              <div class="table-container" style="margin-top: 8px;">
                <table>
                  <thead>
                    <tr>
                      <th>Description</th>
                      <th>Account</th>
                      <th style="text-align: right;">Qty</th>
                      <th style="text-align: right;">Unit Price</th>
                      <th style="text-align: right;">Discount</th>
                      <th style="text-align: right;">Tax Amount</th>
                      <th style="text-align: right;">Total</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr v-for="line in purchaseInvoiceDetails.lines" :key="line.id">
                      <td>{{ line.description }}</td>
                      <td>{{ getAccountName(line.accountId) }}</td>
                      <td style="text-align: right;">{{ line.quantity }}</td>
                      <td style="text-align: right;">{{ formatCurrency(line.unitPrice) }}</td>
                      <td style="text-align: right;">{{ line.discountAmount > 0 ? formatCurrency(line.discountAmount) : '-' }}</td>
                      <td style="text-align: right;">{{ line.taxAmount > 0 ? formatCurrency(line.taxAmount) : '-' }}</td>
                      <td style="text-align: right; font-weight: 500;">{{ formatCurrency(line.lineTotal) }}</td>
                    </tr>
                    <tr class="totals-row">
                      <td colspan="5" style="font-weight: 600;">Subtotal</td>
                      <td colspan="2" style="text-align: right; font-weight: 600;">{{ formatCurrency(purchaseInvoiceDetails.subtotal) }}</td>
                    </tr>
                    <tr class="totals-row" style="border-top: none;">
                      <td colspan="5" style="font-weight: 600;">Tax Amount</td>
                      <td colspan="2" style="text-align: right; font-weight: 600;">{{ formatCurrency(purchaseInvoiceDetails.taxAmount) }}</td>
                    </tr>
                    <tr class="totals-row" style="border-top: none;">
                      <td colspan="5" style="font-weight: 700; color: var(--accent-primary);">Grand Total</td>
                      <td colspan="2" style="text-align: right; font-weight: 700; color: var(--accent-primary); font-size: 1.05rem;">
                        {{ formatCurrency(purchaseInvoiceDetails.totalAmount) }}
                      </td>
                    </tr>
                  </tbody>
                </table>
              </div>

              <!-- Purchase Invoice Attachment -->
              <div v-if="purchaseInvoiceDetails.attachmentUrl" class="notes-box" style="margin-top: 16px;">
                <p class="notes-label" style="font-size: 0.75rem; text-transform: uppercase; font-weight: 600; color: var(--text-secondary); margin-bottom: 6.0px;">Attachment</p>
                <div class="attachment-preview-box" style="display: flex; flex-direction: column; align-items: flex-start;">
                  <a :href="purchaseInvoiceDetails.attachmentUrl" target="_blank" class="attachment-link" style="color: var(--accent-primary); display: inline-flex; align-items: center; gap: 6px; text-decoration: none; font-weight: 500;">
                    <template v-if="isPdf(purchaseInvoiceDetails.attachmentUrl)">
                      <span>📄</span> View PDF Document
                    </template>
                    <template v-else>
                      <span>🖼️</span> View Attached Image
                    </template>
                  </a>
                  <div v-if="!isPdf(purchaseInvoiceDetails.attachmentUrl)" class="attachment-image-thumb-container" style="margin-top: 8px;">
                    <img :src="purchaseInvoiceDetails.attachmentUrl" alt="Attachment" class="attachment-image-thumb" style="max-width: 100%; max-height: 120px; border-radius: var(--radius-sm); border: 1px solid var(--border-color); object-fit: contain; cursor: pointer;" />
                  </div>
                </div>
              </div>
            </div>

            <!-- Payment Details -->
            <div v-else-if="(activeRequest.documentType === 'payment_received' || activeRequest.documentType === 'payment_paid') && paymentDetails">
              <div class="meta-card" style="margin-bottom: 20px;">
                <div>
                  <div class="meta-label">Reference Number</div>
                  <div class="meta-val-highlight">{{ paymentDetails.referenceNumber }}</div>
                </div>
                <div>
                  <div class="meta-label">Payment Date</div>
                  <div class="meta-val">{{ formatDate(paymentDetails.paymentDate) }}</div>
                </div>
                <div>
                  <div class="meta-label">Type</div>
                  <div class="meta-val">
                    <span :class="['badge', paymentDetails.paymentType === 'payment_received' ? 'badge-success' : 'badge-info']">
                      {{ paymentDetails.paymentType === 'payment_received' ? 'Payment Received' : 'Payment Paid' }}
                    </span>
                  </div>
                </div>
                <div>
                  <div class="meta-label">Bank Account</div>
                  <div class="meta-val">{{ getBankAccountName(paymentDetails.bankAccountId) }}</div>
                </div>
                <div>
                  <div class="meta-label">Total Amount</div>
                  <div class="meta-val-highlight" style="color: #10b981;">
                    {{ formatCurrency(paymentDetails.amount) }}
                  </div>
                </div>
              </div>
              <div v-if="paymentDetails.notes || paymentDetails.attachmentUrl" style="margin-bottom: 20px; display: flex; flex-wrap: wrap; gap: 20px;">
                <div v-if="paymentDetails.notes" class="notes-box" style="flex: 1; min-width: 300px; margin-top: 0;">
                  <p class="notes-label" style="font-size: 0.75rem; text-transform: uppercase; font-weight: 600; color: var(--text-secondary); margin-bottom: 6px;">Notes</p>
                  <p style="margin: 0; font-size: 0.9rem; white-space: pre-wrap;">{{ paymentDetails.notes }}</p>
                </div>
                <div v-if="paymentDetails.attachmentUrl" class="notes-box" style="flex: 1; min-width: 300px; margin-top: 0;">
                  <p class="notes-label" style="font-size: 0.75rem; text-transform: uppercase; font-weight: 600; color: var(--text-secondary); margin-bottom: 6px;">Attachment</p>
                  <div class="attachment-preview-box" style="display: flex; flex-direction: column; align-items: flex-start;">
                    <a :href="paymentDetails.attachmentUrl" target="_blank" class="attachment-link" style="color: var(--accent-primary); display: inline-flex; align-items: center; gap: 6px; text-decoration: none; font-weight: 500;">
                      <template v-if="isPdf(paymentDetails.attachmentUrl)">
                        <span>📄</span> View PDF Document
                      </template>
                      <template v-else>
                        <span>🖼️</span> View Attached Image
                      </template>
                    </a>
                    <div v-if="!isPdf(paymentDetails.attachmentUrl)" class="attachment-image-thumb-container" style="margin-top: 8px;">
                      <img :src="paymentDetails.attachmentUrl" alt="Attachment" class="attachment-image-thumb" style="max-width: 100%; max-height: 120px; border-radius: var(--radius-sm); border: 1px solid var(--border-color); object-fit: contain; cursor: pointer;" />
                    </div>
                  </div>
                </div>
              </div>

              <!-- Allocations -->
              <div v-if="paymentDetails.allocations && paymentDetails.allocations.length > 0">
                <h4 style="margin-bottom: 10px; font-weight: 600;">Invoices Allocations</h4>
                <div class="table-container">
                  <table>
                    <thead>
                      <tr>
                        <th>Document Type</th>
                        <th>Invoice Number / ID</th>
                        <th style="text-align: right;">Allocated Amount</th>
                      </tr>
                    </thead>
                    <tbody>
                      <tr v-for="alloc in paymentDetails.allocations" :key="alloc.id">
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
              <div v-else class="empty-state">
                No allocations recorded for this payment.
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
import { invoiceApi } from '@/api/invoices.api'
import { paymentApi } from '@/api/payments.api'
import { customerApi, accountApi, supplierApi, bankAccountApi } from '@/api/master-data.api'
import type { ApprovalRequest } from '@/types/approval.types'
import type { JournalEntry } from '@/types/journal.types'
import type { SalesInvoice, PurchaseInvoice } from '@/types/invoice.types'
import type { Payment } from '@/types/payment.types'
import type { Account, Customer, Supplier, BankAccount } from '@/types/master-data.types'

const approvalStore = useApprovalStore()
const authStore = useAuthStore()

// Notifications
const successMsg = ref<string | null>(null)
const errorMsg = ref<string | null>(null)

// Search, Filter, Pagination states
const searchTerm = ref('')
const selectedDocType = ref('')
const currentPage = ref(1)
const perPage = ref(10)

// Modal states
const showProcessModal = ref(false)
const activeRequest = ref<ApprovalRequest | null>(null)
const comment = ref('')
const submitting = ref(false)

// Loaded details
const loadingDoc = ref(false)
const docLoadError = ref<string | null>(null)
const journalDetails = ref<JournalEntry | null>(null)
const invoiceDetails = ref<SalesInvoice | null>(null)
const purchaseInvoiceDetails = ref<PurchaseInvoice | null>(null)
const paymentDetails = ref<Payment | null>(null)
const accounts = ref<Account[]>([])
const customers = ref<Customer[]>([])
const suppliers = ref<Supplier[]>([])
const bankAccounts = ref<BankAccount[]>([])
const salesInvoices = ref<SalesInvoice[]>([])
const purchaseInvoices = ref<PurchaseInvoice[]>([])

// Computeds for filtering & paging
const uniqueDocTypes = computed(() => {
  const types = new Set<string>()
  approvalStore.pendingApprovals.forEach(req => {
    if (req.documentType) {
      types.add(req.documentType)
    }
  })
  return Array.from(types)
})

const filteredApprovals = computed(() => {
  return approvalStore.pendingApprovals.filter(req => {
    // Search Term filter
    if (searchTerm.value) {
      const q = searchTerm.value.toLowerCase()
      const matchesUser = (req.requestedByName || '').toLowerCase().includes(q) || (req.requestedBy || '').toLowerCase().includes(q)
      const matchesDocType = formatDocType(req.documentType).toLowerCase().includes(q)
      const matchesDocId = (req.documentReference || req.documentId || '').toLowerCase().includes(q)
      if (!matchesUser && !matchesDocType && !matchesDocId) {
        return false
      }
    }
    // Doc Type filter
    if (selectedDocType.value && req.documentType.toLowerCase() !== selectedDocType.value.toLowerCase()) {
      return false
    }
    return true
  })
})

const paginatedApprovals = computed(() => {
  const start = (currentPage.value - 1) * perPage.value
  const end = start + perPage.value
  return filteredApprovals.value.slice(start, end)
})

const totalPages = computed(() => {
  return Math.ceil(filteredApprovals.value.length / perPage.value)
})

const paginationStart = computed(() => {
  if (filteredApprovals.value.length === 0) return 0
  return (currentPage.value - 1) * perPage.value + 1
})

const paginationEnd = computed(() => {
  return Math.min(currentPage.value * perPage.value, filteredApprovals.value.length)
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

function applyFilters() {
  currentPage.value = 1
}

function resetFilters() {
  searchTerm.value = ''
  selectedDocType.value = ''
  currentPage.value = 1
}

function changePage(page: number) {
  if (page >= 1 && page <= totalPages.value) {
    currentPage.value = page
  }
}

onMounted(async () => {
  await fetchPending()
  await loadAccounts()
  await loadCustomers()
  await loadSuppliers()
  await loadBankAccounts()
  await loadInvoices()
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

async function loadCustomers() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return
  try {
    customers.value = await customerApi.listByCompany(companyId)
  } catch (err: any) {
    console.error('Failed to load customers map', err)
  }
}

async function loadSuppliers() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return
  try {
    suppliers.value = await supplierApi.listByCompany(companyId)
  } catch (err: any) {
    console.error('Failed to load suppliers map', err)
  }
}

async function loadBankAccounts() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return
  try {
    bankAccounts.value = await bankAccountApi.listByCompany(companyId)
  } catch (err: any) {
    console.error('Failed to load bank accounts map', err)
  }
}

async function loadInvoices() {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return
  try {
    const [salesRes, purchaseRes] = await Promise.all([
      invoiceApi.listSales({ companyId, page: 1, perPage: 1000 }),
      invoiceApi.listPurchases({ companyId, page: 1, perPage: 1000 })
    ])
    salesInvoices.value = salesRes.data
    purchaseInvoices.value = purchaseRes.data
  } catch (err: any) {
    console.error('Failed to load invoices map', err)
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

const bankAccountsMap = computed(() => {
  const map = new Map<string, BankAccount>()
  bankAccounts.value.forEach(b => map.set(b.id, b))
  return map
})

const accountsMap = computed(() => {
  const map = new Map<string, Account>()
  accounts.value.forEach(a => map.set(a.id, a))
  return map
})

const customersMap = computed(() => {
  const map = new Map<string, Customer>()
  customers.value.forEach(c => map.set(c.id, c))
  return map
})

const suppliersMap = computed(() => {
  const map = new Map<string, Supplier>()
  suppliers.value.forEach(s => map.set(s.id, s))
  return map
})

function getAccountName(accountId: string): string {
  const acc = accountsMap.value.get(accountId)
  return acc ? `${acc.code} - ${acc.name}` : accountId
}

function getCustomerName(customerId: string): string {
  const cust = customersMap.value.get(customerId)
  return cust ? cust.name : customerId
}

function getSupplierName(supplierId: string): string {
  const supp = suppliersMap.value.get(supplierId)
  return supp ? supp.name : supplierId
}

// Open modal and fetch target details
async function viewAndProcess(req: ApprovalRequest) {
  activeRequest.value = req
  comment.value = ''
  docLoadError.value = null
  journalDetails.value = null
  invoiceDetails.value = null
  purchaseInvoiceDetails.value = null
  paymentDetails.value = null
  loadingDoc.value = true
  showProcessModal.value = true

  try {
    if (req.documentType === 'journal_entry') {
      journalDetails.value = await journalApi.getById(req.documentId)
    } else if (req.documentType === 'sales_invoice') {
      invoiceDetails.value = await invoiceApi.getSalesById(req.documentId)
    } else if (req.documentType === 'purchase_invoice') {
      purchaseInvoiceDetails.value = await invoiceApi.getPurchaseById(req.documentId)
    } else if (req.documentType === 'payment_received' || req.documentType === 'payment_paid') {
      paymentDetails.value = await paymentApi.getById(req.documentId)
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
  invoiceDetails.value = null
  purchaseInvoiceDetails.value = null
  paymentDetails.value = null
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

const isPdf = (url: string | undefined): boolean => {
  if (!url) return false
  return url.toLowerCase().endsWith('.pdf')
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
</style>
