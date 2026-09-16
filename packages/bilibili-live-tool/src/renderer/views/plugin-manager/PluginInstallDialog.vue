<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { PluginSnapshot } from '@/platform/desktop';

defineProps<{ modelValue: boolean; manifest: PluginSnapshot | null; isPending: boolean }>();
const emit = defineEmits<{
  'update:modelValue': [value: boolean];
  select: [];
  install: [pluginId: string];
}>();
const { t } = useI18n();
</script>

<template>
  <el-dialog
    :model-value="modelValue"
    :title="t('plugins.install')"
    width="min(90vw, 40rem)"
    @update:model-value="emit('update:modelValue', $event)"
  >
    <el-alert
      type="info"
      show-icon
      :closable="false"
      :title="t('plugins.safety')"
    />
    <div v-if="manifest" class="mt-5 space-y-3 text-sm">
      <div class="font-semibold">
        {{ manifest.name }} v{{ manifest.version }}
      </div>
      <div class="flex flex-wrap gap-2">
        <el-tag v-for="permission in manifest.permissions" :key="permission">
          {{ t(`plugins.permissions.${permission}`) }}
        </el-tag>
      </div>
    </div>
    <template #footer>
      <div class="flex justify-end gap-2">
        <el-button :disabled="isPending" @click="emit('update:modelValue', false)">
          {{ t('common.cancel') }}
        </el-button>
        <el-button :loading="isPending" @click="emit('select')">
          {{ t('plugins.selectPackage') }}
        </el-button>
        <el-button
          type="primary"
          :loading="isPending"
          :disabled="!manifest"
          @click="manifest && emit('install', manifest.id)"
        >
          {{ t('plugins.install') }}
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>
