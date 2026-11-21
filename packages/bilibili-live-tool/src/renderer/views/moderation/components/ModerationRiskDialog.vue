<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { Loading } from '@element-plus/icons-vue';

const props = defineProps<{
  visible: boolean;
  pending: boolean;
}>();

const emit = defineEmits<{
  'update:visible': [value: boolean];
  confirm: [];
}>();

const { t } = useI18n();
const accepted = ref(false);
const statements = computed(() => [
  t('moderation.risk.cookie'),
  t('moderation.risk.availability'),
  t('moderation.risk.accountRisk'),
  t('moderation.risk.selfRisk')
]);

watch(
  () => props.visible,
  value => {
    if (value) accepted.value = false;
  }
);

function submit(): void {
  if (props.pending || !accepted.value) return;
  emit('confirm');
}
</script>

<template>
  <el-dialog
    :model-value="visible"
    :title="t('moderation.risk.title')"
    width="min(560px, 94vw)"
    :close-on-click-modal="!pending"
    @update:model-value="emit('update:visible', $event)"
  >
    <ol class="m-0 flex list-decimal flex-col gap-3 pl-5 text-sm leading-6">
      <li v-for="(statement, index) in statements" :key="index" class="app-muted">
        {{ statement }}
      </li>
    </ol>
    <el-checkbox v-model="accepted" class="mt-5">
      {{ t('moderation.risk.acknowledge') }}
    </el-checkbox>
    <template #footer>
      <div class="flex items-center justify-end gap-2">
        <button
          type="button"
          class="app-chip h-8 px-4"
          :disabled="pending"
          @click="emit('update:visible', false)"
        >
          {{ t('common.cancel') }}
        </button>
        <button
          type="button"
          class="app-btn-primary inline-flex h-8 items-center gap-1.5 px-4 text-sm disabled:cursor-not-allowed disabled:opacity-60"
          style="background: var(--app-red)"
          :disabled="!accepted"
          :aria-busy="pending"
          @click="submit"
        >
          <el-icon v-if="pending" class="is-loading"><Loading /></el-icon>
          {{ t('moderation.risk.confirm') }}
        </button>
      </div>
    </template>
  </el-dialog>
</template>
