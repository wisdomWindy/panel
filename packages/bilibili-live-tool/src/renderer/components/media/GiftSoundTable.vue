<script setup lang="ts">
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
</script>

<template>
  <div class="min-h-[360px] overflow-x-auto border-y border-gray-200">
    <el-table
      v-if="rows.length"
      :data="rows"
      row-key="giftId"
      height="360"
      class="min-w-[820px]"
    >
      <el-table-column
        prop="giftName"
        label="礼物"
        min-width="150"
        show-overflow-tooltip
      />
      <el-table-column label="音效文件" min-width="190" show-overflow-tooltip>
        <template #default="{ row }">
          <span v-if="row.isMissing" class="inline-flex items-center gap-1 text-red-600">
            <el-icon><Warning /></el-icon>{{ row.soundFileName ?? '文件缺失' }}
          </span>
          <span v-else class="text-gray-600">{{ row.soundFileName ?? '未绑定' }}</span>
        </template>
      </el-table-column>
      <el-table-column label="音量" width="220">
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
      <el-table-column label="启用" width="80" align="center">
        <template #default="{ row }">
          <el-switch
            :model-value="row.isEnabled"
            :disabled="!row.soundFileName"
            @update:model-value="$emit('enabled', row.giftId, $event)"
          />
        </template>
      </el-table-column>
      <el-table-column
        label="操作"
        width="156"
        fixed="right"
        align="right"
      >
        <template #default="{ row }">
          <el-tooltip content="预览音效">
            <el-button
              circle
              :icon="VideoPlay"
              :loading="pendingOperations.has('previewSound')"
              :disabled="!row.soundFileName || row.isMissing"
              :aria-label="`预览${row.giftName}音效`"
              @click="$emit('preview', row)"
            />
          </el-tooltip>
          <el-tooltip content="选择 MP3/WAV 文件">
            <el-button
              circle
              :icon="FolderOpened"
              :loading="pendingOperations.has(`bindSound:${row.giftId}`)"
              :aria-label="`替换${row.giftName}音效`"
              @click="$emit('replace', row)"
            />
          </el-tooltip>
          <el-tooltip content="移除绑定">
            <el-button
              circle
              :icon="Delete"
              :disabled="!row.soundFileName"
              :loading="pendingOperations.has(`removeSound:${row.giftId}`)"
              :aria-label="`移除${row.giftName}音效`"
              @click="$emit('remove', row)"
            />
          </el-tooltip>
        </template>
      </el-table-column>
    </el-table>
    <el-empty v-else description="当前尚无可配置礼物" class="h-[360px]" />
  </div>
</template>
