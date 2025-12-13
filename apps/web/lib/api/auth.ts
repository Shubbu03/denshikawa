import { apiClient } from './client';
import { ENDPOINTS } from './endpoints';
import {
    LoginRequest,
    RegisterRequest,
    RefreshTokenRequest,
    LoginResponse,
    RegisterResponse,
    TokenResponse,
    loginRequestSchema,
    registerRequestSchema,
    refreshTokenRequestSchema,
    loginResponseSchema,
    registerResponseSchema,
    tokenResponseSchema,
    GoogleAuthRequest,
    googleAuthRequestSchema,
} from '@/lib/schemas/auth';

export const authApi = {
    login: async (credentials: LoginRequest) => {
        const validated = loginRequestSchema.parse(credentials);
        const { data } = await apiClient.post<LoginResponse>(ENDPOINTS.AUTH.LOGIN, validated);
        return loginResponseSchema.parse(data);
    },

    register: async (credentials: RegisterRequest) => {
        const validated = registerRequestSchema.parse(credentials);
        const { data } = await apiClient.post<RegisterResponse>(ENDPOINTS.AUTH.REGISTER, validated);
        return registerResponseSchema.parse(data);
    },

    refresh: async (refreshToken: string) => {
        const validated = refreshTokenRequestSchema.parse({ refresh_token: refreshToken });
        const { data } = await apiClient.post<TokenResponse>(ENDPOINTS.AUTH.REFRESH, validated);
        return tokenResponseSchema.parse(data);
    },

    logout: async () => {
        await apiClient.post(ENDPOINTS.AUTH.LOGOUT);
    },

    loginWithGoogle: async (idToken: string) => {
        const validated = googleAuthRequestSchema.parse({ id_token: idToken });
        const { data } = await apiClient.post<LoginResponse>(ENDPOINTS.AUTH.GOOGLE, validated);
        return loginResponseSchema.parse(data);
    },
};

