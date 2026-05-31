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
}

export interface UpdatePasswordPayload {
  currentPassword: string | null // required for regular users, ignored for admins
  newPassword: string
}
