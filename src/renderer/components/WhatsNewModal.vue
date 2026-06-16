<script setup lang="ts">
import { ref, computed } from 'vue';
import { ChangelogEntry } from '../services/update-service';

const props = defineProps<{
  visible: boolean;
  version: string;
  changelog: ChangelogEntry | null;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'dontRemind'): void;
}>();

const dontRemind = ref(false);

const changeTypeLabels: Record<string, string> = {
  added: '新功能',
  changed: '改进',
  fixed: '修复',
  removed: '移除',
  security: '安全'
};

const changeTypeColors: Record<string, string> = {
  added: '#67C23A',
  changed: '#409EFF',
  fixed: '#E6A23C',
  removed: '#909399',
  security: '#F56C6C'
});

const hasChangelog = computed(() => {
  return props.changelog && props.changelog.changes.length > 0;
});

function handleClose() {
  if (dontRemind.value) {
    emit('dontRemind');
  }
  emit('close');
}
</script>

<template>
  <el-dialog
    :model-value="visible"
    title="新版本特性"
    width="500px"
    :close-on-click-modal="false"
    @close="handleClose"
  >
    <div v-if="hasChangelog" class="changelog-content">
      <h3 class="text-lg font-semibold mb-4">
        v{{ version }} 更新内容
      </h3>

      <div
        v-for="change in changelog?.changes"
        :key="change.type"
        class="mb-4"
      >
        <div class="flex items-center mb-2">
          <el-tag
            :color="changeTypeColors[change.type]"
            effect="dark"
            size="small"
            class="mr-2"
          >
            {{ changeTypeLabels[change.type] || change.type }}
          </el-tag>
        </div>

        <ul class="pl-5 space-y-1">
          <li
            v-for="(item, index) in change.items"
            :key="index"
            class="text-sm text-gray-600"
          >
            {{ item }}
          </li>
        </ul>
      </div>
    </div>

    <div v-else class="text-center py-8 text-gray-500">
      暂无更新日志
    </div>

    <template #footer>
      <div class="flex items-center justify-between">
        <el-checkbox v-model="dontRemind">
          不再提醒此版本
        </el-checkbox>
        <el-button type="primary" @click="handleClose">
          知道了
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>
