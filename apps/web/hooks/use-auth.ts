import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { authApi, userApi } from '@/lib/api';
import { useAuthStore } from '@/stores/auth-store';
import { useAuthModalStore } from '@/stores/auth-modal-store';
import { queryKeys } from '@/lib/api/query-keys';
import { LoginRequest, RegisterRequest } from '@/lib/schemas/auth';
import { toast } from 'sonner';

export const useAuth = () => {
    const queryClient = useQueryClient();
    const { user, isAuthenticated, login: setAuthState, logout: clearAuthState } = useAuthStore();
    const { close: closeAuthModal } = useAuthModalStore();

    const loginMutation = useMutation({
        mutationFn: (credentials: LoginRequest) => authApi.login(credentials),
        onSuccess: (data) => {
            setAuthState(
                data.user,
                data.tokens.access_token,
                data.tokens.refresh_token
            );
            queryClient.setQueryData(queryKeys.user.me(), data.user);
            closeAuthModal();
            toast.success('Welcome back!');
        },
        onError: (error: any) => {
            const message = error.response?.data?.message || 'Login failed. Please try again.';
            toast.error(message);
        },
    });

    const registerMutation = useMutation({
        mutationFn: (credentials: RegisterRequest) => authApi.register(credentials),
        onSuccess: (data) => {
            setAuthState(
                data.user,
                data.tokens.access_token,
                data.tokens.refresh_token
            );
            queryClient.setQueryData(queryKeys.user.me(), data.user);
            closeAuthModal();
            toast.success('Account created successfully!');
        },
        onError: (error: any) => {
            const message = error.response?.data?.message || 'Registration failed. Please try again.';
            toast.error(message);
        },
    });

    const logoutMutation = useMutation({
        mutationFn: () => authApi.logout(),
        onSuccess: () => {
            clearAuthState();
            queryClient.clear();
            toast.success('Logged out successfully');
        },
        onError: () => {
            clearAuthState();
            queryClient.clear();
        },
    });

    const { data: currentUser, isLoading: isLoadingUser } = useQuery({
        queryKey: queryKeys.user.me(),
        queryFn: () => userApi.getMe(),
        enabled: isAuthenticated && !!useAuthStore.getState().accessToken,
        retry: false,
    });

    const login = loginMutation.mutate;
    const register = registerMutation.mutate;
    const logout = () => logoutMutation.mutate();

    return {
        user: currentUser || user,
        isAuthenticated,
        isLoading: loginMutation.isPending || registerMutation.isPending || isLoadingUser,
        login,
        register,
        logout,
        isLoggingIn: loginMutation.isPending,
        isRegistering: registerMutation.isPending,
    };
};

