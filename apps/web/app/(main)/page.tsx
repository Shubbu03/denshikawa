import { Metadata } from 'next';
import { PopularMangaSection } from '@/components/manga/popular-manga-section';
import { LatestMangaSection } from '@/components/manga/latest-manga-section';

export const metadata: Metadata = {
    title: 'Home | Denshikawa',
    description: 'Discover and read manga online',
};

export default function HomePage() {
    return (
        <div className="container mx-auto px-4 py-6 space-y-8">
            <div className="space-y-2">
                <h1 className="text-3xl font-bold tracking-tight">Welcome to Denshikawa</h1>
                <p className="text-muted-foreground">
                    Discover and read your favorite manga online
                </p>
            </div>

            <PopularMangaSection />
            <LatestMangaSection />
        </div>
    );
}
