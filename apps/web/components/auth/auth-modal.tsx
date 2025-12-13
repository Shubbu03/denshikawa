'use client';

import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
} from '@/components/ui/dialog';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { LoginForm } from './login-form';
import { RegisterForm } from './register-form';
import { useAuthModalStore } from '@/stores/auth-modal-store';
import { ArrowRight, User, Lock } from 'lucide-react';

export function AuthModal() {
    const { isOpen, activeTab, close, setTab } = useAuthModalStore();

    return (
        <Dialog open={isOpen} onOpenChange={(open) => !open && close()}>
            <DialogContent className="sm:max-w-md">
                <DialogHeader>
                    <DialogTitle className="sr-only">Authentication</DialogTitle>
                </DialogHeader>
                <Tabs
                    value={activeTab}
                    onValueChange={(value) => setTab(value as typeof activeTab)}
                    className="w-full"
                >
                    <TabsList className="grid w-full grid-cols-3">
                        <TabsTrigger value="login" className="gap-1.5">
                            <ArrowRight className="h-4 w-4" />
                            <span className="hidden sm:inline">Login</span>
                        </TabsTrigger>
                        <TabsTrigger value="register" className="gap-1.5">
                            <User className="h-4 w-4" />
                            <span className="hidden sm:inline">Register</span>
                        </TabsTrigger>
                        <TabsTrigger value="forgot-password" className="gap-1.5">
                            <Lock className="h-4 w-4" />
                            <span className="hidden sm:inline">Forgot Password</span>
                        </TabsTrigger>
                    </TabsList>
                    <TabsContent value="login" className="mt-4">
                        <LoginForm />
                    </TabsContent>
                    <TabsContent value="register" className="mt-4">
                        <RegisterForm />
                    </TabsContent>
                    <TabsContent value="forgot-password" className="mt-4">
                        <div className="space-y-4 text-center py-8">
                            <p className="text-muted-foreground">
                                Forgot password functionality coming soon.
                            </p>
                        </div>
                    </TabsContent>
                </Tabs>
            </DialogContent>
        </Dialog>
    );
}

