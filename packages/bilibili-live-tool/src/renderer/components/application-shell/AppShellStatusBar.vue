<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { StatusBarItem } from '@/modules/application-shell/view-model';

defineProps<{ items: readonly StatusBarItem[] }>();
const { t } = useI18n();
</script>

<template>
  <footer
    class="app-statusbar flex h-[var(--shell-status-height,32px)] items-center gap-3 overflow-x-auto px-3 text-xs [scrollbar-width:none] [&::-webkit-scrollbar]:hidden"
    aria-live="polite"
  >
    <template v-for="(item, index) in items" :key="item.id">
      <span v-if="index > 0" class="app-status-divider shrink-0" aria-hidden="true" />
      <span class="app-status-item shrink-0">
        <span
          v-if="item.id === 'connection'"
          class="app-status-dot"
          :class="`app-status-dot--${item.tone ?? 'danger'}`"
          aria-hidden="true"
        />
        <span>{{ t(`shell.status.${item.id}`) }}</span>
        <span class="app-status-value" :class="item.tone ? `app-status-value--${item.tone}` : ''">
          {{ item.value }}
        </span>
      </span>
    </template>
  </footer>
</template>

<style scoped>
.app-status-value--success {
  color: var(--app-green);
}
.app-status-value--warning {
  color: var(--app-gold);
}
.app-status-value--danger {
  color: var(--app-red);
}
</style>
