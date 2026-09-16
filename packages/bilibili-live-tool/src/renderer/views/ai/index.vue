<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { CopyDocument, Delete, Document, Refresh, Setting } from '@element-plus/icons-vue';
import { useRouter } from 'vue-router';
import type { AiConfigSaveRequest, AiContextItem, AsrModelManifest } from '@/platform/desktop';
import MembershipPausedNotice from '@/components/member/MembershipPausedNotice.vue';
import { useLiveFeedStore } from '@/stores/live-data/feed';
import { useAIStore } from '@/stores/ai';
import { useMembershipStore } from '@/stores/membership';
import AiContextList from './components/AiContextList.vue';
import AiPersonaConfigDialog from './components/AiPersonaConfigDialog.vue';
import AsrModelDownloadDialog from './components/AsrModelDownloadDialog.vue';

const router = useRouter();
const store = useAIStore();
const membership = useMembershipStore();
const feed = useLiveFeedStore();
const prompt = ref('');
const selectedIds = ref<ReadonlySet<string>>(new Set());
const configVisible = ref(false);
const asrDialogVisible = ref(false);
const manifest = ref<AsrModelManifest | null>(null);

const capability = computed(() => membership.status.capabilities.ai);
const isReady = computed(() => capability.value === 'ready');
const contexts = computed<readonly AiContextItem[]>(() => {
  const limit = store.config?.danmakuContextCount ?? 20;
  return feed.danmu
    .slice(-limit)
    .reverse()
    .map(event =>
      Object.freeze({
        id: event.eventId,
        source: 'danmaku' as const,
        content: `${event.audience.username}: ${event.content}`
      })
    );
});
const selectedContexts = computed(() =>
  contexts.value.filter(item => selectedIds.value.has(item.id))
);
const canGenerate = computed(() => Boolean(prompt.value.trim() || selectedContexts.value.length));
const result = computed(() => {
  const generation = store.generation;
  return generation?.status === 'streaming' || generation?.status === 'completed'
    ? generation.result
    : '';
});
const generationError = computed(() => {
  if (store.generation?.errorCode === 'rate_limited') return '服务限流，请稍后重试';
  return store.error?.message ?? store.generation?.errorCode ?? null;
});
const asrEnabled = computed(
  () => store.asr?.state === 'listening' || store.asr?.state === 'starting'
);
const asrLabel = computed(
  () =>
    ({
      unavailable: '不可用',
      modelMissing: '模型未下载',
      downloading: '下载中',
      ready: '已就绪',
      starting: '启动中',
      listening: '识别中',
      error: '异常'
    })[store.asr?.state ?? 'unavailable']
);

function toggleContext(item: AiContextItem): void {
  const next = new Set(selectedIds.value);
  if (next.has(item.id)) {
    next.delete(item.id);
  } else {
    next.add(item.id);
    prompt.value = [prompt.value.trimEnd(), item.content].filter(Boolean).join('\n');
  }
  selectedIds.value = next;
}

async function generate(): Promise<void> {
  if (!canGenerate.value || store.isGenerating) return;
  await store.generate(prompt.value.trim(), selectedContexts.value).catch(() => undefined);
}

async function regenerate(): Promise<void> {
  const generationId = store.generation?.generationId;
  if (!generationId || store.isGenerating) return;
  await store.regenerate(generationId).catch(() => undefined);
}

function clear(): void {
  prompt.value = '';
  selectedIds.value = new Set();
  store.clear();
}

async function copyResult(): Promise<void> {
  if (!result.value) return;
  try {
    await navigator.clipboard.writeText(result.value);
    ElMessage.success('已复制');
  } catch {
    ElMessage.error('复制失败');
  }
}

async function saveConfig(request: AiConfigSaveRequest): Promise<void> {
  try {
    await store.saveConfig(request);
    configVisible.value = false;
    ElMessage.success('配置已保存');
  } catch {
    // The dialog renders the normalized desktop error.
  }
}

async function toggleAsr(value: boolean | string | number): Promise<void> {
  if (!value) {
    await store.stopAsr().catch(() => undefined);
    return;
  }
  if (store.asr?.state !== 'ready') {
    asrDialogVisible.value = true;
    return;
  }
  await store.startAsr().catch(() => undefined);
}

async function downloadModel(nextManifest: AsrModelManifest, accepted: boolean): Promise<void> {
  try {
    await store.downloadModel(nextManifest, accepted);
    asrDialogVisible.value = false;
  } catch {
    // Download state and normalized error remain owned by the AI store.
  }
}

onMounted(() => {
  if (isReady.value) void store.bootstrap().catch(() => undefined);
});
</script>

<template>
  <div class="mx-auto w-full max-w-5xl">
    <header class="mb-5 flex flex-wrap items-center justify-between gap-3">
      <div>
        <h2 class="m-0 text-xl font-semibold text-gray-900 dark:text-gray-100">
          AI 助手
        </h2>
        <p class="mb-0 mt-1 text-sm text-gray-500">
          主播主动生成的本地文案建议
        </p>
      </div>
      <div class="flex gap-2">
        <el-button :icon="Document" @click="router.push('/ai/log')">
          日志
        </el-button>
        <el-button :icon="Setting" @click="configVisible = true">
          人设配置
        </el-button>
      </div>
    </header>

    <MembershipPausedNotice
      v-if="!isReady"
      :reason="capability"
      @renew="router.push('/membership')"
    />

    <template v-else>
      <el-alert
        v-if="store.error && !store.config"
        class="mb-4"
        type="error"
        :closable="false"
        :title="store.error.message"
        show-icon
      >
        <el-button size="small" :icon="Refresh" @click="store.bootstrap()">
          重试
        </el-button>
      </el-alert>

      <section
        class="flex flex-wrap items-center gap-4 border-y border-gray-200 bg-white px-4 py-3 dark:border-gray-700 dark:bg-gray-900"
      >
        <span class="font-medium">ASR</span>
        <el-switch
          :model-value="asrEnabled"
          :loading="store.pendingOperations.has('asr')"
          @change="toggleAsr"
        />
        <el-tag effect="plain">
          {{ asrLabel }}
        </el-tag>
        <span class="text-sm text-gray-500">{{ store.asr?.modelName ?? '-' }}</span>
        <span class="text-sm text-gray-500">{{ store.asr?.language ?? '-' }}</span>
      </section>

      <section class="mt-5">
        <h3 class="mb-2 mt-0 text-base font-semibold">
          当前上下文
        </h3>
        <AiContextList :items="contexts" :selected-ids="selectedIds" @select="toggleContext" />
      </section>

      <section class="mt-5">
        <label class="mb-2 block text-base font-semibold" for="ai-prompt">输入</label>
        <el-input
          id="ai-prompt"
          v-model="prompt"
          type="textarea"
          :rows="5"
          resize="vertical"
        />
        <div class="mt-3 flex flex-wrap gap-2">
          <el-button
            type="primary"
            :loading="store.isGenerating"
            :disabled="!canGenerate"
            @click="generate"
          >
            生成
          </el-button>
          <el-button
            :icon="Refresh"
            :disabled="!store.generation || store.isGenerating"
            @click="regenerate"
          >
            重新生成
          </el-button>
          <el-button :icon="Delete" :disabled="store.isGenerating" @click="clear">
            清空
          </el-button>
        </div>
      </section>

      <section class="mt-5 border-t border-gray-200 pt-4 dark:border-gray-700">
        <div class="mb-2 flex flex-wrap items-center justify-between gap-2">
          <h3 class="m-0 text-base font-semibold">
            结果·仅本地预览，不发送
          </h3>
          <el-button :icon="CopyDocument" :disabled="!result" @click="copyResult">
            复制
          </el-button>
        </div>
        <el-alert
          v-if="generationError"
          class="mb-3"
          type="error"
          :closable="false"
          :title="generationError"
          show-icon
        />
        <el-input
          :model-value="result"
          type="textarea"
          readonly
          :rows="8"
          resize="vertical"
        />
      </section>
    </template>

    <AiPersonaConfigDialog
      v-model:visible="configVisible"
      :config="store.config"
      :pending="store.pendingOperations.has('saveConfig')"
      :error-message="store.error?.message ?? null"
      @save="saveConfig"
    />
    <AsrModelDownloadDialog
      v-model:visible="asrDialogVisible"
      :manifest="manifest"
      :status="store.asr"
      :pending="store.pendingOperations.has('downloadModel')"
      @download="downloadModel"
    />
  </div>
</template>
