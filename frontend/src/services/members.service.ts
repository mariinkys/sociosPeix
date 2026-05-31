import api from '@/api/axios'
import type { PageResponse } from '@/types/common.types'
import type {
  MemberCreatePayload,
  MemberParams,
  MemberResponse,
  MemberUpdatePayload,
} from '@/types/member.types'

class MembersService {
  async getAll(params?: MemberParams): Promise<PageResponse<MemberResponse>> {
    const { interestIds, ...rest } = params ?? {}
    const { data } = await api.get<PageResponse<MemberResponse>>('/api/members', {
      params: rest,
      paramsSerializer: (p) => {
        const search = new URLSearchParams()
        Object.entries(p).forEach(([key, val]) => {
          if (val !== undefined && val !== null) search.append(key, String(val))
        })
        interestIds?.forEach((id) => search.append('interestIds', String(id)))
        return search.toString()
      },
    })
    return data
  }

  async export(search?: string, interestIds?: number[]): Promise<void> {
    const response = await api.get('/api/members/export', {
      params: { search: search || undefined, interestIds },
      responseType: 'blob',
    })
    const url = URL.createObjectURL(response.data)
    const a = document.createElement('a')
    a.href = url
    a.download = 'members.xlsx'
    a.click()
    URL.revokeObjectURL(url)
  }

  async getToday(): Promise<MemberResponse[]> {
    const { data } = await api.get<MemberResponse[]>('/api/members/birthdays/today')
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

  async delete(id: string): Promise<void> {
    const { data } = await api.delete<void>(`/api/members/${id}`)
    return data
  }
}

export const membersService = new MembersService()
