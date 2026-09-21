<script setup lang="ts">
import { computed } from 'vue';
import { Connection, HomeFilled, UserFilled } from '@element-plus/icons-vue';
import { useLiveRoomStore } from '@/stores/live-data/room';
import { formatCount } from '@/modules/live-data/formatters';

const room = useLiveRoomStore();
const statusText = computed(() => ({
  disconnected: '未连接',
  connecting: '连接中',
  connected: '已连接',
  reconnecting: `重连中 ${room.snapshot.reconnectAttempt}/10`,
  error: '连接错误'
}[room.snapshot.status]));
</script>

<template>
  <section class="mx-auto flex max-w-6xl flex-col gap-5">
    <header class="flex min-h-14 items-center justify-between border-b border-zinc-200 pb-4 dark:border-zinc-800">
      <div class="flex items-center gap-3">
        <span class="flex h-10 w-10 items-center justify-center rounded-lg bg-sky-100 text-sky-700 dark:bg-sky-950 dark:text-sky-300">
          <el-icon :size="20"><Connection /></el-icon>
        </span>
        <h2 class="m-0 text-lg font-semibold text-zinc-950 dark:text-zinc-100">
          直播间
        </h2>
      </div>
      <el-tag
        class="shrink-0"
        :type="room.isConnected ? 'success' : room.snapshot.status === 'reconnecting' ? 'warning' : room.snapshot.status === 'error' ? 'danger' : 'info'"
      >
        {{ statusText }}
      </el-tag>
    </header>

    <el-alert
      v-if="room.error"
      class="mt-5"
      :title="room.error.message"
      type="error"
      :closable="false"
      show-icon
    />

    <dl class="grid grid-cols-1 gap-4 sm:grid-cols-3">
      <div class="rounded-lg border border-zinc-200 bg-white p-5 shadow-sm dark:border-zinc-800 dark:bg-zinc-900">
        <div class="flex items-center justify-between">
          <dt class="text-sm font-medium text-zinc-500 dark:text-zinc-400">
            房间号
          </dt>
          <el-icon class="text-sky-600 dark:text-sky-400">
            <Connection />
          </el-icon>
        </div>
        <dd class="m-0 mt-5 text-2xl font-semibold tabular-nums text-zinc-950 dark:text-zinc-100">
          {{ room.snapshot.roomId ?? '-' }}
        </dd>
      </div>
      <div class="rounded-lg border border-zinc-200 bg-white p-5 shadow-sm dark:border-zinc-800 dark:bg-zinc-900">
        <div class="flex items-center justify-between">
          <dt class="text-sm font-medium text-zinc-500 dark:text-zinc-400">
            房间名称
          </dt>
          <el-icon class="text-amber-600 dark:text-amber-400">
            <HomeFilled />
          </el-icon>
        </div>
        <dd class="m-0 mt-5 truncate text-2xl font-semibold text-zinc-950 dark:text-zinc-100">
          {{ room.snapshot.roomName ?? '-' }}
        </dd>
      </div>
      <div class="rounded-lg border border-zinc-200 bg-white p-5 shadow-sm dark:border-zinc-800 dark:bg-zinc-900">
        <div class="flex items-center justify-between">
          <dt class="text-sm font-medium text-zinc-500 dark:text-zinc-400">
            在线人数
          </dt>
          <el-icon class="text-emerald-600 dark:text-emerald-400">
            <UserFilled />
          </el-icon>
        </div>
        <dd class="m-0 mt-5 text-2xl font-semibold tabular-nums text-zinc-950 dark:text-zinc-100">
          {{ room.snapshot.onlineCount === null ? '-' : formatCount(room.snapshot.onlineCount) }}
        </dd>
      </div>
    </dl>
  </section>
</template>
