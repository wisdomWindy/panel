<script setup lang="ts">
import { ref, watch } from 'vue';
import type { DataRange } from '@/platform/desktop';

const props = defineProps<{ modelValue: DataRange }>();
const emit = defineEmits<{ 'update:modelValue': [value: DataRange] }>();
const mode = ref(props.modelValue.kind);
const custom = ref<[Date, Date] | null>(null);

watch(
  () => props.modelValue.kind,
  value => {
    mode.value = value;
  }
);

function changeMode(value: string | number | boolean): void {
  const next = String(value) as DataRange['kind'];
  mode.value = next;
  if (next !== 'custom') emit('update:modelValue', { kind: next });
}

function changeCustom(value: [Date, Date] | null): void {
  if (!value) return;
  emit('update:modelValue', {
    kind: 'custom',
    startAt: value[0].getTime(),
    endAt: value[1].getTime()
  });
}
</script>

<template>
  <div class="flex flex-wrap items-center gap-2">
    <el-segmented
      :model-value="mode"
      :options="[
        { label: '本场', value: 'liveSession' },
        { label: '今日', value: 'today' },
        { label: '近7天', value: 'last7Days' },
        { label: '自定义', value: 'custom' }
      ]"
      aria-label="数据范围"
      @change="changeMode"
    />
    <el-date-picker
      v-if="mode === 'custom'"
      v-model="custom"
      type="datetimerange"
      range-separator="至"
      start-placeholder="开始时间"
      end-placeholder="结束时间"
      :clearable="false"
      @change="changeCustom"
    />
  </div>
</template>
