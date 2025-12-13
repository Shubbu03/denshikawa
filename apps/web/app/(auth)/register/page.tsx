import { Metadata } from 'next';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { RegisterForm } from '@/components/auth/register-form';
import { AuthGuard } from '@/components/auth/auth-guard';

export const metadata: Metadata = {
    title: 'Register | Denshikawa',
    description: 'Create a new Denshikawa account',
};

export default function RegisterPage() {
    return (
        <AuthGuard requireAuth={false}>
            <Card>
                <CardHeader className="space-y-1">
                    <CardTitle className="text-2xl font-bold">Create an account</CardTitle>
                    <CardDescription>Enter your details to get started</CardDescription>
                </CardHeader>
                <CardContent>
                    <RegisterForm />
                </CardContent>
            </Card>
        </AuthGuard>
    );
}

