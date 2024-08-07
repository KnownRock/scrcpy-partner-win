import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import sveltePreprocess from 'svelte-preprocess'
// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    svelte({
      preprocess: [
        sveltePreprocess({
          typescript: true
        })
      ]
    })
  ],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // prevent vite from obscuring rust errors
  clearScreen: false,
  // tauri expects a fixed port, fail if that port is not available
  server: {
    port: 21420,
    strictPort: true
  },
  // to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    // Tauri supports es2021
    target: process.env.TAURI_PLATFORM === 'windows' ? 'chrome105' : 'safari13',
    // don't minify for debug builds
    minify: process.env.TAURI_DEBUG == null ? 'esbuild' : false,
    // produce sourcemaps for debug builds
    sourcemap: !(process.env.TAURI_DEBUG == null),

    rollupOptions: {
      input: {
        main: new URL('./index.html', import.meta.url).pathname,
        splashscreen: new URL('./splashscreen.html', import.meta.url).pathname,
        tool: new URL('./tool.html', import.meta.url).pathname,
        record: new URL('./record.html', import.meta.url).pathname

      }
    }
  }
})
