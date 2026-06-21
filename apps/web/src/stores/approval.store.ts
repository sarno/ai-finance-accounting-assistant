import { defineStore } from 'pinia'
import { ref } from 'vue'
import { approvalApi } from '@/api/approvals.api'
import type { ApprovalRequest } from '@/types/approval.types'

export const useApprovalStore = defineStore('approval', () => {
  const pendingApprovals  = ref<ApprovalRequest[]>([])
  const selectedApproval  = ref<ApprovalRequest | null>(null)
  const loading           = ref(false)

  async function fetchPending() {
    loading.value = true
    try {
      pendingApprovals.value = await approvalApi.listPending()
    } finally {
      loading.value = false
    }
  }

  async function approve(id: string, comment: string) {
    await approvalApi.approve(id, comment)
    pendingApprovals.value = pendingApprovals.value.filter(a => a.id !== id)
  }

  async function reject(id: string, reason: string) {
    await approvalApi.reject(id, reason)
    pendingApprovals.value = pendingApprovals.value.filter(a => a.id !== id)
  }

  return {
    pendingApprovals, selectedApproval, loading,
    fetchPending, approve, reject,
  }
})
