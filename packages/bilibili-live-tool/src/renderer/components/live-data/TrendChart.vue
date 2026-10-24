<script setup lang="ts">
import { nextTick, onMounted, onUnmounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { init, use, type ECharts } from 'echarts/core';
import { LineChart } from 'echarts/charts';
import { GridComponent, TooltipComponent } from 'echarts/components';
import { CanvasRenderer } from 'echarts/renderers';
import type { StatTrendPoint } from '@/platform/desktop';
import { buildTrendOption, type TrendMetric } from '@/modules/live-data/charts';

use([LineChart, GridComponent, TooltipComponent, CanvasRenderer]);

const props = defineProps<{
    title: string;
    points: readonly StatTrendPoint[];
    metric: TrendMetric;
    color: string;
  }>();
const chartRoot = ref<HTMLDivElement | null>(null);
const { t } = useI18n();
let chart: ECharts | null = null;
let observer: ResizeObserver | null = null;

function render(): void {
  chart?.setOption(buildTrendOption(props.points, props.metric, props.title, props.color), true);
}

onMounted(async () => {
  await nextTick();
  if (!chartRoot.value) return;
  chart = init(chartRoot.value);
  render();
  observer = new ResizeObserver(() => chart?.resize());
  observer.observe(chartRoot.value);
});

watch(() => props.points, render);

onUnmounted(() => {
  observer?.disconnect();
  chart?.dispose();
});
</script>

<template>
  <section class="border-t border-gray-200 py-4 dark:border-zinc-800">
    <h3 class="m-0 mb-2 text-sm font-medium text-gray-700 dark:text-zinc-200">
      {{ title }}
    </h3>
    <div
      v-if="points.length === 0"
      class="flex min-h-[280px] items-center justify-center text-sm text-gray-400 dark:text-zinc-400"
    >
      {{ t('liveData.empty') }}
    </div>
    <div
      v-else
      ref="chartRoot"
      class="min-h-[280px] w-full"
      role="img"
      :aria-label="title"
    />
  </section>
</template>
