<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import type { DataRange } from '@/platform/desktop';

const props = defineProps<{ modelValue: DataRange }>();
const { t } = useI18n();
const emit = defineEmits<{ 'update:modelValue': [value: DataRange] }>();
const mode = ref(props.modelValue.kind);
const custom = ref<[Date, Date] | null>(null);
const options = computed(() => [
  { label: t('liveData.range.liveSession'), value: 'liveSession' },
  { label: t('liveData.range.today'), value: 'today' },
  { label: t('liveData.range.last7Days'), value: 'last7Days' },
  { label: t('liveData.range.custom'), value: 'custom' }
]);

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
      :options="options"
      :aria-label="t('liveData.range.label')"
      @change="changeMode"
    />
    <el-date-picker
      v-if="mode === 'custom'"
      v-model="custom"
      type="datetimerange"
      :range-separator="t('liveData.range.separator')"
      :start-placeholder="t('liveData.range.start')"
      :end-placeholder="t('liveData.range.end')"
      :clearable="false"
      @change="changeCustom"
    />
  </div>
</template>
