<script setup lang="ts">
import type { UpdateSnapshot } from '@/platform/desktop';
import { useI18n } from 'vue-i18n';

defineProps<{ modelValue: boolean; snapshot: UpdateSnapshot; isPending: boolean }>();
const emit = defineEmits<{
  'update:modelValue': [value: boolean];
  download: [];
  install: [];
  skip: [];
}>();
const { t } = useI18n();
</script>

<template>
  <el-dialog
    :model-value="modelValue"
    :title="t('update.availableTitle')"
    width="min(90vw, 40rem)"
    @update:model-value="emit('update:modelValue', $event)"
  >
    <div class="grid grid-cols-2 gap-4 text-sm">
      <div>
        <div class="text-gray-500">
          {{ t('update.currentVersion') }}
        </div>
        <div class="mt-1 font-semibold">
          v{{ snapshot.currentVersion }}
        </div>
      </div>
      <div>
        <div class="text-gray-500">
          {{ t('update.latestVersion') }}
        </div>
        <div class="mt-1 font-semibold">
          v{{ snapshot.candidate?.version }}
        </div>
      </div>
    </div>
    <ul class="mt-5 space-y-2 pl-5">
      <li v-for="note in snapshot.candidate?.releaseNotes" :key="note">
        {{ note }}
      </li>
    </ul>
    <el-progress v-if="snapshot.progressPercent !== null" class="mt-5" :percentage="snapshot.progressPercent" />
    <template #footer>
      <div class="flex flex-wrap justify-end gap-2">
        <el-button v-if="!snapshot.candidate?.isForced" :disabled="isPending" @click="emit('update:modelValue', false)">
          {{ t('update.later') }}
        </el-button>
        <el-button v-if="!snapshot.candidate?.isForced" :disabled="isPending" @click="emit('skip')">
          {{ t('update.skipVersion') }}
        </el-button>
        <el-button
          v-if="snapshot.status === 'downloaded'"
          type="primary"
          :loading="isPending"
          @click="emit('install')"
        >
          {{ t('update.restartInstall') }}
        </el-button>
        <el-button
          v-else
          type="primary"
          :loading="isPending"
          @click="emit('download')"
        >
          {{ t('update.updateNow') }}
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>
