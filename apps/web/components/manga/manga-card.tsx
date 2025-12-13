'use client';

import Link from 'next/link';
import Image from 'next/image';
import { MangaSummary } from '@/lib/schemas/manga';
import { cn } from '@/lib/utils';

interface MangaCardProps {
    manga: MangaSummary;
    className?: string;
    priority?: boolean;
}

export function MangaCard({ manga, className, priority = false }: MangaCardProps) {
    return (
        <Link
            href={`/manga/${manga.mangadex_id}`}
            className={cn(
                'group relative block overflow-hidden rounded-lg border bg-card transition-all hover:shadow-md',
                className
            )}
        >
            <div className="aspect-3/4 relative overflow-hidden bg-muted">
                {manga.cover_url ? (
                    <Image
                        src={manga.cover_url}
                        alt={manga.title}
                        fill
                        unoptimized
                        priority={priority}
                        loading={priority ? undefined : 'lazy'}
                        className="object-cover transition-transform group-hover:scale-105"
                        sizes="(max-width: 640px) 50vw, (max-width: 1024px) 33vw, 20vw"
                    />
                ) : (
                    <div className="flex items-center justify-center h-full text-muted-foreground">
                        No Cover
                    </div>
                )}
            </div>
            <div className="p-3 space-y-1">
                <h3 className="font-medium text-sm line-clamp-2 group-hover:text-primary transition-colors">
                    {manga.title}
                </h3>
                {manga.status && (
                    <p className="text-xs text-muted-foreground capitalize">{manga.status}</p>
                )}
            </div>
        </Link>
    );
}

