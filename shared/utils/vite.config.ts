import path from "path";
import { defineConfig } from "vite";
import dts from "vite-plugin-dts";

export default defineConfig({
  plugins: [dts()],
  build: {
    emptyOutDir: false,
    lib: {
      entry: path.resolve(__dirname, "src/main.ts"),
      formats: ["es"],
      name: "utils",
      // fileName: (format: string) => `utils.${format}.js`,
    },
    // rollupOptions: {
    //   external: [
    //     ...Object.keys(peerDependencies),
    //     ...Object.keys(dependencies),
    //   ],
    // },
    target: "esnext",
    sourcemap: true,
  },
});
