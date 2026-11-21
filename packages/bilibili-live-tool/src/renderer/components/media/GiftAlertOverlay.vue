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
    class="app-card fixed z-[70] w-[min(360px,calc(100vw-40px))] p-4 shadow-xl"
  >
    <div class="flex items-start gap-3">
      <img
        v-if="payload.event.giftIconUrl && !imageLoadFailed"
        :src="payload.event.giftIconUrl"
        :alt="payload.event.giftName"
        class="app-surface-2 size-12 shrink-0 rounded-lg object-contain p-1"
        @error="imageLoadFailed = true"
      >
      <span
        v-else
        class="app-surface-2 flex size-12 shrink-0 items-center justify-center rounded-lg"
      >
        <el-icon :size="22" style="color: var(--app-gold)"><Present /></el-icon>
      </span>
      <div class="min-w-0 flex-1">
        <div class="flex items-start justify-between gap-2">
          <h2 class="app-text m-0 text-sm font-semibold">
            {{ t('giftReminder.title') }}
          </h2>
          <button
            type="button"
            :aria-label="t('common.close')"
            :title="t('common.close')"
            class="app-icon-btn shrink-0"
            @click="emit('dismiss')"
          >
            <el-icon><Close /></el-icon>
          </button>
        </div>
        <p class="app-muted mt-1 mb-0 break-words text-sm">
          {{ payload.event.audience.username }}
        </p>
        <!-- Reuses the existing gift message copy so the overlay adds no locale keys. -->
        <p class="app-text mb-0 break-words text-sm font-semibold">
          {{
            t('dashboard.giftMessage', {
              giftName: payload.event.giftName,
              count: payload.event.quantity
            })
          }}
        </p>
        <p class="mt-2 mb-0 text-base font-semibold" style="color: var(--app-gold)">
          {{ formatMoneyMinor(payload.event.valueMinor) }}
        </p>
      </div>
    </div>
  </aside>
</template>
