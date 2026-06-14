<script setup lang="ts">
import { ref, computed, nextTick, onMounted, onUnmounted } from 'vue';
import { useConfigStore } from '../stores/config';

const configStore = useConfigStore();

const logContainer = ref<HTMLElement>();
const filterLevel = ref('');
const filterKeyword = ref('');
const autoScroll = ref(true);
const showHighlights = ref(true);

interface LogEntry {
  timestamp: number;
  level: 'debug' | 'info' | 'warn' | 'error';
  message: string;
  isHighlight?: boolean;
}

const logs = ref<LogEntry[]>([]);

const filteredLogs = computed(() => {
  let result = logs.value;

  if (filterLevel.value) {
    result = result.filter(log => log.level === filterLevel.value);
  }

  if (filterKeyword.value) {
    const keyword = filterKeyword.value.toLowerCase();
    result = result.filter(log => log.message.toLowerCase().includes(keyword));
  }

  if (!showHighlights.value) {
    result = result.filter(log => !log.isHighlight);
  }

  return result;
});

function formatTime(timestamp: number) {
  return new Date(timestamp).toLocaleTimeString('zh-CN', {
    hour12: false,
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  });
}

function addLog(level: LogEntry['level'], message: string) {
  if (!configStore.config.enableLog) {
    return;
  }

  const logLevel = configStore.config.logLevel;
  const levelPriority = { debug: 0, info: 1, warn: 2, error: 3 };

  if (levelPriority[level] < levelPriority[logLevel as keyof typeof levelPriority]) {
    return;
  }

  logs.value.push({
    timestamp: Date.now(),
    level,
    message
  });

  // 限制日志行数
  if (logs.value.length > configStore.config.maxLogLines) {
    logs.value = logs.value.slice(-configStore.config.maxLogLines);
  }

  // 自动滚动到底部
  if (autoScroll.value) {
    nextTick(() => {
      scrollToBottom();
    });
  }
}

function scrollToBottom() {
  if (logContainer.value) {
    logContainer.value.scrollTop = logContainer.value.scrollHeight;
  }
}

function handleScroll() {
  if (logContainer.value) {
    const { scrollTop, scrollHeight, clientHeight } = logContainer.value;
    autoScroll.value = scrollTop + clientHeight >= scrollHeight - 50;
  }
}

function handleClear() {
  logs.value = [];
  ElMessage.success('已清空日志');
}

function handleExport() {
  const logsText = filteredLogs.value
    .map(log => `[${formatTime(log.timestamp)}] [${log.level.toUpperCase()}] ${log.message}`)
    .join('\n');

  const dataBlob = new Blob([logsText], { type: 'text/plain' });
  const url = URL.createObjectURL(dataBlob);
  const link = document.createElement('a');
  link.href = url;
  link.download = `bilibili-live-logs-${Date.now()}.txt`;
  link.click();
  URL.revokeObjectURL(url);
  ElMessage.success('导出成功');
}

function handleMarkHighlight() {
  addLog('info', '🎬 高光时刻已标记');
  logs.value[logs.value.length - 1].isHighlight = true;
  ElMessage.success('高光时刻已标记');
}

function toggleHighlights() {
  showHighlights.value = !showHighlights.value;
}

onMounted(() => {
  addLog('info', '日志系统已启动');
});

onUnmounted(() => {
  addLog('info', '日志系统已停止');
});
</script>

<template>
  <div class="p-8 h-[calc(100vh-100px)]">
    <el-card>
      <template #header>
        <div class="flex justify-between items-center">
          <span class="font-semibold text-gray-700">日志监控</span>
          <div class="flex items-center gap-2">
            <el-select
              v-model="filterLevel"
              placeholder="筛选级别"
              width="120"
            >
              <el-option label="全部" value="" />
              <el-option label="Debug" value="debug" />
              <el-option label="Info" value="info" />
              <el-option label="Warning" value="warn" />
              <el-option label="Error" value="error" />
            </el-select>
            <el-input
              v-model="filterKeyword"
              placeholder="关键词"
              width="200"
              clearable
            />
            <el-switch v-model="autoScroll" active-text="自动滚动" />
            <el-button @click="handleClear">
              清空
            </el-button>
            <el-button @click="handleMarkHighlight">
              🎬 打点标记
            </el-button>
            <el-button :type="showHighlights ? 'primary' : 'default'" @click="toggleHighlights">
              显示标记
            </el-button>
            <el-button @click="handleExport">
              导出
            </el-button>
          </div>
        </div>
      </template>

      <div
        ref="logContainer"
        class="h-[calc(100vh-220px)] overflow-y-auto bg-gray-900 rounded-md p-3 font-mono text-sm"
        @scroll="handleScroll"
      >
        <div
          v-for="(log, index) in filteredLogs"
          :key="index"
          :class="[
            'flex py-1 border-b border-gray-800 hover:bg-gray-800',
            `log-${log.level}`,
            log.isHighlight ? 'bg-yellow-900' : ''
          ]"
        >
          <span class="text-gray-600 mr-2 min-w-[150px]">{{ formatTime(log.timestamp) }}</span>
          <span
            :class="[
              'mr-2 min-w-[70px] font-bold',
              log.level === 'debug' && 'text-gray-600',
              log.level === 'info' && 'text-blue-400',
              log.level === 'warn' && 'text-yellow-400',
              log.level === 'error' && 'text-red-500'
            ]"
          >
            {{ log.level.toUpperCase() }}
          </span>
          <span
            :class="[
              'flex-1 break-all',
              log.level === 'debug' && 'text-gray-500',
              log.level === 'info' && 'text-gray-300',
              log.level === 'warn' && 'text-yellow-200',
              log.level === 'error' && 'text-red-300'
            ]"
          >
            {{ log.message }}
          </span>
        </div>
        <el-empty v-if="filteredLogs.length === 0" description="暂无日志" />
      </div>
    </el-card>
  </div>
</template>
