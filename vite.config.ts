/// <reference types="vitest" />
/// <reference types="vite/client" />

import react from '@vitejs/plugin-react-swc'
import { join, resolve } from 'path'
import { defineConfig } from 'vite'

export default defineConfig({
  plugins: [react()],
  envDir: join(__dirname),
  envPrefix: ['TRUSTY_'],
  // test: { globals: true, environment: 'jsdom' },
  publicDir: resolve(__dirname, 'websrc/assets/public'),
  root: resolve(__dirname, 'websrc'),
  build: {
    emptyOutDir: true,
    outDir: resolve(__dirname, 'web'),
    rollupOptions: {
      input: {
        app: resolve(__dirname, 'websrc/index.html'),
      },
    },
  },
  resolve: {
    alias: [
      { find: '@', replacement: resolve(__dirname, 'websrc') },
      { find: '~', replacement: resolve(__dirname, 'websrc/assets') },
    ],
  },
  base: '/ui/',
  server: {
    port: 3000,
    base: '/ui/',
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:9999',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\//, ''),
      },
      '/swagger': {
        target: 'http://127.0.0.1:9999',
        changeOrigin: true,
      },
    },
  },
})
