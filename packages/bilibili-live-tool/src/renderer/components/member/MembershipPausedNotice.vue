<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { WarningFilled } from '@element-plus/icons-vue';

const props = defineProps<{
  reason: 'expired' | 'missingModule' | 'unavailable';
}>();

defineEmits<{
  renew: [];
}>();

const { t } = useI18n();
const title = computed(() => t(`membership.paused.${props.reason}`));
</script>

<template>
  <section class="app-card flex min-h-72 flex-col items-center justify-center px-6 text-center">
    <el-icon :size="36" style="color: var(--app-gold)">
      <WarningFilled />
    </el-icon>
    <h3 class="app-text mb-2 mt-4 text-lg font-semibold">
      {{ title }}
    </h3>
    <p v-if="reason === 'expired'" class="app-muted mb-5 mt-0 max-w-xl text-sm">
      {{ t('membership.paused.hint') }}
    </p>
    <button
      type="button"
      class="app-btn-primary inline-flex h-8 items-center px-4 text-sm"
      @click="$emit('renew')"
    >
      {{ reason === 'expired' ? t('membership.paused.renew') : t('membership.paused.goToCenter') }}
    </button>
  </section>
</template>
