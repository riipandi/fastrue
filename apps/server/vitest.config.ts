import path from 'node:path'
import { defineConfig } from 'vitest/config'

export default defineConfig({
  resolve: {
    alias: {
      src: path.resolve(__dirname, './src'),
      test: path.resolve(__dirname, './test'),
    },
  },
  test: {
    deps: {
      fallbackCJS: true, // required to make kysely migrations work
    },
    coverage: {
      enabled: true,
      all: true,
      reporter: ['text', 'json', 'html'],
      include: ['src'],
      exclude: ['**/migrations'],
    },
  },
})
