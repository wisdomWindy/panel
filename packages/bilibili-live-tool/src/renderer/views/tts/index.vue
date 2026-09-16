<script setup lang="ts">
import { computed, onMounted, reactive } from 'vue';
import { RefreshLeft, VideoPlay } from '@element-plus/icons-vue';
import type { TtsConfig, TtsEventKind, TtsIdentityKind } from '@/platform/desktop';
import TtsQueuePanel from '@/components/media/TtsQueuePanel.vue';
import { validateTtsTemplate, validateTtsTemplates } from '@/modules/media/validation';
import { useMediaTtsStore } from '@/stores/media/tts';

const store = useMediaTtsStore();
const draft = reactive<TtsConfig>({
  isEnabled: false,
  selectedEngine: 'system',
  selectedVoiceId: null,
  rate: 1,
  volume: 0.8,
  eventKinds: ['danmu', 'gift', 'captain', 'entry'],
  identityKinds: ['admin', 'captain', 'vip', 'ordinary'],
  templates: {
    danmu: '{用户}说：{内容}',
    gift: '感谢{用户}赠送的{礼物}{数量}个',
    captain: '感谢{用户}开通{等级}'
  }
});

const eventOptions: readonly { label: string; value: TtsEventKind }[] = [
  { label: '弹幕', value: 'danmu' },
  { label: '礼物', value: 'gift' },
  { label: '舰长', value: 'captain' },
  { label: '进房', value: 'entry' }
];
const identityOptions: readonly { label: string; value: TtsIdentityKind }[] = [
  { label: '房管', value: 'admin' },
  { label: '舰长', value: 'captain' },
  { label: '老爷', value: 'vip' },
  { label: '普通', value: 'ordinary' }
];
const templateErrors = reactive<Record<keyof TtsConfig['templates'], string | null>>({
  danmu: null,
  gift: null,
  captain: null
});

const availableVoices = computed(() => store.snapshot?.voices.filter(voice =>
  voice.engine === draft.selectedEngine
) ?? []);
const engineUnavailable = computed(() =>
  draft.selectedEngine === 'cloud' && !availableVoices.value.some(voice => voice.isAvailable)
);
const isDirty = computed(() => store.snapshot
  ? JSON.stringify(draft) !== JSON.stringify(store.snapshot.config)
  : false
);
const savePending = computed(() => store.pendingOperations.has('save'));

function applyConfig(config: TtsConfig): void {
  Object.assign(draft, {
    ...config,
    eventKinds: [...config.eventKinds],
    identityKinds: [...config.identityKinds],
    templates: { ...config.templates }
  });
}

function validateTemplate(kind: keyof TtsConfig['templates']): boolean {
  const result = validateTtsTemplate(draft.templates[kind], kind);
  templateErrors[kind] = result.message;
  return result.valid;
}

function validateDraft(): boolean {
  (['danmu', 'gift', 'captain'] as const).forEach(validateTemplate);
  const result = validateTtsTemplates(draft.templates);
  if (!result.valid) ElMessage.error(result.message ?? '播报模板无效');
  return result.valid;
}

async function handleEnabled(value: boolean): Promise<void> {
  try {
    await store.setEnabled(value);
    draft.isEnabled = value;
  } catch (error) {
    draft.isEnabled = !value;
    ElMessage.error((error as { message?: string }).message ?? '操作失败');
  }
}

async function handleSave(): Promise<void> {
  if (!validateDraft()) return;
  try {
    const snapshot = await store.save({
      ...draft,
      eventKinds: [...draft.eventKinds],
      identityKinds: [...draft.identityKinds],
      templates: { ...draft.templates }
    });
    applyConfig(snapshot.config);
    ElMessage.success('语音播报配置已保存');
  } catch (error) {
    ElMessage.error((error as { message?: string }).message ?? '保存失败');
  }
}

async function handleTest(): Promise<void> {
  if (!validateDraft()) return;
  try {
    await store.test({ ...draft, templates: { ...draft.templates } });
  } catch (error) {
    ElMessage.error((error as { message?: string }).message ?? '测试失败');
  }
}

function handleEngineChange(): void {
  if (!availableVoices.value.some(voice => voice.voiceId === draft.selectedVoiceId && voice.isAvailable)) {
    draft.selectedVoiceId = null;
  }
}

async function handleReset(): Promise<void> {
  try {
    await ElMessageBox.confirm('确定恢复默认播报配置？', '重置语音播报', {
      type: 'warning',
      confirmButtonText: '重置',
      cancelButtonText: '取消'
    });
  } catch {
    return;
  }
  applyConfig({
    isEnabled: false,
    selectedEngine: 'system',
    selectedVoiceId: null,
    rate: 1,
    volume: 0.8,
    eventKinds: ['danmu', 'gift', 'captain', 'entry'],
    identityKinds: ['admin', 'captain', 'vip', 'ordinary'],
    templates: {
      danmu: '{用户}说：{内容}',
      gift: '感谢{用户}赠送的{礼物}{数量}个',
      captain: '感谢{用户}开通{等级}'
    }
  });
}

async function handlePause(isPaused: boolean): Promise<void> {
  try {
    await store.pause(isPaused);
  } catch (error) {
    ElMessage.error((error as { message?: string }).message ?? '操作失败');
  }
}

async function handleClear(): Promise<void> {
  try {
    await store.clear();
  } catch (error) {
    ElMessage.error((error as { message?: string }).message ?? '清空失败');
  }
}

onMounted(async () => {
  try {
    const snapshot = await store.load();
    if (snapshot) applyConfig(snapshot.config);
  } catch {
    // The stable error region below owns retry feedback.
  }
});
</script>

<template>
  <div class="mx-auto w-full max-w-6xl">
    <div class="mb-5 flex flex-wrap items-center justify-between gap-3">
      <div>
        <h2 class="m-0 text-xl font-semibold text-gray-900">
          语音播报
        </h2>
        <p v-if="store.snapshot" class="mt-1 text-sm text-gray-500" aria-live="polite">
          {{ store.snapshot.status.state === 'unavailable' ? '当前语音引擎不可用' : '播报状态：' + store.snapshot.status.state }}
        </p>
      </div>
      <el-switch
        v-model="draft.isEnabled"
        active-text="开启"
        inactive-text="关闭"
        :loading="store.pendingOperations.has('setEnabled')"
        @change="handleEnabled"
      />
    </div>

    <el-alert
      v-if="store.error"
      class="mb-4"
      type="error"
      :closable="false"
      :title="store.error.message"
      show-icon
    >
      <el-button size="small" @click="store.refresh()">
        重试
      </el-button>
    </el-alert>

    <el-skeleton v-if="store.pending && !store.snapshot" :rows="8" animated />

    <template v-else-if="store.snapshot">
      <el-form label-position="top" @submit.prevent="handleSave">
        <div class="grid grid-cols-1 gap-x-6 lg:grid-cols-2">
          <el-form-item label="TTS 引擎">
            <el-select v-model="draft.selectedEngine" class="w-full" @change="handleEngineChange">
              <el-option label="系统默认" value="system" />
              <el-option label="云端" value="cloud" :disabled="!store.snapshot.voices.some(voice => voice.engine === 'cloud' && voice.isAvailable)" />
            </el-select>
            <span v-if="engineUnavailable" class="mt-1 text-xs text-amber-700">云端引擎尚未配置</span>
          </el-form-item>
          <el-form-item label="音色">
            <el-select
              v-model="draft.selectedVoiceId"
              class="w-full"
              clearable
              :disabled="availableVoices.length === 0"
            >
              <el-option
                v-for="voice in availableVoices"
                :key="voice.voiceId"
                :label="voice.displayName"
                :value="voice.voiceId"
                :disabled="!voice.isAvailable"
              />
            </el-select>
          </el-form-item>
          <el-form-item label="语速">
            <el-slider
              v-model="draft.rate"
              :min="0.5"
              :max="2"
              :step="0.1"
              show-input
            />
          </el-form-item>
          <el-form-item label="音量">
            <el-slider
              v-model="draft.volume"
              :min="0"
              :max="1"
              :step="0.1"
              show-input
            />
          </el-form-item>
        </div>

        <el-divider content-position="left">
          播报选择
        </el-divider>
        <div class="grid grid-cols-1 gap-x-6 lg:grid-cols-2">
          <el-form-item label="播报内容">
            <el-checkbox-group v-model="draft.eventKinds">
              <el-checkbox v-for="option in eventOptions" :key="option.value" :value="option.value">
                {{ option.label }}
              </el-checkbox>
            </el-checkbox-group>
          </el-form-item>
          <el-form-item label="身份过滤">
            <el-checkbox-group v-model="draft.identityKinds">
              <el-checkbox v-for="option in identityOptions" :key="option.value" :value="option.value">
                {{ option.label }}
              </el-checkbox>
            </el-checkbox-group>
          </el-form-item>
        </div>

        <el-divider content-position="left">
          播报模板
        </el-divider>
        <div class="grid grid-cols-1 gap-x-6 lg:grid-cols-2">
          <el-form-item label="弹幕" :error="templateErrors.danmu">
            <el-input v-model="draft.templates.danmu" @blur="validateTemplate('danmu')" />
          </el-form-item>
          <el-form-item label="礼物" :error="templateErrors.gift">
            <el-input v-model="draft.templates.gift" @blur="validateTemplate('gift')" />
          </el-form-item>
          <el-form-item label="舰长" :error="templateErrors.captain">
            <el-input v-model="draft.templates.captain" @blur="validateTemplate('captain')" />
          </el-form-item>
        </div>

        <div class="mb-5 flex flex-wrap justify-end gap-2">
          <el-button :icon="RefreshLeft" :disabled="savePending" @click="handleReset">
            重置默认
          </el-button>
          <el-button :icon="VideoPlay" :loading="store.pendingOperations.has('test')" @click="handleTest">
            测试
          </el-button>
          <el-button
            native-type="submit"
            type="primary"
            :loading="savePending"
            :disabled="!isDirty"
          >
            保存
          </el-button>
        </div>
      </el-form>

      <TtsQueuePanel
        :status="store.snapshot.status"
        :pause-pending="store.pendingOperations.has('pause')"
        :clear-pending="store.pendingOperations.has('clear')"
        @pause="handlePause"
        @clear="handleClear"
      />
    </template>
  </div>
</template>
