<script setup lang="ts">
import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import type { DataRange } from '@/platform/desktop';
import DataRangeControl from './DataRangeControl.vue';

const props = defineProps<{
    visible: boolean;
    range: DataRange;
    initialType: 'statistics' | 'rankings';
    pending: boolean;
    paths: readonly string[];
  }>();
defineEmits<{
    close: [];
    submit: [types: readonly ('statistics' | 'rankings')[], range: DataRange];
  }>();
const types = ref<('statistics' | 'rankings')[]>([props.initialType]);
const localRange = ref<DataRange>({ ...props.range });
const { t } = useI18n();

watch(
  () => props.visible,
  visible => {
    if (!visible) return;
    types.value = [props.initialType];
    localRange.value = { ...props.range };
  }
);
</script>

<template>
  <el-dialog
    :model-value="visible"
    :title="t('liveData.exportTitle')"
    width="min(560px, 92vw)"
    @close="$emit('close')"
  >
    <div class="space-y-5">
      <DataRangeControl v-model="localRange" />
      <el-checkbox-group v-model="types">
        <el-checkbox value="statistics">
          {{ t('liveData.statistics') }}
        </el-checkbox>
        <el-checkbox value="rankings">
          {{ t('liveData.rankings') }}
        </el-checkbox>
      </el-checkbox-group>
      <div v-if="paths.length" class="border-y border-gray-200 py-3 text-sm text-gray-600">
        <p v-for="path in paths" :key="path" class="m-0 break-all">
          {{ path }}
        </p>
      </div>
    </div>
    <template #footer>
      <el-button @click="$emit('close')">
        {{ t('common.cancel') }}
      </el-button>
      <el-button
        type="primary"
        :loading="pending"
        :disabled="types.length === 0"
        @click="$emit('submit', types, localRange)"
      >
        {{ t('liveData.exportCsv') }}
      </el-button>
    </template>
  </el-dialog>
</template>
