export interface EmailResponse {
  id: string
  subject: string
  provider: string
  body: string
  recipientEmails: string[]
  recipientCount: number
  createdAt: string
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
