import { defineConfig } from 'tsup'

export default defineConfig({
  entry: ['src/index.ts'],
  format: ['cjs', 'esm'],
  splitting: false,
  target: 'node18',
  sourcemap: true,
  dts: true,
  clean: true,
})
