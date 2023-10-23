import react from "@vitejs/plugin-react";
import path from "path";
import { defineConfig } from "vite";
import dts from "vite-plugin-dts";

import { dependencies,peerDependencies } from "./package.json";

// https://vitejs.dev/config/
export default defineConfig({
  build: {
    emptyOutDir: false,
    lib: {
      entry: path.resolve(__dirname, "src/main.ts"),
      formats: ["es"],
      name: "ui",
    },
    rollupOptions: {
      external: [
        ...Object.keys(peerDependencies),
        ...Object.keys(dependencies),
      ],
    },
    target: "esnext",
    sourcemap: true,
  },
  plugins: [react(), dts()],
});
