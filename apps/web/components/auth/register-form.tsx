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
    FormDescription,
} from '@/components/ui/form';
import { registerRequestSchema, type RegisterRequest } from '@/lib/schemas/auth';
import { useAuth } from '@/hooks/use-auth';
import { useAuthModalStore } from '@/stores/auth-modal-store';
import { Loader2 } from 'lucide-react';

export function RegisterForm() {
    const { register: registerUser, isRegistering } = useAuth();
    const { setTab } = useAuthModalStore();

    const form = useForm<RegisterRequest>({
        resolver: zodResolver(registerRequestSchema),
        defaultValues: {
            email: '',
            username: '',
            password: '',
        },
    });

    const onSubmit = (data: RegisterRequest) => {
        registerUser(data);
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
                    name="username"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel>Username</FormLabel>
                            <FormControl>
                                <Input
                                    placeholder="johndoe"
                                    autoComplete="username"
                                    {...field}
                                />
                            </FormControl>
                            <FormDescription>
                                3-30 characters, letters, numbers, and underscores only
                            </FormDescription>
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
                                    autoComplete="new-password"
                                    {...field}
                                />
                            </FormControl>
                            <FormDescription>
                                At least 8 characters with uppercase, lowercase, and a number
                            </FormDescription>
                            <FormMessage />
                        </FormItem>
                    )}
                />

                <Button type="submit" className="w-full" disabled={isRegistering}>
                    {isRegistering ? (
                        <>
                            <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                            Creating account...
                        </>
                    ) : (
                        'Create account'
                    )}
                </Button>

                <div className="text-center text-sm text-muted-foreground">
                    Already have an account?{' '}
                    <button
                        type="button"
                        onClick={() => setTab('login')}
                        className="text-primary hover:underline"
                    >
                        Log in
                    </button>
                </div>
            </form>
        </Form>
    );
}

