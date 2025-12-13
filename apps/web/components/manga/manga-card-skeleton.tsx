import { cn } from '@/lib/utils';

interface MangaCardSkeletonProps {
    className?: string;
}

export function MangaCardSkeleton({ className }: MangaCardSkeletonProps) {
    return (
        <div
            className={cn(
                'rounded-lg border bg-card overflow-hidden animate-pulse',
                className
            )}
        >
            <div className="aspect-3/4 bg-muted" />
            <div className="p-3 space-y-2">
                <div className="h-4 bg-muted rounded w-3/4" />
                <div className="h-3 bg-muted rounded w-1/2" />
            </div>
        </div>
    );
}

