<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { CopyDocument, Refresh } from '@element-plus/icons-vue';
import { useRouter } from 'vue-router';
import type {
  AiConfigSaveRequest,
  AiModelProfileId,
  AiModelProfileSaveRequest,
  AiOutputDeviceId,
  AiVoiceId,
  AiVoiceReplyRules
} from '@/platform/desktop';
import { useAIStore } from '@/stores/ai';
import AiModelProfilesDialog from './components/AiModelProfilesDialog.vue';
import AiPersonaConfigDialog from './components/AiPersonaConfigDialog.vue';
import AiVoiceReplyControl from './components/AiVoiceReplyControl.vue';

const { t } = useI18n();
const router = useRouter();
const store = useAIStore();
const configVisible = ref(false);
const modelsVisible = ref(false);
const result = computed(() => {
  const generation = store.generation;
  return generation?.status === 'streaming' || generation?.status === 'completed'
    ? generation.result
    : '';
});
const generationError = computed(() => {
  if (store.generation?.errorCode === 'rate_limited') return t('ai.result.rateLimited');
  return store.errorFor('generate')?.message ?? store.generation?.errorCode ?? null;
});

async function copyResult(): Promise<void> {
  if (!result.value) return;
  try {
    await navigator.clipboard.writeText(result.value);
    ElMessage.success(t('ai.result.copied'));
  } catch {
    ElMessage.error(t('ai.result.copyFailed'));
  }
}

async function saveConfig(request: AiConfigSaveRequest): Promise<void> {
  try {
    await store.saveConfig(request);
    configVisible.value = false;
    ElMessage.success(t('ai.messages.configSaved'));
  } catch {
    // The dialog renders the normalized desktop error.
  }
}

async function saveProfile(request: AiModelProfileSaveRequest): Promise<void> {
  try {
    const profile = await store.saveProfile(request);
    if (!store.profiles.some(value => value.isDefault)) {
      await store.setDefaultProfile(profile.profileId);
    }
    ElMessage.success(t('ai.messages.profileSaved'));
  } catch {
    // The dialog renders the normalized desktop error.
  }
}

async function deleteProfile(profileId: AiModelProfileId): Promise<void> {
  try {
    await store.deleteProfile(profileId);
    ElMessage.success(t('ai.messages.profileDeleted'));
  } catch {
    // The dialog renders the normalized desktop error.
  }
}

async function setDefaultProfile(profileId: AiModelProfileId): Promise<void> {
  await store.setDefaultProfile(profileId).catch(() => undefined);
}

async function testProfile(profileId: AiModelProfileId): Promise<void> {
  try {
    await store.testProfile(profileId);
    ElMessage.success(t('ai.messages.profileTested'));
  } catch {
    ElMessage.error(store.error?.message ?? t('ai.messages.profileTestFailed'));
  }
}

async function setVoiceReplyEnabled(isEnabled: boolean, confirmed: boolean): Promise<void> {
  await store.setVoiceReplyEnabled(isEnabled, confirmed).catch(() => undefined);
}

async function confirmAudioRoute(
  deviceId: AiOutputDeviceId,
  voiceId: AiVoiceId,
  volume: number,
  enableAfterSave: boolean
): Promise<void> {
  try {
    await store.setOutputDevice(deviceId, voiceId, volume, true);
    if (enableAfterSave) await store.setVoiceReplyEnabled(true, true);
    ElMessage.success(t('ai.messages.routeConfirmed'));
  } catch {
    // The page readiness alert renders the normalized desktop error.
  }
}

async function testAudioRoute(
  deviceId: AiOutputDeviceId,
  voiceId: AiVoiceId,
  volume: number
): Promise<void> {
  await store.testOutputDevice(deviceId, voiceId, volume).catch(() => undefined);
}

async function updateVoiceRules(rules: AiVoiceReplyRules): Promise<void> {
  await store.updateVoiceRules(rules).catch(() => undefined);
}

onMounted(() => {
  void store.bootstrap().catch(() => undefined);
});
</script>

<template>
  <div class="mx-auto flex w-full max-w-[1180px] flex-col gap-4">
    <header class="flex flex-wrap items-center justify-between gap-3">
      <div class="min-w-0">
        <h2 class="app-text m-0 text-lg font-semibold">
          {{ t('ai.page.title') }}
        </h2>
        <p class="app-muted mt-1 mb-0 text-sm">
          {{ t('ai.page.description') }}
        </p>
      </div>
      <div class="flex flex-wrap items-center gap-2">
        <button
          type="button"
          class="app-icon-btn"
          :aria-label="t('common.refresh')"
          :title="t('common.refresh')"
          @click="store.bootstrap()"
        >
          <el-icon><Refresh /></el-icon>
        </button>
        <nav
          class="app-card flex flex-wrap items-center gap-1 p-1"
          :aria-label="t('ai.page.navLabel')"
        >
          <button
            type="button"
            class="app-chip app-chip--active"
            aria-pressed="true"
          >
            {{ t('ai.page.tabs.reply') }}
          </button>
          <button type="button" class="app-chip" @click="modelsVisible = true">
            {{ t('ai.page.tabs.models') }}
          </button>
          <button type="button" class="app-chip" @click="configVisible = true">
            {{ t('ai.page.tabs.persona') }}
          </button>
          <button type="button" class="app-chip" @click="router.push('/ai/log')">
            {{ t('ai.page.tabs.logs') }}
          </button>
        </nav>
      </div>
    </header>

    <el-alert
      v-if="store.error && !store.config"
      type="error"
      :closable="false"
      :title="store.error.message"
      show-icon
    >
      <el-button size="small" :icon="Refresh" @click="store.bootstrap()">
        {{ t('common.retry') }}
      </el-button>
    </el-alert>

    <AiVoiceReplyControl
      :snapshot="store.voiceReply"
      :profiles="store.profiles"
      :pending="store.pendingOperations"
      @open-models="modelsVisible = true"
      @set-enabled="setVoiceReplyEnabled"
      @confirm-route="confirmAudioRoute"
      @test-output="testAudioRoute"
      @update-rules="updateVoiceRules"
      @pause="store.pauseVoiceQueue"
      @skip="store.skipVoiceQueue"
      @clear="store.clearVoiceQueue"
    />

    <section class="app-card p-5" aria-labelledby="ai-result-title">
      <div class="flex flex-wrap items-start justify-between gap-3">
        <div class="min-w-0">
          <h3 id="ai-result-title" class="app-text m-0 text-base font-semibold">
            {{ t('ai.result.title') }}
          </h3>
          <p class="app-muted mt-1 mb-0 text-sm">
            {{ t('ai.result.localOnly') }}
          </p>
        </div>
        <button
          type="button"
          class="app-btn-primary inline-flex h-8 items-center gap-1.5 px-3 text-sm disabled:cursor-not-allowed disabled:opacity-60"
          :disabled="!result"
          @click="copyResult"
        >
          <el-icon><CopyDocument /></el-icon>
          {{ t('ai.result.copy') }}
        </button>
      </div>

      <el-alert
        v-if="generationError"
        class="mt-3"
        type="error"
        :closable="false"
        :title="generationError"
        show-icon
      />
      <div class="app-surface-2 mt-3 min-h-24 rounded-lg border app-hairline p-3">
        <p
          v-if="result"
          class="app-text m-0 text-sm break-words whitespace-pre-wrap"
        >
          {{ result }}
        </p>
        <p v-else class="app-faint m-0 text-sm">
          {{ t('ai.result.empty') }}
        </p>
      </div>
    </section>

    <AiPersonaConfigDialog
      v-model:visible="configVisible"
      :config="store.config"
      :pending="store.pendingOperations.has('saveConfig')"
      :error-message="store.errorFor('saveConfig')?.message ?? null"
      @save="saveConfig"
    />
    <AiModelProfilesDialog
      v-model:visible="modelsVisible"
      :profiles="store.profiles"
      :pending="store.pendingOperations.has('saveProfile')"
      :error-message="store.errorFor('saveProfile', 'deleteProfile', 'setDefaultProfile', 'testProfile')?.message ?? null"
      @save="saveProfile"
      @delete="deleteProfile"
      @set-default="setDefaultProfile"
      @test="testProfile"
    />
  </div>
</template>
