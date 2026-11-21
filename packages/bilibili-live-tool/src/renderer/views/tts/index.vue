<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { RefreshLeft, VideoPlay } from '@element-plus/icons-vue';
import type { Qwen3Language, Qwen3TtsConfig, TtsConfig, TtsEventKind, TtsIdentityKind } from '@/platform/desktop';
import TtsQueuePanel from '@/components/media/TtsQueuePanel.vue';
import { validateTtsTemplate, validateTtsTemplates } from '@/modules/media/validation';
import { useMediaTtsStore } from '@/stores/media/tts';
import { resolvePreferredVoiceId } from './voice-selection';

const store = useMediaTtsStore();
const { t } = useI18n();
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

const eventOptions = computed<readonly { label: string; value: TtsEventKind }[]>(() => [
  { label: t('tts.events.danmu'), value: 'danmu' },
  { label: t('tts.events.gift'), value: 'gift' },
  { label: t('tts.events.captain'), value: 'captain' },
  { label: t('tts.events.entry'), value: 'entry' }
]);
const identityOptions = computed<readonly { label: string; value: TtsIdentityKind }[]>(() => [
  { label: t('tts.identities.admin'), value: 'admin' },
  { label: t('tts.identities.captain'), value: 'captain' },
  { label: t('tts.identities.vip'), value: 'vip' },
  { label: t('tts.identities.ordinary'), value: 'ordinary' }
]);
const templateErrors = reactive<Record<keyof TtsConfig['templates'], string | null>>({
  danmu: null,
  gift: null,
  captain: null
});

const QWEN3_LANGUAGE_OPTIONS: readonly Qwen3Language[] = [
  'Auto',
  'Chinese',
  'English',
  'Japanese',
  'Korean',
  'German',
  'French',
  'Russian',
  'Portuguese',
  'Spanish',
  'Italian'
];
const qwen3Draft = reactive<Qwen3TtsConfig>({
  isEnabled: false,
  baseUrl: 'http://127.0.0.1:8020',
  voiceId: null,
  language: 'Chinese',
  instruct: '',
  timeoutMs: 8000,
  fallbackToSystem: true,
  aiVoiceId: null,
  aiInstruct: null
});
const qwen3Dirty = computed(() =>
  store.qwen3 ? JSON.stringify(qwen3Draft) !== JSON.stringify(store.qwen3.config) : false
);
const qwen3Health = computed(() => store.qwen3?.health ?? null);
const qwen3VoiceIds = computed(() => store.qwen3?.voiceIds ?? []);
const qwen3SavePending = computed(() => store.pendingOperations.has('qwen3Save'));
const qwen3TestPending = computed(() => store.pendingOperations.has('qwen3Test'));
const qwen3ProbePending = ref(false);

function applyQwen3Config(config: Qwen3TtsConfig): void {
  Object.assign(qwen3Draft, { ...config });
}

function qwen3Payload(): Qwen3TtsConfig {
  return {
    ...qwen3Draft,
    instruct: qwen3Draft.instruct.trim(),
    aiInstruct: qwen3Draft.aiInstruct?.trim() ? qwen3Draft.aiInstruct.trim() : null,
    aiVoiceId: qwen3Draft.aiVoiceId || null,
    voiceId: qwen3Draft.voiceId || null
  };
}

async function handleQwen3Save(): Promise<void> {
  try {
    const snapshot = await store.saveQwen3(qwen3Payload());
    applyQwen3Config(snapshot.config);
    // The cloud voice list follows the service status, so refresh it after saving.
    await store.refresh().catch(() => null);
    ElMessage.success(t('tts.qwen3.saveSuccess'));
  } catch (error) {
    ElMessage.error((error as { message?: string }).message ?? t('common.saveFailed'));
  }
}

async function handleQwen3Probe(): Promise<void> {
  qwen3ProbePending.value = true;
  try {
    const snapshot = await store.loadQwen3();
    if (snapshot) applyQwen3Config(snapshot.config);
  } catch (error) {
    ElMessage.error((error as { message?: string }).message ?? t('tts.qwen3.unreachable'));
  } finally {
    qwen3ProbePending.value = false;
  }
}

async function handleQwen3Test(): Promise<void> {
  try {
    if (qwen3Dirty.value) await store.saveQwen3(qwen3Payload());
    await store.testQwen3();
    ElMessage.success(t('tts.qwen3.testSuccess'));
  } catch (error) {
    ElMessage.error((error as { message?: string }).message ?? t('tts.messages.testFailed'));
  }
}

const availableVoices = computed(
  () => store.snapshot?.voices.filter(voice => voice.engine === draft.selectedEngine) ?? []
);
const engineUnavailable = computed(
  () =>
    draft.selectedEngine === 'cloud' && !availableVoices.value.some(voice => voice.isAvailable)
);
const isDirty = computed(() =>
  store.snapshot ? JSON.stringify(draft) !== JSON.stringify(store.snapshot.config) : false
);
const savePending = computed(() => store.pendingOperations.has('save'));

function applyConfig(config: TtsConfig): void {
  Object.assign(draft, {
    ...config,
    selectedVoiceId: resolvePreferredVoiceId(
      config.selectedEngine,
      config.selectedVoiceId,
      store.snapshot?.voices ?? []
    ),
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
  if (!result.valid) ElMessage.error(result.message ?? t('tts.messages.invalidTemplate'));
  return result.valid;
}

async function handleEnabled(value: boolean): Promise<void> {
  try {
    await store.setEnabled(value);
    draft.isEnabled = value;
  } catch (error) {
    draft.isEnabled = !value;
    ElMessage.error((error as { message?: string }).message ?? t('common.operationFailed'));
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
    ElMessage.success(t('tts.messages.saveSuccess'));
  } catch (error) {
    ElMessage.error((error as { message?: string }).message ?? t('common.saveFailed'));
  }
}

async function handleTest(): Promise<void> {
  if (!validateDraft()) return;
  try {
    await store.test({ ...draft, templates: { ...draft.templates } });
  } catch (error) {
    ElMessage.error((error as { message?: string }).message ?? t('tts.messages.testFailed'));
  }
}

function handleEngineChange(): void {
  const current = availableVoices.value.some(
    voice => voice.voiceId === draft.selectedVoiceId && voice.isAvailable
  )
    ? draft.selectedVoiceId
    : null;
  draft.selectedVoiceId = resolvePreferredVoiceId(
    draft.selectedEngine,
    current,
    store.snapshot?.voices ?? []
  );
}

async function handleReset(): Promise<void> {
  try {
    await ElMessageBox.confirm(t('tts.reset.confirm'), t('tts.reset.title'), {
      type: 'warning',
      confirmButtonText: t('common.reset'),
      cancelButtonText: t('common.cancel')
    });
    const snapshot = await store.save({
      isEnabled: false,
      selectedEngine: 'system',
      selectedVoiceId: resolvePreferredVoiceId('system', null, store.snapshot?.voices ?? []),
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
    applyConfig(snapshot.config);
    ElMessage.success(t('tts.messages.resetSuccess'));
  } catch (error) {
    if (error !== 'cancel' && error !== 'close') {
      ElMessage.error((error as { message?: string }).message ?? t('tts.messages.resetFailed'));
    }
  }
}

async function handlePause(isPaused: boolean): Promise<void> {
  try {
    await store.pause(isPaused);
  } catch (error) {
    ElMessage.error((error as { message?: string }).message ?? t('common.operationFailed'));
  }
}

async function handleClear(): Promise<void> {
  try {
    await store.clear();
  } catch (error) {
    ElMessage.error((error as { message?: string }).message ?? t('tts.messages.clearFailed'));
  }
}

onMounted(async () => {
  try {
    const snapshot = await store.load();
    if (snapshot) applyConfig(snapshot.config);
  } catch {
    // The stable error region below owns retry feedback.
  }
  try {
    const qwen3 = await store.loadQwen3();
    if (qwen3) applyQwen3Config(qwen3.config);
  } catch {
    // An unreachable speech service must not block the broadcast settings.
  }
});
</script>

<template>
  <div class="mx-auto w-full max-w-6xl">
    <div class="mb-5 flex flex-wrap items-center justify-between gap-3">
      <div>
        <h2 class="m-0 text-xl font-semibold text-gray-900 dark:text-zinc-100">
          {{ t('tts.title') }}
        </h2>
        <p v-if="store.snapshot" class="mt-1 text-sm text-gray-500 dark:text-zinc-400" aria-live="polite">
          {{
            store.snapshot.status.state === 'unavailable'
              ? t('tts.engineUnavailable')
              : t('tts.status', { state: store.snapshot.status.state })
          }}
        </p>
      </div>
      <el-switch
        v-model="draft.isEnabled"
        :active-text="t('common.enabled')"
        :inactive-text="t('common.disabled')"
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
        {{ t('common.retry') }}
      </el-button>
    </el-alert>

    <el-skeleton v-if="store.pending && !store.snapshot" :rows="8" animated />

    <template v-else-if="store.snapshot">
      <el-form label-position="top" @submit.prevent="handleSave">
        <div class="grid grid-cols-1 gap-x-6 lg:grid-cols-2">
          <el-form-item :label="t('tts.engine')">
            <el-select v-model="draft.selectedEngine" class="w-full" @change="handleEngineChange">
              <el-option :label="t('tts.systemEngine')" value="system" />
              <el-option
                :label="t('tts.cloudEngine')"
                value="cloud"
                :disabled="
                  !store.snapshot.voices.some(
                    voice => voice.engine === 'cloud' && voice.isAvailable
                  )
                "
              />
            </el-select>
            <span v-if="engineUnavailable" class="mt-1 text-xs text-amber-700 dark:text-amber-400">
              {{ t('tts.cloudUnavailable') }}
            </span>
          </el-form-item>
          <el-form-item :label="t('tts.voice')">
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
          <el-form-item :label="t('tts.rate')">
            <el-slider
              v-model="draft.rate"
              :min="0.5"
              :max="2"
              :step="0.1"
              show-input
            />
          </el-form-item>
          <el-form-item :label="t('tts.volume')">
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
          {{ t('tts.selection') }}
        </el-divider>
        <div class="grid grid-cols-1 gap-x-6 lg:grid-cols-2">
          <el-form-item :label="t('tts.content')">
            <el-checkbox-group v-model="draft.eventKinds">
              <el-checkbox v-for="option in eventOptions" :key="option.value" :value="option.value">
                {{ option.label }}
              </el-checkbox>
            </el-checkbox-group>
          </el-form-item>
          <el-form-item :label="t('tts.identityFilter')">
            <el-checkbox-group v-model="draft.identityKinds">
              <el-checkbox
                v-for="option in identityOptions"
                :key="option.value"
                :value="option.value"
              >
                {{ option.label }}
              </el-checkbox>
            </el-checkbox-group>
          </el-form-item>
        </div>

        <el-divider content-position="left">
          {{ t('tts.templates.title') }}
        </el-divider>
        <div class="grid grid-cols-1 gap-x-6 lg:grid-cols-2">
          <el-form-item :label="t('tts.events.danmu')" :error="templateErrors.danmu">
            <el-input v-model="draft.templates.danmu" @blur="validateTemplate('danmu')" />
          </el-form-item>
          <el-form-item :label="t('tts.events.gift')" :error="templateErrors.gift">
            <el-input v-model="draft.templates.gift" @blur="validateTemplate('gift')" />
          </el-form-item>
          <el-form-item :label="t('tts.events.captain')" :error="templateErrors.captain">
            <el-input v-model="draft.templates.captain" @blur="validateTemplate('captain')" />
          </el-form-item>
        </div>

        <el-divider content-position="left">
          {{ t('tts.qwen3.title') }}
        </el-divider>
        <div class="grid grid-cols-1 gap-x-6 lg:grid-cols-2">
          <el-form-item :label="t('tts.qwen3.enable')">
            <el-switch v-model="qwen3Draft.isEnabled" />
          </el-form-item>
          <el-form-item :label="t('tts.qwen3.fallback')">
            <el-switch v-model="qwen3Draft.fallbackToSystem" />
          </el-form-item>
          <el-form-item :label="t('tts.qwen3.baseUrl')">
            <el-input v-model="qwen3Draft.baseUrl" placeholder="http://127.0.0.1:8020" />
            <span class="mt-1 text-xs text-gray-500 dark:text-zinc-400">
              {{ t('tts.qwen3.baseUrlHint') }}
            </span>
          </el-form-item>
          <el-form-item :label="t('tts.qwen3.language')">
            <el-select v-model="qwen3Draft.language" class="w-full">
              <el-option
                v-for="language in QWEN3_LANGUAGE_OPTIONS"
                :key="language"
                :label="language"
                :value="language"
              />
            </el-select>
          </el-form-item>
          <el-form-item :label="t('tts.qwen3.aiVoice')">
            <el-select v-model="qwen3Draft.aiVoiceId" class="w-full" clearable>
              <el-option
                v-for="voiceId in qwen3VoiceIds"
                :key="voiceId"
                :label="voiceId"
                :value="voiceId"
              />
            </el-select>
            <span class="mt-1 text-xs text-gray-500 dark:text-zinc-400">
              {{ t('tts.qwen3.aiVoiceHint') }}
            </span>
          </el-form-item>
          <el-form-item :label="t('tts.qwen3.timeout')">
            <el-input-number
              v-model="qwen3Draft.timeoutMs"
              class="w-full"
              :min="500"
              :max="60000"
              :step="500"
            />
          </el-form-item>
          <el-form-item class="lg:col-span-2" :label="t('tts.qwen3.instruct')">
            <el-input v-model="qwen3Draft.instruct" type="textarea" :rows="2" />
            <span class="mt-1 text-xs text-gray-500 dark:text-zinc-400">
              {{ t('tts.qwen3.instructHint') }}
            </span>
          </el-form-item>
        </div>
        <div class="mb-3 flex flex-wrap items-center gap-3">
          <el-tag :type="qwen3Health?.ready ? 'success' : 'info'" size="small">
            {{
              qwen3Health?.ready
                ? t('tts.qwen3.ready', { model: qwen3Health?.model ?? '' })
                : qwen3Health
                  ? t('tts.qwen3.loading')
                  : t('tts.qwen3.unreachable')
            }}
          </el-tag>
          <span v-if="qwen3Health?.error" class="text-xs text-amber-700 dark:text-amber-400">
            {{ qwen3Health.error }}
          </span>
        </div>
        <div class="mb-5 flex flex-wrap justify-end gap-2">
          <el-button :loading="qwen3ProbePending" @click="handleQwen3Probe">
            {{ t('tts.qwen3.probe') }}
          </el-button>
          <el-button
            :loading="qwen3TestPending"
            :disabled="!qwen3Draft.isEnabled"
            @click="handleQwen3Test"
          >
            {{ t('common.test') }}
          </el-button>
          <el-button
            type="primary"
            :loading="qwen3SavePending"
            :disabled="!qwen3Dirty"
            @click="handleQwen3Save"
          >
            {{ t('common.save') }}
          </el-button>
        </div>

        <div class="mb-5 flex flex-wrap justify-end gap-2">
          <el-button :icon="RefreshLeft" :disabled="savePending" @click="handleReset">
            {{ t('common.resetDefaults') }}
          </el-button>
          <el-button
            :icon="VideoPlay"
            :loading="store.pendingOperations.has('test')"
            @click="handleTest"
          >
            {{ t('common.test') }}
          </el-button>
          <el-button
            native-type="submit"
            type="primary"
            :loading="savePending"
            :disabled="!isDirty"
          >
            {{ t('common.save') }}
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
