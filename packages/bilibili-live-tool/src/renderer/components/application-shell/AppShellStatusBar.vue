<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { StatusBarItem } from '@/modules/application-shell/view-model';

defineProps<{ items: readonly StatusBarItem[] }>();
const { t } = useI18n();
</script>

<template>
  <footer
    class="flex h-[var(--shell-status-height,36px)] items-center border-t border-zinc-200 bg-white px-3 text-xs text-zinc-500 dark:border-zinc-800 dark:bg-zinc-900 dark:text-zinc-400"
    aria-live="polite"
  >
    <div class="flex min-w-0 flex-1 items-center overflow-x-auto [scrollbar-width:none] [&::-webkit-scrollbar]:hidden">
      <div
        v-for="item in items"
        :key="item.id"
        class="mr-3 flex shrink-0 items-center gap-1.5 border-r border-zinc-200 pr-3 last:border-r-0 dark:border-zinc-700"
      >
        <span
          v-if="item.id === 'connection'"
          class="h-2 w-2 rounded-full"
          :class="item.tone === 'success' ? 'bg-emerald-500' : item.tone === 'warning' ? 'bg-amber-500' : 'bg-red-500'"
        />
        <span>{{ t(`shell.status.${item.id}`) }}</span>
        <span
          class="font-medium tabular-nums text-zinc-800 dark:text-zinc-100"
          :class="item.tone === 'success' ? 'text-emerald-700 dark:text-emerald-400' : item.tone === 'warning' ? 'text-amber-700 dark:text-amber-400' : item.tone === 'danger' ? 'text-red-700 dark:text-red-400' : ''"
        >
          {{ item.value }}
        </span>
      </div>
    </div>
  </footer>
</template>
