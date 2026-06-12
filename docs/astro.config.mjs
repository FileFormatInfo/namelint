import { defineConfig } from "astro/config";

export default defineConfig({
  site: "https://www.namelint.dev",
  output: "static",
  trailingSlash: "ignore",
  compressHTML: false,
  build: {
    format: "preserve"
  }
});
