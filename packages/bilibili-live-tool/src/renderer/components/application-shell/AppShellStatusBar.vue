<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { StatusBarItem } from '@/modules/application-shell/view-model';

defineProps<{ items: readonly StatusBarItem[] }>();
const { t } = useI18n();
</script>

<template>
  <footer
    class="flex h-[var(--shell-status-height,32px)] items-center gap-5 overflow-x-auto border-t border-gray-200 bg-gray-50 px-4 text-xs text-gray-600 dark:border-gray-700 dark:bg-gray-950 dark:text-gray-300"
    aria-live="polite"
  >
    <div v-for="item in items" :key="item.id" class="flex shrink-0 items-center gap-1.5">
      <span
        v-if="item.id === 'connection'"
        class="h-2 w-2 rounded-full"
        :class="item.tone === 'success' ? 'bg-emerald-500' : item.tone === 'warning' ? 'bg-amber-500' : 'bg-red-500'"
      />
      <span>{{ t(`shell.status.${item.id}`) }}</span>
      <span
        class="tabular-nums text-gray-900 dark:text-gray-100"
        :class="item.tone === 'success' ? 'text-emerald-700 dark:text-emerald-400' : item.tone === 'warning' ? 'text-amber-700 dark:text-amber-400' : item.tone === 'danger' ? 'text-red-700 dark:text-red-400' : ''"
      >
        {{ item.value }}
      </span>
    </div>
  </footer>
</template>
