import client from './client'
import type {
  Payment,
  CreatePaymentRequest,
  PaymentFilters,
} from '@/types/payment.types'
import type { PaginatedResponse } from './invoices.api'

export const paymentApi = {
  list: async (filters: PaymentFilters): Promise<PaginatedResponse<Payment>> => {
    const { data, headers } = await client.get<Payment[]>('/payments', { params: filters })
    const total = headers['x-total-count'] ? parseInt(headers['x-total-count'], 10) : data.length
    return {
      data,
      total,
      page: filters.page || 1,
      perPage: filters.perPage || 20,
    }
  },

  getById: async (id: string): Promise<Payment> => {
    const { data } = await client.get<Payment>(`/payments/${id}`)
    return data
  },

  createDraft: async (req: CreatePaymentRequest): Promise<Payment> => {
    const { data } = await client.post<Payment>('/payments/draft', req)
    return data
  },

  updateDraft: async (id: string, req: CreatePaymentRequest): Promise<{ success: boolean; payment: Payment }> => {
    const { data } = await client.put<{ success: boolean; payment: Payment }>(`/payments/${id}`, req)
    return data
  },

  deleteDraft: async (id: string): Promise<void> => {
    await client.delete(`/payments/${id}`)
  },

  submitApproval: async (id: string): Promise<Payment> => {
    const { data } = await client.post<Payment>(`/payments/${id}/submit`)
    return data
  },
}
