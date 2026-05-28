import type { UserRole } from './user.types'

export interface AuthResponse {
  id: number
  email: string
  role: UserRole
}

export interface LoginPayload {
  email: string
  password: string
}

export interface RegisterPayload {
  name: string
  email: string
  password: string
}
