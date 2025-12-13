'use client';

import Link from 'next/link';
import { useRouter } from 'next/navigation';
import { useState, FormEvent } from 'react';
import { Button } from '@/components/ui/button';
import { useAuth } from '@/hooks/use-auth';
import { useAuthModalStore } from '@/stores/auth-modal-store';
import { Search, User, LogOut } from 'lucide-react';
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';

export function Header() {
    const { isAuthenticated, user, logout } = useAuth();
    const { open: openAuthModal } = useAuthModalStore();
    const router = useRouter();
    const [searchQuery, setSearchQuery] = useState('');

    const handleSearch = (e: FormEvent) => {
        e.preventDefault();
        if (searchQuery.trim()) {
            router.push(`/search?q=${encodeURIComponent(searchQuery.trim())}`);
        }
    };

    return (
        <header className="sticky top-0 z-40 w-full border-b bg-background/95 backdrop-blur supports-backdrop-filter:bg-background/60">
            <div className="container flex h-14 items-center justify-between px-4">
                <Link href="/" className="flex items-center space-x-2">
                    <span className="text-xl font-bold">Denshikawa</span>
                </Link>


                <form onSubmit={handleSearch} className="hidden md:flex flex-1 max-w-md mx-4">
                    <div className="relative w-full">
                        <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
                        <input
                            type="search"
                            placeholder="Search manga..."
                            value={searchQuery}
                            onChange={(e) => setSearchQuery(e.target.value)}
                            className="w-full pl-10 pr-4 py-2 rounded-md border bg-background text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        />
                    </div>
                </form>

                <div className="flex items-center gap-2">
                    <Button
                        variant="ghost"
                        size="icon"
                        className="md:hidden"
                        aria-label="Search"
                        onClick={() => router.push('/search')}
                    >
                        <Search className="h-5 w-5" />
                    </Button>

                    {isAuthenticated ? (
                        <DropdownMenu>
                            <DropdownMenuTrigger asChild>
                                <Button variant="ghost" size="icon" className="relative">
                                    <User className="h-5 w-5" />
                                    <span className="sr-only">User menu</span>
                                </Button>
                            </DropdownMenuTrigger>
                            <DropdownMenuContent align="end" className="w-56">
                                <DropdownMenuLabel>
                                    <div className="flex flex-col space-y-1">
                                        <p className="text-sm font-medium">{user?.username}</p>
                                        <p className="text-xs text-muted-foreground">{user?.email}</p>
                                    </div>
                                </DropdownMenuLabel>
                                <DropdownMenuSeparator />
                                <DropdownMenuItem asChild>
                                    <Link href="/library">My Library</Link>
                                </DropdownMenuItem>
                                <DropdownMenuItem asChild>
                                    <Link href="/history">Reading History</Link>
                                </DropdownMenuItem>
                                <DropdownMenuSeparator />
                                <DropdownMenuItem onClick={() => logout()}>
                                    <LogOut className="mr-2 h-4 w-4" />
                                    Log out
                                </DropdownMenuItem>
                            </DropdownMenuContent>
                        </DropdownMenu>
                    ) : (
                        <Button
                            onClick={() => openAuthModal('login')}
                            variant="default"
                            size="sm"
                            className="gap-2"
                        >
                            <User className="h-4 w-4" />
                            <span className="hidden sm:inline">Sign in</span>
                        </Button>
                    )}
                </div>
            </div>
        </header>
    );
}

