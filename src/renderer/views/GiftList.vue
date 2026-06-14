<script setup lang="ts">
import { computed } from 'vue';
import { useDanmakuStore } from '../stores/danmaku';

const danmakuStore = useDanmakuStore();

const totalValue = computed(() => {
  return danmakuStore.giftList.reduce((sum, gift) => sum + gift.price * gift.count, 0);
});

const maxValue = computed(() => {
  if (danmakuStore.giftList.length === 0) return 0;
  return Math.max(...danmakuStore.giftList.map(gift => gift.price * gift.count));
});

function handleClear() {
  ElMessageBox.confirm('确定要清空礼物列表吗？', '提示', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning'
  })
    .then(() => {
      danmakuStore.clearGifts();
      ElMessage.success('已清空列表');
    })
    .catch(() => {});
}

function formatTime(timestamp: number) {
  return new Date(timestamp).toLocaleTimeString('zh-CN', {
    hour12: false,
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  });
}
</script>

<template>
  <div class="p-8">
    <el-card>
      <template #header>
        <div class="flex justify-between items-center">
          <span class="font-semibold text-gray-700">礼物列表</span>
          <div class="flex gap-2">
            <el-button @click="handleClear">
              清空列表
            </el-button>
          </div>
        </div>
      </template>

      <div class="mb-4">
        <el-row :gutter="20">
          <el-col :span="6">
            <div class="p-4 bg-gray-50 rounded-lg">
              <el-statistic title="总礼物数" :value="danmakuStore.giftList.length" />
            </div>
          </el-col>
          <el-col :span="6">
            <div class="p-4 bg-gray-50 rounded-lg">
              <el-statistic title="总价值" :value="totalValue" suffix="银瓜子" />
            </div>
          </el-col>
          <el-col :span="6">
            <div class="p-4 bg-gray-50 rounded-lg">
              <el-statistic title="最高价值" :value="maxValue" suffix="银瓜子" />
            </div>
          </el-col>
          <el-col :span="6">
            <div class="p-4 bg-gray-50 rounded-lg">
              <el-statistic title="感谢发送" :value="0" />
            </div>
          </el-col>
        </el-row>
      </div>

      <div class="max-h-[600px] overflow-y-auto">
        <div
          v-for="(gift, index) in danmakuStore.giftList"
          :key="index"
          class="mb-4 p-4 rounded-lg bg-white shadow-sm border-l-4 border-yellow-400 hover:shadow-md transition-shadow duration-200"
        >
          <div class="flex justify-between items-center mb-2">
            <el-tag type="warning">
              {{ gift.giftName }}
            </el-tag>
            <span class="text-sm text-gray-600">{{ formatTime(gift.timestamp) }}</span>
          </div>
          <div class="flex justify-between items-center">
            <div class="flex items-center gap-2">
              <span class="font-bold text-yellow-600">{{ gift.username }}</span>
              <span class="font-bold">x{{ gift.count }}</span>
            </div>
            <div class="flex items-center gap-2">
              <span class="font-bold text-red-600">{{ gift.price }} 银瓜子</span>
              <el-tag v-if="gift.isCombo" type="danger" size="small">
                连击
              </el-tag>
            </div>
          </div>
        </div>
        <el-empty v-if="danmakuStore.giftList.length === 0" description="暂无礼物记录" />
      </div>
    </el-card>
  </div>
</template>
