<script setup lang="ts">
import { computed, onMounted, onUnmounted, reactive } from 'vue';
import { Delete, FolderOpened, RefreshLeft, VideoPlay } from '@element-plus/icons-vue';
import type { CaptainEffectConfig, SoundBindingSummary } from '@/platform/desktop';
import { validateHexColor, validateVolume } from '@/modules/media/validation';
import { useMediaEffectsStore } from '@/stores/media/effects';

const store = useMediaEffectsStore();
const draft = reactive<CaptainEffectConfig>({
  presetId: 'captain-default',
  durationSeconds: 5,
  position: 'center',
  primaryColor: '#00AEEC',
  secondaryColor: '#FB7299',
  firstSound: null,
  renewalSound: null,
  comboSound: null,
  volume: 0.8,
  comboEnabled: true,
  comboWindowSeconds: 10
});

const positionOptions = [
  { label: '左上', value: 'topLeft' },
  { label: '右上', value: 'topRight' },
  { label: '左下', value: 'bottomLeft' },
  { label: '右下', value: 'bottomRight' },
  { label: '居中', value: 'center' }
] as const;
const soundRows = [
  { label: '首次开通', key: 'firstSound' },
  { label: '续费', key: 'renewalSound' },
  { label: '连续上舰', key: 'comboSound' }
] as const;

const formValid = computed(() =>
  draft.presetId.length > 0 &&
  validateHexColor(draft.primaryColor).valid &&
  validateHexColor(draft.secondaryColor).valid &&
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
    ElMessage.success('上舰特效配置已保存');
  } catch {
    ElMessage.error(store.error?.message ?? '保存失败');
  }
}

async function preview(): Promise<void> {
  if (!formValid.value) return;
  try {
    await store.preview({ ...draft });
  } catch {
    ElMessage.error(store.error?.message ?? '预览失败');
  }
}

async function selectSound(key: typeof soundRows[number]['key']): Promise<void> {
  try {
    draft[key] = await store.selectSound();
  } catch (error) {
    ElMessage.error((error as { message?: string }).message ?? '选择音效失败');
  }
}

async function previewSound(binding: SoundBindingSummary): Promise<void> {
  try {
    await store.previewSound(binding, draft.volume);
  } catch (error) {
    ElMessage.error((error as { message?: string }).message ?? '音效预览失败');
  }
}

async function resetDefaults(): Promise<void> {
  try {
    await ElMessageBox.confirm('恢复默认配置并立即保存？', '重置上舰特效', {
      confirmButtonText: '重置',
      cancelButtonText: '取消',
      type: 'warning'
    });
    const defaults: CaptainEffectConfig = {
      presetId: 'captain-default',
      durationSeconds: 5,
      position: 'center',
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
    ElMessage.success('已恢复默认配置');
  } catch (error) {
    if (error !== 'cancel' && error !== 'close') {
      ElMessage.error(store.error?.message ?? '重置失败');
    }
  }
}

onMounted(load);
onUnmounted(store.dispose);
</script>

<template>
  <section class="min-w-0 p-[18px] xl:p-6">
    <header class="mb-5 flex flex-wrap items-center gap-3 border-b border-gray-200 pb-4">
      <div class="min-w-56 flex-1">
        <h2 class="m-0 text-xl font-semibold text-gray-900">
          上舰特效
        </h2>
        <p class="mb-0 mt-1 text-xs text-gray-500">
          配置首次开通、续费与连续上舰的本地展示
        </p>
      </div>
      <el-button
        :icon="VideoPlay"
        :loading="store.previewPending"
        :disabled="!formValid"
        @click="preview"
      >
        预览
      </el-button>
      <el-button :icon="RefreshLeft" :disabled="store.savePending" @click="resetDefaults">
        重置默认
      </el-button>
      <el-button
        type="primary"
        :loading="store.savePending"
        :disabled="!formValid"
        @click="save"
      >
        保存
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
      <el-alert
        v-if="store.snapshot && store.snapshot.presets.length < 10"
        class="mb-4"
        type="warning"
        :closable="false"
        title="完整特效模板素材尚未提供，当前保留已保存模板"
        show-icon
      />

      <el-form label-position="top" class="grid grid-cols-1 gap-x-8 lg:grid-cols-2">
        <div class="border-b border-gray-200 pb-5 lg:border-b-0 lg:border-r lg:pr-8">
          <h3 class="mb-4 mt-0 text-base font-semibold text-gray-800">
            视觉与位置
          </h3>
          <el-form-item label="特效模板">
            <el-select v-model="draft.presetId" class="w-full" :disabled="!store.snapshot">
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
          <el-form-item label="持续时间">
            <el-segmented v-model="draft.durationSeconds" :options="[3, 5, 8, 10]" class="w-full" />
          </el-form-item>
          <el-form-item label="显示位置">
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
            <el-form-item label="主色">
              <div class="flex w-full items-center gap-3">
                <el-color-picker v-model="draft.primaryColor" />
                <el-input v-model="draft.primaryColor" maxlength="7" />
              </div>
            </el-form-item>
            <el-form-item label="副色">
              <div class="flex w-full items-center gap-3">
                <el-color-picker v-model="draft.secondaryColor" />
                <el-input v-model="draft.secondaryColor" maxlength="7" />
              </div>
            </el-form-item>
          </div>
        </div>

        <div class="pt-5 lg:pt-0">
          <h3 class="mb-4 mt-0 text-base font-semibold text-gray-800">
            声音与连续触发
          </h3>
          <div class="mb-5 divide-y divide-gray-200 border-y border-gray-200">
            <div v-for="row in soundRows" :key="row.key" class="grid min-h-14 grid-cols-[90px_minmax(0,1fr)_auto] items-center gap-3 py-2">
              <span class="text-sm text-gray-700">{{ row.label }}</span>
              <span :class="draft[row.key]?.isMissing ? 'truncate text-sm text-red-600' : 'truncate text-sm text-gray-500'">
                {{ draft[row.key]?.fileName ?? '未选择' }}
              </span>
              <div class="flex items-center gap-1">
                <el-tooltip content="选择 MP3/WAV 音效">
                  <el-button
                    circle
                    :icon="FolderOpened"
                    :aria-label="`选择${row.label}音效`"
                    @click="selectSound(row.key)"
                  />
                </el-tooltip>
                <el-tooltip content="预览音效">
                  <el-button
                    circle
                    :icon="VideoPlay"
                    :disabled="!draft[row.key]"
                    :aria-label="`预览${row.label}音效`"
                    @click="draft[row.key] && previewSound(draft[row.key]!)"
                  />
                </el-tooltip>
                <el-tooltip content="移除音效">
                  <el-button
                    circle
                    :icon="Delete"
                    :disabled="!draft[row.key]"
                    :aria-label="`移除${row.label}音效`"
                    @click="draft[row.key] = null"
                  />
                </el-tooltip>
              </div>
            </div>
          </div>
          <el-form-item label="音量">
            <el-slider
              v-model="draft.volume"
              :min="0"
              :max="1"
              :step="0.1"
              show-input
            />
          </el-form-item>
          <el-form-item label="连续上舰合并">
            <el-switch v-model="draft.comboEnabled" />
          </el-form-item>
          <el-form-item label="合并窗口">
            <el-segmented v-model="draft.comboWindowSeconds" :options="[5, 10, 30]" :disabled="!draft.comboEnabled" />
          </el-form-item>
        </div>
      </el-form>
    </div>
  </section>
</template>
