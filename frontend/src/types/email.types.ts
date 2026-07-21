export type EmailCategory = 'CAMPAIGN' | 'TRANSACTIONAL'

export interface EmailResponse {
  id: string
  subject: string
  provider: string
  body: string
  recipientEmails: string[]
  recipientCount: number
  category: EmailCategory
  createdAt: string
}

export interface EmailProviderStatusResponse {
  provider: string
  dailyLimit: number
  sentToday: number
  remaining: number
}

export interface SendEmailPayload {
  subject: string
  htmlBody: string
  previewHtmlBody?: string
}

export interface SendEmailToInterestsPayload {
  subject: string
  htmlBody: string
  previewHtmlBody?: string
  interestIds: number[]
}

export interface EmailProviderInfo {
  name: string
  dailyLimit: number
}

export interface MultiEmailCheckResponse {
  provider: string
  dailyLimit: number
  sentToday: number
  remaining: number
  totalRecipients: number
  exceedsLimit: boolean
}
