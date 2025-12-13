import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  images: {
    remotePatterns: [
      {
        protocol: 'https',
        hostname: 'uploads.mangadex.org',
        pathname: '/**',
      },
    ],
    unoptimized: true,
  },
};

export default nextConfig;
