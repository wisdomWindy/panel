<script setup lang="ts">
import { reactive, ref, watch, watchEffect } from 'vue';
import type { FourRegionLayout } from '@/platform/desktop';
import { useI18n } from 'vue-i18n';

const props = defineProps<{ visible: boolean; modelValue: FourRegionLayout }>();
const emit = defineEmits<{
  'update:visible': [value: boolean];
  save: [value: FourRegionLayout];
}>();
const { t } = useI18n();
const draft = reactive<FourRegionLayout>({ ...props.modelValue });
const preview = ref<HTMLElement | null>(null);
type ResizeTarget = 'navigation' | 'toolbar' | 'status';
const activeResize = ref<ResizeTarget | null>(null);

const bounds = {
  navigation: { field: 'navigationWidth', min: 176, max: 280, step: 4, sourceSize: 1024 },
  toolbar: { field: 'toolbarHeight', min: 48, max: 64, step: 2, sourceSize: 680 },
  status: { field: 'statusHeight', min: 28, max: 40, step: 2, sourceSize: 680 }
} as const;

watch(
  () => props.visible,
  visible => {
    if (visible) Object.assign(draft, props.modelValue);
  }
);

watchEffect(() => {
  const element = preview.value;
  if (!element) return;
  element.style.setProperty('--layout-preview-navigation', `${draft.navigationWidth / 10.24}%`);
  element.style.setProperty('--layout-preview-toolbar', `${draft.toolbarHeight / 6.8}%`);
  element.style.setProperty('--layout-preview-status', `${draft.statusHeight / 6.8}%`);
});

function resizeValue(target: ResizeTarget, event: PointerEvent): number {
  const element = preview.value;
  if (!element) return draft[bounds[target].field];
  const rect = element.getBoundingClientRect();
  const rule = bounds[target];
  const ratio = target === 'navigation'
    ? (event.clientX - rect.left) / rect.width
    : target === 'toolbar'
      ? (event.clientY - rect.top) / rect.height
      : (rect.bottom - event.clientY) / rect.height;
  const raw = ratio * rule.sourceSize;
  return Math.min(rule.max, Math.max(rule.min, Math.round(raw / rule.step) * rule.step));
}

function handlePointerDown(target: ResizeTarget, event: PointerEvent): void {
  activeResize.value = target;
  preview.value?.setPointerCapture(event.pointerId);
  draft[bounds[target].field] = resizeValue(target, event);
}

function handlePointerMove(event: PointerEvent): void {
  if (activeResize.value) draft[bounds[activeResize.value].field] = resizeValue(activeResize.value, event);
}

function handlePointerEnd(event: PointerEvent): void {
  if (preview.value?.hasPointerCapture(event.pointerId)) preview.value.releasePointerCapture(event.pointerId);
  activeResize.value = null;
}

function handleResizeKey(target: ResizeTarget, event: KeyboardEvent): void {
  const isIncrease = event.key === 'ArrowRight' || event.key === 'ArrowDown';
  const isDecrease = event.key === 'ArrowLeft' || event.key === 'ArrowUp';
  if (!isIncrease && !isDecrease) return;
  event.preventDefault();
  const rule = bounds[target];
  const next = draft[rule.field] + (isIncrease ? rule.step : -rule.step);
  draft[rule.field] = Math.min(rule.max, Math.max(rule.min, next));
}

function save(): void {
  emit('save', Object.freeze({ ...draft }));
  emit('update:visible', false);
}
</script>

<template>
  <el-dialog
    :model-value="visible"
    :title="t('settings.interface.layoutEditor')"
    width="640px"
    append-to-body
    @close="emit('update:visible', false)"
  >
    <div class="grid grid-cols-[minmax(0,1fr)_15rem] gap-7">
      <div
        ref="preview"
        class="app-surface-2 app-hairline relative grid aspect-[8/5] min-h-0 touch-none grid-rows-[var(--layout-preview-toolbar)_minmax(0,1fr)_var(--layout-preview-status)] overflow-hidden rounded-lg border"
        @pointermove="handlePointerMove"
        @pointerup="handlePointerEnd"
        @pointercancel="handlePointerEnd"
      >
        <div class="app-surface app-hairline app-muted border-b p-2 text-xs">
          {{ t('settings.interface.toolbarRegion') }}
        </div>
        <div class="flex min-h-0">
          <div class="app-surface app-hairline app-muted w-[var(--layout-preview-navigation)] border-r p-2 text-xs">
            {{ t('settings.interface.navigationRegion') }}
          </div>
          <div class="app-faint p-2 text-xs">
            {{ t('settings.interface.contentRegion') }}
          </div>
        </div>
        <div class="app-surface app-hairline app-muted border-t p-2 text-xs">
          {{ t('settings.interface.statusRegion') }}
        </div>
        <button
          type="button"
          role="separator"
          aria-orientation="vertical"
          :aria-label="t('settings.interface.navigationWidth')"
          :aria-valuemin="bounds.navigation.min"
          :aria-valuemax="bounds.navigation.max"
          :aria-valuenow="draft.navigationWidth"
          class="absolute bottom-[var(--layout-preview-status)] top-[var(--layout-preview-toolbar)] z-10 w-2 -translate-x-1/2 cursor-col-resize bg-transparent focus-visible:outline focus-visible:outline-2 focus-visible:outline-[color:var(--app-accent)] [left:var(--layout-preview-navigation)]"
          @pointerdown="handlePointerDown('navigation', $event)"
          @keydown="handleResizeKey('navigation', $event)"
        />
        <button
          type="button"
          role="separator"
          aria-orientation="horizontal"
          :aria-label="t('settings.interface.toolbarHeight')"
          :aria-valuemin="bounds.toolbar.min"
          :aria-valuemax="bounds.toolbar.max"
          :aria-valuenow="draft.toolbarHeight"
          class="absolute inset-x-0 top-[var(--layout-preview-toolbar)] z-10 h-2 -translate-y-1/2 cursor-row-resize bg-transparent focus-visible:outline focus-visible:outline-2 focus-visible:outline-[color:var(--app-accent)]"
          @pointerdown="handlePointerDown('toolbar', $event)"
          @keydown="handleResizeKey('toolbar', $event)"
        />
        <button
          type="button"
          role="separator"
          aria-orientation="horizontal"
          :aria-label="t('settings.interface.statusHeight')"
          :aria-valuemin="bounds.status.min"
          :aria-valuemax="bounds.status.max"
          :aria-valuenow="draft.statusHeight"
          class="absolute bottom-[var(--layout-preview-status)] inset-x-0 z-10 h-2 translate-y-1/2 cursor-row-resize bg-transparent focus-visible:outline focus-visible:outline-2 focus-visible:outline-[color:var(--app-accent)]"
          @pointerdown="handlePointerDown('status', $event)"
          @keydown="handleResizeKey('status', $event)"
        />
      </div>
      <el-form label-position="top">
        <el-form-item :label="t('settings.interface.navigationWidth')">
          <el-slider
            v-model="draft.navigationWidth"
            :min="bounds.navigation.min"
            :max="bounds.navigation.max"
            :step="bounds.navigation.step"
            show-input
          />
        </el-form-item>
        <el-form-item :label="t('settings.interface.toolbarHeight')">
          <el-slider
            v-model="draft.toolbarHeight"
            :min="bounds.toolbar.min"
            :max="bounds.toolbar.max"
            :step="bounds.toolbar.step"
            show-input
          />
        </el-form-item>
        <el-form-item :label="t('settings.interface.statusHeight')">
          <el-slider
            v-model="draft.statusHeight"
            :min="bounds.status.min"
            :max="bounds.status.max"
            :step="bounds.status.step"
            show-input
          />
        </el-form-item>
      </el-form>
    </div>
    <template #footer>
      <div class="flex flex-wrap items-center justify-end gap-2">
        <button type="button" class="app-chip h-8 px-4" @click="emit('update:visible', false)">
          {{ t('common.cancel') }}
        </button>
        <button type="button" class="app-btn-primary h-8 px-4 text-sm" @click="save">
          {{ t('common.save') }}
        </button>
      </div>
    </template>
  </el-dialog>
</template>
