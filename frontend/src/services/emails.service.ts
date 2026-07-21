import api from '@/api/axios'
import type { PageResponse, PaginatedParams } from '@/types/common.types'
import type {
  EmailProviderInfo,
  EmailProviderStatusResponse,
  EmailResponse,
  MultiEmailCheckResponse,
  SendEmailPayload,
  SendEmailToInterestsPayload,
} from '@/types/email.types'

export interface InlineImage {
  file: File
  contentId: string
}

// the multipart part's filename IS the contentId - the backend needs no extra
// metadata channel to know which uploaded file corresponds to which cid:
function buildFormData(
  payload: object,
  attachments?: File[],
  inlineImages?: InlineImage[],
): FormData {
  const form = new FormData()
  form.append('data', new Blob([JSON.stringify(payload)], { type: 'application/json' }))
  attachments?.forEach((file) => form.append('attachments', file))
  inlineImages?.forEach((img) => form.append('inlineImages', img.file, img.contentId))
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
    inlineImages?: InlineImage[],
  ): Promise<EmailResponse> {
    const { data } = await api.post<EmailResponse>(
      `/api/emails/send/member/${memberId}`,
      buildFormData(payload, attachments, inlineImages),
    )
    return data
  }

  async sendToAll(
    payload: SendEmailPayload,
    attachments?: File[],
    inlineImages?: InlineImage[],
  ): Promise<EmailResponse> {
    const { data } = await api.post<EmailResponse>(
      '/api/emails/send/all',
      buildFormData(payload, attachments, inlineImages),
    )
    return data
  }

  async sendToInterests(
    payload: SendEmailToInterestsPayload,
    attachments?: File[],
    inlineImages?: InlineImage[],
  ): Promise<EmailResponse> {
    const { data } = await api.post<EmailResponse>(
      '/api/emails/send/interests',
      buildFormData(payload, attachments, inlineImages),
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

  async checkMultiSend(interestIds?: number[]): Promise<MultiEmailCheckResponse> {
    const { data } = await api.get<MultiEmailCheckResponse>('/api/emails/provider/multi-check', {
      params: interestIds && interestIds.length > 0 ? { interestIds } : undefined,
    })
    return data
  }
}

export const emailsService = new EmailsService()
