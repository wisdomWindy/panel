<script setup lang="ts">
import type { ModerationAlert } from '@/platform/desktop';

defineProps<{
    alerts: readonly ModerationAlert[];
    canMute: boolean;
    isExpired: boolean;
    pending: ReadonlySet<string>;
  }>();

defineEmits<{
    view: [alert: ModerationAlert];
    mute: [alert: ModerationAlert];
    ignore: [alert: ModerationAlert];
    blacklist: [alert: ModerationAlert];
  }>();

function dateTime(value: number): string {
  return new Intl.DateTimeFormat(undefined, { dateStyle: 'short', timeStyle: 'medium' }).format(
    value
  );
}
</script>

<template>
  <div
    class="divide-y divide-gray-200 border-y border-gray-200 bg-white dark:divide-gray-700 dark:border-gray-700 dark:bg-gray-900"
  >
    <article
      v-for="alert in alerts"
      :key="alert.alertId"
      class="grid gap-3 px-4 py-3 md:grid-cols-[minmax(0,1fr)_auto]"
    >
      <div class="min-w-0">
        <div class="flex flex-wrap items-center gap-x-3 gap-y-1">
          <strong class="break-words">{{ alert.username }}</strong>
          <span class="text-sm tabular-nums text-gray-500">UID: {{ alert.userUid }}</span>
          <el-tag
            size="small"
            :type="
              alert.status === 'failed'
                ? 'danger'
                : alert.status === 'muted'
                  ? 'success'
                  : 'warning'
            "
          >
            {{ alert.status }}
          </el-tag>
          <span class="text-xs text-gray-500">{{ dateTime(alert.occurredAt) }}</span>
        </div>
        <p class="mb-1 mt-2 text-sm">
          {{ alert.violationDescription }}
        </p>
        <p class="m-0 break-words text-sm text-gray-500">
          内容示例：{{ alert.contentExample }}
        </p>
        <p v-if="alert.errorCode" class="mb-0 mt-1 text-sm text-red-600">
          {{ alert.errorCode }}
        </p>
      </div>
      <div class="flex flex-wrap items-center gap-2 md:justify-end">
        <el-button size="small" @click="$emit('view', alert)">
          查看
        </el-button>
        <el-button
          v-if="!isExpired"
          size="small"
          type="danger"
          :disabled="!canMute"
          :loading="pending.has(`mute:${alert.alertId}`)"
          :title="canMute ? '' : '本地账号凭证不可用'"
          @click="$emit('mute', alert)"
        >
          禁言
        </el-button>
        <el-button v-else size="small" @click="$emit('blacklist', alert)">
          加入黑名单
        </el-button>
        <el-button
          size="small"
          :loading="pending.has(`ignore:${alert.alertId}`)"
          @click="$emit('ignore', alert)"
        >
          忽略
        </el-button>
      </div>
    </article>
  </div>
</template>
