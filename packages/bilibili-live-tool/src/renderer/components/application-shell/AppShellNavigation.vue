<script setup lang="ts">
import type { Component } from 'vue';
import { useI18n } from 'vue-i18n';
import {
  Bell, ChatDotRound, Connection, Cpu, DataAnalysis, MagicStick,
  Microphone, Present, Setting, TrendCharts, UserFilled, VideoCamera, Warning
} from '@element-plus/icons-vue';
import type { ShellNavigationItem } from '@/modules/application-shell/navigation';

defineProps<{ items: readonly ShellNavigationItem[]; activeRoute: string }>();
const emit = defineEmits<{ navigate: [item: ShellNavigationItem] }>();
const { t } = useI18n();

const icons: Readonly<Record<string, Component>> = {
  liveRoom: Connection,
  room: Connection,
  danmaku: ChatDotRound,
  dashboard: DataAnalysis,
  rankings: TrendCharts,
  risk: Warning,
  sensitiveWords: Warning,
  blacklist: UserFilled,
  reminders: Present,
  captainEffect: MagicStick,
  giftReminder: Bell,
  giftSound: Present,
  tts: Microphone,
  recording: VideoCamera,
  ai: Cpu,
  settings: Setting
};

/** A group is active when the current route belongs to one of its children. */
function isGroupActive(item: ShellNavigationItem, activeRoute: string): boolean {
  return item.children?.some(child => child.route === activeRoute) ?? false;
}

function entryRoute(item: ShellNavigationItem): string | undefined {
  return item.route ?? item.children?.[0]?.route;
}

function open(item: ShellNavigationItem): void {
  if (item.isDisabled) return;
  emit('navigate', item);
}
</script>

<template>
  <aside
    class="app-navigation flex h-full w-[72px] shrink-0 flex-col overflow-y-auto py-2 md:w-[var(--shell-navigation-width,208px)]"
  >
    <nav aria-label="primary" class="flex flex-col gap-0.5 px-2">
      <template v-for="item in items" :key="item.id">
        <!-- Grouped entry: the group header opens its first child and the
             children stay visible in the rail so the IA matches PRD 8.1. -->
        <template v-if="item.children">
          <div v-if="isGroupActive(item, activeRoute)" class="app-nav-sub">
            <button
              type="button"
              class="app-nav-link app-nav-link--child"
              :aria-current="activeRoute === child.route ? 'page' : undefined"
              v-for="child in item.children"
              :key="child.id"
              :disabled="child.isDisabled"
              @click="open(child)"
            >
              <el-icon v-if="icons[child.id]" class="shrink-0"><component :is="icons[child.id]" /></el-icon>
              <span class="sr-only md:not-sr-only">{{ t(child.labelKey) }}</span>
            </button>
          </div>
          <button
            v-else
            type="button"
            class="app-nav-link"
            :disabled="item.isDisabled"
            @click="open({ ...item, route: entryRoute(item) })"
          >
            <el-icon v-if="icons[item.id]" class="shrink-0"><component :is="icons[item.id]" /></el-icon>
            <span class="sr-only md:not-sr-only">{{ t(item.labelKey) }}</span>
          </button>
        </template>
        <button
          v-else
          type="button"
          class="app-nav-link"
          :disabled="item.isDisabled"
          :aria-current="activeRoute === item.route ? 'page' : undefined"
          @click="open(item)"
        >
          <el-icon v-if="icons[item.id]" class="shrink-0"><component :is="icons[item.id]" /></el-icon>
          <span class="sr-only md:not-sr-only">{{ t(item.labelKey) }}</span>
        </button>
      </template>
    </nav>
  </aside>
</template>

<style scoped>
.app-nav-sub {
  display: flex;
  flex-direction: column;
  gap: 2px;
  margin: 2px 0 6px 14px;
  padding-left: 10px;
  border-left: 1px solid var(--app-border);
}
.app-nav-link--child {
  padding: 7px 10px;
  font-size: 13px;
}
</style>
