<script setup lang="ts">
import { nextTick, ref, type Component } from 'vue';
import { Bell, Setting } from '@element-plus/icons-vue';
import { useI18n } from 'vue-i18n';
import type { RoomSnapshot } from '@/platform/desktop';
import appIconUrl from '../../../../src-tauri/icons/icon.png';

defineProps<{
  modelValue: string;
  rooms: readonly string[];
  room: RoomSnapshot;
  isPending: boolean;
  roomError: string | null;
  notificationOpen: boolean;
  memberToolbarComponent: Component | null;
}>();

const emit = defineEmits<{
  'update:modelValue': [value: string];
  'update:notificationOpen': [value: boolean];
  toggleConnection: [];
  openNotifications: [];
  openSettings: [];
}>();

const { t } = useI18n();
const notificationTrigger = ref<{ $el?: HTMLElement } | null>(null);

function setNotificationOpen(value: boolean): void {
  emit('update:notificationOpen', value);
  if (!value) void nextTick(() => notificationTrigger.value?.$el?.focus());
}
</script>

<template>
  <header
    class="flex h-[var(--shell-toolbar-height,64px)] min-w-0 items-center border-b border-zinc-200 bg-white px-2 shadow-sm sm:px-3 dark:border-zinc-800 dark:bg-zinc-900"
  >
    <div class="flex shrink-0 items-center gap-2.5 pr-2 sm:pr-3">
      <img
        :src="appIconUrl"
        alt=""
        class="h-7 w-7 sm:h-8 sm:w-8"
        aria-hidden="true"
      >
      <h1 class="sr-only m-0 text-sm font-semibold text-zinc-950 xl:not-sr-only dark:text-zinc-50">
        {{ t('shell.productName') }}
      </h1>
    </div>
    <div class="h-7 w-px shrink-0 bg-zinc-200 dark:bg-zinc-700" />
    <div class="flex min-w-0 flex-1 items-center gap-1.5 px-2 sm:gap-2 sm:px-3">
      <el-tooltip :disabled="roomError === null" :content="roomError ?? ''" placement="bottom-start">
        <el-select
          :model-value="modelValue"
          filterable
          allow-create
          default-first-option
          class="!w-auto min-w-0 max-w-72 flex-1"
          :placeholder="t('shell.toolbar.roomPlaceholder')"
          :aria-label="t('shell.toolbar.roomPlaceholder')"
          :aria-invalid="roomError !== null"
          @update:model-value="emit('update:modelValue', String($event))"
        >
          <el-option
            v-for="value in rooms"
            :key="value"
            :label="value"
            :value="value"
          />
        </el-select>
      </el-tooltip>
      <el-button
        type="primary"
        class="min-w-16 sm:min-w-20"
        :loading="isPending"
        :disabled="room.status === 'connecting'"
        @click="emit('toggleConnection')"
      >
        {{ room.status === 'connected' || room.status === 'reconnecting'
          ? t('shell.toolbar.disconnect')
          : t('shell.toolbar.connect') }}
      </el-button>
    </div>
    <div class="ml-auto flex shrink-0 items-center gap-2">
      <component :is="memberToolbarComponent" v-if="memberToolbarComponent" />
      <el-popover
        :visible="notificationOpen"
        placement="bottom-end"
        :width="280"
        trigger="click"
        @update:visible="setNotificationOpen"
      >
        <template #reference>
          <span class="inline-flex">
            <el-tooltip :content="t('shell.toolbar.notifications')">
              <el-button
                ref="notificationTrigger"
                :icon="Bell"
                circle
                plain
                :aria-label="t('shell.toolbar.notifications')"
                @click="emit('openNotifications')"
              />
            </el-tooltip>
          </span>
        </template>
        <div class="h-20" aria-live="polite" />
      </el-popover>
      <el-tooltip :content="t('shell.toolbar.settings')">
        <el-button
          :icon="Setting"
          circle
          plain
          :aria-label="t('shell.toolbar.settings')"
          @click="emit('openSettings')"
        />
      </el-tooltip>
    </div>
  </header>
</template>
