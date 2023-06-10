/// <reference types="vitest" />
/// <reference types="vite/client" />

import react from '@vitejs/plugin-react-swc'
import { join, resolve } from 'path'
import { defineConfig } from 'vite'

export default defineConfig({
  plugins: [react()],
  envDir: join(__dirname),
  envPrefix: ['API_', 'VITE_'],
  define: {
    'import.meta.env.APP_VERSION': `"${process.env.npm_package_version}"`,
  },
  test: {
    globals: true,
    environment: 'jsdom',
    cache: { dir: resolve(__dirname, 'node_modules/.vitest') },
    include: ['**/*.{test,spec}.{ts,tsx}'],
  },
  publicDir: resolve(__dirname, 'public'),
  root: resolve(__dirname, 'websrc'),
  build: {
    emptyOutDir: true,
    outDir: resolve(__dirname, 'target/web'),
    rollupOptions: {
      input: {
        app: resolve(__dirname, 'websrc/index.html'),
      },
    },
  },
  resolve: {
    alias: [
      { find: '@', replacement: resolve(__dirname, 'websrc') },
      { find: '~', replacement: resolve(__dirname, 'public') },
    ],
  },
  base: '/ui',
  server: {
    port: 3000,
    base: '/ui',
    proxy: {
      '/api': {
        target: 'http://0.0.0.0:9090',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\//, ''),
      },
    },
  },
})
