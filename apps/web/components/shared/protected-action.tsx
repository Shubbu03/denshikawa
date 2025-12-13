'use client';

import { ReactNode } from 'react';
import { useAuth } from '@/hooks/use-auth';
import { useAuthModalStore } from '@/stores/auth-modal-store';

interface ProtectedActionProps {
    children: ReactNode;
    action: () => void | Promise<void>;
    requireAuth?: boolean;
}

export function ProtectedAction({
    children,
    action,
    requireAuth = true,
}: ProtectedActionProps) {
    const { isAuthenticated } = useAuth();
    const { open: openAuthModal } = useAuthModalStore();

    const handleClick = async () => {
        if (requireAuth && !isAuthenticated) {
            openAuthModal('login');
            return;
        }
        await action();
    };

    return (
        <div onClick={handleClick} className="cursor-pointer">
            {children}
        </div>
    );
}

