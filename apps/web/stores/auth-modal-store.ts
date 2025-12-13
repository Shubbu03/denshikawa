import { create } from 'zustand';

type AuthTab = 'login' | 'register' | 'forgot-password';

interface AuthModalState {
    isOpen: boolean;
    activeTab: AuthTab;
    open: (tab?: AuthTab) => void;
    close: () => void;
    setTab: (tab: AuthTab) => void;
}

export const useAuthModalStore = create<AuthModalState>((set) => ({
    isOpen: false,
    activeTab: 'login',
    open: (tab = 'login') => set({ isOpen: true, activeTab: tab }),
    close: () => set({ isOpen: false }),
    setTab: (tab) => set({ activeTab: tab }),
}));

