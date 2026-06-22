export interface PaymentAllocation {
  id: string
  paymentId: string
  documentType: 'sales_invoice' | 'purchase_invoice'
  documentId: string
  allocatedAmount: number
}

export interface Payment {
  id: string
  companyId: string
  referenceNumber: string
  paymentType: 'payment_received' | 'payment_paid'
  counterpartyType: 'customer' | 'supplier'
  counterpartyId: string
  paymentDate: string
  bankAccountId: string
  amount: number
  allocations: PaymentAllocation[]
  status: 'draft' | 'waiting_review' | 'waiting_approval' | 'approved' | 'posted' | 'rejected' | 'cancelled'
  notes?: string
  journalEntryId?: string
  attachmentUrl?: string
  createdBy: string
  createdAt: string
  updatedAt: string
}

export interface CreatePaymentAllocationRequest {
  documentType: 'sales_invoice' | 'purchase_invoice'
  documentId: string
  allocatedAmount: number
}

export interface CreatePaymentRequest {
  companyId: string
  paymentType: 'payment_received' | 'payment_paid'
  counterpartyType: 'customer' | 'supplier'
  counterpartyId: string
  paymentDate: string
  bankAccountId: string
  amount: number
  allocations: CreatePaymentAllocationRequest[]
  notes?: string
  attachmentUrl?: string
}

export interface PaymentFilters {
  companyId?: string
  page?: number
  perPage?: number
}
