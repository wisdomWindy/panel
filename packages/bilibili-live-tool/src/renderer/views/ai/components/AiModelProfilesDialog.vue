<script setup lang="ts">
import { computed, reactive, ref, watch } from 'vue';
import { Delete, Edit, Plus, Promotion, Refresh } from '@element-plus/icons-vue';
import type {
  AiModelProfile,
  AiModelProfileId,
  AiModelProfileSaveRequest,
  AiProviderKind,
  AiReasoningEffort
} from '@/platform/desktop';

const props = defineProps<{
  visible: boolean;
  profiles: readonly AiModelProfile[];
  pending: boolean;
  errorMessage: string | null;
}>();

const emit = defineEmits<{
  'update:visible': [value: boolean];
  save: [request: AiModelProfileSaveRequest];
  delete: [profileId: AiModelProfileId];
  setDefault: [profileId: AiModelProfileId];
  test: [profileId: AiModelProfileId];
}>();

const editorVisible = ref(false);
const draft = reactive({
  profileId: null as AiModelProfileId | null,
  name: '',
  providerKind: 'customOpenAi' as AiProviderKind,
  baseUrl: '',
  model: '',
  reasoningEffort: 'medium' as AiReasoningEffort,
  maxTokens: 512,
  timeoutSeconds: 30,
  isEnabled: true,
  apiKeyReplacement: ''
});

const canSave = computed(() =>
  Boolean(draft.name.trim() && draft.baseUrl.trim() && draft.model.trim())
);

watch(
  () => props.visible,
  visible => {
    if (!visible) editorVisible.value = false;
  }
);

function edit(profile?: AiModelProfile): void {
  Object.assign(draft, profile ? {
    profileId: profile.profileId,
    name: profile.name,
    providerKind: profile.providerKind,
    baseUrl: profile.baseUrl,
    model: profile.model,
    reasoningEffort: profile.reasoningEffort,
    maxTokens: profile.maxTokens,
    timeoutSeconds: profile.timeoutSeconds,
    isEnabled: profile.isEnabled,
    apiKeyReplacement: ''
  } : {
    profileId: null,
    name: '',
    providerKind: 'customOpenAi',
    baseUrl: '',
    model: '',
    reasoningEffort: 'medium',
    maxTokens: 512,
    timeoutSeconds: 30,
    isEnabled: true,
    apiKeyReplacement: ''
  });
  editorVisible.value = true;
}

function submit(): void {
  if (!canSave.value) return;
  emit('save', {
    profile: {
      profileId: draft.profileId,
      name: draft.name.trim(),
      providerKind: draft.providerKind,
      baseUrl: draft.baseUrl.trim(),
      model: draft.model.trim(),
      reasoningEffort: draft.reasoningEffort,
      maxTokens: draft.maxTokens,
      timeoutSeconds: draft.timeoutSeconds,
      proxyUrl: null,
      isEnabled: draft.isEnabled
    },
    apiKeyReplacement: draft.apiKeyReplacement.trim() || null
  });
}

function connectionLabel(profile: AiModelProfile): string {
  return ({
    untested: '未测试',
    testing: '测试中',
    available: '可用',
    unavailable: '不可用'
  })[profile.connectionState];
}
</script>

<template>
  <el-dialog
    :model-value="visible"
    title="AI 模型配置"
    width="min(920px, 96vw)"
    :close-on-click-modal="!pending"
    @update:model-value="emit('update:visible', $event)"
  >
    <el-alert
      v-if="errorMessage"
      class="mb-4"
      type="error"
      :closable="false"
      :title="errorMessage"
      show-icon
    />

    <div class="mb-3 flex justify-end">
      <el-button :icon="Plus" type="primary" @click="edit()">
        新增档案
      </el-button>
    </div>

    <el-table :data="profiles" empty-text="尚未配置模型档案">
      <el-table-column label="档案名" min-width="150">
        <template #default="{ row }">
          <span>{{ row.name }}</span>
          <el-tag v-if="row.isDefault" class="ml-2" size="small">
            默认
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column prop="providerKind" label="Provider" width="130" />
      <el-table-column prop="model" label="模型" min-width="160" />
      <el-table-column label="状态" width="100">
        <template #default="{ row }">
          {{ connectionLabel(row) }}
        </template>
      </el-table-column>
      <el-table-column label="操作" width="250" fixed="right">
        <template #default="{ row }">
          <el-button
            link
            :icon="Refresh"
            :disabled="pending"
            @click="emit('test', row.profileId)"
          >
            测试
          </el-button>
          <el-button
            link
            :icon="Edit"
            :disabled="pending"
            @click="edit(row)"
          >
            编辑
          </el-button>
          <el-button
            v-if="!row.isDefault"
            link
            :icon="Promotion"
            :disabled="pending || !row.isEnabled"
            @click="emit('setDefault', row.profileId)"
          >
            设为默认
          </el-button>
          <el-button
            v-if="!row.isDefault"
            link
            type="danger"
            :icon="Delete"
            :disabled="pending"
            @click="emit('delete', row.profileId)"
          >
            删除
          </el-button>
        </template>
      </el-table-column>
    </el-table>

    <el-drawer
      v-model="editorVisible"
      :title="draft.profileId ? '编辑模型档案' : '新增模型档案'"
      size="min(520px, 94vw)"
      append-to-body
    >
      <el-form label-position="top" @submit.prevent="submit">
        <el-form-item label="档案名" required>
          <el-input v-model="draft.name" maxlength="60" />
        </el-form-item>
        <el-form-item label="Provider 类型" required>
          <el-select v-model="draft.providerKind" class="w-full">
            <el-option label="云模型" value="cloud" />
            <el-option label="聚合 API" value="aggregator" />
            <el-option label="本地模型服务" value="local" />
            <el-option label="自定义 OpenAI 兼容" value="customOpenAi" />
          </el-select>
        </el-form-item>
        <el-form-item label="Base URL" required>
          <el-input v-model="draft.baseUrl" placeholder="https://api.example.com/v1" />
        </el-form-item>
        <el-form-item label="模型名" required>
          <el-input v-model="draft.model" />
        </el-form-item>
        <el-form-item label="API Key">
          <el-input v-model="draft.apiKeyReplacement" type="password" show-password />
        </el-form-item>
        <div class="grid gap-x-4 sm:grid-cols-2">
          <el-form-item label="推理强度">
            <el-select v-model="draft.reasoningEffort" class="w-full">
              <el-option label="none" value="none" />
              <el-option label="minimal" value="minimal" />
              <el-option label="low" value="low" />
              <el-option label="medium" value="medium" />
              <el-option label="high" value="high" />
              <el-option label="xhigh" value="xhigh" />
              <el-option label="max" value="max" />
            </el-select>
          </el-form-item>
          <el-form-item label="最大 Token">
            <el-input-number
              v-model="draft.maxTokens"
              :min="1"
              :max="8192"
              class="w-full"
            />
          </el-form-item>
          <el-form-item label="超时（秒）">
            <el-input-number
              v-model="draft.timeoutSeconds"
              :min="1"
              :max="120"
              class="w-full"
            />
          </el-form-item>
          <el-form-item label="启用档案">
            <el-switch v-model="draft.isEnabled" />
          </el-form-item>
        </div>
      </el-form>
      <template #footer>
        <el-button :disabled="pending" @click="editorVisible = false">
          取消
        </el-button>
        <el-button
          type="primary"
          :loading="pending"
          :disabled="!canSave"
          @click="submit"
        >
          保存
        </el-button>
      </template>
    </el-drawer>
  </el-dialog>
</template>
