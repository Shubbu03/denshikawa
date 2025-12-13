import { Metadata } from 'next';

export const metadata: Metadata = {
    title: 'Home | Denshikawa',
    description: 'Discover and read manga online',
};

export default function HomePage() {
    return (
        <div className="container mx-auto p-4">
            <h1 className="text-4xl font-bold">Welcome to Denshikawa</h1>
            <p className="mt-4 text-muted-foreground">
                Your manga reading experience starts here.
            </p>
        </div>
    );
}

