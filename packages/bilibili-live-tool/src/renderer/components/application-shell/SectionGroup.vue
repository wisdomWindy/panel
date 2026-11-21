<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRoute, useRouter } from 'vue-router';

/**
 * Layout for the grouped navigation entries (risk / reminders / settings).
 *
 * The rail keeps one entry per PRD 8.1 group, while the child pages stay
 * addressable routes so deep links and the back button keep working.
 */
const route = useRoute();
const router = useRouter();
const { t } = useI18n();

const tabs = computed(() => {
  const parent = route.matched[0];
  const base = parent?.path ?? '';
  return (parent?.children ?? []).map(child => ({
    to: `${base}/${child.path}`.replace(/\/+/g, '/'),
    labelKey: String(child.meta?.titleKey ?? child.path),
    name: String(child.name ?? child.path)
  }));
});

const isActive = (to: string): boolean => route.path === to;

function open(to: string): void {
  if (!isActive(to)) void router.push(to);
}
</script>

<template>
  <section class="flex h-full min-h-0 flex-col gap-3">
    <nav class="flex flex-wrap items-center gap-1.5" :aria-label="t(String(route.meta.titleKey ?? 'shell.productName'))">
      <button
        v-for="tab in tabs"
        :key="tab.name"
        type="button"
        class="app-chip"
        :class="{ 'app-chip--active': isActive(tab.to) }"
        :aria-current="isActive(tab.to) ? 'page' : undefined"
        @click="open(tab.to)"
      >
        {{ t(tab.labelKey) }}
      </button>
    </nav>
    <div class="min-h-0 flex-1 overflow-y-auto">
      <router-view />
    </div>
  </section>
</template>
