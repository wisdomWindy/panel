<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { useDanmakuStore } from '../../../stores/danmaku';

const danmakuStore = useDanmakuStore();

const activeTab = ref('gift');
const updateTimer = ref<ReturnType<typeof setInterval> | null>(null);

// 排行榜数据
const giftRank = computed(() => danmakuStore.getGiftRank(20));
const danmakuRank = computed(() => danmakuStore.getDanmakuRank(20));
const captainRank = computed(() => danmakuStore.getCaptainRank());

// 格式化时间
function formatTime(timestamp: number) {
  return new Date(timestamp).toLocaleTimeString('zh-CN', {
    hour12: false,
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  });
}

// 获取排名样式
function getRankClass(rank: number) {
  if (rank === 1) return 'bg-yellow-100 text-yellow-800';
  if (rank === 2) return 'bg-gray-100 text-gray-800';
  if (rank === 3) return 'bg-orange-100 text-orange-800';
  return '';
}

// 获取排名图标
function getRankIcon(rank: number) {
  if (rank === 1) return '🥇';
  if (rank === 2) return '🥈';
  if (rank === 3) return '🥉';
  return rank.toString();
}

// 自动更新
onMounted(() => {
  updateTimer.value = setInterval(() => {
    // 数据会自动更新，因为使用的是 computed
  }, 5000);
});

onUnmounted(() => {
  if (updateTimer.value) {
    clearInterval(updateTimer.value);
  }
});
</script>

<template>
  <el-card>
    <template #header>
      <span class="font-semibold text-gray-700">排行榜</span>
    </template>

    <el-tabs v-model="activeTab">
      <!-- 送礼榜 -->
      <el-tab-pane label="送礼榜" name="gift">
        <el-table :data="giftRank" stripe class="w-full">
          <el-table-column label="排名" width="80" align="center">
            <template #default="{ row }">
              <span :class="getRankClass(row.rank)" class="px-2 py-1 rounded">
                {{ getRankIcon(row.rank) }}
              </span>
            </template>
          </el-table-column>
          <el-table-column prop="username" label="用户名" />
          <el-table-column label="送礼价值" width="150" align="right">
            <template #default="{ row }">
              <span class="font-mono">{{ row.totalValue.toLocaleString() }} 电池</span>
            </template>
          </el-table-column>
        </el-table>
        <el-empty v-if="giftRank.length === 0" description="暂无送礼数据" />
      </el-tab-pane>

      <!-- 弹幕榜 -->
      <el-tab-pane label="弹幕榜" name="danmaku">
        <el-table :data="danmakuRank" stripe class="w-full">
          <el-table-column label="排名" width="80" align="center">
            <template #default="{ row }">
              <span :class="getRankClass(row.rank)" class="px-2 py-1 rounded">
                {{ getRankIcon(row.rank) }}
              </span>
            </template>
          </el-table-column>
          <el-table-column prop="username" label="用户名" />
          <el-table-column label="弹幕数量" width="150" align="right">
            <template #default="{ row }">
              <span class="font-mono">{{ row.count }} 条</span>
            </template>
          </el-table-column>
        </el-table>
        <el-empty v-if="danmakuRank.length === 0" description="暂无弹幕数据" />
      </el-tab-pane>

      <!-- 舰长贡献榜 -->
      <el-tab-pane label="舰长贡献榜" name="captain">
        <el-table :data="captainRank" stripe class="w-full">
          <el-table-column label="排名" width="80" align="center">
            <template #default="{ row }">
              <span :class="getRankClass(row.rank)" class="px-2 py-1 rounded">
                {{ getRankIcon(row.rank) }}
              </span>
            </template>
          </el-table-column>
          <el-table-column prop="username" label="用户名" />
          <el-table-column label="舰长等级" width="120" align="center">
            <template #default="{ row }">
              <el-tag :type="row.level === 3 ? 'danger' : row.level === 2 ? 'warning' : 'success'">
                {{ row.levelName }}
              </el-tag>
            </template>
          </el-table-column>
          <el-table-column label="贡献时间" width="150" align="center">
            <template #default="{ row }">
              <span class="font-mono">{{ formatTime(row.timestamp) }}</span>
            </template>
          </el-table-column>
        </el-table>
        <el-empty v-if="captainRank.length === 0" description="暂无舰长数据" />
      </el-tab-pane>
    </el-tabs>
  </el-card>
</template>
