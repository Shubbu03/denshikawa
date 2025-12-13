'use client';

import { useState, useEffect, useMemo, Suspense } from 'react';
import { useSearchParams, useRouter } from 'next/navigation';
import { useMangaSearch } from '@/hooks/use-manga';
import { MangaCard } from '@/components/manga/manga-card';
import { MangaCardSkeleton } from '@/components/manga/manga-card-skeleton';
import { Button } from '@/components/ui/button';
import { Loader2, Search as SearchIcon } from 'lucide-react';
import { useDebounce } from '@/hooks/use-debounce';

function SearchContent() {
    const searchParams = useSearchParams();
    const router = useRouter();
    const [searchQuery, setSearchQuery] = useState(searchParams.get('q') || '');
    const debouncedQuery = useDebounce(searchQuery, 500);

    const { data, fetchNextPage, hasNextPage, isFetchingNextPage, isLoading } = useMangaSearch(
        debouncedQuery,
        debouncedQuery.length > 0
    );

    useEffect(() => {
        if (debouncedQuery) {
            router.replace(`/search?q=${encodeURIComponent(debouncedQuery)}`, { scroll: false });
        } else if (searchQuery === '') {
            router.replace('/search', { scroll: false });
        }
    }, [debouncedQuery, router, searchQuery]);

    const allManga = useMemo(() => {
        return data?.pages.flatMap((page) => page.data) ?? [];
    }, [data]);

    const handleSearch = (e: React.FormEvent) => {
        e.preventDefault();
    };

    return (
        <div className="container mx-auto px-4 py-6 space-y-6">
            <div className="space-y-2">
                <h1 className="text-3xl font-bold tracking-tight">Search Manga</h1>
                <p className="text-muted-foreground">Find your favorite manga</p>
            </div>

            <form onSubmit={handleSearch} className="w-full max-w-2xl">
                <div className="relative">
                    <SearchIcon className="absolute left-3 top-1/2 -translate-y-1/2 h-5 w-5 text-muted-foreground" />
                    <input
                        type="search"
                        placeholder="Search manga by title..."
                        value={searchQuery}
                        onChange={(e) => setSearchQuery(e.target.value)}
                        className="w-full pl-10 pr-4 py-3 rounded-lg border bg-background text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                    />
                </div>
            </form>

            {/* Results */}
            {debouncedQuery.length === 0 ? (
                <div className="flex flex-col items-center justify-center py-12 text-center">
                    <SearchIcon className="h-12 w-12 text-muted-foreground mb-4" />
                    <p className="text-muted-foreground">Enter a search query to find manga</p>
                </div>
            ) : isLoading ? (
                <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-4">
                    {Array.from({ length: 12 }).map((_, i) => (
                        <MangaCardSkeleton key={i} />
                    ))}
                </div>
            ) : allManga.length === 0 ? (
                <div className="flex flex-col items-center justify-center py-12 text-center">
                    <SearchIcon className="h-12 w-12 text-muted-foreground mb-4" />
                    <p className="text-lg font-medium mb-2">No results found</p>
                    <p className="text-muted-foreground">Try a different search term</p>
                </div>
            ) : (
                <>
                    <div className="flex items-center justify-between">
                        <p className="text-sm text-muted-foreground">
                            Found {data?.pages[0]?.total ?? 0} results for &quot;{debouncedQuery}&quot;
                        </p>
                    </div>
                    <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-4">
                        {allManga.map((manga, index) => (
                            <MangaCard key={manga.id} manga={manga} priority={index < 12} />
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
                </>
            )}
        </div>
    );
}

export default function SearchPage() {
    return (
        <Suspense fallback={
            <div className="container mx-auto px-4 py-6">
                <div className="space-y-2">
                    <h1 className="text-3xl font-bold tracking-tight">Search Manga</h1>
                    <p className="text-muted-foreground">Find your favorite manga</p>
                </div>
                <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-4 mt-6">
                    {Array.from({ length: 12 }).map((_, i) => (
                        <MangaCardSkeleton key={i} />
                    ))}
                </div>
            </div>
        }>
            <SearchContent />
        </Suspense>
    );
}
