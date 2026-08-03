import api from '@/api/axios'
import type { FormOcrPage, FormOcrResponse } from '@/types/formOcr.types'

class FormOcrService {
  async analyze(file: File): Promise<FormOcrPage[]> {
    const formData = new FormData()
    formData.append('file', file)

    const { data } = await api.post<FormOcrResponse>('/api/formOcr/analyze', formData, {
      headers: { 'Content-Type': 'multipart/form-data' },
    })

    return data.pages
  }
}

export const formOcrService = new FormOcrService()
