import api from '@/api/axios'
import type { CountryResponse } from '@/types/country.types'

class CountriesService {
  async getAll(): Promise<CountryResponse[]> {
    const { data } = await api.get<CountryResponse[]>('/api/countries')
    return data
  }
}

export const countriesService = new CountriesService()
