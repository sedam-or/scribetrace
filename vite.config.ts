import { defineConfig } from "vite";

// Fixed port: tauri.conf.json devUrl points here. Strict so a clash fails loudly.
export default defineConfig({
  clearScreen: false,
  server: { port: 1420, strictPort: true, host: "127.0.0.1" },
  build: { target: "es2022", outDir: "dist", emptyOutDir: true },
});
