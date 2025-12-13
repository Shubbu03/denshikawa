import { Metadata } from 'next';
import Link from 'next/link';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { LoginForm } from '@/components/auth/login-form';
import { AuthGuard } from '@/components/auth/auth-guard';

export const metadata: Metadata = {
    title: 'Login | Denshikawa',
    description: 'Log in to your Denshikawa account',
};

export default function LoginPage() {
    return (
        <AuthGuard requireAuth={false}>
            <Card>
                <CardHeader className="space-y-1">
                    <CardTitle className="text-2xl font-bold">Welcome back</CardTitle>
                    <CardDescription>Log in to your account to continue</CardDescription>
                </CardHeader>
                <CardContent>
                    <LoginForm />
                </CardContent>
            </Card>
        </AuthGuard>
    );
}

