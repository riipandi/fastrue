import { defineConfig } from 'tsup'

export default defineConfig({
  entry: ['src/index.ts'],
  outDir: '../../dist/fastrue-client-js',
  format: ['cjs', 'esm'],
  splitting: false,
  target: 'es2020',
  sourcemap: true,
  dts: true,
  clean: true,
})
