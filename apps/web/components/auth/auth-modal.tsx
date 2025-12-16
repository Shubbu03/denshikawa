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
                    <TabsList className="flex w-full gap-1 p-1 h-auto">
                        <TabsTrigger
                            value="login"
                            className="gap-1.5 px-3 py-2 text-xs sm:text-sm flex-1"
                        >
                            <ArrowRight className="h-3.5 w-3.5 sm:h-4 sm:w-4 shrink-0" />
                            <span className="hidden sm:inline">Login</span>
                        </TabsTrigger>
                        <TabsTrigger
                            value="register"
                            className="gap-1.5 px-3 py-2 text-xs sm:text-sm flex-1"
                        >
                            <User className="h-3.5 w-3.5 sm:h-4 sm:w-4 shrink-0" />
                            <span className="hidden sm:inline">Register</span>
                        </TabsTrigger>
                        <TabsTrigger
                            value="forgot-password"
                            className="gap-1.5 px-2 sm:px-3 py-2 text-xs sm:text-sm flex-1 min-w-0"
                        >
                            <Lock className="h-3.5 w-3.5 sm:h-4 sm:w-4 shrink-0" />
                            <span className="hidden sm:inline truncate">Reset</span>
                        </TabsTrigger>
                    </TabsList>
                    <TabsContent value="login" className="mt-6">
                        <LoginForm />
                    </TabsContent>
                    <TabsContent value="register" className="mt-6">
                        <RegisterForm />
                    </TabsContent>
                    <TabsContent value="forgot-password" className="mt-6">
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

