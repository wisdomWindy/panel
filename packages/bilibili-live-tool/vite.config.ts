import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import electron from 'vite-plugin-electron';
import path from 'path';
import AutoImport from 'unplugin-auto-import/vite';
import Components from 'unplugin-vue-components/vite';
import { ElementPlusResolver } from 'unplugin-vue-components/resolvers';

export default defineConfig({
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
    }),
    electron([
      {
        entry: 'src/main/index.ts',
        vite: {
          build: {
            outDir: 'dist/main',
            rollupOptions: {
              external: ['electron']
            }
          }
        }
      },
      {
        entry: 'src/preload/index.ts',
        onstart(options) {
          options.reload();
        },
        vite: {
          build: {
            outDir: 'dist/preload',
            lib: {
              entry: 'src/preload/index.ts',
              formats: ['cjs'],
              fileName: () => 'index.js'
            },
            rollupOptions: {
              external: ['electron']
            }
          }
        }
      }
    ])
  ],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src/renderer'),
      'events': path.resolve(__dirname, 'src/renderer/modules/utils/event-emitter.ts')
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

          if (id.includes('/three/')) {
            return 'three-vendor';
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
});
