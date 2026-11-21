<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { Connection, HomeFilled, UserFilled } from '@element-plus/icons-vue';
import { useLiveRoomStore } from '@/stores/live-data/room';
import { formatCount } from '@/modules/live-data/formatters';

const room = useLiveRoomStore();
const { t } = useI18n();
const statusText = computed(
  () =>
    ({
      disconnected: t('connection.disconnected'),
      connecting: t('connection.connecting'),
      connected: t('connection.connected'),
      reconnecting: t('connection.reconnecting', { attempt: room.snapshot.reconnectAttempt }),
      error: t('connection.connectionError')
    })[room.snapshot.status]
);
</script>

<template>
  <section class="mx-auto flex max-w-6xl flex-col gap-5">
    <header
      class="app-hairline flex min-h-14 items-center justify-between border-b pb-4"
    >
      <div class="flex items-center gap-3">
        <span
          class="app-surface-2 flex h-10 w-10 items-center justify-center rounded-lg"
          style="color: var(--app-accent)"
        >
          <el-icon :size="20"><Connection /></el-icon>
        </span>
        <h2 class="app-text m-0 text-lg font-semibold">
          {{ t('connection.liveRoom') }}
        </h2>
      </div>
      <el-tag
        class="shrink-0"
        :type="
          room.isConnected
            ? 'success'
            : room.snapshot.status === 'reconnecting'
              ? 'warning'
              : room.snapshot.status === 'error'
                ? 'danger'
                : 'info'
        "
      >
        {{ statusText }}
      </el-tag>
    </header>

    <el-alert
      v-if="room.error"
      :title="room.error.message"
      type="error"
      :closable="false"
      show-icon
    />

    <dl class="grid grid-cols-1 gap-4 sm:grid-cols-3">
      <div class="app-card p-5">
        <div class="flex items-center justify-between">
          <dt class="app-muted m-0 text-sm font-medium">
            {{ t('connection.roomId') }}
          </dt>
          <el-icon style="color: var(--app-accent)">
            <Connection />
          </el-icon>
        </div>
        <dd class="app-text m-0 mt-5 text-2xl font-semibold tabular-nums">
          {{ room.snapshot.roomId ?? '-' }}
        </dd>
      </div>
      <div class="app-card p-5">
        <div class="flex items-center justify-between">
          <dt class="app-muted m-0 text-sm font-medium">
            {{ t('connection.roomName') }}
          </dt>
          <el-icon style="color: var(--app-gold)">
            <HomeFilled />
          </el-icon>
        </div>
        <dd class="app-text m-0 mt-5 truncate text-2xl font-semibold">
          {{ room.snapshot.roomName ?? '-' }}
        </dd>
      </div>
      <div class="app-card p-5">
        <div class="flex items-center justify-between">
          <dt class="app-muted m-0 text-sm font-medium">
            {{ t('connection.onlineCount') }}
          </dt>
          <el-icon style="color: var(--app-green)">
            <UserFilled />
          </el-icon>
        </div>
        <dd class="app-text m-0 mt-5 text-2xl font-semibold tabular-nums">
          {{ room.snapshot.onlineCount === null ? '-' : formatCount(room.snapshot.onlineCount) }}
        </dd>
      </div>
    </dl>
  </section>
</template>
