import client from './client'
import type {
  SalesInvoice,
  PurchaseInvoice,
  CreateSalesInvoiceRequest,
  CreatePurchaseInvoiceRequest,
  InvoiceFilters,
} from '@/types/invoice.types'

export interface PaginatedResponse<T> {
  data: T[]
  total: number
  page: number
  perPage: number
}

export const invoiceApi = {
  listSales: async (filters: InvoiceFilters): Promise<PaginatedResponse<SalesInvoice>> => {
    const { data, headers } = await client.get<SalesInvoice[]>('/sales-invoices', { params: filters })
    const total = headers['x-total-count'] ? parseInt(headers['x-total-count'], 10) : data.length
    return {
      data,
      total,
      page: filters.page || 1,
      perPage: filters.perPage || 20,
    }
  },

  getSalesById: async (id: string): Promise<SalesInvoice> => {
    const { data } = await client.get<SalesInvoice>(`/sales-invoices/${id}`)
    return data
  },

  createSalesDraft: async (req: CreateSalesInvoiceRequest): Promise<SalesInvoice> => {
    const { data } = await client.post<SalesInvoice>('/sales-invoices/draft', req)
    return data
  },

  updateSalesDraft: async (id: string, req: CreateSalesInvoiceRequest): Promise<SalesInvoice> => {
    const { data } = await client.put<SalesInvoice>(`/sales-invoices/${id}`, req)
    return data
  },

  deleteSalesDraft: async (id: string): Promise<void> => {
    await client.delete(`/sales-invoices/${id}`)
  },

  submitSalesApproval: async (id: string): Promise<void> => {
    await client.post(`/sales-invoices/${id}/submit`)
  },

  listPurchases: async (filters: InvoiceFilters): Promise<PaginatedResponse<PurchaseInvoice>> => {
    const { data, headers } = await client.get<PurchaseInvoice[]>('/purchase-invoices', { params: filters })
    const total = headers['x-total-count'] ? parseInt(headers['x-total-count'], 10) : data.length
    return {
      data,
      total,
      page: filters.page || 1,
      perPage: filters.perPage || 20,
    }
  },

  getPurchaseById: async (id: string): Promise<PurchaseInvoice> => {
    const { data } = await client.get<PurchaseInvoice>(`/purchase-invoices/${id}`)
    return data
  },

  createPurchaseDraft: async (req: CreatePurchaseInvoiceRequest): Promise<PurchaseInvoice> => {
    const { data } = await client.post<PurchaseInvoice>('/purchase-invoices/draft', req)
    return data
  },

  updatePurchaseDraft: async (id: string, req: CreatePurchaseInvoiceRequest): Promise<PurchaseInvoice> => {
    const { data } = await client.put<PurchaseInvoice>(`/purchase-invoices/${id}`, req)
    return data
  },

  deletePurchaseDraft: async (id: string): Promise<void> => {
    await client.delete(`/purchase-invoices/${id}`)
  },

  submitPurchaseApproval: async (id: string): Promise<void> => {
    await client.post(`/purchase-invoices/${id}/submit`)
  },

  uploadAttachment: async (file: File): Promise<string> => {
    const formData = new FormData()
    formData.append('file', file)
    const { data } = await client.post<{ url: string }>('/upload', formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
    })
    return data.url
  },

  uploadDocumentForOcr: async (file: File): Promise<any> => {
    const formData = new FormData()
    formData.append('file', file)
    formData.append('documentType', 'purchase_invoice')
    const { data } = await client.post<any>('/upload', formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
    })
    return data
  },

  getDocument: async (id: string): Promise<any> => {
    const { data } = await client.get<any>(`/documents/${id}`)
    return data
  },

  createPurchaseFromDocument: async (req: any): Promise<PurchaseInvoice> => {
    const { data } = await client.post<PurchaseInvoice>('/purchase-invoices/from-document', req)
    return data
  },
}
