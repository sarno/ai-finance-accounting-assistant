import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { journalApi } from '@/api/journals.api'
import type {
  JournalEntry,
  CreateJournalDraftRequest,
  JournalFilters,
} from '@/types/journal.types'

export const useJournalStore = defineStore('journal', () => {
  // ─── State ─────────────────────────────────────────────────────────────────
  const journals        = ref<JournalEntry[]>([])
  const selectedJournal = ref<JournalEntry | null>(null)
  const loading         = ref(false)
  const error           = ref<string | null>(null)
  const total           = ref(0)
  const filters         = ref<JournalFilters>({ page: 1, perPage: 20 })

  // ─── Getters ───────────────────────────────────────────────────────────────
  const draftJournals   = computed(() => journals.value.filter(j => j.status === 'draft'))
  const postedJournals  = computed(() => journals.value.filter(j => j.status === 'posted'))

  // ─── Actions ───────────────────────────────────────────────────────────────
  async function fetchJournals() {
    loading.value = true
    error.value = null
    try {
      const result = await journalApi.list(filters.value)
      journals.value = result.data
      total.value    = result.total
    } catch (e: any) {
      error.value = e.message
    } finally {
      loading.value = false
    }
  }

  async function createDraft(req: CreateJournalDraftRequest) {
    loading.value = true
    try {
      const created = await journalApi.createDraft(req)
      journals.value.unshift(created)
      return created
    } finally {
      loading.value = false
    }
  }

  async function updateDraft(id: string, req: CreateJournalDraftRequest) {
    loading.value = true
    try {
      const updated = await journalApi.updateDraft(id, req)
      const idx = journals.value.findIndex(j => j.id === id)
      if (idx !== -1) {
        journals.value[idx] = updated
      }
      return updated
    } finally {
      loading.value = false
    }
  }

  async function deleteDraft(id: string) {
    loading.value = true
    try {
      await journalApi.deleteDraft(id)
      journals.value = journals.value.filter(j => j.id !== id)
    } finally {
      loading.value = false
    }
  }

  async function submitForApproval(id: string) {
    await journalApi.submitApproval(id)
    const idx = journals.value.findIndex(j => j.id === id)
    if (idx !== -1) journals.value[idx].status = 'waiting_approval'
  }

  async function postJournal(id: string) {
    await journalApi.post(id)
    const idx = journals.value.findIndex(j => j.id === id)
    if (idx !== -1) journals.value[idx].status = 'posted'
  }

  function setFilters(newFilters: Partial<JournalFilters>) {
    filters.value = { ...filters.value, ...newFilters }
  }

  return {
    journals, selectedJournal, loading, error, total, filters,
    draftJournals, postedJournals,
    fetchJournals, createDraft, updateDraft, deleteDraft, submitForApproval, postJournal, setFilters,
  }
})
