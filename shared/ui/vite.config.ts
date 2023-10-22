import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path';
import dts from 'vite-plugin-dts';

// https://vitejs.dev/config/
export default defineConfig({
  build: {
    emptyOutDir: false,
    lib: {
      entry: path.resolve(__dirname, 'src/main.ts'),
      formats: ['es'],
      name: 'ui',
    },
    rollupOptions: {
      external: ['react', 'react/jsx-runtime'],
      // output: {
      //   globals: {
      //     React: 'react',
      //   },
      // },
    },
  },
  plugins: [react(), dts()],
})
