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
  isActive?: boolean
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

export interface CreateUserRequest {
  email: string
  fullName: string
  password?: string
  roles: string[]
}

export interface UpdateUserRequest {
  fullName: string
  roles: string[]
  isActive: boolean
}

