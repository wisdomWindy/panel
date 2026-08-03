<script setup lang="ts">
import { useDanmakuStore } from '../../stores/danmaku';

const danmakuStore = useDanmakuStore();

function handleClear() {
  ElMessageBox.confirm('确定要清空弹幕列表吗？', '提示', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning'
  })
    .then(() => {
      danmakuStore.clearDanmaku();
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
          <span class="font-semibold text-gray-700">弹幕列表</span>
          <el-button @click="handleClear">
            清空列表
          </el-button>
        </div>
      </template>

      <div class="mb-4">
        <el-row :gutter="20">
          <el-col :span="24">
            <div class="p-4 bg-gray-50 rounded-lg">
              <el-statistic title="总弹幕数" :value="danmakuStore.danmakuList.length" />
            </div>
          </el-col>
        </el-row>
      </div>

      <div class="max-h-[600px] overflow-y-auto">
        <div
          v-for="(danmaku, index) in danmakuStore.danmakuList"
          :key="index"
          class="mb-4 p-4 rounded-lg bg-blue-50 shadow-sm border-l-4 border-blue-400 hover:shadow-md transition-shadow duration-200"
        >
          <div class="flex justify-between items-center mb-2">
            <el-tag type="info" size="small">收到</el-tag>
            <span class="text-sm text-gray-600">{{ formatTime(danmaku.timestamp) }}</span>
          </div>
          <div class="flex items-center mb-2">
            <span class="font-bold text-blue-500 mr-2">{{ danmaku.username || '用户' }}:</span>
            <span class="text-gray-800">{{ danmaku.content }}</span>
          </div>
        </div>
        <el-empty v-if="danmakuStore.danmakuList.length === 0" description="暂无弹幕" />
      </div>
    </el-card>
  </div>
</template>
