import type { PaginatedParams } from './common.types'
import type { InterestResponse } from './interest.types'

export interface MemberParams extends PaginatedParams {
  interestIds?: number[]
}

export interface MemberResponse {
  id: string
  name: string
  surname: string
  secondSurname: string
  fullName: string
  email: string
  birthdate: string
  phone: string
  notes: string
  genderId: number
  countryId: number

  interests: InterestResponse[]

  createdAt: string
}

export interface MemberCreatePayload {
  name: string
  surname: string
  secondSurname: string | null
  email: string
  birthdate: string | null
  phone: string | null
  notes: string | null
  genderId: number | null
  countryId: number | null

  interestIds: number[]
}

export interface MemberUpdatePayload {
  name: string
  surname: string
  secondSurname: string | null
  email: string
  birthdate: string | null
  phone: string | null
  notes: string | null
  genderId: number | null
  countryId: number | null

  interestIds: number[]
}
