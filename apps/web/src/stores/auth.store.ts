import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { authApi } from '@/api/auth.api'
import type { LoginRequest, User } from '@/types/auth.types'

function decodeJwt(token: string): any {
  try {
    const base64Url = token.split('.')[1]
    const base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/')
    const jsonPayload = decodeURIComponent(
      window.atob(base64)
        .split('')
        .map(c => '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2))
        .join('')
    )
    return JSON.parse(jsonPayload)
  } catch (e) {
    return null
  }
}

export const useAuthStore = defineStore('auth', () => {
  // ─── State ───────────────────────────────────────────────────────────────────
  const user = ref<User | null>(
    (() => {
      const stored = localStorage.getItem('user')
      if (stored) {
        try {
          return JSON.parse(stored)
        } catch {
          // ignore
        }
      }
      
      // Auto-reconstruct user from access token claims if present (for existing sessions)
      const token = localStorage.getItem('access_token')
      if (token) {
        const claims = decodeJwt(token)
        if (claims) {
          return {
            id: claims.sub,
            companyId: claims.company_id,
            email: '',
            fullName: 'User',
            roles: claims.roles || [],
          }
        }
      }
      return null
    })()
  )
  const accessToken   = ref<string | null>(localStorage.getItem('access_token'))
  const refreshToken  = ref<string | null>(localStorage.getItem('refresh_token'))

  // ─── Getters ─────────────────────────────────────────────────────────────────
  const isAuthenticated = computed(() => !!accessToken.value)
  const currentUser     = computed(() => user.value)
  const userRoles       = computed(() => user.value?.roles ?? [])

  const hasRole = (role: string) =>
    userRoles.value.some(r => {
      const normalizedApiRole = r.toLowerCase().replace(/_/g, '')
      const normalizedCheckRole = role.toLowerCase().replace(/_/g, '')
      return normalizedApiRole === normalizedCheckRole
    })
  const canApprove = computed(() =>
    hasRole('Owner') || hasRole('FinanceManager') || hasRole('Admin')
  )

  // ─── Actions ─────────────────────────────────────────────────────────────────
  async function login(credentials: LoginRequest) {
    const response = await authApi.login(credentials)

    accessToken.value  = response.accessToken
    refreshToken.value = response.refreshToken
    user.value         = response.user

    localStorage.setItem('access_token', response.accessToken)
    localStorage.setItem('refresh_token', response.refreshToken)
    localStorage.setItem('user', JSON.stringify(response.user))
  }

  async function logout() {
    try {
      await authApi.logout()
    } catch (e) {
      console.warn('Backend logout call failed:', e)
    } finally {
      accessToken.value  = null
      refreshToken.value = null
      user.value         = null
      localStorage.removeItem('access_token')
      localStorage.removeItem('refresh_token')
      localStorage.removeItem('user')
    }
  }

  async function refreshAccessToken() {
    if (!refreshToken.value) throw new Error('No refresh token')
    const response = await authApi.refresh(refreshToken.value)
    accessToken.value  = response.accessToken
    refreshToken.value = response.refreshToken
    localStorage.setItem('access_token', response.accessToken)
    localStorage.setItem('refresh_token', response.refreshToken)
  }

  return {
    user, accessToken, refreshToken,
    isAuthenticated, currentUser, userRoles, canApprove,
    login, logout, refreshAccessToken, hasRole,
  }
})
