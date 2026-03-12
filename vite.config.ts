import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import path from "path";

export default defineConfig({
  plugins: [react()],
  root: ".",
  resolve: {
    alias: {
      "@frontend": path.resolve(__dirname, "src/frontend"),
      "@plugins": path.resolve(__dirname, "plugins")
    }
  },
  build: {
    outDir: "dist",
    sourcemap: true
  },
  server: {
    port: 5173,
    strictPort: true
  }
});

