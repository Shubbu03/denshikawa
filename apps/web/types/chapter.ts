export interface ChapterPage {
    url: string;
    width: number;
    height: number;
    pageNumber: number;
}

export interface ReaderSettings {
    fitMode: 'width' | 'height' | 'contain' | 'cover';
    darkMode: boolean;
    showControls: boolean;
    autoScroll: boolean;
}

