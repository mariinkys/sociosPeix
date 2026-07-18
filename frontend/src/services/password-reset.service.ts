import api from '@/api/axios'
import type { ForgotPasswordPayload, ResetPasswordPayload } from '@/types/auth.types'

class PasswordResetService {
  async requestReset(payload: ForgotPasswordPayload): Promise<void> {
    await api.post<void>('/api/auth/forgot-password', payload)
  }

  async resetPassword(payload: ResetPasswordPayload): Promise<void> {
    await api.post<void>('/api/auth/reset-password', payload)
  }
}

export const passwordResetService = new PasswordResetService()
