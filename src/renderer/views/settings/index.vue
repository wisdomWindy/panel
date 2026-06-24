<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import {
  Connection, ChatDotRound, Timer, Filter, Document,
  UserFilled, Bell, Setting, VideoCamera
} from '@element-plus/icons-vue';
import { useConfigStore } from '../../stores/config';
import { getDanmakuService } from '../../services/danmaku-service';
import { showSuccess } from '../../utils/ui';
import {
  buildSettingsExportConfig,
  createSettingsImportPatches
} from './utils/config-io.js';

import ConnectionSettings from './components/ConnectionSettings.vue';
import DanmakuSettings from './components/DanmakuSettings.vue';
import AutoSendSettings from './components/AutoSendSettings.vue';
import FilterSettings from './components/FilterSettings.vue';
import LogSettings from './components/LogSettings.vue';
import UpdateSettings from './components/UpdateSettings.vue';
import PluginSettings from './components/PluginSettings.vue';
import LanguageSettings from './components/LanguageSettings.vue';
import TTSSettings from './components/TTSSettings.vue';
import ASRSettings from './components/ASRSettings.vue';
import AISettings from './components/AISettings.vue';
import ModerationSettings from './components/ModerationSettings.vue';
import RecordingSettings from './components/RecordingSettings.vue';
import RoleSettings from './components/RoleSettings.vue';

const danmakuService = getDanmakuService();
const configStore = useConfigStore();
const { t } = useI18n();

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
    connection: t('settings.connection'),
    danmaku: t('settings.danmaku'),
    auto: t('settings.autoSend'),
    filter: t('settings.filter'),
    log: t('settings.log'),
    roles: t('settings.roles'),
    update: t('settings.update'),
    plugins: t('settings.plugins'),
    recording: t('settings.recording'),
    language: t('settings.languageSettings'),
    tts: t('settings.tts'),
    asr: t('settings.asr'),
    ai: t('settings.ai'),
    moderation: t('settings.moderation')
  };
  return titles[activeTab.value] || t('settings.title');
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

  showSuccess(t('settings.saved'));
}

function handleReset() {
  location.reload();
}

function handleExport() {
  const config = buildSettingsExportConfig({
    core: {
      ...configStore.config,
      ...connectionRef.value?.getValues(),
      ...danmakuRef.value?.getValues(),
      ...autoRef.value?.getValues(),
      ...filterRef.value?.getValues(),
      ...logRef.value?.getValues(),
      ...updateRef.value?.getValues()
    },
    recording: { ...useRecordingStore().config },
    tts: { ...useTTSStore().config },
    asr: { ...useASRStore().config },
    ai: { ...useAIStore().config },
    moderation: { ...useModerationStore().config }
  });

  const configText = JSON.stringify(config, null, 2);
  const dataBlob = new Blob([configText], { type: 'application/json' });
  const url = URL.createObjectURL(dataBlob);
  const link = document.createElement('a');
  link.href = url;
  link.download = `bilibili-live-config-${Date.now()}.json`;
  link.click();
  URL.revokeObjectURL(url);
  showSuccess(t('settings.exported'));
}

function handleImport() {
  importDialogVisible.value = true;
}

function confirmImport() {
  try {
    const parsed = JSON.parse(importConfig.value);
    const patches = createSettingsImportPatches(parsed);
    const recordingStore = useRecordingStore();
    const ttsStore = useTTSStore();
    const asrStore = useASRStore();
    const aiStore = useAIStore();
    const moderationStore = useModerationStore();

    if (patches.core) {
      configStore.updateConfig(patches.core);
      if (typeof patches.core.minSendInterval === 'number') {
        danmakuService.updateMinInterval(patches.core.minSendInterval);
      }
    }
    if (patches.recording) {
      void recordingStore.updateConfig(patches.recording);
    }
    if (patches.tts) {
      ttsStore.updateConfig(patches.tts);
    }
    if (patches.asr) {
      asrStore.updateConfig(patches.asr);
    }
    if (patches.ai) {
      aiStore.updateConfig(patches.ai);
    }
    if (patches.moderation) {
      moderationStore.updateConfig(patches.moderation);
    }

    importDialogVisible.value = false;
    showSuccess(t('settings.imported'));
  } catch {
    ElMessage.error(t('settings.invalidImport'));
  }
}

// 需要导入的 stores
import { useTTSStore } from '../../stores/tts';
import { useASRStore } from '../../stores/asr';
import { useAIStore } from '../../stores/ai';
import { useModerationStore } from '../../stores/moderation';
import { useRecordingStore } from '../../stores/recording';
import { computed } from 'vue';
import { ElMessage } from 'element-plus';
</script>

<template>
  <div class="h-[calc(100vh-100px)] overflow-hidden">
    <el-row :gutter="20" class="h-full">
      <el-col :span="6" class="h-full">
        <el-card class="h-full overflow-y-auto">
          <template #header>
            <span>{{ t('settings.menu') }}</span>
          </template>

          <el-menu :default-active="activeTab" @select="handleTabSelect">
            <el-menu-item index="connection">
              <el-icon><Connection /></el-icon>
              <span>{{ t('settings.connection') }}</span>
            </el-menu-item>
            <el-menu-item index="danmaku">
              <el-icon><ChatDotRound /></el-icon>
              <span>{{ t('settings.danmaku') }}</span>
            </el-menu-item>
            <el-menu-item index="auto">
              <el-icon><Timer /></el-icon>
              <span>{{ t('settings.autoSend') }}</span>
            </el-menu-item>
            <el-menu-item index="filter">
              <el-icon><Filter /></el-icon>
              <span>{{ t('settings.filter') }}</span>
            </el-menu-item>
            <el-menu-item index="roles">
              <el-icon><UserFilled /></el-icon>
              <span>{{ t('settings.roles') }}</span>
            </el-menu-item>
            <el-menu-item index="log">
              <el-icon><Document /></el-icon>
              <span>{{ t('settings.log') }}</span>
            </el-menu-item>
            <el-menu-item index="update">
              <el-icon><Bell /></el-icon>
              <span>{{ t('settings.update') }}</span>
            </el-menu-item>
            <el-menu-item index="plugins">
              <el-icon><Setting /></el-icon>
              <span>{{ t('settings.plugins') }}</span>
            </el-menu-item>
            <el-menu-item index="recording">
              <el-icon><VideoCamera /></el-icon>
              <span>{{ t('settings.recording') }}</span>
            </el-menu-item>
            <el-menu-item index="language">
              <el-icon><Setting /></el-icon>
              <span>{{ t('settings.languageSettings') }}</span>
            </el-menu-item>
            <el-menu-item index="tts">
              <el-icon><Setting /></el-icon>
              <span>{{ t('settings.tts') }}</span>
            </el-menu-item>
            <el-menu-item index="asr">
              <el-icon><Setting /></el-icon>
              <span>{{ t('settings.asr') }}</span>
            </el-menu-item>
            <el-menu-item index="ai">
              <el-icon><Setting /></el-icon>
              <span>{{ t('settings.ai') }}</span>
            </el-menu-item>
            <el-menu-item index="moderation">
              <el-icon><Setting /></el-icon>
              <span>{{ t('settings.moderation') }}</span>
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
              {{ t('common.save') }}
            </el-button>
            <el-button @click="handleReset">
              {{ t('common.reset') }}
            </el-button>
            <el-button @click="handleExport">
              {{ t('settings.exportConfig') }}
            </el-button>
            <el-button @click="handleImport">
              {{ t('settings.importConfig') }}
            </el-button>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <el-dialog v-model="importDialogVisible" :title="t('settings.importConfig')" width="500px">
      <el-form>
        <el-form-item :label="t('settings.configFile')">
          <el-input
            v-model="importConfig"
            type="textarea"
            :rows="10"
            :placeholder="t('settings.importPlaceholder')"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="importDialogVisible = false">{{ t('common.cancel') }}</el-button>
          <el-button type="primary" @click="confirmImport">{{ t('common.import') }}</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>
