export type UserRole = 'ADMIN' | 'USER'

export interface UserResponse {
  id: string
  name: string
  email: string
  role: UserRole
  createdAt: string
}

export interface UserUpdatePayload {
  name: string
  email: string
  password: string
}
