import client from './client'
import type {
  JournalEntry,
  CreateJournalDraftRequest,
  JournalFilters,
  PaginatedResponse,
} from '@/types/journal.types'

export const journalApi = {
  list: async (filters: JournalFilters): Promise<PaginatedResponse<JournalEntry>> => {
    const { data } = await client.get<JournalEntry[]>('/journals', { params: filters })
    return {
      data,
      total: data.length,
      page: filters.page || 1,
      perPage: filters.perPage || 20,
    }
  },

  getById: async (id: string): Promise<JournalEntry> => {
    const { data } = await client.get<JournalEntry>(`/journals/${id}`)
    return data
  },

  createDraft: async (req: CreateJournalDraftRequest): Promise<JournalEntry> => {
    const { data } = await client.post<JournalEntry>('/journals/draft', req)
    return data
  },

  updateDraft: async (id: string, req: CreateJournalDraftRequest): Promise<JournalEntry> => {
    const { data } = await client.put<JournalEntry>(`/journals/${id}`, req)
    return data
  },

  deleteDraft: async (id: string): Promise<void> => {
    await client.delete(`/journals/${id}`)
  },

  submitApproval: async (id: string): Promise<void> => {
    await client.post(`/journals/${id}/submit`)
  },

  post: async (id: string): Promise<JournalEntry> => {
    const { data } = await client.post<JournalEntry>(`/journals/${id}/post`)
    return data
  },

  approve: async (id: string): Promise<JournalEntry> => {
    const { data } = await client.post<JournalEntry>(`/journals/${id}/approve`)
    return data
  },
}
