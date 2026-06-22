export interface SalesInvoice {
  id: string
  companyId: string
  branchId?: string
  invoiceNumber: string
  customerId: string
  invoiceDate: string
  dueDate: string
  lines: InvoiceLine[]
  subtotal: number
  taxAmount: number
  totalAmount: number
  status: 'draft' | 'waiting_review' | 'waiting_approval' | 'approved' | 'posted' | 'rejected' | 'cancelled'
  notes?: string
  journalEntryId?: string
  createdBy: string
  createdAt: string
  updatedAt: string
}

export interface PurchaseInvoice {
  id: string
  companyId: string
  branchId?: string
  supplierInvoiceNumber: string
  internalReference: string
  supplierId: string
  invoiceDate: string
  dueDate: string
  lines: PurchaseInvoiceLine[]
  subtotal: number
  taxAmount: number
  totalAmount: number
  status: 'draft' | 'waiting_review' | 'waiting_approval' | 'approved' | 'posted' | 'rejected' | 'cancelled'
  aiConfidence?: number
  uploadedDocumentId?: string
  notes?: string
  attachmentUrl?: string
  journalEntryId?: string
  createdBy: string
  createdAt: string
  updatedAt: string
}

export interface InvoiceLine {
  id: string
  itemId?: string
  description: string
  quantity: number
  unitPrice: number
  discountAmount: number
  taxTypeId?: string
  taxRate?: number
  taxAmount: number
  lineTotal: number
  accountId: string
  sortOrder: number
}

export interface PurchaseInvoiceLine {
  id: string
  itemId?: string
  description: string
  quantity: number
  unitPrice: number
  discountAmount: number
  taxTypeId?: string
  taxRate?: number
  taxAmount: number
  lineTotal: number
  accountId: string
  sortOrder: number
}

export interface CreateSalesInvoiceRequest {
  companyId: string
  branchId?: string
  invoiceNumber: string
  customerId: string
  invoiceDate: string
  dueDate: string
  lines: CreateInvoiceLineRequest[]
  notes?: string
}

export interface CreatePurchaseInvoiceRequest {
  companyId: string
  branchId?: string
  supplierInvoiceNumber: string
  internalReference: string
  supplierId: string
  invoiceDate: string
  dueDate: string
  lines: CreatePurchaseInvoiceLineRequest[]
  notes?: string
  attachmentUrl?: string
}

export interface CreateInvoiceLineRequest {
  itemId?: string
  description: string
  quantity: number
  unitPrice: number
  discountAmount?: number
  taxTypeId?: string
  accountId: string
  sortOrder: number
}

export interface CreatePurchaseInvoiceLineRequest {
  itemId?: string
  description: string
  quantity: number
  unitPrice: number
  discountAmount?: number
  taxTypeId?: string
  accountId: string
  sortOrder: number
}

export interface InvoiceFilters {
  companyId?: string
  page?: number
  perPage?: number
}
