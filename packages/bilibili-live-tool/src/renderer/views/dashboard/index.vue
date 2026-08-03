<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { Monitor, User, ChatDotRound, Present } from '@element-plus/icons-vue';
import { init, use, type ECharts } from 'echarts/core';
import { BarChart, LineChart } from 'echarts/charts';
import { GridComponent, TitleComponent, TooltipComponent } from 'echarts/components';
import { CanvasRenderer } from 'echarts/renderers';
import { useRoomStore } from '../../stores/room';
import { useDanmakuStore } from '../../stores/danmaku';
import StatCard from './components/StatCard.vue';
import Leaderboard from './components/Leaderboard.vue';

use([BarChart, LineChart, GridComponent, TitleComponent, TooltipComponent, CanvasRenderer]);

const { t } = useI18n();
const roomStore = useRoomStore();
const danmakuStore = useDanmakuStore();

const onlineChartRef = ref<HTMLDivElement>();
const danmakuChartRef = ref<HTMLDivElement>();
const giftChartRef = ref<HTMLDivElement>();

let onlineChart: ECharts | null = null;
let danmakuChart: ECharts | null = null;
let giftChart: ECharts | null = null;

const onlineHistory = ref<number[]>(Array.from({ length: 10 }, () => roomStore.onlineCount || 0));
const danmakuHistory = ref<number[]>(Array.from({ length: 10 }, () => 0));
const timeLabels = ref<string[]>(Array.from({ length: 10 }, (_, i) => {
  const d = new Date();
  d.setMinutes(d.getMinutes() - (9 - i));
  return d.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit', second: '2-digit' });
}));

let updateTimer: ReturnType<typeof setInterval> | null = null;

const totalValue = computed(() => {
  return danmakuStore.giftList.reduce((sum, gift) => sum + gift.price * gift.count, 0);
});

const maxValue = computed(() => {
  if (danmakuStore.giftList.length === 0) return 0;
  return Math.max(...danmakuStore.giftList.map(gift => gift.price * gift.count));
});

const giftTypes = computed(() => {
  const types = new Set(danmakuStore.giftList.map(gift => gift.giftName));
  return Array.from(types).join(', ');
});

const recentActivities = computed(() => {
  const recentDanmaku = danmakuStore.danmakuList
    .slice(0, 5)
    .map(d => ({
      type: 'danmaku' as const,
      message: `${d.username}: ${d.content}`,
      timestamp: d.timestamp
    }));

  const recentGift = danmakuStore.giftList.slice(0, 5).map(g => ({
    type: 'gift' as const,
    message: `${g.username} ${t('dashboard.giftMessage', { giftName: g.giftName, count: g.count })}`,
    timestamp: g.timestamp
  }));

  return [...recentDanmaku, ...recentGift].sort((a, b) => b.timestamp - a.timestamp).slice(0, 10);
});

const giftRankData = computed(() => {
  const userGifts: Record<string, number> = {};
  for (const gift of danmakuStore.giftList) {
    userGifts[gift.username] = (userGifts[gift.username] || 0) + gift.price * gift.count;
  }
  return Object.entries(userGifts)
    .sort(([, a], [, b]) => b - a)
    .slice(0, 5);
});

function formatTime(timestamp: number) {
  return new Date(timestamp).toLocaleTimeString('zh-CN', {
    hour12: false,
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  });
}

function formatRoomNameStatistic() {
  return roomStore.roomInfo?.roomName || t('dashboard.disconnected');
}

function initOnlineChart() {
  if (!onlineChartRef.value) return;
  onlineChart = init(onlineChartRef.value);
  onlineChart.setOption({
    title: { text: t('dashboard.onlineTrend'), left: 'center', textStyle: { fontSize: 14 } },
    tooltip: { trigger: 'axis' },
    xAxis: { type: 'category', data: timeLabels.value },
    yAxis: { type: 'value', name: t('dashboard.people') },
    series: [{
      name: t('dashboard.onlineCount'),
      type: 'line',
      data: onlineHistory.value,
      smooth: true,
      areaStyle: { color: 'rgba(64,158,255,0.3)' }
    }]
  });
}

function initDanmakuChart() {
  if (!danmakuChartRef.value) return;
  danmakuChart = init(danmakuChartRef.value);
  danmakuChart.setOption({
    title: { text: t('dashboard.danmakuTrend'), left: 'center', textStyle: { fontSize: 14 } },
    tooltip: { trigger: 'axis' },
    xAxis: { type: 'category', data: timeLabels.value },
    yAxis: { type: 'value', name: t('dashboard.count') },
    series: [{
      name: t('dashboard.danmakuCount'),
      type: 'line',
      data: danmakuHistory.value,
      smooth: true,
      areaStyle: { color: 'rgba(103,194,58,0.3)' }
    }]
  });
}

function initGiftChart() {
  if (!giftChartRef.value) return;
  giftChart = init(giftChartRef.value);
  const data = giftRankData.value;
  giftChart.setOption({
    title: { text: t('dashboard.giftRank'), left: 'center', textStyle: { fontSize: 14 } },
    tooltip: { trigger: 'axis', axisPointer: { type: 'shadow' } },
    xAxis: { type: 'category', data: data.map(d => d[0]) },
    yAxis: { type: 'value', name: t('dashboard.silverCoin') },
    series: [{
      name: t('dashboard.totalValue'),
      type: 'bar',
      data: data.map(d => d[1]),
      itemStyle: { color: '#ffa500' }
    }]
  });
}

function updateCharts() {
  timeLabels.value = [...timeLabels.value.slice(1), new Date().toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit', second: '2-digit' })];

  const newOnline = roomStore.onlineCount;
  onlineHistory.value = [...onlineHistory.value.slice(1), Math.max(0, newOnline)];

  const newDanmaku = danmakuStore.danmakuList.length;
  danmakuHistory.value = [...danmakuHistory.value.slice(1), newDanmaku];

  if (onlineChart) {
    onlineChart.setOption({
      xAxis: { data: timeLabels.value },
      series: [{ data: onlineHistory.value }]
    });
  }
  if (danmakuChart) {
    danmakuChart.setOption({
      xAxis: { data: timeLabels.value },
      series: [{ data: danmakuHistory.value }]
    });
  }
  if (giftChart) {
    const data = giftRankData.value;
    giftChart.setOption({
      xAxis: { data: data.map(d => d[0]) },
      series: [{ data: data.map(d => d[1]) }]
    });
  }
}

onMounted(async () => {
  await nextTick();
  initOnlineChart();
  initDanmakuChart();
  initGiftChart();

  updateTimer = setInterval(() => {
    updateCharts();
  }, 5000);

  window.addEventListener('resize', () => {
    onlineChart?.resize();
    danmakuChart?.resize();
    giftChart?.resize();
  });
});

watch(() => danmakuStore.giftList, () => {
  updateCharts();
}, { deep: true });

onUnmounted(() => {
  if (updateTimer) clearInterval(updateTimer);
  onlineChart?.dispose();
  danmakuChart?.dispose();
  giftChart?.dispose();
});
</script>

<template>
  <div class="p-8">
    <el-row :gutter="20">
      <el-col :span="24">
        <el-card>
          <template #header>
            <div class="flex justify-between items-center">
              <span class="font-semibold text-gray-700">系统概览</span>
              <el-tag :type="roomStore.isConnected ? 'success' : 'danger'">
                {{ roomStore.isConnected ? t('dashboard.connected') : t('dashboard.disconnected') }}
              </el-tag>
            </div>
          </template>

          <el-row :gutter="20">
            <el-col :span="6">
              <div class="p-4 bg-gray-50 rounded-lg">
                <el-statistic :title="t('dashboard.room')" :value="0" :formatter="formatRoomNameStatistic">
                  <template #prefix>
                    <el-icon><Monitor /></el-icon>
                  </template>
                </el-statistic>
              </div>
            </el-col>
            <el-col :span="6">
              <div class="p-4 bg-gray-50 rounded-lg">
                <el-statistic :title="t('dashboard.onlineCount')" :value="roomStore.onlineCount">
                  <template #prefix>
                    <el-icon><User /></el-icon>
                  </template>
                </el-statistic>
              </div>
            </el-col>
            <el-col :span="6">
              <div class="p-4 bg-gray-50 rounded-lg">
                <el-statistic :title="t('dashboard.danmakuCount')" :value="danmakuStore.danmakuList.length">
                  <template #prefix>
                    <el-icon><ChatDotRound /></el-icon>
                  </template>
                </el-statistic>
              </div>
            </el-col>
            <el-col :span="6">
              <div class="p-4 bg-gray-50 rounded-lg">
                <el-statistic :title="t('dashboard.giftCount')" :value="danmakuStore.giftList.length">
                  <template #prefix>
                    <el-icon><Present /></el-icon>
                  </template>
                </el-statistic>
              </div>
            </el-col>
          </el-row>
        </el-card>
      </el-col>
    </el-row>

    <el-row :gutter="20" class="mt-4">
      <el-col :span="24">
        <el-card>
          <template #header>
            <span>{{ t('dashboard.giftStats') }}</span>
          </template>
          <div class="grid grid-cols-3 gap-5">
            <StatCard :label="t('dashboard.totalValue')" :value="`${totalValue} ${t('dashboard.silverCoin')}`" />
            <StatCard :label="t('dashboard.maxValue')" :value="`${maxValue} ${t('dashboard.silverCoin')}`" />
            <StatCard :label="t('dashboard.giftType')" :value="giftTypes" />
          </div>
        </el-card>
      </el-col>
    </el-row>

    <el-row :gutter="20" class="mt-4">
      <el-col :span="24">
        <el-card>
          <template #header>
            <span class="font-semibold text-gray-700">{{ t('dashboard.recentActivity') }}</span>
          </template>
          <div class="max-h-[300px] overflow-y-auto">
            <div v-for="(activity, index) in recentActivities" :key="index" class="flex items-center py-3 border-b border-gray-100 last:border-b-0 hover:bg-blue-50 transition-colors duration-200 rounded px-2">
              <div class="mr-3 text-xl">
                <el-icon :class="activity.type === 'danmaku' ? 'text-blue-500' : 'text-yellow-500'">
                  <ChatDotRound v-if="activity.type === 'danmaku'" />
                  <Present v-else />
                </el-icon>
              </div>
              <div class="flex-1">
                <div class="text-base text-gray-800 mb-1">
                  {{ activity.message }}
                </div>
                <div class="text-sm text-gray-500">
                  {{ formatTime(activity.timestamp) }}
                </div>
              </div>
            </div>
            <el-empty v-if="recentActivities.length === 0" :description="t('dashboard.noActivity')" />
          </div>
        </el-card>
      </el-col>
    </el-row>

    <el-row :gutter="20" class="mt-4">
      <el-col :span="12">
        <el-card>
          <div ref="onlineChartRef" class="h-[300px]" />
        </el-card>
      </el-col>
      <el-col :span="12">
        <el-card>
          <div ref="danmakuChartRef" class="h-[300px]" />
        </el-card>
      </el-col>
    </el-row>

    <el-row :gutter="20" class="mt-4">
      <el-col :span="24">
        <el-card>
          <div ref="giftChartRef" class="h-[300px]" />
        </el-card>
      </el-col>
    </el-row>

    <el-row :gutter="20" class="mt-4">
      <el-col :span="24">
        <Leaderboard />
      </el-col>
    </el-row>
  </div>
</template>
