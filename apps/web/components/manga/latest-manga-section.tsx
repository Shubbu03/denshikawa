'use client';

import { useLatestManga } from '@/hooks/use-manga';
import { MangaCard } from './manga-card';
import { MangaCardSkeleton } from './manga-card-skeleton';
import { Button } from '@/components/ui/button';
import { Loader2 } from 'lucide-react';

export function LatestMangaSection() {
    const { data, fetchNextPage, hasNextPage, isFetchingNextPage, isLoading } = useLatestManga();

    if (isLoading) {
        return (
            <section className="space-y-4">
                <h2 className="text-2xl font-semibold">Latest Updates</h2>
                <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-4">
                    {Array.from({ length: 12 }).map((_, i) => (
                        <MangaCardSkeleton key={i} />
                    ))}
                </div>
            </section>
        );
    }

    const allManga = data?.pages.flatMap((page) => page.data) ?? [];
    const initialLoadCount = data?.pages[0]?.data.length ?? 0;

    return (
        <section className="space-y-4">
            <h2 className="text-2xl font-semibold">Latest Updates</h2>
            <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-4">
                {allManga.map((manga, index) => (
                    <MangaCard
                        key={manga.id}
                        manga={manga}
                        priority={index < initialLoadCount}
                    />
                ))}
                {isFetchingNextPage && (
                    <>
                        {Array.from({ length: 6 }).map((_, i) => (
                            <MangaCardSkeleton key={`skeleton-${i}`} />
                        ))}
                    </>
                )}
            </div>
            {hasNextPage && (
                <div className="flex justify-center pt-4">
                    <Button
                        onClick={() => fetchNextPage()}
                        disabled={isFetchingNextPage}
                        variant="outline"
                    >
                        {isFetchingNextPage ? (
                            <>
                                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                                Loading...
                            </>
                        ) : (
                            'Load More'
                        )}
                    </Button>
                </div>
            )}
        </section>
    );
}

