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
}

export interface SendEmailToInterestsPayload {
  subject: string
  htmlBody: string
  interestIds: number[]
}
