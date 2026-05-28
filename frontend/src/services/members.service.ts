// services/users.service.ts
import api from '@/api/axios'
import type { PageResponse, PaginatedParams } from '@/types/common.types'
import type { MemberResponse } from '@/types/member.types'

class MembersService {
  async getAll(params?: PaginatedParams): Promise<PageResponse<MemberResponse>> {
    const { data } = await api.get<PageResponse<MemberResponse>>('/api/members', { params })
    return data
  }
}

export const membersService = new MembersService()
