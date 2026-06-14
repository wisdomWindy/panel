<script setup lang="ts">
import { ref } from 'vue';
import { useDanmakuStore } from '../stores/danmaku';
import { getDanmakuService } from '../services/danmaku-service';

const danmakuService = getDanmakuService();

const danmakuStore = useDanmakuStore();
const sendDialogVisible = ref(false);
const sending = ref(false);

const sendForm = ref({
  content: ''
});

function handleSend() {
  if (!danmakuService.isConnected) {
    ElMessage.warning('请先连接直播间');
    return;
  }
  sendDialogVisible.value = true;
}

function confirmSend() {
  if (!sendForm.value.content.trim()) {
    ElMessage.warning('请输入弹幕内容');
    return;
  }

  sending.value = true;
  danmakuService
    .sendDanmaku(sendForm.value.content.trim())
    .then(() => {
      sendDialogVisible.value = false;
      sendForm.value.content = '';
    })
    .catch(error => {
      ElMessage.error(`发送失败: ${error}`);
    })
    .finally(() => {
      sending.value = false;
    });
}

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

function formatEmptyStatistic() {
  return '—';
}
</script>

<template>
  <div class="p-8">
    <el-card>
      <template #header>
        <div class="flex justify-between items-center">
          <span class="font-semibold text-gray-700">弹幕列表</span>
          <div class="flex gap-2">
            <el-button type="primary" @click="handleSend">
              发送弹幕
            </el-button>
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
              <el-statistic title="总弹幕数" :value="danmakuStore.danmakuList.length" />
            </div>
          </el-col>
          <el-col :span="6">
            <div class="p-4 bg-gray-50 rounded-lg">
              <el-statistic title="今日发送" :value="0" />
            </div>
          </el-col>
          <el-col :span="6">
            <div class="p-4 bg-gray-50 rounded-lg">
              <el-statistic title="发送成功率" :value="0" suffix="%" />
            </div>
          </el-col>
          <el-col :span="6">
            <div class="p-4 bg-gray-50 rounded-lg">
              <el-statistic title="最后发送" :value="0" :formatter="formatEmptyStatistic" />
            </div>
          </el-col>
        </el-row>
      </div>

      <div class="max-h-[600px] overflow-y-auto">
        <div
          v-for="(danmaku, index) in danmakuStore.danmakuList"
          :key="index"
          :class="[
            'mb-4 p-4 rounded-lg bg-white shadow-sm border-l-4 border-blue-400 hover:shadow-md transition-shadow duration-200',
            danmaku.type === 'sent' ? 'bg-green-50 border-l-green-400' : 'bg-blue-50 border-l-blue-400'
          ]"
        >
          <div class="flex justify-between items-center mb-2">
            <el-tag :type="danmaku.type === 'sent' ? 'success' : 'info'" size="small">
              {{ danmaku.type === 'sent' ? '已发送' : '收到' }}
            </el-tag>
            <span class="text-sm text-gray-600">{{ formatTime(danmaku.timestamp) }}</span>
          </div>
          <div class="flex items-center mb-2">
            <span class="font-bold text-blue-500 mr-2">{{ danmaku.username || '用户' }}:</span>
            <span class="text-gray-800">{{ danmaku.content }}</span>
          </div>
          <div v-if="danmaku.type === 'sent'" class="flex justify-end">
            <el-tag v-if="danmaku.success" type="success">
              成功
            </el-tag>
            <el-tag v-else type="danger">
              失败: {{ danmaku.message || '未知错误' }}
            </el-tag>
          </div>
        </div>
        <el-empty v-if="danmakuStore.danmakuList.length === 0" description="暂无弹幕" />
      </div>
    </el-card>

    <!-- 发送弹幕对话框 -->
    <el-dialog v-model="sendDialogVisible" title="发送弹幕" width="400px">
      <el-form :model="sendForm" label-width="80px">
        <el-form-item label="弹幕内容">
          <el-input
            v-model="sendForm.content"
            type="textarea"
            :rows="3"
            placeholder="请输入要发送的弹幕内容"
            maxlength="100"
            show-word-limit
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="sendDialogVisible = false">取消</el-button>
          <el-button type="primary" :loading="sending" @click="confirmSend">发送</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>
