<script setup lang="ts">
import type { ModerationAlert } from '@/platform/desktop';

defineProps<{
    visible: boolean;
    alert: ModerationAlert | null;
  }>();

defineEmits<{
    'update:visible': [value: boolean];
  }>();
</script>

<template>
  <el-drawer
    :model-value="visible"
    title="用户详情"
    size="min(460px, 92vw)"
    @update:model-value="$emit('update:visible', $event)"
  >
    <template v-if="alert">
      <div class="border-y border-gray-200 py-4 dark:border-gray-700">
        <div class="font-semibold">
          {{ alert.username }}
        </div>
        <div class="mt-1 text-sm tabular-nums text-gray-500">
          UID: {{ alert.userUid }}
        </div>
      </div>
      <section class="mt-5">
        <h3 class="mb-2 mt-0 text-base font-semibold">
          违规记录
        </h3>
        <p class="mb-2 text-sm">
          {{ alert.violationDescription }}
        </p>
        <p class="m-0 break-words text-sm text-gray-500">
          内容示例：{{ alert.contentExample }}
        </p>
      </section>
    </template>
  </el-drawer>
</template>
