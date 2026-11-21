<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Loading, Warning } from '@element-plus/icons-vue';
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
  <div class="min-h-[360px] overflow-x-auto">
    <el-table
      v-if="rows.length"
      :data="rows"
      row-key="giftId"
      height="360"
      class="min-w-[880px]"
    >
      <el-table-column
        prop="giftName"
        :label="t('tts.events.gift')"
        min-width="150"
        show-overflow-tooltip
      />
      <el-table-column :label="t('giftSound.file')" min-width="190" show-overflow-tooltip>
        <template #default="{ row }">
          <span
            v-if="row.isMissing"
            class="inline-flex items-center gap-1"
            style="color: var(--app-red)"
          >
            <el-icon><Warning /></el-icon>
            {{ row.soundFileName ?? t('giftSound.fileMissing') }}
          </span>
          <span v-else :class="row.soundFileName ? 'app-text' : 'app-faint'">
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
            :aria-label="`${t('captainEffect.volume')}: ${row.giftName}`"
            @update:model-value="$emit('volume', row.giftId, $event)"
          />
        </template>
      </el-table-column>
      <el-table-column :label="t('common.enable')" width="80" align="center">
        <template #default="{ row }">
          <el-switch
            :model-value="row.isEnabled"
            :disabled="!row.soundFileName"
            :aria-label="`${t('common.enable')}: ${row.giftName}`"
            @update:model-value="$emit('enabled', row.giftId, $event)"
          />
        </template>
      </el-table-column>
      <el-table-column
        :label="t('common.actions')"
        width="236"
        fixed="right"
        align="right"
      >
        <template #default="{ row }">
          <div class="flex items-center justify-end gap-1.5">
            <button
              type="button"
              class="app-chip whitespace-nowrap disabled:cursor-not-allowed disabled:opacity-40"
              :disabled="
                !row.soundFileName || row.isMissing || pendingOperations.has('previewSound')
              "
              :aria-label="`${t('giftSound.preview')}: ${row.giftName}`"
              @click="$emit('preview', row)"
            >
              <el-icon v-if="pendingOperations.has('previewSound')" class="is-loading">
                <Loading />
              </el-icon>
              {{ t('giftSound.preview') }}
            </button>
            <button
              type="button"
              class="app-chip whitespace-nowrap disabled:cursor-not-allowed disabled:opacity-40"
              :disabled="pendingOperations.has(`bindSound:${row.giftId}`)"
              :aria-label="`${t('giftSound.replaceSound')}: ${row.giftName}`"
              @click="$emit('replace', row)"
            >
              <el-icon
                v-if="pendingOperations.has(`bindSound:${row.giftId}`)"
                class="is-loading"
              >
                <Loading />
              </el-icon>
              {{ row.soundFileName ? t('giftSound.replace') : t('giftSound.bind') }}
            </button>
            <button
              type="button"
              class="app-chip whitespace-nowrap disabled:cursor-not-allowed disabled:opacity-40"
              :disabled="!row.soundFileName || pendingOperations.has(`removeSound:${row.giftId}`)"
              :aria-label="`${t('giftSound.removeBinding')}: ${row.giftName}`"
              @click="$emit('remove', row)"
            >
              <el-icon
                v-if="pendingOperations.has(`removeSound:${row.giftId}`)"
                class="is-loading"
              >
                <Loading />
              </el-icon>
              {{ t('giftSound.remove') }}
            </button>
          </div>
        </template>
      </el-table-column>
    </el-table>
    <el-empty v-else :description="t('giftSound.empty')" class="h-[360px]" />
  </div>
</template>
