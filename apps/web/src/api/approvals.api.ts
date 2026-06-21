import client from './client'
import type { ApprovalRequest } from '@/types/approval.types'

export const approvalApi = {
  listPending: async (): Promise<ApprovalRequest[]> => {
    const { data } = await client.get<ApprovalRequest[]>('/approvals?status=pending')
    return data
  },

  getById: async (id: string): Promise<ApprovalRequest> => {
    const { data } = await client.get<ApprovalRequest>(`/approvals/${id}`)
    return data
  },

  approve: async (id: string, comment: string): Promise<void> => {
    await client.post(`/approvals/${id}/approve`, { comment })
  },

  reject: async (id: string, reason: string): Promise<void> => {
    await client.post(`/approvals/${id}/reject`, { reason })
  },
}
