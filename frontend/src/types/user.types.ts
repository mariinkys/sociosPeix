export type UserRole = 'ADMIN' | 'USER'

export interface UserResponse {
  id: number
  name: string
  email: string
  role: UserRole
  createdAt: string
}
