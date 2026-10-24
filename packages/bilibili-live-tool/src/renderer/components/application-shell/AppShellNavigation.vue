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
</script>

<template>
  <aside
    class="h-full w-[72px] shrink-0 overflow-y-auto border-r border-zinc-200 bg-white py-2 md:w-[var(--shell-navigation-width,208px)] dark:border-zinc-800 dark:bg-zinc-900"
  >
    <el-menu :default-active="activeRoute" class="border-r-0 bg-transparent">
      <template v-for="item in items" :key="item.id">
        <el-sub-menu v-if="item.children" :index="item.id">
          <template #title>
            <el-icon v-if="icons[item.id]">
              <component :is="icons[item.id]" />
            </el-icon>
            <span class="sr-only md:not-sr-only">{{ t(item.labelKey) }}</span>
          </template>
          <el-menu-item
            v-for="child in item.children"
            :key="child.id"
            :index="child.route"
            :disabled="child.isDisabled"
            :aria-current="activeRoute === child.route ? 'page' : undefined"
            @click="emit('navigate', child)"
          >
            <el-icon v-if="icons[child.id]">
              <component :is="icons[child.id]" />
            </el-icon>
            <span class="sr-only md:not-sr-only">{{ t(child.labelKey) }}</span>
          </el-menu-item>
        </el-sub-menu>
        <el-menu-item
          v-else
          :index="item.route"
          :disabled="item.isDisabled"
          :aria-current="activeRoute === item.route ? 'page' : undefined"
          @click="emit('navigate', item)"
        >
          <el-icon v-if="icons[item.id]">
            <component :is="icons[item.id]" />
          </el-icon>
          <span class="sr-only md:not-sr-only">{{ t(item.labelKey) }}</span>
        </el-menu-item>
      </template>
    </el-menu>
  </aside>
</template>
