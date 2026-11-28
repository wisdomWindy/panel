<script setup lang="ts">
import { computed, nextTick, ref, type Component } from 'vue';
import { Bell, Setting } from '@element-plus/icons-vue';
import { useI18n } from 'vue-i18n';
import type { RoomSnapshot } from '@/platform/desktop';
import appIconUrl from '../../../../src-tauri/icons/icon.png';

const props = defineProps<{
    modelValue: string;
    room: RoomSnapshot;
    isPending: boolean;
    roomError: string | null;
    notificationOpen: boolean;
    rememberSession: boolean;
    memberToolbarComponent: Component | null;
  }>();

const emit = defineEmits<{
    'update:modelValue': [value: string];
    'update:notificationOpen': [value: boolean];
    'update:rememberSession': [value: boolean];
    toggleConnection: [];
    openNotifications: [];
    openSettings: [];
  }>();

const { t } = useI18n();
const notificationTrigger = ref<{ $el?: HTMLElement } | null>(null);

const isConnected = computed(
  () => props.room.status === 'connected' || props.room.status === 'reconnecting'
);
const connectionTone = computed(() => {
  if (props.room.status === 'connected') return 'success';
  if (props.room.status === 'reconnecting' || props.room.status === 'connecting') return 'warning';
  return 'danger';
});

function setNotificationOpen(value: boolean): void {
  emit('update:notificationOpen', value);
  if (!value) void nextTick(() => notificationTrigger.value?.$el?.focus());
}
</script>

<template>
  <header
    class="app-toolbar flex h-[var(--shell-toolbar-height,48px)] min-w-0 items-center gap-3 px-3"
  >
    <div class="flex shrink-0 items-center gap-2 pr-1">
      <img :src="appIconUrl" alt="" class="h-6 w-6" aria-hidden="true">
      <h1 class="m-0 text-sm font-semibold app-text">
        {{ t('shell.productName') }}
      </h1>
    </div>

    <div class="app-status-divider shrink-0" />

    <div class="flex min-w-0 flex-1 items-center gap-2">
      <el-tooltip :disabled="roomError === null" :content="roomError ?? ''" placement="bottom-start">
        <el-input
          :model-value="modelValue"
          class="app-room-input !w-auto min-w-0 max-w-64 flex-1"
          type="password"
          show-password
          clearable
          :maxlength="128"
          :placeholder="t('shell.toolbar.roomPlaceholder')"
          :aria-label="t('shell.toolbar.roomPlaceholder')"
          :aria-invalid="roomError !== null"
          @update:model-value="emit('update:modelValue', String($event))"
        />
      </el-tooltip>
      <button
        type="button"
        class="app-btn-primary flex h-8 items-center gap-2 px-3 text-sm"
        :disabled="isPending || props.room.status === 'connecting'"
        @click="emit('toggleConnection')"
      >
        <span class="app-status-dot" :class="`app-status-dot--${connectionTone}`" />
        {{
          isConnected
            ? t('shell.toolbar.disconnect')
            : t('shell.toolbar.connect')
        }}
      </button>
      <el-tooltip :content="t('shell.toolbar.rememberSessionHint')" placement="bottom">
        <el-checkbox
          :model-value="rememberSession"
          class="app-faint shrink-0 text-xs"
          :aria-label="t('shell.toolbar.rememberSessionHint')"
          @update:model-value="emit('update:rememberSession', Boolean($event))"
        >
          {{ t('shell.toolbar.rememberSession') }}
        </el-checkbox>
      </el-tooltip>
    </div>

    <div class="ml-auto flex shrink-0 items-center gap-1.5">
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
            <button
              ref="notificationTrigger"
              type="button"
              class="app-icon-btn"
              :aria-label="t('shell.toolbar.notifications')"
              @click="emit('openNotifications')"
            >
              <el-icon><Bell /></el-icon>
            </button>
          </span>
        </template>
        <div class="h-20" aria-live="polite" />
      </el-popover>
      <button
        type="button"
        class="app-icon-btn"
        :aria-label="t('shell.toolbar.settings')"
        @click="emit('openSettings')"
      >
        <el-icon><Setting /></el-icon>
      </button>
    </div>
  </header>
</template>
