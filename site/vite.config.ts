import { defineConfig } from "vite";
import { resolve } from "node:path";

export default defineConfig({
  root: resolve(__dirname),
  publicDir: resolve(__dirname, "public"),
  build: {
    outDir: resolve(__dirname, "../dist/site"),
    emptyOutDir: true,
    target: "es2022"
  },
  server: {
    host: "127.0.0.1",
    port: 4173
  }
});
