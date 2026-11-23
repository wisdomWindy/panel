<script setup lang="ts">
import { computed, onMounted, onUnmounted, reactive, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { Loading, RefreshLeft, VideoPlay } from '@element-plus/icons-vue';
import type { Qwen3Language, Qwen3TtsConfig, TtsConfig, TtsEventKind, TtsIdentityKind } from '@/platform/desktop';
import TtsQueuePanel from '@/components/media/TtsQueuePanel.vue';
import SpeechPrepareDialog from '@/components/media/SpeechPrepareDialog.vue';
import { retriesSpeechInstall } from '@/modules/media/speech-retry';
import { validateTtsTemplate, validateTtsTemplates } from '@/modules/media/validation';
import { useMediaTtsStore } from '@/stores/media/tts';
import { resolvePreferredVoiceId } from './voice-selection';
import { mergeVoiceOptions } from '@/modules/media/qwen3-voices';

const store = useMediaTtsStore();
const { t } = useI18n();
const draft = reactive<TtsConfig>({
  isEnabled: false,
  selectedEngine: 'system',
  selectedVoiceId: null,
  rate: 1,
  volume: 0.8,
  eventKinds: ['danmu', 'gift', 'captain', 'entry'],
  identityKinds: ['admin', 'captain', 'ordinary'],
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
  // `vip` (老爷) is never reported by the long-link protocol, so it would be a
  // filter that can never match; it stays out of the UI for that reason.
  { label: t('tts.identities.ordinary'), value: 'ordinary' }
]);
const templateErrors = reactive<Record<keyof TtsConfig['templates'], string | null>>({
  danmu: null,
  gift: null,
  captain: null
});

const QWEN3_MIRROR_OPTIONS: readonly Qwen3PipMirror[] = ['tsinghua', 'aliyun', 'official'];

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
  aiInstruct: null,
  pipMirror: 'tsinghua'
});
const qwen3Dirty = computed(() =>
  store.qwen3 ? JSON.stringify(qwen3Draft) !== JSON.stringify(store.qwen3.config) : false
);
const qwen3Health = computed(() => store.qwen3?.health ?? null);
// The service reports its speakers when reachable; the upstream CustomVoice
// table is compiled in so the select is never empty (PRD 9.11).
const qwen3VoiceOptions = computed(() => mergeVoiceOptions(store.qwen3?.voiceIds ?? []));
/** Codes that only mean "the model is still warming up", not a failure. */
const WARMUP_CODES: readonly string[] = ['TTS_TIMEOUT', 'TTS_MODEL_LOADING', 'qwen3_tts_unreachable'];

const qwen3IsWarmingUp = computed(() => {
  const health = store.qwen3?.health;
  return !!health && (health.status === 'loading' || (!health.ready && health.error === null));
});

const errorIsWarmup = computed(() => {
  if (!store.error) return false;
  return WARMUP_CODES.includes(store.error.code) || qwen3IsWarmingUp.value;
});

let warmupTimer: ReturnType<typeof setInterval> | null = null;

/** Polls the snapshot while the model loads so the notice clears by itself. */
function watchWarmup(): void {
  if (warmupTimer) return;
  warmupTimer = setInterval(() => {
    if (!qwen3IsWarmingUp.value) {
      if (warmupTimer) {
        clearInterval(warmupTimer);
        warmupTimer = null;
      }
      if (store.error && WARMUP_CODES.includes(store.error.code)) store.error = null;
      return;
    }
    void store.loadQwen3().catch(() => undefined);
  }, 4000);
}

const qwen3SavePending = computed(() => store.pendingOperations.has('qwen3Save'));

/** True when the local service reports CPU synthesis. */
const qwen3RunsOnCpu = computed(() => store.qwen3?.health?.device === 'cpu');
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
    // Enabling 播报 is the only action that should be needed: bring the local
    // service up in the background instead of asking for a second click.
    if (value) await ensureQwen3Service();
  } catch (error) {
    draft.isEnabled = !value;
    ElMessage.error((error as { message?: string }).message ?? t('common.operationFailed'));
  }
}

/** One-stop preparation: runtime environment -> service -> model weights. */
/** One-stop preparation shared with the AI page; owned by the TTS store. */
async function ensureQwen3Service(): Promise<void> {
  try {
    await store.ensureSpeechReady();
    ElMessage.success(t('tts.environment.ready'));
  } catch (error) {
    ElMessage.error((error as { message?: string }).message ?? t('tts.environment.failed'));
  }
}

/** Retrying an install or service failure must re-run the whole preparation. */
/**
 * Retrying a speech failure must re-run the whole preparation so the progress
 * panel comes back; only unrelated errors fall back to a plain refresh.
 */
async function handleRetry(): Promise<void> {
  if (draft.isEnabled || retriesSpeechInstall(store.error?.code)) {
    await ensureQwen3Service();
    return;
  }
  await store.refresh();
}

async function handleSaveAll(): Promise<void> {
  try {
    if (qwen3Dirty.value) await store.saveQwen3(qwen3Payload()).then(snapshot => applyQwen3Config(snapshot.config));
    await handleSave();
  } catch {
    // handleQwen3Save and handleSave already report their own failures.
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
      identityKinds: ['admin', 'captain', 'ordinary'],
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
  // Entering this page is the only place where the runtime is installed and
  // started, so a stored "enabled" flag works without touching start-up.
  watchWarmup();
  const status = await store.loadEnvironmentStatus().catch(() => null);
  if (draft.isEnabled) {
    if (!status?.ready) {
      await ensureQwen3Service();
    } else {
      void store
        .startQwen3Service()
        .then(() => store.loadQwen3Models().catch(() => undefined))
        .catch(() => undefined);
    }
  }
});
</script>

<template>
  <SpeechPrepareDialog />



  <section class="mx-auto flex w-full max-w-[1180px] flex-col gap-4">
    <header class="flex flex-wrap items-center gap-3">
      <div class="min-w-0">
        <h2 class="app-text m-0 text-lg font-semibold">
          {{ t('tts.title') }}
        </h2>
        <p v-if="store.snapshot" class="app-muted mt-1 mb-0 text-sm" aria-live="polite">
          {{
            store.snapshot.status.state === 'unavailable'
              ? t('tts.engineUnavailable')
              : t('tts.status', { state: store.snapshot.status.state })
          }}
        </p>
      </div>
      <el-switch
        v-model="draft.isEnabled"
        class="ml-auto shrink-0"
        :active-text="t('common.enabled')"
        :inactive-text="t('common.disabled')"
        :loading="store.pendingOperations.has('setEnabled')"
        @change="handleEnabled"
      />
    </header>

    <el-alert
      v-if="store.error && !errorIsWarmup"
      type="error"
      :closable="false"
      :title="store.error.message"
      show-icon
    >
      <el-button size="small" @click="handleRetry">
        {{ t('common.retry') }}
      </el-button>
    </el-alert>

    <!-- The first model load takes minutes; that is not a failure to report. -->
    <el-alert
      v-else-if="store.error && errorIsWarmup"
      type="info"
      :closable="false"
      :title="t('tts.environment.modelWarming')"
      show-icon
    />

    <!-- The model belongs to the speech feature, next to its engine settings. -->
    

    <el-skeleton
      v-if="store.pending && !store.snapshot"
      class="app-card p-4"
      :rows="8"
      animated
    />

    <template v-else-if="store.snapshot">
      <el-form class="flex flex-col gap-4" label-position="top" @submit.prevent="handleSaveAll">
        <!-- One action bar for the whole page: probe, test, reset and save. -->
        <div class="app-card flex flex-wrap items-center justify-end gap-2 p-3">
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
          <el-button :icon="RefreshLeft" :disabled="savePending" @click="handleReset">
            {{ t('common.resetDefaults') }}
          </el-button>
          <button
            type="submit"
            class="app-btn-primary inline-flex h-8 items-center gap-1.5 px-3 text-sm disabled:cursor-not-allowed disabled:opacity-60"
            :disabled="savePending || qwen3SavePending || !(isDirty || qwen3Dirty)"
          >
            <el-icon v-if="savePending || qwen3SavePending" class="is-loading">
              <Loading />
            </el-icon>
            {{ t('common.save') }}
          </button>
        </div>
        <section class="app-card p-4" aria-labelledby="tts-voice-title">
          <h3 id="tts-voice-title" class="app-text m-0 text-base font-semibold">
            {{ t('tts.sections.voice') }}
          </h3>
          <p class="app-muted mt-1 mb-0 text-sm">
            {{ t('tts.sections.voiceHint') }}
          </p>
          <div class="mt-4 grid grid-cols-1 gap-x-6 lg:grid-cols-2">
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
              <p
                v-if="engineUnavailable"
                class="app-faint m-0 mt-1 w-full text-xs"
                style="color: var(--app-gold)"
              >
                {{ t('tts.cloudUnavailable') }}
              </p>
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
        </section>

        <section class="app-card p-4" aria-labelledby="tts-selection-title">
          <h3 id="tts-selection-title" class="app-text m-0 text-base font-semibold">
            {{ t('tts.selection') }}
          </h3>
          <p class="app-muted mt-1 mb-0 text-sm">
            {{ t('tts.sections.selectionHint') }}
          </p>
          <div class="mt-4 grid grid-cols-1 gap-x-6 lg:grid-cols-2">
            <el-form-item :label="t('tts.content')">
              <el-checkbox-group v-model="draft.eventKinds">
                <el-checkbox
                  v-for="option in eventOptions"
                  :key="option.value"
                  :value="option.value"
                >
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
        </section>

        <section class="app-card p-4" aria-labelledby="tts-templates-title">
          <h3 id="tts-templates-title" class="app-text m-0 text-base font-semibold">
            {{ t('tts.templates.title') }}
          </h3>
          <p class="app-muted mt-1 mb-0 text-sm">
            {{ t('tts.sections.templatesHint') }}
          </p>
          <div class="mt-4 grid grid-cols-1 gap-x-6 lg:grid-cols-2">
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
        </section>

        <section class="app-card p-4" aria-labelledby="tts-qwen3-title">
          <div class="flex flex-wrap items-start gap-3">
            <div class="mr-auto min-w-0">
              <h3 id="tts-qwen3-title" class="app-text m-0 text-base font-semibold">
                {{ t('tts.qwen3.title') }}
              </h3>
              <p class="app-muted mt-1 mb-0 text-sm">
                {{ t('tts.sections.qwen3Hint') }}
              </p>
            </div>
            <span
              class="app-badge shrink-0"
              :class="qwen3Health?.ready ? 'app-badge--admin' : 'app-badge--plain'"
              role="status"
            >
              {{
                qwen3Health?.ready
                  ? t('tts.qwen3.ready', { model: qwen3Health?.model ?? '' })
                  : qwen3Health
                    ? t('tts.qwen3.loading')
                    : t('tts.qwen3.unreachable')
              }}
            </span>
          </div>
          <p
            v-if="qwen3Health?.error"
            class="app-faint mt-2 mb-0 text-xs"
            style="color: var(--app-gold)"
          >
            {{ qwen3Health.error }}
          </p>
          <div class="mt-4 grid grid-cols-1 gap-x-6 lg:grid-cols-2">
            <el-form-item :label="t('tts.qwen3.enable')">
              <div class="flex w-full flex-col gap-1">
                <el-switch v-model="qwen3Draft.isEnabled" />
                <!-- CPU synthesis is roughly half a second per character, which
                     matters for live traffic; say so instead of surprising them. -->
                <p
                  v-if="qwen3Draft.isEnabled && qwen3RunsOnCpu"
                  class="app-faint m-0 text-xs"
                  style="color: var(--app-gold)"
                >
                  {{ t('tts.qwen3.cpuHint') }}
                </p>
              </div>
            </el-form-item>
            <el-form-item :label="t('tts.qwen3.fallback')">
              <el-switch v-model="qwen3Draft.fallbackToSystem" />
            </el-form-item>
            <!-- 服务地址 is intentionally not editable: the app starts its own
                 service on a free port and persists that address (PRD 9.11). -->
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
            <el-form-item :label="t('tts.qwen3.voice')">
              <el-select
                v-model="qwen3Draft.voiceId"
                class="w-full"
                clearable
                filterable
                allow-create
                default-first-option
              >
                <el-option
                  v-for="voice in qwen3VoiceOptions"
                  :key="voice.value"
                  :label="voice.label"
                  :value="voice.value"
                />
              </el-select>
            </el-form-item>
            <el-form-item :label="t('tts.qwen3.aiVoice')">
              <el-select
                v-model="qwen3Draft.aiVoiceId"
                class="w-full"
                clearable
                filterable
                allow-create
                default-first-option
              >
                <el-option
                  v-for="voice in qwen3VoiceOptions"
                  :key="voice.value"
                  :label="voice.label"
                  :value="voice.value"
                />
              </el-select>
              <p class="app-faint m-0 mt-1 w-full text-xs">
                {{ t('tts.qwen3.aiVoiceHint') }}
              </p>
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
            <el-form-item :label="t('tts.qwen3.pipMirror')">
              <el-select v-model="qwen3Draft.pipMirror" class="w-full">
                <el-option
                  v-for="mirror in QWEN3_MIRROR_OPTIONS"
                  :key="mirror"
                  :label="t(`tts.qwen3.mirrors.${mirror}`)"
                  :value="mirror"
                />
              </el-select>
              <p class="app-faint m-0 mt-1 w-full text-xs">
                {{ t('tts.qwen3.pipMirrorHint') }}
              </p>
            </el-form-item>
            <el-form-item class="lg:col-span-2" :label="t('tts.qwen3.instruct')">
              <el-input v-model="qwen3Draft.instruct" type="textarea" :rows="2" />
              <p class="app-faint m-0 mt-1 w-full text-xs">
                {{ t('tts.qwen3.instructHint') }}
              </p>
            </el-form-item>
          </div>
        </section>

      </el-form>

      <div class="app-card p-4">
        <TtsQueuePanel
          :status="store.snapshot.status"
          :pause-pending="store.pendingOperations.has('pause')"
          :clear-pending="store.pendingOperations.has('clear')"
          :style="{
            borderTopWidth: '0px',
            borderBottomWidth: '0px',
            paddingTop: '0px',
            paddingBottom: '0px'
          }"
          @pause="handlePause"
          @clear="handleClear"
        />
      </div>
    </template>
  </section>
</template>
