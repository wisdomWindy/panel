<script setup lang="ts">
import { computed, onUnmounted, ref, watch } from 'vue';
import { Present, Close } from '@element-plus/icons-vue';
import { useI18n } from 'vue-i18n';
import { formatMoneyMinor } from '@/modules/media/formatters';
import type { GiftAlertPayload, GiftPopupPosition } from '@/platform/desktop';

const props = defineProps<{
  payload: GiftAlertPayload | null;
}>();

const emit = defineEmits<{
  dismiss: [];
}>();

const { t } = useI18n();
const positionClasses: Record<GiftPopupPosition, string> = {
  topLeft: 'left-5 top-5',
  topRight: 'right-5 top-5',
  bottomLeft: 'bottom-5 left-5',
  bottomRight: 'bottom-5 right-5',
  center: 'left-1/2 top-1/2 translate-x-[-50%] translate-y-[-50%]'
};
const placement = computed(() =>
  props.payload ? positionClasses[props.payload.popupPosition] : positionClasses.bottomRight
);
const imageLoadFailed = ref(false);

let dismissTimer: ReturnType<typeof setTimeout> | null = null;

function clearDismissTimer(): void {
  if (dismissTimer !== null) {
    clearTimeout(dismissTimer);
    dismissTimer = null;
  }
}

watch(
  () => props.payload,
  payload => {
    clearDismissTimer();
    imageLoadFailed.value = false;
    if (!payload) return;
    const eventId = payload.event.eventId;
    dismissTimer = setTimeout(() => {
      if (props.payload?.event.eventId === eventId) emit('dismiss');
    }, payload.popupDurationSeconds * 1_000);
  },
  { immediate: true }
);

onUnmounted(clearDismissTimer);
</script>

<template>
  <aside
    v-if="payload"
    aria-live="assertive"
    :class="placement"
    class="fixed z-[70] w-[min(360px,calc(100vw-40px))] rounded-lg border border-amber-200 bg-white p-4 shadow-xl dark:border-amber-700 dark:bg-zinc-900"
  >
    <div class="flex items-start gap-3">
      <img
        v-if="payload.event.giftIconUrl && !imageLoadFailed"
        :src="payload.event.giftIconUrl"
        :alt="payload.event.giftName"
        class="size-12 shrink-0 rounded-lg bg-amber-50 object-contain p-1 dark:bg-amber-950"
        @error="imageLoadFailed = true"
      >
      <span v-else class="flex size-12 shrink-0 items-center justify-center rounded-lg bg-amber-100 text-amber-700 dark:bg-amber-900 dark:text-amber-200">
        <el-icon :size="22"><Present /></el-icon>
      </span>
      <div class="min-w-0 flex-1">
        <div class="flex items-start justify-between gap-2">
          <h2 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100">
            {{ t('giftReminder.title') }}
          </h2>
          <button
            type="button"
            :aria-label="t('common.close')"
            :title="t('common.close')"
            class="flex size-7 shrink-0 items-center justify-center rounded text-zinc-500 hover:bg-zinc-100 hover:text-zinc-800 dark:hover:bg-zinc-800 dark:hover:text-zinc-100"
            @click="emit('dismiss')"
          >
            <el-icon><Close /></el-icon>
          </button>
        </div>
        <p class="mt-1 break-words text-sm text-zinc-600 dark:text-zinc-300">
          {{ payload.event.audience.username }} 送出
        </p>
        <p class="break-words text-sm font-semibold text-zinc-900 dark:text-zinc-100">
          {{ payload.event.giftName }} ×{{ payload.event.quantity }}
        </p>
        <p class="mt-2 text-base font-semibold text-amber-700 dark:text-amber-300">
          {{ formatMoneyMinor(payload.event.valueMinor) }}
        </p>
      </div>
    </div>
  </aside>
</template>
