<script setup lang="ts">
import { computed, reactive, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { Delete, Headset, Setting, VideoPause, VideoPlay } from '@element-plus/icons-vue';
import type {
  AiModelProfile,
  AiOutputDeviceId,
  AiVoiceId,
  AiVoiceQueueItem,
  AiVoiceReplyRules,
  AiVoiceReplySnapshot,
  AiTriggerScope
} from '@/platform/desktop';

const props = defineProps<{
  snapshot: AiVoiceReplySnapshot | null;
  profiles: readonly AiModelProfile[];
  pending: ReadonlySet<string>;
}>();

const emit = defineEmits<{
  openModels: [];
  setEnabled: [isEnabled: boolean, isAudioRouteConfirmed: boolean];
  confirmRoute: [
    deviceId: AiOutputDeviceId,
    voiceId: AiVoiceId,
    volume: number,
    enableAfterSave: boolean
  ];
  testOutput: [deviceId: AiOutputDeviceId, voiceId: AiVoiceId, volume: number];
  updateRules: [rules: AiVoiceReplyRules];
  pause: [isPaused: boolean];
  skip: [];
  clear: [];
}>();

const { t } = useI18n();
const routeVisible = ref(false);
const rulesVisible = ref(false);
const route = reactive({
  deviceId: null as AiOutputDeviceId | null,
  voiceId: null as AiVoiceId | null,
  volume: 0.8,
  confirmed: false
});
const rules = reactive({
  triggerScope: 'questions' as AiVoiceReplyRules['triggerScope'],
  keywordsText: '',
  globalCooldownSeconds: 5,
  userCooldownSeconds: 10,
  queueCapacity: 20,
  isDuplicateFilterEnabled: true
});

/**
 * Mirrors `DEFAULT_AI_VOICE_ID` in the Rust member module. This id is not a
 * SAPI voice: it selects the Qwen3-TTS reply voice, and only falls back to the
 * system voice when Qwen3-TTS is disabled or unreachable.
 */
const DEFAULT_AI_VOICE_ID = 'gentle-peach' as AiVoiceId;

const qwen3VoiceAvailable = computed(() =>
  (props.snapshot?.voices ?? []).some(voice => voice.voiceId.startsWith('qwen3:'))
);
const routeVoiceHint = computed(() => {
  if (route.voiceId !== DEFAULT_AI_VOICE_ID) return null;
  return qwen3VoiceAvailable.value
    ? t('ai.voice.route.defaultUsesQwen3')
    : t('ai.voice.route.defaultNeedsQwen3');
});

const defaultProfile = computed(() =>
  props.profiles.find(profile => profile.profileId === props.snapshot?.defaultProfileId) ?? null
);
const selectedDevice = computed(() =>
  props.snapshot?.devices.find(device => device.deviceId === props.snapshot?.queue.selectedDeviceId) ?? null
);
const selectedVoice = computed(() =>
  props.snapshot?.voices.find(voice => voice.voiceId === props.snapshot?.queue.selectedVoiceId) ?? null
);
const canEnable = computed(() => Boolean(
  defaultProfile.value?.isEnabled
  && selectedDevice.value?.isAvailable
  && selectedVoice.value?.isAvailable
));
const isDraftRouteAvailable = computed(() => Boolean(
  route.deviceId
  && props.snapshot?.devices.some(device => device.deviceId === route.deviceId && device.isAvailable)
  && route.voiceId
  && props.snapshot.voices.some(voice => voice.voiceId === route.voiceId && voice.isAvailable)
));
const isPaused = computed(() => props.snapshot?.queue.state === 'paused');
const pendingCount = computed(() => props.snapshot?.queue.pending.length ?? 0);
const queueStateLabel = computed(() => {
  const state = props.snapshot?.queue.state;
  if (state === 'paused') return t('ai.queue.paused');
  if (state === 'speaking') return t('ai.queue.speaking');
  if (state === 'unavailable') return t('ai.queue.unavailable');
  return t('ai.queue.idle');
});
const scopeOptions = computed<readonly { label: string; value: AiTriggerScope }[]>(() => [
  { label: t('ai.voice.scope.all'), value: 'all' },
  { label: t('ai.voice.scope.questions'), value: 'questions' },
  { label: t('ai.voice.scope.captains'), value: 'captains' },
  { label: t('ai.voice.scope.keywords'), value: 'keywords' },
  { label: t('ai.voice.scope.highValue'), value: 'highValue' }
]);
const globalCooldownPresets = [3, 5, 10, 30];
const userCooldownPresets = [5, 10, 30, 60];

function withCurrent(presets: readonly number[], current: number): number[] {
  return presets.includes(current) ? [...presets] : [...presets, current].sort((a, b) => a - b);
}

const globalCooldownOptions = computed(() =>
  withCurrent(globalCooldownPresets, rules.globalCooldownSeconds)
);
const userCooldownOptions = computed(() =>
  withCurrent(userCooldownPresets, rules.userCooldownSeconds)
);

watch(
  () => props.snapshot,
  snapshot => {
    if (!snapshot) return;
    route.deviceId = snapshot.queue.selectedDeviceId
      ?? snapshot.devices.find(device => device.isDefault)?.deviceId
      ?? snapshot.devices[0]?.deviceId
      ?? null;
    route.voiceId = snapshot.queue.selectedVoiceId
      ?? snapshot.voices.find(voice => voice.isDefault)?.voiceId
      ?? snapshot.voices[0]?.voiceId
      ?? null;
    route.volume = snapshot.queue.volume;
    route.confirmed = snapshot.queue.isAudioRouteConfirmed;
    Object.assign(rules, {
      ...snapshot.rules,
      keywordsText: snapshot.rules.keywords.join('\n')
    });
  },
  { immediate: true }
);

function toggleEnabled(value: boolean | string | number): void {
  const enabled = Boolean(value);
  if (!enabled) {
    emit('setEnabled', false, false);
    return;
  }
  if (!canEnable.value || !props.snapshot?.queue.isAudioRouteConfirmed) {
    routeVisible.value = true;
    return;
  }
  emit('setEnabled', true, true);
}

function saveRoute(enableAfterSave: boolean): void {
  if (!route.deviceId || !route.voiceId || !isDraftRouteAvailable.value || !route.confirmed) return;
  emit('confirmRoute', route.deviceId, route.voiceId, route.volume, enableAfterSave);
  routeVisible.value = false;
}

function testRoute(): void {
  if (!route.deviceId || !route.voiceId || !isDraftRouteAvailable.value) return;
  emit('testOutput', route.deviceId, route.voiceId, route.volume);
}

function changeDevice(deviceId: AiOutputDeviceId): void {
  route.deviceId = deviceId;
  route.confirmed = false;
  routeVisible.value = true;
}

function changeScope(value: AiTriggerScope): void {
  rules.triggerScope = value;
  if (value === 'keywords') {
    rulesVisible.value = true;
    return;
  }
  saveRules();
}

function changeGlobalCooldown(value: number): void {
  rules.globalCooldownSeconds = Number(value);
  saveRules();
}

function changeUserCooldown(value: number): void {
  rules.userCooldownSeconds = Number(value);
  saveRules();
}

function saveRules(): void {
  emit('updateRules', {
    triggerScope: rules.triggerScope,
    keywords: rules.keywordsText
      .split(/\r?\n/)
      .map(value => value.trim())
      .filter(Boolean),
    globalCooldownSeconds: rules.globalCooldownSeconds,
    userCooldownSeconds: rules.userCooldownSeconds,
    queueCapacity: rules.queueCapacity,
    isDuplicateFilterEnabled: rules.isDuplicateFilterEnabled
  });
  rulesVisible.value = false;
}

function queueInitial(item: AiVoiceQueueItem | null): string {
  if (!item) return '?';
  return [...item.text.trim()][0] ?? '?';
}
</script>

<template>
  <div class="grid gap-4 xl:grid-cols-[minmax(0,1.35fr)_minmax(0,1fr)]">
    <section class="app-card flex flex-col" aria-labelledby="ai-voice-title">
      <div class="flex flex-wrap items-start justify-between gap-3 px-5 py-4">
        <div class="min-w-0">
          <h3 id="ai-voice-title" class="app-text m-0 text-base font-semibold">
            {{ t('ai.voice.title') }}
          </h3>
          <p class="app-muted mt-1 mb-0 text-xs">
            {{ t('ai.voice.hint') }}
          </p>
        </div>
        <div class="flex flex-wrap items-center gap-2">
          <button type="button" class="app-chip" @click="rulesVisible = true">
            <el-icon><Setting /></el-icon>
            {{ t('ai.voice.rulesAction') }}
          </button>
          <button type="button" class="app-chip" @click="routeVisible = true">
            <el-icon><Headset /></el-icon>
            {{ t('ai.voice.routeGuide') }}
          </button>
          <el-switch
            :model-value="snapshot?.isEnabled ?? false"
            :loading="pending.has('setVoiceReplyEnabled')"
            :disabled="!(snapshot?.isEnabled ?? false) && !canEnable"
            :aria-label="t('ai.voice.title')"
            @change="toggleEnabled"
          />
        </div>
      </div>

      <div class="flex flex-wrap items-center gap-3 border-t app-hairline px-5 py-3">
        <span class="app-muted w-32 shrink-0 text-sm">{{ t('ai.voice.modelLabel') }}</span>
        <span class="app-text min-w-0 flex-1 truncate text-sm font-medium">
          {{ defaultProfile?.name ?? t('ai.voice.emptyModel') }}
        </span>
        <span v-if="defaultProfile" class="app-faint shrink-0 text-xs">
          {{ defaultProfile.model }}
        </span>
        <button type="button" class="app-chip" @click="emit('openModels')">
          {{ t('ai.voice.switchModel') }}
        </button>
      </div>

      <div class="flex flex-wrap items-center gap-3 border-t app-hairline px-5 py-3">
        <span class="app-muted w-32 shrink-0 text-sm">{{ t('ai.voice.triggerScope') }}</span>
        <div class="w-44 shrink-0">
          <el-select
            :model-value="rules.triggerScope"
            class="w-full"
            :aria-label="t('ai.voice.triggerScope')"
            @change="changeScope"
          >
            <el-option
              v-for="option in scopeOptions"
              :key="option.value"
              :label="option.label"
              :value="option.value"
            />
          </el-select>
        </div>
      </div>

      <div class="flex flex-wrap items-center gap-3 border-t app-hairline px-5 py-3">
        <span class="app-muted w-32 shrink-0 text-sm">{{ t('ai.voice.globalCooldown') }}</span>
        <div class="w-44 shrink-0">
          <el-select
            :model-value="rules.globalCooldownSeconds"
            class="w-full"
            :aria-label="t('ai.voice.globalCooldown')"
            @change="changeGlobalCooldown"
          >
            <el-option
              v-for="seconds in globalCooldownOptions"
              :key="seconds"
              :label="t('ai.voice.seconds', { count: seconds })"
              :value="seconds"
            />
          </el-select>
        </div>
      </div>

      <div class="flex flex-wrap items-center gap-3 border-t app-hairline px-5 py-3">
        <span class="app-muted w-32 shrink-0 text-sm">{{ t('ai.voice.userCooldown') }}</span>
        <div class="w-44 shrink-0">
          <el-select
            :model-value="rules.userCooldownSeconds"
            class="w-full"
            :aria-label="t('ai.voice.userCooldown')"
            @change="changeUserCooldown"
          >
            <el-option
              v-for="seconds in userCooldownOptions"
              :key="seconds"
              :label="t('ai.voice.seconds', { count: seconds })"
              :value="seconds"
            />
          </el-select>
        </div>
      </div>

      <div class="flex flex-wrap items-center gap-3 border-t app-hairline px-5 py-3">
        <span class="app-muted w-32 shrink-0 text-sm">{{ t('ai.voice.outputDevice') }}</span>
        <div class="w-64 shrink-0">
          <el-select
            :model-value="snapshot?.queue.selectedDeviceId ?? null"
            class="w-full"
            :placeholder="t('ai.voice.outputMissing')"
            :aria-label="t('ai.voice.outputDevice')"
            @change="changeDevice"
          >
            <el-option
              v-for="device in snapshot?.devices ?? []"
              :key="device.deviceId"
              :label="device.displayName"
              :value="device.deviceId"
              :disabled="!device.isAvailable"
            />
          </el-select>
        </div>
        <button
          type="button"
          class="app-chip"
          :disabled="!isDraftRouteAvailable"
          @click="testRoute"
        >
          {{ t('ai.voice.testPlayback') }}
        </button>
      </div>

      <div class="flex flex-col gap-2 px-5 pb-4">
        <el-alert
          v-if="!defaultProfile"
          type="warning"
          :closable="false"
          :title="t('ai.voice.modelRequired')"
          show-icon
        />
        <el-alert
          v-else-if="!selectedDevice"
          type="warning"
          :closable="false"
          :title="t('ai.voice.deviceRequired')"
          show-icon
        />
      </div>
    </section>

    <section class="app-card flex min-h-0 flex-col" aria-labelledby="ai-queue-title">
      <div class="flex flex-wrap items-center justify-between gap-2 px-5 py-4">
        <div class="min-w-0">
          <h3 id="ai-queue-title" class="app-text m-0 text-base font-semibold">
            {{ t('ai.queue.title') }}
          </h3>
          <p class="app-muted mt-1 mb-0 text-xs">
            {{ t('ai.queue.pending', { count: pendingCount }) }}
            <span class="app-faint">
              {{ t('ai.queue.capacity', { capacity: snapshot?.queue.capacity ?? 20 }) }}
            </span>
          </p>
        </div>
        <span class="app-faint rounded-full border app-hairline px-2.5 py-1 text-xs">
          {{ queueStateLabel }}
        </span>
      </div>

      <div class="flex max-h-80 min-h-0 flex-1 flex-col gap-2 overflow-y-auto border-t app-hairline px-5 py-3">
        <div v-if="snapshot?.queue.current" class="app-surface-2 rounded-lg p-3">
          <div class="flex items-center gap-2">
            <span
              class="app-surface app-text flex h-7 w-7 shrink-0 items-center justify-center rounded-full text-xs font-semibold"
              aria-hidden="true"
            >
              {{ queueInitial(snapshot.queue.current) }}
            </span>
            <span class="app-muted text-xs">{{ t('ai.queue.speaking') }}</span>
          </div>
          <p class="app-text mt-2 mb-0 text-sm break-words">
            {{ snapshot.queue.current.text }}
          </p>
        </div>

        <ul v-if="snapshot?.queue.pending.length" class="m-0 flex list-none flex-col gap-1.5 p-0">
          <li
            v-for="(item, index) in snapshot.queue.pending"
            :key="item.itemId"
            class="flex items-center gap-2 text-xs"
          >
            <span class="app-faint w-5 shrink-0 text-right tabular-nums">{{ index + 1 }}</span>
            <span class="app-muted min-w-0 flex-1 truncate">{{ item.text }}</span>
          </li>
        </ul>

        <p
          v-if="!snapshot?.queue.current && !snapshot?.queue.pending.length"
          class="app-faint m-0 py-6 text-center text-sm"
        >
          {{ t('ai.queue.empty') }}
        </p>
      </div>

      <div class="flex flex-wrap gap-2 border-t app-hairline px-5 py-3">
        <button
          type="button"
          class="app-chip disabled:cursor-not-allowed disabled:opacity-60"
          :disabled="!snapshot"
          @click="emit('pause', !isPaused)"
        >
          <el-icon><component :is="isPaused ? VideoPlay : VideoPause" /></el-icon>
          {{ isPaused ? t('ai.queue.resume') : t('ai.queue.pause') }}
        </button>
        <button
          type="button"
          class="app-chip disabled:cursor-not-allowed disabled:opacity-60"
          :disabled="!snapshot?.queue.current"
          @click="emit('skip')"
        >
          {{ t('ai.queue.skip') }}
        </button>
        <button
          type="button"
          class="app-chip disabled:cursor-not-allowed disabled:opacity-60"
          :disabled="!snapshot?.queue.pending.length"
          @click="emit('clear')"
        >
          <el-icon><Delete /></el-icon>
          {{ t('ai.queue.clear') }}
        </button>
      </div>
    </section>
  </div>

  <el-dialog v-model="routeVisible" width="min(560px, 94vw)">
    <template #header>
      <h3 class="app-text m-0 text-base font-semibold">
        {{ t('ai.voice.route.title') }}
      </h3>
      <p class="app-muted mt-1 mb-0 text-sm">
        {{ t('ai.voice.route.hint') }}
      </p>
    </template>
    <el-form label-position="top">
      <el-form-item :label="t('ai.voice.route.device')" required>
        <el-select v-model="route.deviceId" class="w-full" @change="route.confirmed = false">
          <el-option
            v-for="device in snapshot?.devices ?? []"
            :key="device.deviceId"
            :label="device.displayName"
            :value="device.deviceId"
            :disabled="!device.isAvailable"
          />
        </el-select>
      </el-form-item>
      <el-form-item :label="t('ai.voice.route.voice')" required>
        <el-select v-model="route.voiceId" class="w-full" @change="route.confirmed = false">
          <el-option
            v-for="voice in snapshot?.voices ?? []"
            :key="voice.voiceId"
            :label="voice.displayName"
            :value="voice.voiceId"
            :disabled="!voice.isAvailable"
          />
        </el-select>
        <p v-if="routeVoiceHint" class="app-faint m-0 mt-1 w-full text-xs">
          {{ routeVoiceHint }}
        </p>
      </el-form-item>
      <el-form-item :label="t('ai.voice.route.volume')">
        <el-slider
          v-model="route.volume"
          :min="0"
          :max="1"
          :step="0.05"
        />
      </el-form-item>
      <div class="flex flex-wrap items-center gap-3">
        <button
          type="button"
          class="app-chip disabled:cursor-not-allowed disabled:opacity-60"
          :disabled="!isDraftRouteAvailable"
          @click="testRoute"
        >
          {{ t('ai.voice.route.test') }}
        </button>
        <el-checkbox v-model="route.confirmed">
          {{ t('ai.voice.route.confirmCheckbox') }}
        </el-checkbox>
      </div>
    </el-form>
    <template #footer>
      <div class="flex flex-wrap items-center justify-end gap-2">
        <button type="button" class="app-chip" @click="routeVisible = false">
          {{ t('common.cancel') }}
        </button>
        <button
          type="button"
          class="app-btn-primary h-8 px-4 text-sm disabled:cursor-not-allowed disabled:opacity-60"
          :disabled="!isDraftRouteAvailable || !route.confirmed"
          @click="saveRoute(!(snapshot?.isEnabled ?? false))"
        >
          {{ snapshot?.isEnabled ? t('ai.voice.route.confirm') : t('ai.voice.route.confirmAndEnable') }}
        </button>
      </div>
    </template>
  </el-dialog>

  <el-dialog v-model="rulesVisible" width="min(620px, 94vw)">
    <template #header>
      <h3 class="app-text m-0 text-base font-semibold">
        {{ t('ai.voice.rule.title') }}
      </h3>
      <p class="app-muted mt-1 mb-0 text-sm">
        {{ t('ai.voice.rule.hint') }}
      </p>
    </template>
    <el-form label-position="top">
      <el-form-item :label="t('ai.voice.rule.triggerScope')">
        <el-select v-model="rules.triggerScope" class="w-full">
          <el-option
            v-for="option in scopeOptions"
            :key="option.value"
            :label="option.label"
            :value="option.value"
          />
        </el-select>
      </el-form-item>
      <el-form-item v-if="rules.triggerScope === 'keywords'" :label="t('ai.voice.rule.keywords')">
        <el-input v-model="rules.keywordsText" type="textarea" :rows="3" />
      </el-form-item>
      <div class="grid gap-x-4 sm:grid-cols-3">
        <el-form-item :label="t('ai.voice.rule.globalCooldown')">
          <el-input-number
            v-model="rules.globalCooldownSeconds"
            :min="1"
            :max="60"
            class="w-full"
          />
        </el-form-item>
        <el-form-item :label="t('ai.voice.rule.userCooldown')">
          <el-input-number
            v-model="rules.userCooldownSeconds"
            :min="5"
            :max="300"
            class="w-full"
          />
        </el-form-item>
        <el-form-item :label="t('ai.voice.rule.capacity')">
          <el-input-number
            v-model="rules.queueCapacity"
            :min="5"
            :max="100"
            class="w-full"
          />
        </el-form-item>
      </div>
      <el-form-item :label="t('ai.voice.rule.duplicateFilter')">
        <el-switch v-model="rules.isDuplicateFilterEnabled" />
      </el-form-item>
    </el-form>
    <template #footer>
      <div class="flex flex-wrap items-center justify-end gap-2">
        <button type="button" class="app-chip" @click="rulesVisible = false">
          {{ t('common.cancel') }}
        </button>
        <button type="button" class="app-btn-primary h-8 px-4 text-sm" @click="saveRules">
          {{ t('common.save') }}
        </button>
      </div>
    </template>
  </el-dialog>
</template>
