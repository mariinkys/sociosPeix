import api from '@/api/axios'
import type { PageResponse, PaginatedParams } from '@/types/common.types'
import type { UserResponse } from '@/types/user.types'

class UsersService {
  async getAll(params?: PaginatedParams): Promise<PageResponse<UserResponse>> {
    const { data } = await api.get<PageResponse<UserResponse>>('/api/users', { params })
    return data
  }
}

export const usersService = new UsersService()
