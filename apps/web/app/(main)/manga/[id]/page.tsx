'use client';

import { useParams } from 'next/navigation';
import { useMangaDetails, useMangaChapters } from '@/hooks/use-manga';
import { MangaCardSkeleton } from '@/components/manga/manga-card-skeleton';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import { Loader2, BookOpen, Calendar, Users, Tag, Bookmark, BookmarkCheck } from 'lucide-react';
import Link from 'next/link';
import { ProtectedAction } from '@/components/shared/protected-action';
import { useAuth } from '@/hooks/use-auth';
import Image from 'next/image';
import { formatDate } from '@/lib/utils/format';

export default function MangaDetailsPage() {
    const params = useParams();
    const mangaId = params.id as string;
    const { isAuthenticated } = useAuth();

    const { data: manga, isLoading: isLoadingManga } = useMangaDetails(mangaId);
    const { data: chapters, isLoading: isLoadingChapters } = useMangaChapters(mangaId, 'en');

    if (isLoadingManga) {
        return (
            <div className="container mx-auto px-4 py-6">
                <div className="flex flex-col md:flex-row gap-6">
                    <div className="w-full md:w-64 shrink-0">
                        <MangaCardSkeleton className="w-full" />
                    </div>
                    <div className="flex-1 space-y-4">
                        <div className="h-8 bg-muted rounded w-3/4 animate-pulse" />
                        <div className="h-4 bg-muted rounded w-1/2 animate-pulse" />
                        <div className="space-y-2">
                            {Array.from({ length: 5 }).map((_, i) => (
                                <div key={i} className="h-4 bg-muted rounded animate-pulse" />
                            ))}
                        </div>
                    </div>
                </div>
            </div>
        );
    }

    if (!manga) {
        return (
            <div className="container mx-auto px-4 py-6">
                <div className="flex flex-col items-center justify-center py-12 text-center">
                    <BookOpen className="h-12 w-12 text-muted-foreground mb-4" />
                    <p className="text-lg font-medium mb-2">Manga not found</p>
                    <p className="text-muted-foreground mb-4">The manga you're looking for doesn't exist.</p>
                    <Button asChild variant="outline">
                        <Link href="/">Go Home</Link>
                    </Button>
                </div>
            </div>
        );
    }

    const sortedChapters = [...(chapters || [])].sort((a, b) => {
        const numA = a.chapter_number ? parseFloat(a.chapter_number) : 0;
        const numB = b.chapter_number ? parseFloat(b.chapter_number) : 0;
        return numB - numA;
    });

    return (
        <div className="container mx-auto px-4 py-6 space-y-6">
            {/* Header Section */}
            <div className="flex flex-col md:flex-row gap-6">
                {/* Cover Image */}
                <div className="w-full md:w-64 shrink-0">
                    <div className="aspect-3/4 relative overflow-hidden rounded-lg border bg-muted">
                        {manga.cover_url ? (
                            <Image
                                src={manga.cover_url}
                                alt={manga.title}
                                fill
                                unoptimized
                                priority
                                className="object-cover"
                                sizes="(max-width: 768px) 100vw, 256px"
                            />
                        ) : (
                            <div className="flex items-center justify-center h-full text-muted-foreground">
                                No Cover
                            </div>
                        )}
                    </div>
                </div>

                <div className="flex-1 space-y-4">
                    <div>
                        <h1 className="text-3xl md:text-4xl font-bold tracking-tight mb-2">{manga.title}</h1>
                        {manga.alt_titles.length > 0 && (
                            <p className="text-muted-foreground">{manga.alt_titles[0]}</p>
                        )}
                    </div>

                    <div className="flex flex-wrap gap-2">
                        <Badge variant="secondary" className="gap-1">
                            <Tag className="h-3 w-3" />
                            {manga.status}
                        </Badge>
                        {manga.year && (
                            <Badge variant="secondary" className="gap-1">
                                <Calendar className="h-3 w-3" />
                                {manga.year}
                            </Badge>
                        )}
                        <Badge variant="outline">{manga.content_rating}</Badge>
                    </div>

                    {(manga.author_names.length > 0 || manga.artist_names.length > 0) && (
                        <div className="space-y-1">
                            {manga.author_names.length > 0 && (
                                <div className="flex items-center gap-2 text-sm">
                                    <Users className="h-4 w-4 text-muted-foreground" />
                                    <span className="text-muted-foreground">Authors:</span>
                                    <span>{manga.author_names.join(', ')}</span>
                                </div>
                            )}
                            {manga.artist_names.length > 0 && (
                                <div className="flex items-center gap-2 text-sm">
                                    <Users className="h-4 w-4 text-muted-foreground" />
                                    <span className="text-muted-foreground">Artists:</span>
                                    <span>{manga.artist_names.join(', ')}</span>
                                </div>
                            )}
                        </div>
                    )}

                    <div className="flex flex-wrap gap-2 pt-2">
                        <ProtectedAction action={() => { }}>
                            <Button variant="default" size="sm" className="gap-2">
                                {isAuthenticated ? (
                                    <>
                                        <BookmarkCheck className="h-4 w-4" />
                                        Bookmarked
                                    </>
                                ) : (
                                    <>
                                        <Bookmark className="h-4 w-4" />
                                        Bookmark
                                    </>
                                )}
                            </Button>
                        </ProtectedAction>
                        {sortedChapters.length > 0 && (
                            <Button asChild variant="outline" size="sm" className="gap-2">
                                <Link href={`/manga/${mangaId}/chapters/${sortedChapters[0].mangadex_id}`}>
                                    <BookOpen className="h-4 w-4" />
                                    Start Reading
                                </Link>
                            </Button>
                        )}
                    </div>

                    {manga.description && (
                        <div className="space-y-2">
                            <h2 className="text-lg font-semibold">Description</h2>
                            <p className="text-sm text-muted-foreground leading-relaxed whitespace-pre-line">
                                {manga.description}
                            </p>
                        </div>
                    )}

                    {manga.tags.length > 0 && (
                        <div className="space-y-2">
                            <h2 className="text-lg font-semibold">Tags</h2>
                            <div className="flex flex-wrap gap-2">
                                {manga.tags.map((tag) => (
                                    <Badge key={tag.id} variant="outline">
                                        {tag.name}
                                    </Badge>
                                ))}
                            </div>
                        </div>
                    )}
                </div>
            </div>

            <Separator />

            <div className="space-y-4">
                <div className="flex items-center justify-between">
                    <h2 className="text-2xl font-semibold">Chapters</h2>
                    {sortedChapters.length > 0 && (
                        <p className="text-sm text-muted-foreground">
                            {sortedChapters.length} chapter{sortedChapters.length !== 1 ? 's' : ''}
                        </p>
                    )}
                </div>

                {isLoadingChapters ? (
                    <div className="space-y-2">
                        {Array.from({ length: 5 }).map((_, i) => (
                            <div key={i} className="h-16 bg-muted rounded animate-pulse" />
                        ))}
                    </div>
                ) : sortedChapters.length === 0 ? (
                    <div className="flex flex-col items-center justify-center py-12 text-center">
                        <BookOpen className="h-12 w-12 text-muted-foreground mb-4" />
                        <p className="text-muted-foreground">No chapters available</p>
                    </div>
                ) : (
                    <div className="space-y-2">
                        {sortedChapters.map((chapter) => (
                            <Link
                                key={chapter.mangadex_id}
                                href={`/manga/${mangaId}/chapters/${chapter.mangadex_id}`}
                                className="flex items-center justify-between p-4 rounded-lg border bg-card hover:bg-accent transition-colors group"
                            >
                                <div className="flex-1 min-w-0">
                                    <div className="flex items-center gap-2 mb-1">
                                        <span className="font-medium">
                                            {chapter.chapter_number
                                                ? `Chapter ${chapter.chapter_number}`
                                                : 'Oneshot'}
                                        </span>
                                        {chapter.volume && (
                                            <Badge variant="outline" className="text-xs">
                                                Vol. {chapter.volume}
                                            </Badge>
                                        )}
                                    </div>
                                    {chapter.title && (
                                        <p className="text-sm text-muted-foreground truncate">{chapter.title}</p>
                                    )}
                                    <div className="flex items-center gap-4 mt-2 text-xs text-muted-foreground">
                                        {chapter.page_count && (
                                            <span>{chapter.page_count} pages</span>
                                        )}
                                        {chapter.published_at && (
                                            <span>{formatDate(chapter.published_at)}</span>
                                        )}
                                        {chapter.scanlation_group_name && (
                                            <span>{chapter.scanlation_group_name}</span>
                                        )}
                                    </div>
                                </div>
                                <BookOpen className="h-5 w-5 text-muted-foreground group-hover:text-foreground transition-colors shrink-0 ml-4" />
                            </Link>
                        ))}
                    </div>
                )}
            </div>
        </div>
    );
}

