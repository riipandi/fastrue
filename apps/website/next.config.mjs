import nextMDX from '@next/mdx'

// Avoid build and lint error in Docker or Vercel deployment
const isProduction = process.env.NODE_ENV === 'production' || process.env.IS_VERCEL_ENV === 'true'

const withMDX = nextMDX({
  extension: /\.mdx?$/,
  options: {
    remarkPlugins: [],
    rehypePlugins: [],
  },
})

/** @type {import('next').NextConfig} */
const nextConfig = {
  images: {
    remotePatterns: [{ protocol: 'https', hostname: '**' }],
  },
  pageExtensions: ['ts', 'tsx', 'mdx'],
  reactStrictMode: true,
  poweredByHeader: false,
  output: 'standalone',
  compiler: { removeConsole: isProduction },
  eslint: { ignoreDuringBuilds: isProduction },
  typescript: { ignoreBuildErrors: isProduction },
}

export default withMDX(nextConfig)
