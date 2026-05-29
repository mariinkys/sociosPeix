// services/users.service.ts
import api from '@/api/axios'
import type { GenderResponse } from '@/types/gender.types'

class GendersService {
  async getAll(): Promise<GenderResponse[]> {
    const { data } = await api.get<GenderResponse[]>('/api/genders')
    return data
  }
}

export const gendersService = new GendersService()
