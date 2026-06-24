<script setup lang="ts">
import { ref } from 'vue';
import {
  Connection, ChatDotRound, Timer, Filter, Document,
  UserFilled, Bell, Setting, VideoCamera
} from '@element-plus/icons-vue';
import { useConfigStore } from '../stores/config';
import { getDanmakuService } from '../services/danmaku-service';
import { showSuccess } from '../utils/ui';

import ConnectionSettings from '../components/settings/ConnectionSettings.vue';
import DanmakuSettings from '../components/settings/DanmakuSettings.vue';
import AutoSendSettings from '../components/settings/AutoSendSettings.vue';
import FilterSettings from '../components/settings/FilterSettings.vue';
import LogSettings from '../components/settings/LogSettings.vue';
import UpdateSettings from '../components/settings/UpdateSettings.vue';
import PluginSettings from '../components/settings/PluginSettings.vue';
import LanguageSettings from '../components/settings/LanguageSettings.vue';
import TTSSettings from '../components/settings/TTSSettings.vue';
import ASRSettings from '../components/settings/ASRSettings.vue';
import AISettings from '../components/settings/AISettings.vue';
import ModerationSettings from '../components/settings/ModerationSettings.vue';
import RecordingSettings from '../components/settings/RecordingSettings.vue';
import RoleSettings from '../components/settings/RoleSettings.vue';

const danmakuService = getDanmakuService();
const configStore = useConfigStore();

const activeTab = ref('connection');
const importDialogVisible = ref(false);
const importConfig = ref('');

// 子组件引用
const connectionRef = ref<InstanceType<typeof ConnectionSettings>>();
const danmakuRef = ref<InstanceType<typeof DanmakuSettings>>();
const autoRef = ref<InstanceType<typeof AutoSendSettings>>();
const filterRef = ref<InstanceType<typeof FilterSettings>>();
const logRef = ref<InstanceType<typeof LogSettings>>();
const updateRef = ref<InstanceType<typeof UpdateSettings>>();
const ttsRef = ref<InstanceType<typeof TTSSettings>>();
const asrRef = ref<InstanceType<typeof ASRSettings>>();
const aiRef = ref<InstanceType<typeof AISettings>>();
const moderationRef = ref<InstanceType<typeof ModerationSettings>>();

const tabTitle = computed(() => {
  const titles: Record<string, string> = {
    connection: '连接设置',
    danmaku: '弹幕设置',
    auto: '自动发送',
    filter: '过滤设置',
    log: '日志设置',
    roles: '角色回复设置',
    update: '更新设置',
    plugins: '插件管理',
    recording: '录制设置',
    language: '语言设置',
    tts: '语音播报',
    asr: '语音识别',
    ai: 'AI 对话',
    moderation: '禁言防护'
  };
  return titles[activeTab.value] || '设置';
});

function handleTabSelect(tab: string) {
  activeTab.value = tab;
}

function handleSave() {
  // 收集所有子组件的值并保存
  if (connectionRef.value) {
    configStore.updateConfig(connectionRef.value.getValues());
  }
  if (danmakuRef.value) {
    const values = danmakuRef.value.getValues();
    configStore.updateConfig(values);
    danmakuService.updateMinInterval(values.minSendInterval);
  }
  if (autoRef.value) {
    configStore.updateConfig(autoRef.value.getValues());
  }
  if (filterRef.value) {
    configStore.updateConfig(filterRef.value.getValues());
  }
  if (logRef.value) {
    configStore.updateConfig(logRef.value.getValues());
  }
  if (updateRef.value) {
    configStore.updateConfig(updateRef.value.getValues());
  }
  if (ttsRef.value) {
    const ttsStore = useTTSStore();
    ttsStore.updateConfig(ttsRef.value.getValues());
  }
  if (asrRef.value) {
    const asrStore = useASRStore();
    asrStore.updateConfig(asrRef.value.getValues());
  }
  if (aiRef.value) {
    const aiStore = useAIStore();
    aiStore.updateConfig(aiRef.value.getValues());
  }
  if (moderationRef.value) {
    const moderationStore = useModerationStore();
    moderationStore.updateConfig(moderationRef.value.getValues());
  }

  showSuccess('设置已保存');
}

function handleReset() {
  location.reload();
}

function handleExport() {
  const config = {
    connection: connectionRef.value?.getValues(),
    danmaku: danmakuRef.value?.getValues(),
    auto: autoRef.value?.getValues(),
    filter: filterRef.value?.getValues(),
    log: logRef.value?.getValues()
  };

  const configText = JSON.stringify(config, null, 2);
  const dataBlob = new Blob([configText], { type: 'application/json' });
  const url = URL.createObjectURL(dataBlob);
  const link = document.createElement('a');
  link.href = url;
  link.download = `bilibili-live-config-${Date.now()}.json`;
  link.click();
  URL.revokeObjectURL(url);
  showSuccess('配置已导出');
}

function handleImport() {
  importDialogVisible.value = true;
}

function confirmImport() {
  try {
    JSON.parse(importConfig.value);
    // 导入逻辑需要根据实际需求完善
    importDialogVisible.value = false;
    showSuccess('配置已导入');
  } catch {
    ElMessage.error('配置文件格式错误');
  }
}

// 需要导入的 stores
import { useTTSStore } from '../stores/tts';
import { useASRStore } from '../stores/asr';
import { useAIStore } from '../stores/ai';
import { useModerationStore } from '../stores/moderation';
import { computed } from 'vue';
import { ElMessage } from 'element-plus';
</script>

<template>
  <div class="h-[calc(100vh-100px)] overflow-hidden">
    <el-row :gutter="20" class="h-full">
      <el-col :span="6" class="h-full">
        <el-card class="h-full overflow-y-auto">
          <template #header>
            <span>设置菜单</span>
          </template>

          <el-menu :default-active="activeTab" @select="handleTabSelect">
            <el-menu-item index="connection">
              <el-icon><Connection /></el-icon>
              <span>连接设置</span>
            </el-menu-item>
            <el-menu-item index="danmaku">
              <el-icon><ChatDotRound /></el-icon>
              <span>弹幕设置</span>
            </el-menu-item>
            <el-menu-item index="auto">
              <el-icon><Timer /></el-icon>
              <span>自动发送</span>
            </el-menu-item>
            <el-menu-item index="filter">
              <el-icon><Filter /></el-icon>
              <span>过滤设置</span>
            </el-menu-item>
            <el-menu-item index="roles">
              <el-icon><UserFilled /></el-icon>
              <span>角色回复</span>
            </el-menu-item>
            <el-menu-item index="log">
              <el-icon><Document /></el-icon>
              <span>日志设置</span>
            </el-menu-item>
            <el-menu-item index="update">
              <el-icon><Bell /></el-icon>
              <span>更新设置</span>
            </el-menu-item>
            <el-menu-item index="plugins">
              <el-icon><Setting /></el-icon>
              <span>插件管理</span>
            </el-menu-item>
            <el-menu-item index="recording">
              <el-icon><VideoCamera /></el-icon>
              <span>录制设置</span>
            </el-menu-item>
            <el-menu-item index="language">
              <el-icon><Setting /></el-icon>
              <span>语言设置</span>
            </el-menu-item>
            <el-menu-item index="tts">
              <el-icon><Setting /></el-icon>
              <span>语音播报</span>
            </el-menu-item>
            <el-menu-item index="asr">
              <el-icon><Setting /></el-icon>
              <span>语音识别</span>
            </el-menu-item>
            <el-menu-item index="ai">
              <el-icon><Setting /></el-icon>
              <span>AI 对话</span>
            </el-menu-item>
            <el-menu-item index="moderation">
              <el-icon><Setting /></el-icon>
              <span>禁言防护</span>
            </el-menu-item>
          </el-menu>
        </el-card>
      </el-col>
      <el-col :span="18" class="h-full">
        <el-card class="h-full overflow-y-auto">
          <template #header>
            <span>{{ tabTitle }}</span>
          </template>

          <div v-show="activeTab === 'connection'">
            <ConnectionSettings ref="connectionRef" />
          </div>
          <div v-show="activeTab === 'danmaku'">
            <DanmakuSettings ref="danmakuRef" />
          </div>
          <div v-show="activeTab === 'auto'">
            <AutoSendSettings ref="autoRef" />
          </div>
          <div v-show="activeTab === 'filter'">
            <FilterSettings ref="filterRef" />
          </div>
          <div v-show="activeTab === 'roles'">
            <RoleSettings />
          </div>
          <div v-show="activeTab === 'log'">
            <LogSettings ref="logRef" />
          </div>
          <div v-show="activeTab === 'update'">
            <UpdateSettings ref="updateRef" />
          </div>
          <div v-show="activeTab === 'plugins'">
            <PluginSettings />
          </div>
          <div v-show="activeTab === 'recording'">
            <RecordingSettings />
          </div>
          <div v-show="activeTab === 'language'">
            <LanguageSettings />
          </div>
          <div v-show="activeTab === 'tts'">
            <TTSSettings ref="ttsRef" />
          </div>
          <div v-show="activeTab === 'asr'">
            <ASRSettings ref="asrRef" />
          </div>
          <div v-show="activeTab === 'ai'">
            <AISettings ref="aiRef" />
          </div>
          <div v-show="activeTab === 'moderation'">
            <ModerationSettings ref="moderationRef" />
          </div>

          <div class="mt-5 flex justify-end gap-2">
            <el-button type="primary" @click="handleSave">
              保存设置
            </el-button>
            <el-button @click="handleReset">
              重置
            </el-button>
            <el-button @click="handleExport">
              导出配置
            </el-button>
            <el-button @click="handleImport">
              导入配置
            </el-button>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <el-dialog v-model="importDialogVisible" title="导入配置" width="500px">
      <el-form>
        <el-form-item label="配置文件">
          <el-input
            v-model="importConfig"
            type="textarea"
            :rows="10"
            placeholder="粘贴配置文件内容"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="importDialogVisible = false">取消</el-button>
          <el-button type="primary" @click="confirmImport">导入</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>
