import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import api from '@/api/axios'
import type { AuthResponse, LoginPayload, RegisterPayload } from '@/types/auth.types'
import type { UserResponse } from '@/types/user.types'

export const useAuthStore = defineStore('auth', () => {
  const user = ref<AuthResponse | null>(null)
  const loading = ref(false)

  const isAuthenticated = computed(() => user.value !== null)
  const isAdmin = computed(() => user.value?.role === 'ADMIN')

  async function login(payload: LoginPayload): Promise<void> {
    const { data } = await api.post<AuthResponse>('/api/auth/login', payload)
    user.value = data
  }

  async function register(payload: RegisterPayload): Promise<void> {
    const { data } = await api.post<AuthResponse>('/api/auth/register', payload)
    user.value = data
  }

  async function logout(): Promise<void> {
    await api.post<void>('/api/auth/logout')
    user.value = null
  }

  async function fetchCurrentUser(): Promise<void> {
    loading.value = true
    try {
      const { data } = await api.get<UserResponse>('/api/users/me')
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
