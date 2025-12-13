'use client';

import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import {
    Form,
    FormControl,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from '@/components/ui/form';
import { loginRequestSchema, type LoginRequest } from '@/lib/schemas/auth';
import { useAuth } from '@/hooks/use-auth';
import { useAuthModalStore } from '@/stores/auth-modal-store';
import { Loader2 } from 'lucide-react';

export function LoginForm() {
    const { login, isLoggingIn } = useAuth();
    const { setTab } = useAuthModalStore();

    const form = useForm<LoginRequest>({
        resolver: zodResolver(loginRequestSchema),
        defaultValues: {
            email: '',
            password: '',
        },
    });

    const onSubmit = (data: LoginRequest) => {
        login(data);
    };

    return (
        <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
                <FormField
                    control={form.control}
                    name="email"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel>Email</FormLabel>
                            <FormControl>
                                <Input
                                    type="email"
                                    placeholder="you@example.com"
                                    autoComplete="email"
                                    {...field}
                                />
                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    )}
                />

                <FormField
                    control={form.control}
                    name="password"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel>Password</FormLabel>
                            <FormControl>
                                <Input
                                    type="password"
                                    placeholder="••••••••"
                                    autoComplete="current-password"
                                    {...field}
                                />
                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    )}
                />

                <Button type="submit" className="w-full" disabled={isLoggingIn}>
                    {isLoggingIn ? (
                        <>
                            <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                            Logging in...
                        </>
                    ) : (
                        'Log in'
                    )}
                </Button>

                <div className="text-center text-sm text-muted-foreground">
                    Don't have an account?{' '}
                    <button
                        type="button"
                        onClick={() => setTab('register')}
                        className="text-primary hover:underline"
                    >
                        Sign up
                    </button>
                </div>
            </form>
        </Form>
    );
}

