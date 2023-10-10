/// <reference types="vitest" />
/// <reference types="vite/client" />

import { join, resolve } from 'node:path'
import react from '@vitejs/plugin-react-swc'
import { defineConfig } from 'vite'

export default defineConfig({
  plugins: [react()],
  envDir: join(__dirname),
  envPrefix: ['VITE_'],
  define: {
    'import.meta.env.APP_VERSION': `"${process.env.npm_package_version}"`,
  },
  test: {
    globals: true,
    environment: 'jsdom',
    cache: { dir: './node_modules/.vitest' },
    include: ['./**/*.{test,spec}.{ts,tsx}'],
  },
  publicDir: resolve(__dirname, 'public'),
  root: resolve(__dirname),
  build: {
    emptyOutDir: true,
    outDir: resolve(__dirname, 'dist'),
    rollupOptions: {
      input: {
        app: resolve(__dirname, 'index.html'),
      },
    },
  },
  resolve: {
    alias: [
      { find: '@', replacement: resolve(__dirname, 'src') },
      { find: '~', replacement: resolve(__dirname, 'public') },
    ],
  },
  base: '/',
  server: {
    port: 8091,
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:8090',
        changeOrigin: true,
      },
      '/swagger': {
        target: 'http://127.0.0.1:8090',
        changeOrigin: true,
      },
    },
  },
})
