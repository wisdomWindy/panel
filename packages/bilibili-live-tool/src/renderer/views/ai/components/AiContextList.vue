<script setup lang="ts">
import type { AiContextItem } from '@/platform/desktop';

defineProps<{
    items: readonly AiContextItem[];
    selectedIds: ReadonlySet<string>;
  }>();

defineEmits<{
    select: [item: AiContextItem];
  }>();

const sourceLabel = (source: AiContextItem['source']) =>
  ({ danmaku: '弹幕', manual: '人工' })[source];
</script>

<template>
  <div
    class="h-48 overflow-y-auto border-y border-gray-200 bg-white dark:border-gray-700 dark:bg-gray-900"
  >
    <button
      v-for="item in items"
      :key="item.id"
      type="button"
      class="flex w-full items-start gap-3 border-0 border-b border-solid border-gray-100 bg-transparent px-3 py-2 text-left text-sm hover:bg-gray-50 dark:border-gray-800 dark:hover:bg-gray-800"
      :class="
        selectedIds.has(item.id)
          ? 'text-pink-600 dark:text-pink-400'
          : 'text-gray-700 dark:text-gray-200'
      "
      @click="$emit('select', item)"
    >
      <el-tag size="small" effect="plain">
        {{ sourceLabel(item.source) }}
      </el-tag>
      <span class="min-w-0 flex-1 break-words">{{ item.content }}</span>
    </button>
  </div>
</template>
