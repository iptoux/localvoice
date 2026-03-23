import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import { visualizer } from "rollup-plugin-visualizer";
import path from "path";

const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [
    react(),
    tailwindcss(),
    visualizer({
      filename: "dist/stats.html",
      open: false, // don't auto-open during normal builds
      gzipSize: true,
      template: "treemap",
    }),
  ],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },

  build: {
    rollupOptions: {
      output: {
        // Split large vendor chunks to reduce the main bundle size.
        // This addresses the >500kB Vite warning for recharts + lucide-react.
        manualChunks: {
          "vendor-react": ["react", "react-dom", "react-router-dom"],
          "vendor-charts": ["recharts"],
          "vendor-icons": ["lucide-react"],
          "vendor-i18n": ["i18next", "react-i18next"],
          "vendor-ui": ["radix-ui", "class-variance-authority", "clsx", "tailwind-merge"],
        },
      },
    },
  },

  // Vitest configuration — runs in jsdom so React components can be rendered.
  test: {
    globals: true,
    environment: "jsdom",
    setupFiles: ["./src/test/setup.ts"],
    // Alias @tauri-apps/* to the mock implementations in test environment.
    alias: {
      "@tauri-apps/api/core": path.resolve(__dirname, "./src/test/mocks/tauri-api-core.ts"),
      "@tauri-apps/api/event": path.resolve(__dirname, "./src/test/mocks/tauri-api-event.ts"),
    },
  },
}));
