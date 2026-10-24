/// <reference types="vitest/config" />

import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import path from 'path';
import AutoImport from 'unplugin-auto-import/vite';
import Components from 'unplugin-vue-components/vite';
import { ElementPlusResolver } from 'unplugin-vue-components/resolvers';

export default defineConfig(({ mode }) => ({
  plugins: [
    vue(),
    AutoImport({
      dts: 'auto-imports.d.ts',
      imports: [],
      resolvers: [ElementPlusResolver()]
    }),
    Components({
      dts: 'components.d.ts',
      resolvers: [ElementPlusResolver()]
    })
  ],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src/renderer'),
      '@member-channel': path.resolve(
        __dirname,
        mode === 'member'
          ? 'src/renderer/platform/desktop/channel.member.ts'
          : 'src/renderer/platform/desktop/channel.free.ts'
      )
    }
  },
  server: {
    host: '127.0.0.1',
    port: 5173,
    strictPort: true,
    watch: {
      ignored: ['**/src-tauri/target/**']
    }
  },
  test: {
    server: {
      deps: {
        inline: ['element-plus']
      }
    }
  },
  root: '.',
  build: {
    outDir: 'dist/renderer',
    emptyOutDir: true,
    minify: 'esbuild',
    chunkSizeWarningLimit: 550,
    rollupOptions: {
      onwarn(warning, warn) {
        const isVueuseAnnotationWarning =
          warning.message?.includes('contains an annotation that Rollup cannot interpret') &&
          warning.id?.includes('/@vueuse/core/dist/index.js');

        if (isVueuseAnnotationWarning) {
          return;
        }

        warn(warning);
      },
      output: {
        manualChunks(id) {
          if (!id.includes('node_modules')) {
            return;
          }

          if (id.includes('/zrender/')) {
            return 'zrender-vendor';
          }

          if (id.includes('/echarts/')) {
            return 'echarts-vendor';
          }

          if (id.includes('/element-plus/') || id.includes('/@element-plus/')) {
            return 'element-plus-vendor';
          }

          if (
            id.includes('/vue/') ||
            id.includes('/vue-router/') ||
            id.includes('/pinia/') ||
            id.includes('/@vue/')
          ) {
            return 'vue-vendor';
          }

        }
      }
    }
  },
  esbuild: {
    drop: process.env.NODE_ENV === 'production' ? ['console', 'debugger'] : []
  },
  publicDir: 'public'
}));
