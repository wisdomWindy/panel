<script setup lang="ts">
import { computed } from 'vue';
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
  <section class="mx-auto max-w-5xl">
    <header class="flex h-14 items-center justify-between border-b border-gray-200">
      <h2 class="m-0 text-lg font-semibold text-gray-900">
        直播间
      </h2>
      <el-tag :type="room.isConnected ? 'success' : room.snapshot.status === 'reconnecting' ? 'warning' : room.snapshot.status === 'error' ? 'danger' : 'info'">
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

    <dl class="mt-6 grid grid-cols-1 gap-px overflow-hidden border border-gray-200 bg-gray-200 sm:grid-cols-3">
      <div class="bg-white p-5">
        <dt class="text-sm text-gray-500">
          房间号
        </dt>
        <dd class="m-0 mt-2 text-xl font-semibold tabular-nums text-gray-900">
          {{ room.snapshot.roomId ?? '-' }}
        </dd>
      </div>
      <div class="bg-white p-5">
        <dt class="text-sm text-gray-500">
          房间名称
        </dt>
        <dd class="m-0 mt-2 truncate text-xl font-semibold text-gray-900">
          {{ room.snapshot.roomName ?? '-' }}
        </dd>
      </div>
      <div class="bg-white p-5">
        <dt class="text-sm text-gray-500">
          在线人数
        </dt>
        <dd class="m-0 mt-2 text-xl font-semibold tabular-nums text-gray-900">
          {{ room.snapshot.onlineCount === null ? '-' : formatCount(room.snapshot.onlineCount) }}
        </dd>
      </div>
    </dl>
  </section>
</template>
