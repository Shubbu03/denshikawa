import { apiClient } from './client';
import { ENDPOINTS } from './endpoints';
import { UserResponse, userResponseSchema } from '@/lib/schemas/auth';

export const userApi = {
    getMe: async () => {
        const { data } = await apiClient.get<UserResponse>(ENDPOINTS.USER.ME);
        return userResponseSchema.parse(data);
    },
};

