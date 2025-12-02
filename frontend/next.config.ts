import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  async rewrites() {
    return [
      {
        source: '/api/:path*',
        destination: 'http://109.205.177.65/api/:path*',
      },
    ];
  },
};

export default nextConfig;
