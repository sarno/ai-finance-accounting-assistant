<template>
  <MainLayout>
    <div class="page-header">
      <div>
        <h1>🤖 AI Invoice Automation</h1>
        <p class="page-title-desc">Upload documents and verify AI extraction results in real-time.</p>
      </div>
      <router-link to="/purchases" class="btn btn-secondary">
        &larr; Back to Invoices
      </router-link>
    </div>

    <!-- Error/Success Alerts -->
    <div v-if="errorMsg" class="alert alert-danger" style="margin-top: 16px;">{{ errorMsg }}</div>
    <div v-if="successMsg" class="alert alert-success" style="margin-top: 16px;">{{ successMsg }}</div>

    <!-- 1. UPLOADER VIEW (when no doc is loaded) -->
    <div 
      v-if="!documentData && !uploading && !processing" 
      class="card upload-card" 
      @dragover.prevent="dragOver = true"
      @dragleave.prevent="dragOver = false"
      @drop.prevent="handleDrop"
      :class="{ 'drag-over': dragOver }"
    >
      <div class="upload-zone" @click="triggerFileInput">
        <div class="upload-icon">📄</div>
        <h2>Drag and drop your invoice here</h2>
        <p>or click to browse from your computer</p>
        <div class="upload-meta">Supports PDF, PNG, JPG, JPEG (Max 10MB)</div>
        <input ref="fileInput" type="file" style="display: none" accept=".pdf,.png,.jpg,.jpeg" @change="handleFileSelect" />
      </div>
    </div>

    <!-- 2. UPLOADING / PROCESSING VIEW -->
    <div v-else-if="uploading || processing" class="card loading-card">
      <div class="loading-spinner"></div>
      <h2>{{ loadingTitle }}</h2>
      <p style="color: var(--text-secondary); margin-top: 8px;">{{ loadingSubtitle }}</p>
      <div class="processing-steps" style="margin-top: 32px; max-width: 400px; margin-left: auto; margin-right: auto; text-align: left;">
        <div v-for="(step, idx) in processingSteps" :key="idx" class="step-item" :class="{ 'step-active': step.active, 'step-done': step.done }">
          <span class="step-bullet">{{ step.done ? '✓' : idx + 1 }}</span>
          <span class="step-text">{{ step.text }}</span>
        </div>
      </div>
    </div>

    <!-- 3. SPLIT WORKSPACE VIEW (when doc is loaded and processed) -->
    <div v-else-if="documentData" class="workspace-grid">
      <!-- LEFT COLUMN: Document Preview -->
      <div class="card preview-card">
        <div class="card-header-flex">
          <h3>Document Preview</h3>
          <span class="file-name-badge" :title="documentData.original_file_name">
            {{ truncateFileName(documentData.original_file_name) }}
          </span>
        </div>
        <div class="preview-container">
          <iframe v-if="isPdf" :src="documentDataUrl" class="preview-iframe"></iframe>
          <img v-else :src="documentDataUrl" class="preview-image" alt="Invoice Preview" />
        </div>
      </div>

      <!-- RIGHT COLUMN: Details & Form -->
      <div class="details-container">
        <!-- AI Extracted Fields & Validation checklist -->
        <div class="card form-card">
          <div class="card-header-flex">
            <h3>Extracted Fields</h3>
            <span class="confidence-badge" :class="confidenceClass">
              Confidence: {{ (aiConfidence * 100).toFixed(0) }}%
            </span>
          </div>

          <form @submit.prevent style="margin-top: 16px;">
            <div class="form-grid-2">
              <div class="form-group">
                <label class="form-label">Supplier *</label>
                <SearchableDropdown
                  v-model="form.supplierId"
                  :options="suppliers"
                  placeholder="Select supplier..."
                  no-results-text="No suppliers found"
                  :required="true"
                  :get-option-key="(s) => s.id"
                  :get-option-label="(s) => s.name"
                  :get-option-search-text="(s) => s.name"
                />
              </div>
              <div class="form-group">
                <label class="form-label">Invoice Number *</label>
                <input v-model="form.invoiceNo" type="text" class="form-input" required />
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

            <div class="form-grid-3" style="margin-top: 16px;">
              <div class="form-group">
                <label class="form-label">Subtotal (Auto-calc)</label>
                <input :value="formatCurrency(form.subtotal)" type="text" class="form-input" readonly />
              </div>
              <div class="form-group">
                <label class="form-label">Tax Amount (PPN)</label>
                <input :value="formatCurrency(form.taxAmount)" type="text" class="form-input" readonly />
              </div>
              <div class="form-group">
                <label class="form-label">Total Amount</label>
                <input :value="formatCurrency(form.totalAmount)" type="text" class="form-input" style="font-weight: 600; color: var(--accent-primary);" readonly />
              </div>
            </div>
          </form>

          <!-- Compliance / Validation Checklist -->
          <div class="checklist-section" style="margin-top: 24px; border-top: 1px solid var(--border-color); padding-top: 20px;">
            <h4>Compliance & AI Validation Checks</h4>
            <div class="checklist-container">
              <div v-for="item in checklist" :key="item.name" class="checklist-item" :class="{ 'passed': item.passed, 'failed': !item.passed }">
                <span class="checklist-icon">{{ item.passed ? '✓' : '✗' }}</span>
                <span class="checklist-msg">{{ item.message }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Interactive Invoice Lines Editor -->
        <div class="card lines-card">
          <div class="card-header-flex">
            <h3>Invoice Line Items</h3>
            <button class="btn btn-secondary btn-sm" @click="addLine">+ Add Item Line</button>
          </div>
          <div class="table-container" style="margin-top: 16px; max-height: 350px; overflow-y: auto;">
            <table>
              <thead>
                <tr>
                  <th>Description</th>
                  <th style="width: 80px; text-align: right;">Qty</th>
                  <th style="width: 140px; text-align: right;">Unit Price</th>
                  <th style="width: 200px;">Expense Account</th>
                  <th style="width: 150px;">Tax Type</th>
                  <th style="text-align: right; width: 120px;">Total</th>
                  <th style="width: 40px; text-align: center;"></th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="(line, idx) in invoiceLines" :key="idx">
                  <td>
                    <input v-model="line.description" type="text" class="form-input table-input" required />
                  </td>
                  <td>
                    <input v-model.number="line.quantity" type="number" min="0.001" step="any" class="form-input table-input" style="text-align: right;" required />
                  </td>
                  <td>
                    <input v-model.number="line.unitPrice" type="number" min="0" step="any" class="form-input table-input" style="text-align: right;" required />
                  </td>
                  <td>
                    <SearchableDropdown
                      v-model="line.expenseAccountId"
                      :options="accounts"
                      placeholder="Select account..."
                      no-results-text="No accounts"
                      :required="true"
                      container-class="table-dropdown"
                      :get-option-key="(a) => a.id"
                      :get-option-label="(a) => `${a.code} - ${a.name}`"
                      :get-option-search-text="(a) => `${a.code} ${a.name}`"
                    />
                  </td>
                  <td>
                    <select v-model="line.taxTypeId" class="form-select table-input">
                      <option :value="null">No Tax</option>
                      <option v-for="t in taxTypes" :key="t.id" :value="t.id">
                        {{ t.name }} ({{ t.defaultRate > 1 ? t.defaultRate : t.defaultRate * 100 }}%)
                      </option>
                    </select>
                  </td>
                  <td style="text-align: right; font-weight: 500;">
                    {{ formatCurrency(line.quantity * line.unitPrice) }}
                  </td>
                  <td style="text-align: center;">
                    <button class="btn-delete" @click="removeLine(idx)" :disabled="invoiceLines.length === 1">🗑️</button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <!-- Suggested Journal Entry Preview -->
        <div class="card journal-card">
          <div class="card-header-flex">
            <h3>Suggested Journal Entries Preview</h3>
            <span class="badge badge-info">AI Suggested Posting</span>
          </div>
          <div class="table-container" style="margin-top: 16px;">
            <table>
              <thead>
                <tr>
                  <th>Account Code</th>
                  <th>Account Name</th>
                  <th style="text-align: right;">Debit</th>
                  <th style="text-align: right;">Credit</th>
                  <th>Description</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="(jl, index) in suggestedJournalLines" :key="index">
                  <td style="font-family: monospace; font-size: 0.85rem;">{{ jl.accountCode }}</td>
                  <td>{{ jl.accountName }}</td>
                  <td style="text-align: right; color: var(--success); font-weight: 500;">
                    {{ jl.debit > 0 ? formatCurrency(jl.debit) : '-' }}
                  </td>
                  <td style="text-align: right; color: var(--accent-primary); font-weight: 500;">
                    {{ jl.credit > 0 ? formatCurrency(jl.credit) : '-' }}
                  </td>
                  <td style="color: var(--text-secondary); font-size: 0.85rem; max-width: 200px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">
                    {{ jl.description }}
                  </td>
                </tr>
                <tr style="border-top: 2px solid var(--border-color); font-weight: 600; background-color: var(--bg-primary);">
                  <td colspan="2" style="text-align: right;">Total Balance</td>
                  <td style="text-align: right; color: var(--success);">{{ formatCurrency(journalTotals.debit) }}</td>
                  <td style="text-align: right; color: var(--accent-primary);">{{ formatCurrency(journalTotals.credit) }}</td>
                  <td>
                    <span v-if="journalTotals.debit === journalTotals.credit" style="color: var(--success); font-weight: bold;">✓ Balanced</span>
                    <span v-else style="color: var(--danger); font-weight: bold;">✗ Unbalanced</span>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <!-- Approval Workflow panel -->
        <div class="card approval-card" style="margin-bottom: 40px;">
          <h3>Submit & Posting Action</h3>
          <p style="color: var(--text-secondary); font-size: 0.9rem; margin-top: 8px;">
            Verify all fields, checklist validations, and journal entries. Once satisfied, submit this transaction as a draft or escalate to the approval workflow pool.
          </p>

          <div class="form-group" style="margin-top: 16px;">
            <label class="form-label">Submitter Notes / Comments (Optional)</label>
            <textarea v-model="approvalComments" class="form-input" rows="3" placeholder="Add any comments or explanations here..."></textarea>
          </div>

          <div class="workflow-actions" style="margin-top: 24px; display: flex; gap: 12px; justify-content: flex-end;">
            <button class="btn btn-secondary" @click="resetWorkspace">Reset / Upload Another</button>
            <button class="btn btn-secondary" :disabled="submitting" @click="submitInvoice(false)">
              💾 Save Draft
            </button>
            <button class="btn btn-primary" :disabled="submitting" @click="submitInvoice(true)">
              🚀 Submit for Approval
            </button>
          </div>
        </div>
      </div>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth.store'
import MainLayout from '@/components/MainLayout.vue'
import SearchableDropdown from '@/components/SearchableDropdown.vue'
import { supplierApi, accountApi, taxTypeApi } from '@/api/master-data.api'
import { invoiceApi } from '@/api/invoices.api'
import type { Supplier, Account, TaxType } from '@/types/master-data.types'

const router = useRouter()
const authStore = useAuthStore()

// File Drop State
const dragOver = ref(false)

// Workflow State
const uploading = ref(false)
const processing = ref(false)
const errorMsg = ref<string | null>(null)
const successMsg = ref<string | null>(null)
const fileInput = ref<HTMLInputElement | null>(null)
const submitting = ref(false)

// Master data lists
const suppliers = ref<Supplier[]>([])
const accounts = ref<Account[]>([])
const taxTypes = ref<TaxType[]>([])

// Uploaded Document data
const documentData = ref<any>(null)
const documentDataUrl = ref<string>('')

// Extracted fields form
const form = ref({
  supplierId: '',
  invoiceNo: '',
  invoiceDate: '',
  dueDate: '',
  subtotal: 0,
  taxAmount: 0,
  totalAmount: 0
})

// Validation checklist
const checklist = ref<any[]>([])

// Invoice lines editor
interface InvoiceLineEdit {
  description: string
  quantity: number
  unitPrice: number
  expenseAccountId: string
  taxTypeId: string | null
}
const invoiceLines = ref<InvoiceLineEdit[]>([])

// Approval/Workflow info
const approvalComments = ref('')

// Step indicators for UI
const processingSteps = ref([
  { text: 'Unggah file dokumen ke penyimpanan', active: false, done: false },
  { text: 'Ekstraksi data menggunakan AI OCR', active: false, done: false },
  { text: 'Verifikasi kepatuhan dan aturan audit', active: false, done: false }
])

const loadingTitle = ref('Memproses Dokumen')
const loadingSubtitle = ref('Dokumen Anda sedang dikirim ke server...')

// Computed helpers
const isPdf = computed(() => {
  if (!documentData.value) return false
  const path = documentData.value.storage_path || ''
  return path.toLowerCase().endsWith('.pdf') || documentData.value.mime_type?.includes('pdf')
})

const aiConfidence = computed(() => {
  return documentData.value?.ai_confidence || 0
})

const confidenceClass = computed(() => {
  const conf = aiConfidence.value
  if (conf >= 0.8) return 'confidence-high'
  if (conf >= 0.5) return 'confidence-medium'
  return 'confidence-low'
})

// Find default AP account
const apAccount = computed(() => {
  return accounts.value.find(a => a.code === '2100' || a.name.toLowerCase().includes('payable') || a.name.toLowerCase().includes('hutang')) || accounts.value[0]
})

// Truncate long filenames
const truncateFileName = (name: string) => {
  if (!name) return ''
  if (name.length <= 25) return name
  return name.substring(0, 15) + '...' + name.substring(name.length - 8)
}

// Recalculate subtotal, tax amount, and total amount when line items edit
const watchLines = () => {
  let sub = 0
  let tax = 0
  
  invoiceLines.value.forEach(line => {
    const lineVal = line.quantity * line.unitPrice
    sub += lineVal
    if (line.taxTypeId) {
      const rate = getTaxRate(line.taxTypeId)
      tax += lineVal * rate
    }
  })
  
  form.value.subtotal = Math.round(sub * 100) / 100
  form.value.taxAmount = Math.round(tax * 100) / 100
  form.value.totalAmount = Math.round((sub + tax) * 100) / 100
}

watch(invoiceLines, watchLines, { deep: true })

// Helper to get tax rate
const getTaxRate = (taxTypeId: string | null) => {
  if (!taxTypeId) return 0
  const tax = taxTypes.value.find(t => t.id === taxTypeId)
  if (!tax) return 0
  return tax.defaultRate > 1 ? tax.defaultRate / 100 : tax.defaultRate
}

// Format Currency
const formatCurrency = (val: number) => {
  return new Intl.NumberFormat('id-ID', { style: 'currency', currency: 'IDR', minimumFractionDigits: 0 }).format(val)
}

// Fetch master data on mount
onMounted(async () => {
  const companyId = authStore.currentUser?.companyId
  if (!companyId) return
  
  try {
    const [sups, accs, taxes] = await Promise.all([
      supplierApi.listByCompany(companyId),
      accountApi.listByCompany(companyId),
      taxTypeApi.listByCompany(companyId)
    ])
    
    suppliers.value = sups
    accounts.value = accs.filter(a => a.isActive).sort((a, b) => a.code.localeCompare(b.code))
    taxTypes.value = taxes.filter(t => t.isActive)
  } catch (err: any) {
    console.error('Failed to load master data:', err)
    errorMsg.value = 'Failed to load master data. Please reload the page.'
  }
})

// Triggering uploader
const triggerFileInput = () => {
  fileInput.value?.click()
}

const handleFileSelect = (event: Event) => {
  const target = event.target as HTMLInputElement
  if (target.files && target.files[0]) {
    uploadFile(target.files[0])
  }
}

const handleDrop = (event: DragEvent) => {
  dragOver.value = false
  if (event.dataTransfer?.files && event.dataTransfer.files[0]) {
    uploadFile(event.dataTransfer.files[0])
  }
}

// Upload file & poll
const uploadFile = async (file: File) => {
  errorMsg.value = null
  successMsg.value = null
  uploading.value = true
  processing.value = true
  
  // Set steps
  processingSteps.value = [
    { text: 'Unggah file dokumen ke penyimpanan', active: true, done: false },
    { text: 'Ekstraksi data menggunakan AI OCR', active: false, done: false },
    { text: 'Verifikasi kepatuhan dan aturan audit', active: false, done: false }
  ]
  
  try {
    const res = await invoiceApi.uploadDocumentForOcr(file)
    processingSteps.value[0].done = true
    processingSteps.value[0].active = false
    processingSteps.value[1].active = true
    
    loadingTitle.value = 'AI Menganalisis Invoice'
    loadingSubtitle.value = 'AI sedang mengidentifikasi supplier, nomor faktur, nominal total, dan tanggal...'
    
    const docId = res.id
    // Poll for status
    pollDocumentStatus(docId)
  } catch (err: any) {
    console.error(err)
    errorMsg.value = err.response?.data?.message || err.message || 'File upload failed.'
    uploading.value = false
    processing.value = false
  }
}

const pollDocumentStatus = (docId: string) => {
  let attempts = 0
  const interval = setInterval(async () => {
    attempts++
    if (attempts > 30) {
      clearInterval(interval)
      errorMsg.value = 'AI OCR process timed out. Please try again.'
      uploading.value = false
      processing.value = false
      return
    }
    
    try {
      const doc = await invoiceApi.getDocument(docId)
      if (doc.status === 'completed') {
        clearInterval(interval)
        
        processingSteps.value[1].done = true
        processingSteps.value[1].active = false
        processingSteps.value[2].active = true
        
        loadingTitle.value = 'Menjalankan Kepatuhan'
        loadingSubtitle.value = 'Mencocokkan nomor faktur duplikat dan validasi data supplier...'
        
        // Wait 1 sec for styling transition
        setTimeout(() => {
          documentData.value = doc
          documentDataUrl.value = `/uploads/${doc.storage_path}`
          
          // Populate fields
          const extracted = doc.extracted_fields || {}
          
          form.value.invoiceNo = extracted.invoice_number || ''
          
          // Format date strings to YYYY-MM-DD
          form.value.invoiceDate = formatDateString(extracted.invoice_date)
          form.value.dueDate = formatDateString(extracted.due_date)
          
          const subtotalExt = parseNumber(extracted.subtotal)
          const taxAmountExt = parseNumber(extracted.tax_amount)
          const totalAmountExt = parseNumber(extracted.total_amount)
          
          form.value.subtotal = subtotalExt
          form.value.taxAmount = taxAmountExt
          form.value.totalAmount = totalAmountExt
          
          // Match supplier in list
          const supplierName = extracted.supplier_name || ''
          const matchedSup = suppliers.value.find(s => 
            s.name.toLowerCase().includes(supplierName.toLowerCase()) || 
            supplierName.toLowerCase().includes(s.name.toLowerCase())
          )
          form.value.supplierId = matchedSup?.id || ''
          
          // Populate checklist
          checklist.value = doc.validation_results?.checklist || []
          
          // Pre-populate invoice lines with a single default line matching subtotal
          initializeLines(subtotalExt, taxAmountExt)
          
          uploading.value = false
          processing.value = false
        }, 1000)
      } else if (doc.status === 'failed') {
        clearInterval(interval)
        errorMsg.value = doc.error_message || 'AI OCR extraction failed.'
        uploading.value = false
        processing.value = false
      }
    } catch (err: any) {
      console.error(err)
      clearInterval(interval)
      errorMsg.value = 'Failed to poll document status: ' + err.message
      uploading.value = false
      processing.value = false
    }
  }, 2000)
}

const formatDateString = (dateStr: string | null) => {
  if (!dateStr) return ''
  if (dateStr.includes('T')) {
    return dateStr.split('T')[0]
  }
  const regex = /(\d{4})-(\d{2})-(\d{2})/
  const match = dateStr.match(regex)
  if (match) return match[0]
  
  return dateStr
}

const parseNumber = (val: string | null | number) => {
  if (val === null || val === undefined) return 0
  if (typeof val === 'number') return val
  const cleaned = val.replace(/[^0-9.-]/g, '')
  return parseFloat(cleaned) || 0
}

const initializeLines = (subtotal: number, taxAmount: number) => {
  // Try to find a sensible default expense account
  // Look for expense code (typically starts with '5')
  const defaultExpense = accounts.value.find(a => a.code.startsWith('5')) || accounts.value[0]
  // Look for tax type
  const defaultTax = taxTypes.value[0] || null

  invoiceLines.value = [
    {
      description: 'AI Generated Purchase Expense',
      quantity: 1,
      unitPrice: subtotal,
      expenseAccountId: defaultExpense?.id || '',
      taxTypeId: taxAmount > 0 ? (defaultTax?.id || null) : null
    }
  ]
}

const addLine = () => {
  const defaultExpense = accounts.value.find(a => a.code.startsWith('5')) || accounts.value[0]
  invoiceLines.value.push({
    description: 'Beban Pembelian',
    quantity: 1,
    unitPrice: 0,
    expenseAccountId: defaultExpense?.id || '',
    taxTypeId: null
  })
}

const removeLine = (index: number) => {
  if (invoiceLines.value.length > 1) {
    invoiceLines.value.splice(index, 1)
  }
}

// Suggested journal entries preview generator
const suggestedJournalLines = computed(() => {
  const jl: any[] = []
  
  invoiceLines.value.forEach(line => {
    const account = accounts.value.find(a => a.id === line.expenseAccountId)
    const amount = line.quantity * line.unitPrice
    
    jl.push({
      accountCode: account?.code || '?',
      accountName: account?.name || 'Pilih Akun...',
      debit: amount,
      credit: 0,
      description: line.description || 'Beban Pembelian'
    })
    
    if (line.taxTypeId) {
      const taxType = taxTypes.value.find(t => t.id === line.taxTypeId)
      const taxRate = getTaxRate(line.taxTypeId)
      const taxAmount = amount * taxRate
      
      if (taxAmount > 0) {
        const taxAccount = accounts.value.find(a => a.id === taxType?.payableAccountId)
        jl.push({
          accountCode: taxAccount?.code || '?',
          accountName: taxAccount?.name || 'PPN Masukan (Pajak)',
          debit: taxAmount,
          credit: 0,
          description: `PPN: ${line.description || 'Beban Pembelian'}`
        })
      }
    }
  })
  
  const apAcc = apAccount.value
  jl.push({
    accountCode: apAcc?.code || '2100',
    accountName: apAcc?.name || 'Hutang Usaha',
    debit: 0,
    credit: form.value.totalAmount,
    description: `Hutang atas invoice #${form.value.invoiceNo || 'Draft'}`
  })
  
  return jl
})

const journalTotals = computed(() => {
  let debit = 0
  let credit = 0
  suggestedJournalLines.value.forEach(line => {
    debit += line.debit
    credit += line.credit
  })
  return {
    debit: Math.round(debit * 100) / 100,
    credit: Math.round(credit * 100) / 100
  }
})

// Submitting invoice
const submitInvoice = async (forApproval: boolean) => {
  if (!form.value.supplierId) {
    errorMsg.value = 'Please select a supplier.'
    return
  }
  if (!form.value.invoiceNo) {
    errorMsg.value = 'Please enter an invoice number.'
    return
  }
  if (!form.value.invoiceDate) {
    errorMsg.value = 'Please select an invoice date.'
    return
  }
  if (journalTotals.value.debit !== journalTotals.value.credit) {
    errorMsg.value = 'Suggested journal entry is unbalanced. Please check quantities and unit prices.'
    return
  }

  errorMsg.value = null
  successMsg.value = null
  submitting.value = true

  const payload = {
    uploadedDocumentId: documentData.value.id,
    supplierId: form.value.supplierId,
    extractedFields: {
      invoiceNo: form.value.invoiceNo,
      invoiceDate: form.value.invoiceDate,
      dueDate: form.value.dueDate || form.value.invoiceDate,
      subtotal: form.value.subtotal,
      taxAmount: form.value.taxAmount,
      totalAmount: form.value.totalAmount
    },
    lines: invoiceLines.value.map(line => ({
      description: line.description,
      quantity: line.quantity,
      unitPrice: line.unitPrice,
      expenseAccountId: line.expenseAccountId,
      taxTypeId: line.taxTypeId
    })),
    aiConfidence: aiConfidence.value
  }

  try {
    const createdInvoice = await invoiceApi.createPurchaseFromDocument(payload)
    
    if (forApproval) {
      await invoiceApi.submitPurchaseApproval(createdInvoice.id)
      successMsg.value = 'Purchase invoice draft created and submitted for approval successfully!'
    } else {
      successMsg.value = 'Purchase invoice draft saved successfully!'
    }
    
    setTimeout(() => {
      router.push('/purchases')
    }, 1500)
  } catch (err: any) {
    console.error(err)
    errorMsg.value = err.response?.data?.message || err.message || 'Failed to submit invoice.'
    submitting.value = false
  }
}

// Reset view
const resetWorkspace = () => {
  documentData.value = null
  documentDataUrl.value = ''
  form.value = {
    supplierId: '',
    invoiceNo: '',
    invoiceDate: '',
    dueDate: '',
    subtotal: 0,
    taxAmount: 0,
    totalAmount: 0
  }
  invoiceLines.value = []
  checklist.value = []
  approvalComments.value = ''
  errorMsg.value = null
  successMsg.value = null
}
</script>

<style scoped>
.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.upload-card {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 400px;
  border: 2px dashed var(--border-color);
  background: var(--bg-secondary);
  cursor: pointer;
  transition: all var(--transition-normal);
}

.upload-card:hover, .upload-card.drag-over {
  border-color: var(--accent-primary);
  box-shadow: 0 0 20px var(--accent-primary-glow);
  background-color: var(--bg-primary);
}

.upload-zone {
  text-align: center;
  padding: 40px;
}

.upload-icon {
  font-size: 4rem;
  margin-bottom: 16px;
  animation: bounce 2s infinite;
}

.upload-meta {
  margin-top: 12px;
  font-size: 0.85rem;
  color: var(--text-muted);
}

.loading-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 400px;
  text-align: center;
}

.loading-spinner {
  width: 50px;
  height: 50px;
  border: 4px solid var(--border-color);
  border-top-color: var(--accent-primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 24px;
}

.workspace-grid {
  display: grid;
  grid-template-columns: 1.1fr 0.9fr;
  gap: 24px;
  margin-top: 24px;
  align-items: start;
}

@media (max-width: 1200px) {
  .workspace-grid {
    grid-template-columns: 1fr;
  }
}

.preview-card {
  height: calc(100vh - 180px);
  position: sticky;
  top: 20px;
  display: flex;
  flex-direction: column;
}

.card-header-flex {
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-bottom: 1px solid var(--border-color);
  padding-bottom: 12px;
  margin-bottom: 16px;
}

.file-name-badge {
  background-color: var(--bg-tertiary);
  color: var(--text-secondary);
  font-size: 0.8rem;
  padding: 4px 10px;
  border-radius: var(--radius-sm);
  max-width: 200px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-family: monospace;
}

.preview-container {
  flex-grow: 1;
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  overflow: hidden;
  background-color: var(--bg-tertiary);
  display: flex;
  justify-content: center;
  align-items: center;
}

.preview-iframe {
  width: 100%;
  height: 100%;
  border: none;
}

.preview-image {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
}

.details-container {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.confidence-badge {
  font-size: 0.8rem;
  padding: 4px 10px;
  border-radius: var(--radius-sm);
  font-weight: 600;
}

.confidence-high {
  background-color: var(--success-bg);
  color: var(--success);
}

.confidence-medium {
  background-color: var(--warning-bg);
  color: var(--warning);
}

.confidence-low {
  background-color: var(--danger-bg);
  color: var(--danger);
}

.checklist-container {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin-top: 12px;
}

.checklist-item {
  display: flex;
  align-items: flex-start;
  gap: 10px;
  padding: 10px 12px;
  border-radius: var(--radius-sm);
  font-size: 0.9rem;
}

.checklist-item.passed {
  background-color: var(--success-bg);
  border-left: 4px solid var(--success);
}

.checklist-item.failed {
  background-color: var(--danger-bg);
  border-left: 4px solid var(--danger);
}

.checklist-icon {
  font-weight: bold;
}

.checklist-item.passed .checklist-icon {
  color: var(--success);
}

.checklist-item.failed .checklist-icon {
  color: var(--danger);
}

.table-input {
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  padding: 6px 10px;
  width: 100%;
  background: var(--bg-secondary);
}

.table-input:focus {
  border-color: var(--border-focus);
  outline: none;
}

.table-dropdown {
  width: 100%;
  min-width: 180px;
}

.btn-delete {
  background: none;
  border: none;
  cursor: pointer;
  font-size: 1.1rem;
  opacity: 0.6;
  transition: opacity var(--transition-fast);
}

.btn-delete:hover:not(:disabled) {
  opacity: 1;
}

.btn-delete:disabled {
  cursor: not-allowed;
  opacity: 0.2;
}

.step-item {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 12px;
  opacity: 0.4;
  transition: opacity var(--transition-fast);
}

.step-item.step-active {
  opacity: 1;
  font-weight: 600;
}

.step-item.step-done {
  opacity: 0.8;
}

.step-bullet {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  border: 1px solid var(--text-muted);
  font-size: 0.85rem;
}

.step-active .step-bullet {
  border-color: var(--accent-primary);
  background-color: var(--accent-primary-glow);
  color: var(--accent-primary);
}

.step-done .step-bullet {
  border-color: var(--success);
  background-color: var(--success-bg);
  color: var(--success);
}

@keyframes bounce {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-10px); }
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>
