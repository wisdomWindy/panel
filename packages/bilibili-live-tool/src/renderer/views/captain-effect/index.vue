<script setup lang="ts">
import { computed, onMounted, onUnmounted, reactive } from 'vue';
import { useI18n } from 'vue-i18n';
import { Delete, FolderOpened, RefreshLeft, VideoPlay } from '@element-plus/icons-vue';
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
  <section class="min-w-0">
    <header class="mb-5 flex flex-wrap items-center gap-3 border-b border-gray-200 pb-4 dark:border-zinc-800">
      <div class="min-w-56 flex-1">
        <h2 class="m-0 text-xl font-semibold text-gray-900 dark:text-zinc-100">
          {{ t('captainEffect.title') }}
        </h2>
        <p class="mb-0 mt-1 text-xs text-gray-500 dark:text-zinc-400">
          {{ t('captainEffect.description') }}
        </p>
      </div>
      <el-button
        :icon="VideoPlay"
        :loading="store.previewPending"
        :disabled="!formValid"
        @click="preview"
      >
        {{ t('captainEffect.preview') }}
      </el-button>
      <el-button :icon="RefreshLeft" :disabled="store.savePending" @click="resetDefaults">
        {{ t('common.resetDefaults') }}
      </el-button>
      <el-button
        type="primary"
        :loading="store.savePending"
        :disabled="!formValid"
        @click="save"
      >
        {{ t('common.save') }}
      </el-button>
    </header>

    <el-alert
      v-if="store.error"
      class="mb-4"
      type="error"
      :closable="false"
      :title="store.error.message"
      show-icon
    />

    <div v-loading="store.pending" class="min-h-[420px]">
      <el-form label-position="top" class="grid grid-cols-1 gap-x-8 lg:grid-cols-2">
        <div class="border-b border-gray-200 pb-5 dark:border-zinc-800 lg:border-b-0 lg:border-r lg:pr-8">
          <h3 class="mb-4 mt-0 text-base font-semibold text-gray-800 dark:text-zinc-200">
            {{ t('captainEffect.visual') }}
          </h3>
          <el-form-item :label="t('captainEffect.template')">
            <el-select
              v-model="draft.presetId"
              class="w-full"
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
          </el-form-item>
          <el-form-item :label="t('captainEffect.duration')">
            <el-segmented v-model="draft.durationSeconds" :options="[3, 5, 8, 10]" class="w-full" />
          </el-form-item>
          <el-form-item :label="t('captainEffect.position')">
            <el-select v-model="draft.position" class="w-full">
              <el-option
                v-for="item in positionOptions"
                :key="item.value"
                :label="item.label"
                :value="item.value"
              />
            </el-select>
          </el-form-item>
          <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
            <el-form-item :label="t('captainEffect.windowWidth')">
              <el-input-number
                v-model="draft.windowWidth"
                class="w-full"
                :min="320"
                :max="1920"
                :step="10"
                controls-position="right"
              />
            </el-form-item>
            <el-form-item :label="t('captainEffect.windowHeight')">
              <el-input-number
                v-model="draft.windowHeight"
                class="w-full"
                :min="180"
                :max="1080"
                :step="10"
                controls-position="right"
              />
            </el-form-item>
          </div>
          <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
            <el-form-item :label="t('captainEffect.primaryColor')">
              <div class="flex w-full items-center gap-3">
                <el-color-picker v-model="draft.primaryColor" />
                <el-input v-model="draft.primaryColor" maxlength="7" />
              </div>
            </el-form-item>
            <el-form-item :label="t('captainEffect.secondaryColor')">
              <div class="flex w-full items-center gap-3">
                <el-color-picker v-model="draft.secondaryColor" />
                <el-input v-model="draft.secondaryColor" maxlength="7" />
              </div>
            </el-form-item>
          </div>
        </div>

        <div class="pt-5 lg:pt-0">
          <h3 class="mb-4 mt-0 text-base font-semibold text-gray-800 dark:text-zinc-200">
            {{ t('captainEffect.soundAndCombo') }}
          </h3>
          <div class="mb-5 divide-y divide-gray-200 border-y border-gray-200 dark:divide-zinc-800 dark:border-zinc-800">
            <div
              v-for="row in soundRows"
              :key="row.key"
              class="grid min-h-14 grid-cols-[90px_minmax(0,1fr)_auto] items-center gap-3 py-2"
            >
              <span class="text-sm text-gray-700 dark:text-zinc-300">{{ row.label }}</span>
              <span
                :class="
                  draft[row.key]?.isMissing
                    ? 'truncate text-sm text-red-600 dark:text-red-400'
                    : 'truncate text-sm text-gray-500 dark:text-zinc-400'
                "
              >
                {{ draft[row.key]?.fileName ?? t('captainEffect.notSelected') }}
              </span>
              <div class="flex items-center gap-1">
                <el-tooltip :content="t('captainEffect.selectSound')">
                  <el-button
                    circle
                    :icon="FolderOpened"
                    :aria-label="`${t('captainEffect.selectSound')}: ${row.label}`"
                    @click="selectSound(row.key)"
                  />
                </el-tooltip>
                <el-tooltip :content="t('captainEffect.previewSound')">
                  <el-button
                    circle
                    :icon="VideoPlay"
                    :disabled="!draft[row.key]"
                    :aria-label="`${t('captainEffect.previewSound')}: ${row.label}`"
                    @click="draft[row.key] && previewSound(draft[row.key]!)"
                  />
                </el-tooltip>
                <el-tooltip :content="t('captainEffect.removeSound')">
                  <el-button
                    circle
                    :icon="Delete"
                    :disabled="!draft[row.key]"
                    :aria-label="`${t('captainEffect.removeSound')}: ${row.label}`"
                    @click="draft[row.key] = null"
                  />
                </el-tooltip>
              </div>
            </div>
          </div>
          <el-form-item :label="t('captainEffect.volume')">
            <el-slider
              v-model="draft.volume"
              :min="0"
              :max="1"
              :step="0.1"
              show-input
            />
          </el-form-item>
          <el-form-item :label="t('captainEffect.combo')">
            <el-switch v-model="draft.comboEnabled" />
          </el-form-item>
          <el-form-item :label="t('captainEffect.comboWindow')">
            <el-segmented
              v-model="draft.comboWindowSeconds"
              :options="[5, 10, 30]"
              :disabled="!draft.comboEnabled"
            />
          </el-form-item>
        </div>
      </el-form>
    </div>
  </section>
</template>
