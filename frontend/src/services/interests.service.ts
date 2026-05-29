import api from '@/api/axios'
import type { InterestPayload, InterestResponse } from '@/types/interest.types'

class InterestsService {
  async getAll(): Promise<InterestResponse[]> {
    const { data } = await api.get<InterestResponse[]>('/api/interests')
    return data
  }

  async getById(id: number): Promise<InterestResponse> {
    const { data } = await api.get<InterestResponse>(`/api/interests/${id}`)
    return data
  }

  async create(payload: InterestPayload): Promise<InterestResponse> {
    const { data } = await api.post<InterestResponse>('/api/interests', payload)
    return data
  }

  async update(id: number, payload: InterestPayload): Promise<InterestResponse> {
    const { data } = await api.put<InterestResponse>(`/api/interests/${id}`, payload)
    return data
  }

  async delete(id: number): Promise<void> {
    await api.delete(`/api/interests/${id}`)
  }
}

export const interestsService = new InterestsService()
