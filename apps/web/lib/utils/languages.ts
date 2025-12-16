export const LANGUAGE_NAMES: Record<string, string> = {
    'en': 'English',
    'es': 'Spanish',
    'es-la': 'Spanish (Latin America)',
    'fr': 'French',
    'de': 'German',
    'it': 'Italian',
    'pt-br': 'Portuguese (Brazil)',
    'ru': 'Russian',
    'ja': 'Japanese',
    'ko': 'Korean',
    'zh': 'Chinese',
    'zh-hk': 'Chinese (Hong Kong)',
    'ar': 'Arabic',
    'th': 'Thai',
    'vi': 'Vietnamese',
    'id': 'Indonesian',
    'hi': 'Hindi',
    'pl': 'Polish',
    'tr': 'Turkish',
    'uk': 'Ukrainian',
    'cs': 'Czech',
    'ca': 'Catalan',
    'el': 'Greek',
    'he': 'Hebrew',
    'bg': 'Bulgarian',
    'mn': 'Mongolian',
    'ro': 'Romanian',
    'eu': 'Basque',
};

export function getLanguageName(code: string): string {
    return LANGUAGE_NAMES[code] || code.toUpperCase();
}

export function getLanguageCode(name: string): string | null {
    const entry = Object.entries(LANGUAGE_NAMES).find(([_, displayName]) => displayName === name);
    return entry ? entry[0] : null;
}
