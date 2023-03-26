import type { Options } from "tsup";

const config: Options = {
  entry: ["src/index.ts"],
  format: ["iife", "cjs", "esm"],
  dts: true,
  sourcemap: true,
};

export default config;
