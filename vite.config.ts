/// <reference types="vitest" />
/// <reference types="vite/client" />

import react from '@vitejs/plugin-react-swc'
import { join, resolve } from 'path'
import { defineConfig } from 'vite'

export default defineConfig({
  plugins: [react()],
  envDir: join(__dirname),
  envPrefix: ['WASTA_'],
  // test: { globals: true, environment: 'jsdom' },
  publicDir: resolve(__dirname, 'public'),
  root: resolve(__dirname, 'src-web'),
  build: {
    emptyOutDir: true,
    outDir: resolve(__dirname, 'web'),
    rollupOptions: {
      input: {
        app: resolve(__dirname, 'src-web/index.html'),
      },
    },
  },
  resolve: {
    alias: [
      { find: '@', replacement: resolve(__dirname, 'src-web') },
      { find: '~', replacement: resolve(__dirname, 'public') },
    ],
  },
  base: '/ui/',
  server: {
    port: 3000,
    base: '/ui/',
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:3030',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\//, ''),
      },
      '/swagger': {
        target: 'http://127.0.0.1:3030',
        changeOrigin: true,
      },
    },
  },
})
