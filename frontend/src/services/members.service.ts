// services/users.service.ts
import api from '@/api/axios'
import type { PageResponse, PaginatedParams } from '@/types/common.types'
import type { MemberCreatePayload, MemberResponse, MemberUpdatePayload } from '@/types/member.types'

class MembersService {
  async getAll(params?: PaginatedParams): Promise<PageResponse<MemberResponse>> {
    const { data } = await api.get<PageResponse<MemberResponse>>('/api/members', { params })
    return data
  }

  async getById(id: string): Promise<MemberResponse> {
    const { data } = await api.get<MemberResponse>(`/api/members/${id}`)
    return data
  }

  async create(payload: MemberCreatePayload): Promise<MemberResponse> {
    const { data } = await api.post<MemberResponse>('/api/members', payload)
    return data
  }

  async update(id: string, payload: MemberUpdatePayload): Promise<MemberResponse> {
    const { data } = await api.put<MemberResponse>(`/api/members/${id}`, payload)
    return data
  }
}

export const membersService = new MembersService()
