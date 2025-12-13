import { z } from 'zod';

export const loginRequestSchema = z.object({
    email: z.string().email('Invalid email format'),
    password: z.string().min(8, 'Password must be at least 8 characters'),
});

export const registerRequestSchema = z.object({
    email: z.string().email('Invalid email format'),
    username: z.string()
        .min(3, 'Username must be at least 3 characters')
        .max(30, 'Username must be at most 30 characters')
        .regex(/^[a-zA-Z0-9_]+$/, 'Username can only contain letters, numbers, and underscores'),
    password: z.string()
        .min(8, 'Password must be at least 8 characters')
        .regex(/[A-Z]/, 'Password must contain at least one uppercase letter')
        .regex(/[a-z]/, 'Password must contain at least one lowercase letter')
        .regex(/[0-9]/, 'Password must contain at least one number'),
});

export const refreshTokenRequestSchema = z.object({
    refresh_token: z.string(),
});

export const tokenResponseSchema = z.object({
    access_token: z.string(),
    refresh_token: z.string(),
    expires_in: z.number(),
});

export const userResponseSchema = z.object({
    id: z.string(),
    email: z.string(),
    username: z.string(),
    role: z.string(),
    created_at: z.string(),
});

export const loginResponseSchema = z.object({
    user: userResponseSchema,
    tokens: tokenResponseSchema,
});

export const registerResponseSchema = z.object({
    user: userResponseSchema,
    tokens: tokenResponseSchema,
});

export const googleAuthRequestSchema = z.object({
    id_token: z.string(),
});

export type LoginRequest = z.infer<typeof loginRequestSchema>;
export type RegisterRequest = z.infer<typeof registerRequestSchema>;
export type RefreshTokenRequest = z.infer<typeof refreshTokenRequestSchema>;
export type TokenResponse = z.infer<typeof tokenResponseSchema>;
export type UserResponse = z.infer<typeof userResponseSchema>;
export type LoginResponse = z.infer<typeof loginResponseSchema>;
export type RegisterResponse = z.infer<typeof registerResponseSchema>;
export type GoogleAuthRequest = z.infer<typeof googleAuthRequestSchema>;

