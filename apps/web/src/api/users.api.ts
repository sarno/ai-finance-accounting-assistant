import client from './client'
import type { User, CreateUserRequest, UpdateUserRequest } from '@/types/auth.types'

export const usersApi = {
  list: async (): Promise<User[]> => {
    const { data } = await client.get<User[]>('/users')
    return data
  },

  create: async (req: CreateUserRequest): Promise<User> => {
    const { data } = await client.post<User>('/users', req)
    return data
  },

  update: async (id: string, req: UpdateUserRequest): Promise<User> => {
    const { data } = await client.put<User>(`/users/${id}`, req)
    return data
  },

  delete: async (id: string): Promise<void> => {
    await client.delete(`/users/${id}`)
  },
}
