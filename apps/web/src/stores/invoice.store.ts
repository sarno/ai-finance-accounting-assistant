import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoiceApi } from '@/api/invoices.api'
import type {
  SalesInvoice,
  CreateSalesInvoiceRequest,
  InvoiceFilters,
} from '@/types/invoice.types'

export const useInvoiceStore = defineStore('invoice', () => {
  const invoices        = ref<SalesInvoice[]>([])
  const selectedInvoice = ref<SalesInvoice | null>(null)
  const loading         = ref(false)
  const error           = ref<string | null>(null)
  const total           = ref(0)
  const filters         = ref<InvoiceFilters>({ page: 1, perPage: 20 })

  async function fetchSalesInvoices() {
    loading.value = true
    error.value = null
    try {
      const result = await invoiceApi.listSales(filters.value)
      invoices.value = result.data
      total.value    = result.total
    } catch (e: any) {
      error.value = e.message
    } finally {
      loading.value = false
    }
  }

  async function createSalesDraft(req: CreateSalesInvoiceRequest) {
    loading.value = true
    try {
      const created = await invoiceApi.createSalesDraft(req)
      invoices.value.unshift(created)
      return created
    } finally {
      loading.value = false
    }
  }

  async function updateSalesDraft(id: string, req: CreateSalesInvoiceRequest) {
    loading.value = true
    try {
      const updated = await invoiceApi.updateSalesDraft(id, req)
      const idx = invoices.value.findIndex(i => i.id === id)
      if (idx !== -1) {
        invoices.value[idx] = updated
      }
      return updated
    } finally {
      loading.value = false
    }
  }

  async function deleteSalesDraft(id: string) {
    loading.value = true
    try {
      await invoiceApi.deleteSalesDraft(id)
      invoices.value = invoices.value.filter(i => i.id !== id)
    } finally {
      loading.value = false
    }
  }

  async function submitSalesApproval(id: string) {
    await invoiceApi.submitSalesApproval(id)
    const idx = invoices.value.findIndex(i => i.id === id)
    if (idx !== -1) invoices.value[idx].status = 'waiting_approval'
  }

  function setFilters(newFilters: Partial<InvoiceFilters>) {
    filters.value = { ...filters.value, ...newFilters }
  }

  return {
    invoices, selectedInvoice, loading, error, total, filters,
    fetchSalesInvoices, createSalesDraft, updateSalesDraft, deleteSalesDraft, submitSalesApproval, setFilters,
  }
})
