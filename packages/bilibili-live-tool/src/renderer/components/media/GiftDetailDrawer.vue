<script setup lang="ts">
import type { GiftEvent } from '@/platform/desktop';
import { formatClockTime, formatMoneyMinor } from '@/modules/media/formatters';

defineProps<{
  modelValue: boolean;
  gift: GiftEvent | null;
}>();

defineEmits<{ 'update:modelValue': [value: boolean] }>();
</script>

<template>
  <el-drawer
    :model-value="modelValue"
    title="礼物详情"
    size="360px"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <dl v-if="gift" class="m-0 divide-y divide-gray-200 border-y border-gray-200">
      <div class="grid grid-cols-[88px_minmax(0,1fr)] gap-3 py-3">
        <dt class="text-sm text-gray-500">
          用户
        </dt>
        <dd class="m-0 break-words text-sm text-gray-900">
          {{ gift.audience.username }}
        </dd>
      </div>
      <div class="grid grid-cols-[88px_minmax(0,1fr)] gap-3 py-3">
        <dt class="text-sm text-gray-500">
          礼物
        </dt>
        <dd class="m-0 break-words text-sm text-gray-900">
          {{ gift.giftName }}
        </dd>
      </div>
      <div class="grid grid-cols-[88px_minmax(0,1fr)] gap-3 py-3">
        <dt class="text-sm text-gray-500">
          数量
        </dt>
        <dd class="m-0 text-sm tabular-nums text-gray-900">
          {{ gift.quantity }}
        </dd>
      </div>
      <div class="grid grid-cols-[88px_minmax(0,1fr)] gap-3 py-3">
        <dt class="text-sm text-gray-500">
          价值
        </dt>
        <dd class="m-0 text-sm tabular-nums text-gray-900">
          {{ formatMoneyMinor(gift.valueMinor) }}
        </dd>
      </div>
      <div class="grid grid-cols-[88px_minmax(0,1fr)] gap-3 py-3">
        <dt class="text-sm text-gray-500">
          时间
        </dt>
        <dd class="m-0 text-sm tabular-nums text-gray-900">
          {{ formatClockTime(gift.occurredAt) }}
        </dd>
      </div>
    </dl>
  </el-drawer>
</template>
