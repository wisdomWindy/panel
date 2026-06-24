<script setup lang="ts">
import { reactive, computed } from 'vue';
import { useRoomStore } from '../../stores/room';
import { useConfigStore } from '../../stores/config';
import { getDanmakuService } from '../../services/danmaku-service';

const danmakuService = getDanmakuService();

const roomStore = useRoomStore();
const configStore = useConfigStore();

const form = reactive({
  cookie: '',
  autoReconnect: configStore.config.autoReconnect,
  reconnectInterval: configStore.config.reconnectInterval,
  maxReconnectAttempts: configStore.config.maxReconnectAttempts
});

const isConnected = computed(() => roomStore.isConnected);

function saveConfig() {
  configStore.updateConfig({
    autoReconnect: form.autoReconnect,
    reconnectInterval: form.reconnectInterval,
    maxReconnectAttempts: form.maxReconnectAttempts
  });
  ElMessage.success('配置已保存');
}

async function handleDisconnect() {
  try {
    await danmakuService.disconnect();
    ElMessage.info('已断开连接');
  } catch (error) {
    ElMessage.error(`断开连接失败: ${error}`);
  }
}
</script>

<template>
  <div class="p-8">
    <el-card>
      <template #header>
        <div class="flex justify-between items-center">
          <span class="font-semibold text-gray-700">连接配置</span>
          <el-tag :type="isConnected ? 'success' : 'info'">
            {{ isConnected ? '已连接' : '未连接' }}
          </el-tag>
        </div>
      </template>

      <el-form :model="form" label-width="140px">
        <el-form-item label="Cookie">
          <el-input
            v-model="form.cookie"
            type="textarea"
            :rows="6"
            placeholder="请输入 Cookie（可选，用于发送弹幕）"
          />
        </el-form-item>

        <el-form-item label="自动重连">
          <el-switch v-model="form.autoReconnect" />
        </el-form-item>

        <el-form-item label="重连间隔">
          <el-input-number
            v-model="form.reconnectInterval"
            :min="1000"
            :max="60000"
            :step="1000"
          />
          <span class="ml-2">毫秒</span>
        </el-form-item>

        <el-form-item label="最大重连次数">
          <el-input-number
            v-model="form.maxReconnectAttempts"
            :min="1"
            :max="20"
          />
        </el-form-item>

        <el-form-item>
          <el-button type="primary" @click="saveConfig">
            保存配置
          </el-button>
          <el-button
            v-if="isConnected"
            type="danger"
            class="ml-2"
            @click="handleDisconnect"
          >
            断开连接
          </el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <el-card v-if="isConnected" class="mt-5">
      <template #header>
        <span>连接信息</span>
      </template>

      <el-descriptions :column="2" border>
        <el-descriptions-item label="房间号">
          {{ roomStore.currentRoomId }}
        </el-descriptions-item>
        <el-descriptions-item label="在线人数">
          {{ roomStore.onlineCount.toLocaleString() }}
        </el-descriptions-item>
        <el-descriptions-item label="用户ID">
          {{ roomStore.uid || '未登录' }}
        </el-descriptions-item>
        <el-descriptions-item label="CSRF Token">
          {{ roomStore.csrfToken ? '已设置' : '未设置' }}
        </el-descriptions-item>
      </el-descriptions>
    </el-card>
  </div>
</template>
