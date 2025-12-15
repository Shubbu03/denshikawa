import type { Metadata, Viewport } from 'next';
import { Geist, Geist_Mono } from 'next/font/google';
import { Providers } from './providers';
import { Toaster } from '@/components/ui/sonner';
import { AuthModal } from '@/components/auth/auth-modal';
import '../globals.css';

const geistSans = Geist({
    variable: '--font-geist-sans',
    subsets: ['latin'],
});

const geistMono = Geist_Mono({
    variable: '--font-geist-mono',
    subsets: ['latin'],
});

const metadataBase = process.env.NEXT_PUBLIC_SITE_URL
    ? new URL(process.env.NEXT_PUBLIC_SITE_URL)
    : new URL('http://localhost:3000');

export const metadata: Metadata = {
    metadataBase,
    title: 'Denshikawa',
    description: 'Discover and read your favorite manga online',
    keywords: [
        'manga',
        'read manga',
        'online manga',
        'denshikawa',
        'manga reader',
    ],
    applicationName: 'Denshikawa',
    authors: [{ name: 'Denshikawa' }],
    category: 'Entertainment',
    icons: {
        icon: '/favicon.ico',
    },
    openGraph: {
        type: 'website',
        title: 'Denshikawa',
        description: 'Discover and read your favorite manga online',
        siteName: 'Denshikawa',
        locale: 'en_US',
        images: [
            {
                url: '/denshi-logo.png',
                width: 512,
                height: 512,
                alt: 'Denshikawa',
            },
        ],
    },
    twitter: {
        card: 'summary_large_image',
        title: 'Denshikawa',
        description: 'Discover and read your favorite manga online',
        images: ['/denshi-logo.png'],
    },
    alternates: {
        canonical: '/',
    },
};

export const viewport: Viewport = {
    themeColor: '#0f172a',
};

export default function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="en" suppressHydrationWarning>
            <body
                className={`${geistSans.variable} ${geistMono.variable} antialiased`}
            >
                <Providers>
                    {children}
                    <AuthModal />
                    <Toaster />
                </Providers>
            </body>
        </html>
    );
}

