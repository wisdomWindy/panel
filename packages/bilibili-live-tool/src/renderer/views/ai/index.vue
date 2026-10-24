<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { CopyDocument, Document, Refresh, Setting } from '@element-plus/icons-vue';
import { useRouter } from 'vue-router';
import type {
  AiConfigSaveRequest,
  AiModelProfileId,
  AiModelProfileSaveRequest,
  AiOutputDeviceId,
  AiVoiceId,
  AiVoiceReplyRules,
  AsrModelId
} from '@/platform/desktop';
import { useAIStore } from '@/stores/ai';
import AiModelProfilesDialog from './components/AiModelProfilesDialog.vue';
import AiPersonaConfigDialog from './components/AiPersonaConfigDialog.vue';
import AiVoiceReplyControl from './components/AiVoiceReplyControl.vue';
import AsrModelDownloadDialog from './components/AsrModelDownloadDialog.vue';

const router = useRouter();
const store = useAIStore();
const configVisible = ref(false);
const modelsVisible = ref(false);
const asrDialogVisible = ref(false);
const result = computed(() => {
  const generation = store.generation;
  return generation?.status === 'streaming' || generation?.status === 'completed'
    ? generation.result
    : '';
});
const generationError = computed(() => {
  if (store.generation?.errorCode === 'rate_limited') return '服务限流，请稍后重试';
  return store.errorFor('generate')?.message ?? store.generation?.errorCode ?? null;
});
const asrEnabled = computed(
  () => store.asr?.state === 'listening' || store.asr?.state === 'starting'
);
const asrLabel = computed(
  () =>
    ({
      unavailable: '不可用',
      modelMissing: '模型未下载',
      downloading: '下载中',
      ready: '已就绪',
      starting: '启动中',
      listening: '识别中',
      error: '异常'
    })[store.asr?.state ?? 'unavailable']
);
const asrErrorMessage = computed(() => {
  const messages: Readonly<Record<string, string>> = {
    asr_engine_initialization_failed: '语音识别引擎初始化失败，请检查模型文件',
    asr_microphone_unavailable: '无法打开默认麦克风，请检查设备和麦克风权限',
    asr_audio_capture_failed: '麦克风采集中断，请检查设备后重新开启',
    asr_audio_format_unsupported: '默认麦克风的音频格式暂不支持',
    asr_inference_failed: '语音识别失败，请重新开启'
  };
  if (store.asr?.errorCode && messages[store.asr.errorCode]) return messages[store.asr.errorCode];
  if (store.asr?.errorCode === 'asr_engine_unavailable') {
    return '模型已就绪，但语音识别引擎暂不可用';
  }
  if (store.asr?.errorCode === 'asr_model_missing') return null;
  return store.asr?.state === 'error' ? (store.error?.message ?? '语音识别启动失败') : null;
});

async function copyResult(): Promise<void> {
  if (!result.value) return;
  try {
    await navigator.clipboard.writeText(result.value);
    ElMessage.success('已复制');
  } catch {
    ElMessage.error('复制失败');
  }
}

async function saveConfig(request: AiConfigSaveRequest): Promise<void> {
  try {
    await store.saveConfig(request);
    configVisible.value = false;
    ElMessage.success('配置已保存');
  } catch {
    // The dialog renders the normalized desktop error.
  }
}

async function saveProfile(request: AiModelProfileSaveRequest): Promise<void> {
  try {
    const profile = await store.saveProfile(request);
    if (!store.profiles.some(value => value.isDefault)) {
      await store.setDefaultProfile(profile.profileId);
    }
    ElMessage.success('模型档案已保存');
  } catch {
    // The dialog renders the normalized desktop error.
  }
}

async function deleteProfile(profileId: AiModelProfileId): Promise<void> {
  try {
    await store.deleteProfile(profileId);
    ElMessage.success('模型档案已删除');
  } catch {
    // The dialog renders the normalized desktop error.
  }
}

async function setDefaultProfile(profileId: AiModelProfileId): Promise<void> {
  await store.setDefaultProfile(profileId).catch(() => undefined);
}

async function testProfile(profileId: AiModelProfileId): Promise<void> {
  try {
    await store.testProfile(profileId);
    ElMessage.success('模型连接正常');
  } catch {
    ElMessage.error(store.error?.message ?? '模型连接失败');
  }
}

async function setVoiceReplyEnabled(isEnabled: boolean, confirmed: boolean): Promise<void> {
  await store.setVoiceReplyEnabled(isEnabled, confirmed).catch(() => undefined);
}

async function confirmAudioRoute(
  deviceId: AiOutputDeviceId,
  voiceId: AiVoiceId,
  volume: number,
  enableAfterSave: boolean
): Promise<void> {
  try {
    await store.setOutputDevice(deviceId, voiceId, volume, true);
    if (enableAfterSave) await store.setVoiceReplyEnabled(true, true);
    ElMessage.success('音频路由已确认');
  } catch {
    // The page readiness alert renders the normalized desktop error.
  }
}

async function testAudioRoute(
  deviceId: AiOutputDeviceId,
  voiceId: AiVoiceId,
  volume: number
): Promise<void> {
  await store.testOutputDevice(deviceId, voiceId, volume).catch(() => undefined);
}

async function updateVoiceRules(rules: AiVoiceReplyRules): Promise<void> {
  await store.updateVoiceRules(rules).catch(() => undefined);
}

async function toggleAsr(value: boolean | string | number): Promise<void> {
  if (!value) {
    await store.stopAsr().catch(() => undefined);
    return;
  }
  const hasInstalledModel = store.asrModels.some(model => model.isInstalled);
  if (!hasInstalledModel) {
    asrDialogVisible.value = true;
    return;
  }
  try {
    await store.startAsr();
  } catch {
    ElMessage.error(asrErrorMessage.value ?? '语音识别启动失败');
  }
}

async function downloadModel(modelId: AsrModelId, accepted: boolean): Promise<void> {
  try {
    await store.downloadModel(modelId, accepted);
    asrDialogVisible.value = false;
  } catch {
    // Download state and normalized error remain owned by the AI store.
  }
}

onMounted(() => {
  void store.bootstrap().catch(() => undefined);
});
</script>

<template>
  <div class="mx-auto w-full max-w-5xl">
    <header class="mb-5 flex flex-wrap items-center justify-between gap-3">
      <div>
        <h2 class="m-0 text-xl font-semibold text-gray-900 dark:text-gray-100">
          AI 助手
        </h2>
        <p class="mb-0 mt-1 text-sm text-gray-500">
          自动生成并播报直播互动回复
        </p>
      </div>
      <div class="flex gap-2">
        <el-button :icon="Document" @click="router.push('/ai/log')">
          日志
        </el-button>
        <el-button :icon="Setting" @click="configVisible = true">
          人设配置
        </el-button>
      </div>
    </header>

    <AiVoiceReplyControl
      :snapshot="store.voiceReply"
      :profiles="store.profiles"
      :pending="store.pendingOperations"
      @open-models="modelsVisible = true"
      @set-enabled="setVoiceReplyEnabled"
      @confirm-route="confirmAudioRoute"
      @test-output="testAudioRoute"
      @update-rules="updateVoiceRules"
      @pause="store.pauseVoiceQueue"
      @skip="store.skipVoiceQueue"
      @clear="store.clearVoiceQueue"
    />

    <el-alert
      v-if="store.error && !store.config"
      class="mb-4"
      type="error"
      :closable="false"
      :title="store.error.message"
      show-icon
    >
      <el-button size="small" :icon="Refresh" @click="store.bootstrap()">
        重试
      </el-button>
    </el-alert>

    <section
      class="flex flex-wrap items-center gap-4 border-y border-gray-200 bg-white px-4 py-3 dark:border-gray-700 dark:bg-gray-900"
    >
      <span class="font-medium">ASR</span>
      <el-switch
        :model-value="asrEnabled"
        :loading="store.pendingOperations.has('asr')"
        @change="toggleAsr"
      />
      <el-tag effect="plain">
        {{ asrLabel }}
      </el-tag>
      <span class="text-sm text-gray-500">{{ store.asr?.modelName ?? '-' }}</span>
      <span class="text-sm text-gray-500">{{ store.asr?.language ?? '-' }}</span>
      <span v-if="asrErrorMessage" class="text-sm text-red-600 dark:text-red-400">
        {{ asrErrorMessage }}
      </span>
    </section>

    <section v-if="store.asrTranscript" class="mt-4" aria-live="polite">
      <h3 class="mb-2 text-base font-semibold">
        语音识别结果
      </h3>
      <el-input
        :model-value="store.asrTranscript"
        type="textarea"
        readonly
        :rows="3"
      />
    </section>

    <section class="mt-5 border-t border-gray-200 pt-4 dark:border-gray-700">
      <div class="mb-2 flex flex-wrap items-center justify-between gap-2">
        <h3 class="m-0 text-base font-semibold">
          结果·仅本地预览，不发送
        </h3>
        <el-button :icon="CopyDocument" :disabled="!result" @click="copyResult">
          复制
        </el-button>
      </div>
      <el-alert
        v-if="generationError"
        class="mb-3"
        type="error"
        :closable="false"
        :title="generationError"
        show-icon
      />
      <el-input
        :model-value="result"
        type="textarea"
        readonly
        :rows="8"
        resize="vertical"
      />
    </section>

    <AiPersonaConfigDialog
      v-model:visible="configVisible"
      :config="store.config"
      :pending="store.pendingOperations.has('saveConfig')"
      :error-message="store.errorFor('saveConfig')?.message ?? null"
      @save="saveConfig"
    />
    <AiModelProfilesDialog
      v-model:visible="modelsVisible"
      :profiles="store.profiles"
      :pending="store.pendingOperations.has('saveProfile')"
      :error-message="store.errorFor('saveProfile', 'deleteProfile', 'setDefaultProfile', 'testProfile')?.message ?? null"
      @save="saveProfile"
      @delete="deleteProfile"
      @set-default="setDefaultProfile"
      @test="testProfile"
    />
    <AsrModelDownloadDialog
      v-model:visible="asrDialogVisible"
      :models="store.asrModels"
      :status="store.asr"
      :pending="store.pendingOperations.has('downloadModel')"
      :error-message="store.errorFor('downloadModel')?.message ?? null"
      @download="downloadModel"
    />
  </div>
</template>
