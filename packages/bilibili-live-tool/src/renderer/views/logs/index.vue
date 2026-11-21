<script setup lang="ts">
import { ref, computed, nextTick, onMounted, onUnmounted } from 'vue';
import { Delete, Download, Search, Star } from '@element-plus/icons-vue';
import { ElMessage } from 'element-plus';
import { useI18n } from 'vue-i18n';
import { useConfigStore } from '../../stores/config';

interface LogEntry {
  timestamp: number;
  level: 'debug' | 'info' | 'warn' | 'error';
  message: string;
  isHighlight?: boolean;
}

const configStore = useConfigStore();
const { locale, t } = useI18n();

const logContainer = ref<HTMLElement>();
const filterLevel = ref<LogEntry['level'] | ''>('');
const filterKeyword = ref('');
const autoScroll = ref(true);
const showHighlights = ref(true);

const levelFilters = [
  { value: '' as const, labelKey: 'logs.all' },
  { value: 'debug' as const, labelKey: 'logs.debug' },
  { value: 'info' as const, labelKey: 'logs.info' },
  { value: 'warn' as const, labelKey: 'logs.warn' },
  { value: 'error' as const, labelKey: 'logs.error' }
];

const levelColors: Record<LogEntry['level'], string> = {
  debug: 'var(--app-text-3)',
  info: 'var(--app-blue)',
  warn: 'var(--app-orange)',
  error: 'var(--app-red)'
};

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
  return new Date(timestamp).toLocaleTimeString(locale.value, {
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

  // Cap the buffer at the configured maximum number of rows.
  if (logs.value.length > configStore.config.maxLogLines) {
    logs.value = logs.value.slice(-configStore.config.maxLogLines);
  }

  // Keep the viewport pinned to the newest row while auto-scroll is on.
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
  ElMessage.success(t('logs.cleared'));
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
  ElMessage.success(t('logs.exported'));
}

function handleMarkHighlight() {
  addLog('info', t('logs.highlightMarked'));
  const latest = logs.value[logs.value.length - 1];
  if (latest) {
    latest.isHighlight = true;
  }
  ElMessage.success(t('logs.highlightMarked'));
}

function toggleHighlights() {
  showHighlights.value = !showHighlights.value;
}

function rowStyle(log: LogEntry): Record<string, string> | undefined {
  return log.isHighlight
    ? { background: 'color-mix(in srgb, var(--app-gold) 16%, transparent)' }
    : undefined;
}

onMounted(() => {
  addLog('info', t('logs.started'));
});

onUnmounted(() => {
  addLog('info', t('logs.stopped'));
});
</script>

<template>
  <section class="mx-auto flex w-full max-w-[1180px] min-w-0 flex-col gap-4">
    <header class="flex flex-wrap items-center justify-between gap-3">
      <div class="mr-auto min-w-0">
        <h2 class="app-text m-0 text-lg font-semibold">
          {{ t('logs.title') }}
        </h2>
        <p class="app-muted mt-1 mb-0 text-sm">
          {{ t('logs.pageHint') }}
        </p>
      </div>
      <div class="flex flex-wrap items-center gap-2">
        <el-input
          v-model="filterKeyword"
          class="w-56"
          :placeholder="t('logs.keyword')"
          :aria-label="t('logs.keyword')"
          clearable
        >
          <template #prefix>
            <el-icon><Search /></el-icon>
          </template>
        </el-input>
        <button type="button" class="app-chip" @click="handleMarkHighlight">
          <el-icon><Star /></el-icon>
          {{ t('logs.markHighlight') }}
        </button>
        <button type="button" class="app-chip" @click="handleClear">
          <el-icon><Delete /></el-icon>
          {{ t('logs.clear') }}
        </button>
        <button
          type="button"
          class="app-btn-primary inline-flex h-8 items-center gap-1.5 px-3.5 text-sm"
          @click="handleExport"
        >
          <el-icon><Download /></el-icon>
          {{ t('common.export') }}
        </button>
      </div>
    </header>

    <div class="flex flex-wrap items-center gap-2">
      <span class="app-faint text-xs">{{ t('logs.filterLevel') }}</span>
      <button
        v-for="option in levelFilters"
        :key="option.value || 'all'"
        type="button"
        class="app-chip"
        :aria-pressed="filterLevel === option.value"
        @click="filterLevel = option.value"
      >
        {{ t(option.labelKey) }}
      </button>
      <div class="ml-auto flex flex-wrap items-center gap-3">
        <el-switch v-model="autoScroll" :active-text="t('logs.autoScroll')" />
        <button
          type="button"
          class="app-chip"
          :aria-pressed="showHighlights"
          @click="toggleHighlights"
        >
          {{ t('logs.showHighlights') }}
        </button>
      </div>
    </div>

    <section class="app-card flex min-h-[420px] flex-col overflow-hidden">
      <div class="app-hairline flex flex-wrap items-center justify-between gap-2 border-b px-4 py-2.5">
        <span class="app-muted text-xs font-semibold">{{ t('logs.console') }}</span>
        <span class="app-faint text-xs tabular-nums">
          {{ t('logs.count', { count: filteredLogs.length }) }}
        </span>
      </div>
      <div
        ref="logContainer"
        class="app-surface-2 min-h-0 flex-1 overflow-y-auto p-3 font-mono text-xs leading-5"
        @scroll="handleScroll"
      >
        <div
          v-for="(log, index) in filteredLogs"
          :key="index"
          class="app-hairline flex gap-3 border-b py-1 last:border-b-0"
          :style="rowStyle(log)"
        >
          <span class="app-faint w-[88px] shrink-0 tabular-nums">
            {{ formatTime(log.timestamp) }}
          </span>
          <span
            class="w-14 shrink-0 font-semibold uppercase"
            :style="{ color: levelColors[log.level] }"
          >
            {{ log.level }}
          </span>
          <span class="app-text min-w-0 flex-1 break-all">{{ log.message }}</span>
        </div>
        <div v-if="filteredLogs.length === 0" class="flex min-h-[220px] items-center justify-center">
          <el-empty :image-size="72" :description="t('logs.noLogs')" />
        </div>
      </div>
    </section>
  </section>
</template>
