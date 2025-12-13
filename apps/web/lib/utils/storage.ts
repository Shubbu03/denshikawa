const isClient = typeof window !== 'undefined';

export const storage = {
    get: <T>(key: string, defaultValue: T | null = null): T | null => {
        if (!isClient) return defaultValue;
        try {
            const item = window.localStorage.getItem(key);
            return item ? (JSON.parse(item) as T) : defaultValue;
        } catch {
            return defaultValue;
        }
    },

    set: <T>(key: string, value: T): boolean => {
        if (!isClient) return false;
        try {
            window.localStorage.setItem(key, JSON.stringify(value));
            return true;
        } catch {
            return false;
        }
    },

    remove: (key: string): boolean => {
        if (!isClient) return false;
        try {
            window.localStorage.removeItem(key);
            return true;
        } catch {
            return false;
        }
    },

    clear: (): boolean => {
        if (!isClient) return false;
        try {
            window.localStorage.clear();
            return true;
        } catch {
            return false;
        }
    },
};

