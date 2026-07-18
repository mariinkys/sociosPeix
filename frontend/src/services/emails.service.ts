import api from '@/api/axios'
import type { PageResponse, PaginatedParams } from '@/types/common.types'
import type {
  EmailProviderInfo,
  EmailProviderStatusResponse,
  EmailResponse,
  SendEmailPayload,
  SendEmailToInterestsPayload,
} from '@/types/email.types'

function buildFormData(payload: object, attachments?: File[]): FormData {
  const form = new FormData()
  form.append('data', new Blob([JSON.stringify(payload)], { type: 'application/json' }))
  attachments?.forEach((file) => form.append('attachments', file))
  return form
}

class EmailsService {
  async getAll(params?: PaginatedParams): Promise<PageResponse<EmailResponse>> {
    const { data } = await api.get<PageResponse<EmailResponse>>('/api/emails', { params })
    return data
  }

  async getToday(): Promise<EmailResponse[]> {
    const { data } = await api.get<EmailResponse[]>('/api/emails/today')
    return data
  }

  async getById(id: string): Promise<EmailResponse> {
    const { data } = await api.get<EmailResponse>(`/api/emails/${id}`)
    return data
  }

  async getByMember(
    memberId: string,
    params?: PaginatedParams,
  ): Promise<PageResponse<EmailResponse>> {
    const { data } = await api.get<PageResponse<EmailResponse>>(`/api/emails/member/${memberId}`, {
      params,
    })
    return data
  }

  async sendToMember(
    memberId: string,
    payload: SendEmailPayload,
    attachments?: File[],
  ): Promise<EmailResponse> {
    const { data } = await api.post<EmailResponse>(
      `/api/emails/send/member/${memberId}`,
      buildFormData(payload, attachments),
    )
    return data
  }

  async sendToAll(payload: SendEmailPayload, attachments?: File[]): Promise<EmailResponse> {
    const { data } = await api.post<EmailResponse>(
      '/api/emails/send/all',
      buildFormData(payload, attachments),
    )
    return data
  }

  async sendToInterests(
    payload: SendEmailToInterestsPayload,
    attachments?: File[],
  ): Promise<EmailResponse> {
    const { data } = await api.post<EmailResponse>(
      '/api/emails/send/interests',
      buildFormData(payload, attachments),
    )
    return data
  }

  async getProviderStatus(): Promise<EmailProviderStatusResponse> {
    const { data } = await api.get<EmailProviderStatusResponse>(`api/emails/provider/status`)
    return data
  }

  async listProviders(): Promise<EmailProviderInfo[]> {
    const { data } = await api.get<EmailProviderInfo[]>('/api/emails/providers')
    return data
  }

  async setActiveProvider(provider: string): Promise<void> {
    await api.put<void>('/api/emails/active-provider', { provider })
  }
}

export const emailsService = new EmailsService()
