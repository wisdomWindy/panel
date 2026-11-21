<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { ModerationAlert } from '@/platform/desktop';

defineProps<{
  visible: boolean;
  alert: ModerationAlert | null;
}>();

defineEmits<{
  'update:visible': [value: boolean];
}>();

const { t } = useI18n();
</script>

<template>
  <el-drawer
    :model-value="visible"
    :title="t('moderation.user.title')"
    size="min(460px, 92vw)"
    @update:model-value="$emit('update:visible', $event)"
  >
    <div v-if="alert" class="flex flex-col gap-4">
      <section class="app-card p-4">
        <div class="app-text font-semibold">
          {{ alert.username }}
        </div>
        <div class="app-faint mt-1 text-xs tabular-nums">
          {{ t('moderation.user.uid') }}: {{ alert.userUid }}
        </div>
      </section>
      <section class="app-card p-4">
        <h3 class="app-text m-0 text-sm font-semibold">
          {{ t('moderation.user.violation') }}
        </h3>
        <p class="app-muted mb-2 mt-2 text-sm">
          {{ alert.violationDescription }}
        </p>
        <p class="app-faint m-0 break-words text-sm">
          {{ t('moderation.contentExample', { content: alert.contentExample }) }}
        </p>
      </section>
    </div>
  </el-drawer>
</template>
