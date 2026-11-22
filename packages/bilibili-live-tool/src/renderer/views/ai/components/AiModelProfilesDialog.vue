<script setup lang="ts">
import { computed, reactive, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
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
  /** Mainstream presets plus the operator's own ids. */
  modelOptions: readonly { value: string; label: string; vendor: string }[];
}>();

const emit = defineEmits<{
  'update:visible': [value: boolean];
  save: [request: AiModelProfileSaveRequest];
  delete: [profileId: AiModelProfileId];
  setDefault: [profileId: AiModelProfileId];
  test: [profileId: AiModelProfileId];
  addModel: [value: string];
  refreshModels: [payload: { baseUrl: string; apiKey: string | null }];
}>();

const { t } = useI18n();
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
const defaultProfileName = computed(() =>
  props.profiles.find(profile => profile.isDefault)?.name ?? t('ai.voice.emptyModel')
);

watch(
  () => props.visible,
  visible => {
    if (!visible) editorVisible.value = false;
  }
);

/** Asks the provider for its live model list using the form values. */
function refreshModels(): void {
  const baseUrl = draft.baseUrl.trim();
  if (!baseUrl) return;
  emit('refreshModels', {
    baseUrl,
    apiKey: draft.apiKeyReplacement.trim() || null
  });
}

/** Keeps an id the presets do not cover and makes it selectable right away. */
function addModel(): void {
  const value = draft.model.trim();
  if (!value) return;
  emit('addModel', value);
}

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
  return t(`ai.models.connection.${profile.connectionState}`);
}

function providerLabel(kind: AiProviderKind): string {
  return t(`ai.models.provider.${kind}`);
}
</script>

<template>
  <el-dialog
    :model-value="visible"
    width="min(920px, 96vw)"
    :close-on-click-modal="!pending"
    @update:model-value="emit('update:visible', $event)"
  >
    <template #header>
      <h3 class="app-text m-0 text-base font-semibold">
        {{ t('ai.models.title') }}
      </h3>
      <p class="app-muted mt-1 mb-0 text-sm">
        {{ t('ai.models.hint') }}
      </p>
    </template>

    <el-alert
      v-if="errorMessage"
      class="mb-4"
      type="error"
      :closable="false"
      :title="errorMessage"
      show-icon
    />

    <div class="mb-3 flex flex-wrap items-center justify-between gap-3">
      <p class="app-faint m-0 text-xs">
        {{ t('ai.models.defaultHint', { name: defaultProfileName }) }}
      </p>
      <button
        type="button"
        class="app-btn-primary inline-flex h-8 items-center gap-1.5 px-3 text-sm"
        @click="edit()"
      >
        <el-icon><Plus /></el-icon>
        {{ t('ai.models.add') }}
      </button>
    </div>

    <el-table :data="profiles" :empty-text="t('ai.models.empty')">
      <el-table-column :label="t('ai.models.column.name')" min-width="150">
        <template #default="{ row }">
          <span class="app-text">{{ row.name }}</span>
          <el-tag v-if="row.isDefault" class="ml-2" size="small">
            {{ t('ai.models.default') }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column :label="t('ai.models.column.provider')" width="150">
        <template #default="{ row }">
          {{ providerLabel(row.providerKind) }}
        </template>
      </el-table-column>
      <el-table-column prop="model" :label="t('ai.models.column.model')" min-width="160" />
      <el-table-column :label="t('ai.models.column.status')" width="100">
        <template #default="{ row }">
          {{ connectionLabel(row) }}
        </template>
      </el-table-column>
      <el-table-column :label="t('ai.models.column.actions')" width="250" fixed="right">
        <template #default="{ row }">
          <el-button
            link
            :icon="Refresh"
            :disabled="pending"
            @click="emit('test', row.profileId)"
          >
            {{ t('ai.models.test') }}
          </el-button>
          <el-button
            link
            :icon="Edit"
            :disabled="pending"
            @click="edit(row)"
          >
            {{ t('ai.models.edit') }}
          </el-button>
          <el-button
            v-if="!row.isDefault"
            link
            :icon="Promotion"
            :disabled="pending || !row.isEnabled"
            @click="emit('setDefault', row.profileId)"
          >
            {{ t('ai.models.setDefault') }}
          </el-button>
          <el-button
            v-if="!row.isDefault"
            link
            type="danger"
            :icon="Delete"
            :disabled="pending"
            @click="emit('delete', row.profileId)"
          >
            {{ t('ai.models.delete') }}
          </el-button>
        </template>
      </el-table-column>
    </el-table>
    <p class="app-faint mt-3 mb-0 text-xs">
      {{ t('ai.models.secretHint') }}
    </p>

    <el-drawer v-model="editorVisible" size="min(520px, 94vw)" append-to-body>
      <template #header>
        <h3 class="app-text m-0 text-base font-semibold">
          {{ draft.profileId ? t('ai.models.editor.editTitle') : t('ai.models.editor.createTitle') }}
        </h3>
        <p class="app-muted mt-1 mb-0 text-sm">
          {{ t('ai.models.editor.hint') }}
        </p>
      </template>
      <el-form label-position="top" @submit.prevent="submit">
        <el-form-item :label="t('ai.models.editor.name')" required>
          <el-input v-model="draft.name" maxlength="60" />
        </el-form-item>
        <el-form-item :label="t('ai.models.editor.providerKind')" required>
          <el-select v-model="draft.providerKind" class="w-full">
            <el-option :label="t('ai.models.provider.cloud')" value="cloud" />
            <el-option :label="t('ai.models.provider.aggregator')" value="aggregator" />
            <el-option :label="t('ai.models.provider.local')" value="local" />
            <el-option :label="t('ai.models.provider.customOpenAi')" value="customOpenAi" />
          </el-select>
        </el-form-item>
        <el-form-item :label="t('ai.models.editor.baseUrl')" required>
          <el-input v-model="draft.baseUrl" placeholder="https://api.example.com/v1" />
        </el-form-item>
        <el-form-item :label="t('ai.models.editor.model')" required>
          <div class="flex w-full min-w-0 items-center gap-2">
            <!-- Pick from the mainstream list or type any OpenAI-compatible id. -->
            <el-select
              v-model="draft.model"
              class="min-w-0 flex-1"
              filterable
              allow-create
              default-first-option
              :placeholder="t('ai.models.editor.modelPlaceholder')"
            >
              <el-option
                v-for="option in props.modelOptions"
                :key="option.value"
                :label="option.vendor ? `${option.label} · ${option.vendor}` : option.label"
                :value="option.value"
              />
            </el-select>
            <button
              type="button"
              class="app-chip h-9 shrink-0 px-3 text-sm"
              @click="addModel"
            >
              {{ t('ai.models.editor.addModel') }}
            </button>
            <button
              type="button"
              class="app-chip h-9 shrink-0 px-3 text-sm"
              @click="refreshModels"
            >
              {{ t('ai.models.editor.refreshModels') }}
            </button>
          </div>
        </el-form-item>
        <el-form-item :label="t('ai.models.editor.apiKey')">
          <el-input v-model="draft.apiKeyReplacement" type="password" show-password />
          <p class="app-faint m-0 mt-1 w-full text-xs">
            {{ t('ai.models.editor.apiKeyHint') }}
          </p>
        </el-form-item>
        <div class="grid gap-x-4 sm:grid-cols-2">
          <el-form-item :label="t('ai.models.editor.reasoningEffort')">
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
          <el-form-item :label="t('ai.models.editor.maxTokens')">
            <el-input-number
              v-model="draft.maxTokens"
              :min="1"
              :max="8192"
              class="w-full"
            />
          </el-form-item>
          <el-form-item :label="t('ai.models.editor.timeout')">
            <el-input-number
              v-model="draft.timeoutSeconds"
              :min="1"
              :max="120"
              class="w-full"
            />
          </el-form-item>
          <el-form-item :label="t('ai.models.editor.enabled')">
            <el-switch v-model="draft.isEnabled" />
          </el-form-item>
        </div>
      </el-form>
      <template #footer>
        <div class="flex flex-wrap items-center justify-end gap-2">
          <button
            type="button"
            class="app-chip disabled:cursor-not-allowed disabled:opacity-60"
            :disabled="pending"
            @click="editorVisible = false"
          >
            {{ t('common.cancel') }}
          </button>
          <button
            type="button"
            class="app-btn-primary h-8 px-4 text-sm disabled:cursor-not-allowed disabled:opacity-60"
            :disabled="pending || !canSave"
            @click="submit"
          >
            {{ t('common.save') }}
          </button>
        </div>
      </template>
    </el-drawer>
  </el-dialog>
</template>
