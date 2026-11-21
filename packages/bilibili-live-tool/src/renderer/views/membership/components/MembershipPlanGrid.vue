<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Loading, WarningFilled } from '@element-plus/icons-vue';

defineProps<{ pending: boolean; errorMessage: string | null }>();
defineEmits<{ retry: [] }>();

const { t } = useI18n();
</script>

<template>
  <section aria-labelledby="membership-plans-title">
    <h3 id="membership-plans-title" class="app-text mb-3 mt-0 text-base font-semibold">
      {{ t('membership.plans.title') }}
    </h3>
    <div class="app-card flex flex-wrap items-center gap-3 px-4 py-3">
      <el-icon class="shrink-0" :size="16" style="color: var(--app-gold)">
        <WarningFilled />
      </el-icon>
      <p class="app-muted m-0 text-sm">
        {{ errorMessage ?? t('membership.serviceUnavailable') }}
      </p>
      <button
        type="button"
        class="app-chip ml-auto disabled:cursor-not-allowed disabled:opacity-50"
        :disabled="pending"
        :aria-busy="pending"
        @click="$emit('retry')"
      >
        <el-icon v-if="pending" class="is-loading"><Loading /></el-icon>
        {{ t('common.retry') }}
      </button>
    </div>
  </section>
</template>
