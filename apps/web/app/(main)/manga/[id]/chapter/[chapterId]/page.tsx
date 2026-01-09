'use client';

import { useParams, useRouter } from 'next/navigation';
import { useState, useEffect, useCallback } from 'react';
import { useChapterPages } from '@/hooks/use-manga';
import { Button } from '@/components/ui/button';
import { Loader2, ChevronLeft, ChevronRight, X, Maximize2, Minimize2 } from 'lucide-react';
import Image from 'next/image';
import Link from 'next/link';

export default function ChapterReaderPage() {
    const params = useParams();
    const router = useRouter();
    const mangaId = params.id as string;
    const chapterId = params.chapterId as string;
    const [currentPage, setCurrentPage] = useState(0);
    const [useDataSaver, setUseDataSaver] = useState(false);
    const [isFullscreen, setIsFullscreen] = useState(false);

    const { data: chapterPages, isLoading, error } = useChapterPages(chapterId);

    const totalPages = chapterPages?.pages.length ?? 0;
    const currentPageData = chapterPages?.pages[currentPage];

    // Keyboard navigation
    useEffect(() => {
        const handleKeyPress = (e: KeyboardEvent) => {
            if (e.key === 'ArrowLeft' || e.key === 'ArrowUp') {
                if (currentPage > 0) {
                    setCurrentPage((prev) => prev - 1);
                }
            } else if (e.key === 'ArrowRight' || e.key === 'ArrowDown' || e.key === ' ') {
                e.preventDefault();
                if (currentPage < totalPages - 1) {
                    setCurrentPage((prev) => prev + 1);
                }
            } else if (e.key === 'Escape') {
                setIsFullscreen(false);
            } else if (e.key === 'f' || e.key === 'F') {
                setIsFullscreen((prev) => !prev);
            }
        };

        window.addEventListener('keydown', handleKeyPress);
        return () => window.removeEventListener('keydown', handleKeyPress);
    }, [currentPage, totalPages, isFullscreen]);

    const handleNextPage = useCallback(() => {
        if (currentPage < totalPages - 1) {
            setCurrentPage((prev) => prev + 1);
        }
    }, [currentPage, totalPages]);

    const handlePreviousPage = useCallback(() => {
        if (currentPage > 0) {
            setCurrentPage((prev) => prev - 1);
        }
    }, [currentPage]);

    // Scroll to top when page changes
    useEffect(() => {
        window.scrollTo({ top: 0, behavior: 'smooth' });
    }, [currentPage]);

    if (isLoading) {
        return (
            <div className="flex items-center justify-center min-h-screen">
                <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
            </div>
        );
    }

    if (error || !chapterPages || totalPages === 0) {
        return (
            <div className="container mx-auto px-4 py-6">
                <div className="flex flex-col items-center justify-center py-12 text-center">
                    <p className="text-lg font-medium mb-2">Chapter not found</p>
                    <p className="text-muted-foreground mb-4">Unable to load chapter pages.</p>
                    <Button asChild variant="outline">
                        <Link href={`/manga/${mangaId}`}>Back to Manga</Link>
                    </Button>
                </div>
            </div>
        );
    }

    return (
        <div className={`${isFullscreen ? 'fixed inset-0 z-50 bg-black' : 'min-h-screen bg-background'}`}>
            {/* Header Controls */}
            <div className={`sticky top-0 z-10 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 border-b ${isFullscreen ? 'hidden' : ''}`}>
                <div className="container mx-auto px-4 py-3">
                    <div className="flex items-center justify-between">
                        <div className="flex items-center gap-4">
                            <Button variant="ghost" size="sm" asChild>
                                <Link href={`/manga/${mangaId}`}>
                                    <X className="h-4 w-4 mr-2" />
                                    Close
                                </Link>
                            </Button>
                            <span className="text-sm text-muted-foreground">
                                Page {currentPage + 1} of {totalPages}
                            </span>
                        </div>
                        <div className="flex items-center gap-2">
                            <Button
                                variant="outline"
                                size="sm"
                                onClick={() => setUseDataSaver(!useDataSaver)}
                            >
                                {useDataSaver ? 'High Quality' : 'Data Saver'}
                            </Button>
                            <Button
                                variant="outline"
                                size="sm"
                                onClick={() => setIsFullscreen(!isFullscreen)}
                            >
                                {isFullscreen ? (
                                    <Minimize2 className="h-4 w-4" />
                                ) : (
                                    <Maximize2 className="h-4 w-4" />
                                )}
                            </Button>
                        </div>
                    </div>
                </div>
            </div>

            {/* Page Image */}
            <div className="flex flex-col items-center justify-center min-h-[calc(100vh-80px)] py-4">
                {currentPageData && (
                    <div className="relative w-full max-w-4xl mx-auto">
                        <Image
                            src={useDataSaver ? currentPageData.url_data_saver : currentPageData.url}
                            alt={`Page ${currentPage + 1}`}
                            width={1200}
                            height={1600}
                            unoptimized
                            priority={currentPage < 3}
                            className="w-full h-auto"
                            onError={(e) => {
                                // Fallback to high quality if data saver fails
                                if (useDataSaver) {
                                    const target = e.target as HTMLImageElement;
                                    target.src = currentPageData.url;
                                }
                            }}
                        />
                    </div>
                )}
            </div>

            {/* Navigation Controls */}
            <div className={`sticky bottom-0 z-10 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 border-t ${isFullscreen ? 'hidden' : ''}`}>
                <div className="container mx-auto px-4 py-4">
                    <div className="flex items-center justify-between">
                        <Button
                            variant="outline"
                            onClick={handlePreviousPage}
                            disabled={currentPage === 0}
                        >
                            <ChevronLeft className="h-4 w-4 mr-2" />
                            Previous
                        </Button>
                        <div className="flex items-center gap-2">
                            <input
                                type="range"
                                min="0"
                                max={totalPages - 1}
                                value={currentPage}
                                onChange={(e) => setCurrentPage(Number(e.target.value))}
                                className="w-48"
                            />
                            <span className="text-sm text-muted-foreground min-w-[80px] text-center">
                                {currentPage + 1} / {totalPages}
                            </span>
                        </div>
                        <Button
                            variant="outline"
                            onClick={handleNextPage}
                            disabled={currentPage === totalPages - 1}
                        >
                            Next
                            <ChevronRight className="h-4 w-4 ml-2" />
                        </Button>
                    </div>
                </div>
            </div>

            {/* Fullscreen overlay controls */}
            {isFullscreen && (
                <div className="fixed bottom-4 left-1/2 -translate-x-1/2 z-50 flex items-center gap-2 bg-background/90 backdrop-blur rounded-lg border px-4 py-2">
                    <Button
                        variant="ghost"
                        size="sm"
                        onClick={handlePreviousPage}
                        disabled={currentPage === 0}
                    >
                        <ChevronLeft className="h-4 w-4" />
                    </Button>
                    <span className="text-sm text-foreground min-w-[100px] text-center">
                        {currentPage + 1} / {totalPages}
                    </span>
                    <Button
                        variant="ghost"
                        size="sm"
                        onClick={handleNextPage}
                        disabled={currentPage === totalPages - 1}
                    >
                        <ChevronRight className="h-4 w-4" />
                    </Button>
                    <div className="w-px h-6 bg-border mx-2" />
                    <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => setIsFullscreen(false)}
                    >
                        <X className="h-4 w-4" />
                    </Button>
                </div>
            )}
        </div>
    );
}
