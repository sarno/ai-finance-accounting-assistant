import client from './client'
import type {
  SalesInvoice,
  CreateSalesInvoiceRequest,
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
}
