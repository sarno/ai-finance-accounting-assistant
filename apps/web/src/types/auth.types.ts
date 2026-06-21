// ─── Auth Types ───────────────────────────────────────────────────────────────

export interface LoginRequest {
  email: string
  password: string
}

export interface User {
  id: string
  companyId: string
  email: string
  fullName: string
  roles: string[]
}

export interface LoginResponse {
  accessToken: string
  refreshToken: string
  user: User
}

export interface RefreshResponse {
  accessToken: string
  refreshToken: string
}
