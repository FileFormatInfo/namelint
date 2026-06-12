import { defineConfig } from "astro/config";

export default defineConfig({
  site: "https://www.namelint.dev",
  output: "static",
  compressHTML: false,
  build: {
    format: "file"
  }
});
