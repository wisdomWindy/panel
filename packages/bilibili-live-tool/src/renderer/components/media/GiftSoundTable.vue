<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Delete, FolderOpened, VideoPlay, Warning } from '@element-plus/icons-vue';
import type { GiftId, GiftSoundBinding } from '@/platform/desktop';

defineProps<{
    rows: readonly GiftSoundBinding[];
    pendingOperations: ReadonlySet<string>;
  }>();

defineEmits<{
    preview: [row: GiftSoundBinding];
    replace: [row: GiftSoundBinding];
    remove: [row: GiftSoundBinding];
    volume: [giftId: GiftId, value: number];
    enabled: [giftId: GiftId, value: boolean];
  }>();
const { t } = useI18n();
</script>

<template>
  <div class="min-h-[360px] overflow-x-auto border-y border-gray-200 dark:border-zinc-800">
    <el-table
      v-if="rows.length"
      :data="rows"
      row-key="giftId"
      height="360"
      class="min-w-[820px]"
    >
      <el-table-column
        prop="giftName"
        :label="t('tts.events.gift')"
        min-width="150"
        show-overflow-tooltip
      />
      <el-table-column :label="t('giftSound.file')" min-width="190" show-overflow-tooltip>
        <template #default="{ row }">
          <span v-if="row.isMissing" class="inline-flex items-center gap-1 text-red-600 dark:text-red-400">
            <el-icon><Warning /></el-icon>
            {{ row.soundFileName ?? t('giftSound.fileMissing') }}
          </span>
          <span v-else class="text-gray-600 dark:text-zinc-300">
            {{ row.soundFileName ?? t('giftSound.unbound') }}
          </span>
        </template>
      </el-table-column>
      <el-table-column :label="t('captainEffect.volume')" width="220">
        <template #default="{ row }">
          <el-slider
            :model-value="row.volume"
            :min="0"
            :max="1"
            :step="0.1"
            :disabled="!row.soundFileName"
            @update:model-value="$emit('volume', row.giftId, $event)"
          />
        </template>
      </el-table-column>
      <el-table-column :label="t('common.enable')" width="80" align="center">
        <template #default="{ row }">
          <el-switch
            :model-value="row.isEnabled"
            :disabled="!row.soundFileName"
            @update:model-value="$emit('enabled', row.giftId, $event)"
          />
        </template>
      </el-table-column>
      <el-table-column
        :label="t('common.actions')"
        width="156"
        fixed="right"
        align="right"
      >
        <template #default="{ row }">
          <el-tooltip :content="t('captainEffect.previewSound')">
            <el-button
              circle
              :icon="VideoPlay"
              :loading="pendingOperations.has('previewSound')"
              :disabled="!row.soundFileName || row.isMissing"
              :aria-label="`${t('captainEffect.previewSound')}: ${row.giftName}`"
              @click="$emit('preview', row)"
            />
          </el-tooltip>
          <el-tooltip :content="t('giftSound.selectFile')">
            <el-button
              circle
              :icon="FolderOpened"
              :loading="pendingOperations.has(`bindSound:${row.giftId}`)"
              :aria-label="`${t('giftSound.replaceSound')}: ${row.giftName}`"
              @click="$emit('replace', row)"
            />
          </el-tooltip>
          <el-tooltip :content="t('giftSound.removeBinding')">
            <el-button
              circle
              :icon="Delete"
              :disabled="!row.soundFileName"
              :loading="pendingOperations.has(`removeSound:${row.giftId}`)"
              :aria-label="`${t('giftSound.removeBinding')}: ${row.giftName}`"
              @click="$emit('remove', row)"
            />
          </el-tooltip>
        </template>
      </el-table-column>
    </el-table>
    <el-empty v-else :description="t('giftSound.empty')" class="h-[360px]" />
  </div>
</template>
