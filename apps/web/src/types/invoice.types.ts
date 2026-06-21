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

export interface InvoiceLine {
  id: string
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

export interface CreateInvoiceLineRequest {
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
