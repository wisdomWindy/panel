<script setup lang="ts">
import { nextTick, ref, type Component } from 'vue';
import { Bell, Setting } from '@element-plus/icons-vue';
import { useI18n } from 'vue-i18n';
import type { RoomSnapshot } from '@/platform/desktop';

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
    class="flex h-[var(--shell-toolbar-height,56px)] min-w-0 items-center gap-3 border-b border-gray-200 bg-white px-4 dark:border-gray-700 dark:bg-gray-900"
  >
    <h1 class="m-0 shrink-0 text-base font-semibold text-gray-900 dark:text-gray-100">
      {{ t('shell.productName') }}
    </h1>
    <el-tooltip :disabled="roomError === null" :content="roomError ?? ''" placement="bottom-start">
      <el-select
        :model-value="modelValue"
        filterable
        allow-create
        default-first-option
        class="!w-52"
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
      :loading="isPending"
      :disabled="room.status === 'connecting'"
      @click="emit('toggleConnection')"
    >
      {{ room.status === 'connected' || room.status === 'reconnecting'
        ? t('shell.toolbar.disconnect')
        : t('shell.toolbar.connect') }}
    </el-button>
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
          <el-tooltip :content="t('shell.toolbar.notifications')">
            <el-button
              ref="notificationTrigger"
              :icon="Bell"
              circle
              :aria-label="t('shell.toolbar.notifications')"
              @click="emit('openNotifications')"
            />
          </el-tooltip>
        </template>
        <div class="h-20" aria-live="polite" />
      </el-popover>
      <el-tooltip :content="t('shell.toolbar.settings')">
        <el-button
          :icon="Setting"
          circle
          :aria-label="t('shell.toolbar.settings')"
          @click="emit('openSettings')"
        />
      </el-tooltip>
    </div>
  </header>
</template>
