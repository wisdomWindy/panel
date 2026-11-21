<script setup lang="ts">
import { computed, onMounted, onUnmounted, reactive } from 'vue';
import { useI18n } from 'vue-i18n';
import { Delete, FolderOpened, Loading, RefreshLeft, VideoPlay } from '@element-plus/icons-vue';
import type { CaptainEffectConfig, SoundBindingSummary } from '@/platform/desktop';
import { validateHexColor, validateVolume } from '@/modules/media/validation';
import { useMediaEffectsStore } from '@/stores/media/effects';

const store = useMediaEffectsStore();
const { t } = useI18n();
const draft = reactive<CaptainEffectConfig>({
  presetId: 'captain-default',
  durationSeconds: 5,
  position: 'center',
  windowWidth: 560,
  windowHeight: 280,
  primaryColor: '#00AEEC',
  secondaryColor: '#FB7299',
  firstSound: null,
  renewalSound: null,
  comboSound: null,
  volume: 0.8,
  comboEnabled: true,
  comboWindowSeconds: 10
});

  type SoundKey = 'firstSound' | 'renewalSound' | 'comboSound';
const positionOptions = computed(
  () =>
      [
        { label: t('captainEffect.positions.topLeft'), value: 'topLeft' },
        { label: t('captainEffect.positions.topRight'), value: 'topRight' },
        { label: t('captainEffect.positions.bottomLeft'), value: 'bottomLeft' },
        { label: t('captainEffect.positions.bottomRight'), value: 'bottomRight' },
        { label: t('captainEffect.positions.center'), value: 'center' }
      ] as const
);
const soundRows = computed<readonly { label: string; key: SoundKey }[]>(() => [
  { label: t('captainEffect.sounds.first'), key: 'firstSound' },
  { label: t('captainEffect.sounds.renewal'), key: 'renewalSound' },
  { label: t('captainEffect.sounds.combo'), key: 'comboSound' }
]);

const formValid = computed(
  () =>
    draft.presetId.length > 0 &&
      validateHexColor(draft.primaryColor).valid &&
      validateHexColor(draft.secondaryColor).valid &&
      Number.isInteger(draft.windowWidth) &&
      draft.windowWidth >= 320 &&
      draft.windowWidth <= 1920 &&
      Number.isInteger(draft.windowHeight) &&
      draft.windowHeight >= 180 &&
      draft.windowHeight <= 1080 &&
      validateVolume(draft.volume).valid
);

function replaceDraft(config: CaptainEffectConfig): void {
  Object.assign(draft, {
    ...config,
    firstSound: config.firstSound ? { ...config.firstSound } : null,
    renewalSound: config.renewalSound ? { ...config.renewalSound } : null,
    comboSound: config.comboSound ? { ...config.comboSound } : null
  });
}

function handlePresetChange(presetId: string): void {
  const preset = store.snapshot?.presets.find(item => item.presetId === presetId);
  if (!preset) return;
  draft.presetId = preset.presetId;
  draft.primaryColor = preset.primaryColor;
  draft.secondaryColor = preset.secondaryColor;
}

async function load(): Promise<void> {
  try {
    const value = await store.load();
    if (value) replaceDraft(value.config);
  } catch {
    // The stable page error region owns load feedback.
  }
}

async function save(): Promise<void> {
  if (!formValid.value) return;
  try {
    const value = await store.save({ ...draft });
    replaceDraft(value.config);
    ElMessage.success(t('captainEffect.saved'));
  } catch {
    ElMessage.error(store.error?.message ?? t('captainEffect.saveFailed'));
  }
}

async function preview(): Promise<void> {
  if (!formValid.value) return;
  try {
    await store.preview({ ...draft });
  } catch {
    ElMessage.error(store.error?.message ?? t('captainEffect.previewFailed'));
  }
}

async function selectSound(key: SoundKey): Promise<void> {
  try {
    draft[key] = await store.selectSound();
  } catch (error) {
    ElMessage.error(
      (error as { message?: string }).message ?? t('captainEffect.selectSoundFailed')
    );
  }
}

async function previewSound(binding: SoundBindingSummary): Promise<void> {
  try {
    await store.previewSound(binding, draft.volume);
  } catch (error) {
    ElMessage.error(
      (error as { message?: string }).message ?? t('captainEffect.soundPreviewFailed')
    );
  }
}

async function resetDefaults(): Promise<void> {
  try {
    await ElMessageBox.confirm(t('captainEffect.resetConfirm'), t('captainEffect.resetTitle'), {
      confirmButtonText: t('common.reset'),
      cancelButtonText: t('common.cancel'),
      type: 'warning'
    });
    const defaults: CaptainEffectConfig = {
      presetId: 'captain-default',
      durationSeconds: 5,
      position: 'center',
      windowWidth: 560,
      windowHeight: 280,
      primaryColor: '#00AEEC',
      secondaryColor: '#FB7299',
      firstSound: null,
      renewalSound: null,
      comboSound: null,
      volume: 0.8,
      comboEnabled: true,
      comboWindowSeconds: 10
    };
    const value = await store.save(defaults);
    replaceDraft(value.config);
    ElMessage.success(t('captainEffect.resetSuccess'));
  } catch (error) {
    if (error !== 'cancel' && error !== 'close') {
      ElMessage.error(store.error?.message ?? t('captainEffect.resetFailed'));
    }
  }
}

onMounted(load);
onUnmounted(store.dispose);
</script>

<template>
  <section class="mx-auto flex w-full max-w-[1180px] flex-col gap-4">
    <header class="flex flex-wrap items-center gap-2">
      <div class="mr-auto min-w-0">
        <h2 class="app-text m-0 text-lg font-semibold">{{ t('captainEffect.title') }}</h2>
        <p class="app-muted mt-1 mb-0 text-sm">{{ t('captainEffect.description') }}</p>
      </div>
      <button
        type="button"
        class="app-chip disabled:cursor-not-allowed disabled:opacity-50"
        :aria-busy="store.previewPending"
        :disabled="!formValid || store.previewPending"
        @click="preview"
      >
        <el-icon v-if="store.previewPending" class="is-loading"><Loading /></el-icon>
        <el-icon v-else><VideoPlay /></el-icon>
        {{ t('captainEffect.preview') }}
      </button>
      <button
        type="button"
        class="app-chip disabled:cursor-not-allowed disabled:opacity-50"
        :disabled="store.savePending"
        @click="resetDefaults"
      >
        <el-icon><RefreshLeft /></el-icon>
        {{ t('common.resetDefaults') }}
      </button>
      <button
        type="button"
        class="app-btn-primary inline-flex h-8 items-center gap-1.5 px-4 text-sm disabled:cursor-not-allowed disabled:opacity-60"
        :aria-busy="store.savePending"
        :disabled="!formValid || store.savePending"
        @click="save"
      >
        <el-icon v-if="store.savePending" class="is-loading"><Loading /></el-icon>
        {{ t('common.save') }}
      </button>
    </header>

    <el-alert
      v-if="store.error"
      type="error"
      :closable="false"
      :title="store.error.message"
      show-icon
    />

    <div v-loading="store.pending" class="grid grid-cols-1 gap-4 xl:grid-cols-2">
      <section class="app-card p-5" aria-labelledby="captain-effect-visual-title">
        <h3 id="captain-effect-visual-title" class="app-text m-0 text-base font-semibold">
          {{ t('captainEffect.visual') }}
        </h3>
        <p class="app-muted mt-1 mb-0 text-sm">{{ t('captainEffect.visualHint') }}</p>

        <div class="mt-4 flex flex-col gap-3">
          <div class="flex min-h-8 items-center justify-between gap-3 border-t app-hairline pt-3">
            <span class="app-muted shrink-0 text-sm">{{ t('captainEffect.template') }}</span>
            <el-select
              v-model="draft.presetId"
              class="w-44"
              :aria-label="t('captainEffect.template')"
              :disabled="!store.snapshot"
              @change="handlePresetChange"
            >
              <el-option
                v-if="!store.snapshot?.presets.some(item => item.presetId === draft.presetId)"
                :label="draft.presetId"
                :value="draft.presetId"
              />
              <el-option
                v-for="preset in store.snapshot?.presets"
                :key="preset.presetId"
                :label="preset.displayName"
                :value="preset.presetId"
              />
            </el-select>
          </div>
          <div class="flex min-h-8 items-center justify-between gap-3 border-t app-hairline pt-3">
            <span class="app-muted shrink-0 text-sm">{{ t('captainEffect.duration') }}</span>
            <el-segmented
              v-model="draft.durationSeconds"
              :options="[3, 5, 8, 10]"
              :aria-label="t('captainEffect.duration')"
            />
          </div>
          <div class="flex min-h-8 items-center justify-between gap-3 border-t app-hairline pt-3">
            <span class="app-muted shrink-0 text-sm">{{ t('captainEffect.position') }}</span>
            <el-select
              v-model="draft.position"
              class="w-44"
              :aria-label="t('captainEffect.position')"
            >
              <el-option
                v-for="item in positionOptions"
                :key="item.value"
                :label="item.label"
                :value="item.value"
              />
            </el-select>
          </div>
          <div class="flex min-h-8 items-center justify-between gap-3 border-t app-hairline pt-3">
            <span class="app-muted shrink-0 text-sm">{{ t('captainEffect.windowWidth') }}</span>
            <el-input-number
              v-model="draft.windowWidth"
              class="!w-40"
              :min="320"
              :max="1920"
              :step="10"
              controls-position="right"
              :aria-label="t('captainEffect.windowWidth')"
            />
          </div>
          <div class="flex min-h-8 items-center justify-between gap-3 border-t app-hairline pt-3">
            <span class="app-muted shrink-0 text-sm">{{ t('captainEffect.windowHeight') }}</span>
            <el-input-number
              v-model="draft.windowHeight"
              class="!w-40"
              :min="180"
              :max="1080"
              :step="10"
              controls-position="right"
              :aria-label="t('captainEffect.windowHeight')"
            />
          </div>
          <div class="flex min-h-8 items-center justify-between gap-3 border-t app-hairline pt-3">
            <span class="app-muted shrink-0 text-sm">{{ t('captainEffect.primaryColor') }}</span>
            <div class="flex items-center gap-2">
              <el-color-picker
                v-model="draft.primaryColor"
                :aria-label="t('captainEffect.primaryColor')"
              />
              <el-input v-model="draft.primaryColor" maxlength="7" class="!w-28" />
            </div>
          </div>
          <div class="flex min-h-8 items-center justify-between gap-3 border-t app-hairline pt-3">
            <span class="app-muted shrink-0 text-sm">{{ t('captainEffect.secondaryColor') }}</span>
            <div class="flex items-center gap-2">
              <el-color-picker
                v-model="draft.secondaryColor"
                :aria-label="t('captainEffect.secondaryColor')"
              />
              <el-input v-model="draft.secondaryColor" maxlength="7" class="!w-28" />
            </div>
          </div>
        </div>
      </section>

      <section class="app-card p-5" aria-labelledby="captain-effect-sound-title">
        <h3 id="captain-effect-sound-title" class="app-text m-0 text-base font-semibold">
          {{ t('captainEffect.soundAndCombo') }}
        </h3>
        <p class="app-muted mt-1 mb-0 text-sm">{{ t('captainEffect.soundHint') }}</p>

        <div class="mt-4 flex flex-col">
          <div
            v-for="row in soundRows"
            :key="row.key"
            class="flex flex-wrap items-center gap-2 border-t app-hairline py-2 first:border-t-0 first:pt-0"
          >
            <span class="app-muted w-20 shrink-0 text-sm">{{ row.label }}</span>
            <span
              class="min-w-0 flex-1 truncate text-sm"
              :class="draft[row.key] ? 'app-text' : 'app-faint'"
              :style="draft[row.key]?.isMissing ? { color: 'var(--app-red)' } : undefined"
            >
              {{ draft[row.key]?.fileName ?? t('captainEffect.notSelected') }}
            </span>
            <div class="flex shrink-0 items-center gap-1">
              <el-tooltip :content="t('captainEffect.selectSound')">
                <button
                  type="button"
                  class="app-icon-btn"
                  :aria-label="`${t('captainEffect.selectSound')}: ${row.label}`"
                  @click="selectSound(row.key)"
                >
                  <el-icon><FolderOpened /></el-icon>
                </button>
              </el-tooltip>
              <el-tooltip :content="t('captainEffect.previewSound')">
                <button
                  type="button"
                  class="app-icon-btn disabled:cursor-not-allowed disabled:opacity-40"
                  :disabled="!draft[row.key]"
                  :aria-label="`${t('captainEffect.previewSound')}: ${row.label}`"
                  @click="draft[row.key] && previewSound(draft[row.key]!)"
                >
                  <el-icon><VideoPlay /></el-icon>
                </button>
              </el-tooltip>
              <el-tooltip :content="t('captainEffect.removeSound')">
                <button
                  type="button"
                  class="app-icon-btn disabled:cursor-not-allowed disabled:opacity-40"
                  :disabled="!draft[row.key]"
                  :aria-label="`${t('captainEffect.removeSound')}: ${row.label}`"
                  @click="draft[row.key] = null"
                >
                  <el-icon><Delete /></el-icon>
                </button>
              </el-tooltip>
            </div>
          </div>
        </div>

        <div class="mt-3 flex flex-col gap-3">
          <div class="border-t app-hairline pt-3">
            <div class="flex items-center justify-between gap-3">
              <span class="app-muted shrink-0 text-sm">{{ t('captainEffect.volume') }}</span>
              <span class="app-text text-sm tabular-nums">{{ Math.round(draft.volume * 100) }}%</span>
            </div>
            <el-slider
              v-model="draft.volume"
              class="mt-1"
              :min="0"
              :max="1"
              :step="0.1"
              :aria-label="t('captainEffect.volume')"
            />
          </div>
          <div class="flex min-h-8 items-center justify-between gap-3 border-t app-hairline pt-3">
            <span class="app-muted shrink-0 text-sm">{{ t('captainEffect.combo') }}</span>
            <el-switch v-model="draft.comboEnabled" :aria-label="t('captainEffect.combo')" />
          </div>
          <div class="flex min-h-8 items-center justify-between gap-3 border-t app-hairline pt-3">
            <span class="app-muted shrink-0 text-sm">{{ t('captainEffect.comboWindow') }}</span>
            <el-segmented
              v-model="draft.comboWindowSeconds"
              :options="[5, 10, 30]"
              :disabled="!draft.comboEnabled"
              :aria-label="t('captainEffect.comboWindow')"
            />
          </div>
        </div>
      </section>
    </div>
  </section>
</template>
