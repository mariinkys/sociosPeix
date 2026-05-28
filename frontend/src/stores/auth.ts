import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import api from '@/api/axios'

export type UserRole = 'ADMIN' | 'USER'

export interface User {
  id: number
  name: string
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

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const loading = ref(false)

  const isAuthenticated = computed(() => user.value !== null)
  const isAdmin = computed(() => user.value?.role === 'ADMIN')

  // TODO: Type API Responses and move current interfaces to proper location
  async function login(payload: LoginPayload): Promise<void> {
    const { data } = await api.post<User>('/api/auth/login', payload)
    user.value = data
  }

  async function register(payload: RegisterPayload): Promise<void> {
    const { data } = await api.post<User>('/api/auth/register', payload)
    user.value = data
  }

  async function logout(): Promise<void> {
    await api.post('/api/auth/logout')
    user.value = null
  }

  async function fetchCurrentUser(): Promise<void> {
    loading.value = true
    try {
      const { data } = await api.get<User>('/api/users/me')
      user.value = data
    } catch {
      user.value = null
    } finally {
      loading.value = false
    }
  }

  function clearUser(): void {
    user.value = null
  }

  return {
    user,
    loading,
    isAuthenticated,
    isAdmin,
    login,
    register,
    logout,
    fetchCurrentUser,
    clearUser,
  }
})
