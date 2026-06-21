import client from './client'
import type { LoginRequest, LoginResponse, RefreshResponse } from '@/types/auth.types'

export const authApi = {
  login: async (req: LoginRequest): Promise<LoginResponse> => {
    const { data } = await client.post<LoginResponse>('/auth/login', req)
    return data
  },

  refresh: async (refreshToken: string): Promise<RefreshResponse> => {
    const { data } = await client.post<RefreshResponse>('/auth/refresh', { refresh_token: refreshToken })
    return data
  },

  logout: async (): Promise<void> => {
    await client.post('/auth/logout')
  },
}
