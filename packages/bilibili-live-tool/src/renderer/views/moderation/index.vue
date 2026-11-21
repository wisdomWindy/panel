<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { Document, Refresh, Setting, WarningFilled } from '@element-plus/icons-vue';
import { useRouter } from 'vue-router';
import type { ModerationAlert, ModerationRules, MuteDuration } from '@/platform/desktop';
import MembershipPausedNotice from '@/components/member/MembershipPausedNotice.vue';
import { useLiveSafetyStore } from '@/stores/live-data/safety';
import { useMembershipStore } from '@/stores/membership';
import { useModerationStore } from '@/stores/moderation';
import ModerationAlertList from './components/ModerationAlertList.vue';
import ModerationRiskDialog from './components/ModerationRiskDialog.vue';
import ModerationRulesDialog from './components/ModerationRulesDialog.vue';
import ModerationUserDrawer from './components/ModerationUserDrawer.vue';

const router = useRouter();
const { t } = useI18n();
const store = useModerationStore();
const membership = useMembershipStore();
const safety = useLiveSafetyStore();
const riskVisible = ref(false);
const rulesVisible = ref(false);
const muteVisible = ref(false);
const drawerVisible = ref(false);
const selectedAlert = ref<ModerationAlert | null>(null);
const muteDuration = ref<MuteDuration>('1h');

const capability = computed(() => membership.status.capabilities.moderation);
const isReady = computed(() => capability.value === 'ready');
const isExpired = computed(() => capability.value === 'expired');
const canMute = computed(() => isReady.value && Boolean(store.summary?.isCredentialAvailable));

async function requestAuto(value: boolean | string | number): Promise<void> {
  const enabled = Boolean(value);
  if (!enabled) {
    await store.setAuto(false, false).catch(() => undefined);
    return;
  }
  if (!canMute.value) return;
  if (!store.rules?.isRiskConfirmed) {
    riskVisible.value = true;
    return;
  }
  await store.setAuto(true, true).catch(() => undefined);
}

async function confirmRisk(): Promise<void> {
  try {
    await store.setAuto(true, true);
    riskVisible.value = false;
  } catch {
    // The switch remains off because state changes only from the native snapshot.
  }
}

async function saveRules(rules: ModerationRules): Promise<void> {
  try {
    await store.updateRules(rules);
    rulesVisible.value = false;
    ElMessage.success(t('moderation.rulesSaved'));
  } catch {
    // The dialog renders the normalized native error and preserves its draft.
  }
}

function openMute(alert: ModerationAlert): void {
  selectedAlert.value = alert;
  muteDuration.value = store.rules?.defaultMuteDuration ?? '1h';
  muteVisible.value = true;
}

async function confirmMute(): Promise<void> {
  if (!selectedAlert.value || !canMute.value) return;
  try {
    await store.mute(selectedAlert.value.alertId, muteDuration.value);
    muteVisible.value = false;
  } catch {
    // The alert is retained for manual handling when the native action fails.
  }
}

function viewAlert(alert: ModerationAlert): void {
  selectedAlert.value = alert;
  drawerVisible.value = true;
}

async function ignoreAlert(alert: ModerationAlert): Promise<void> {
  await store.ignore(alert.alertId).catch(() => undefined);
}

async function blacklistAlert(alert: ModerationAlert): Promise<void> {
  try {
    await safety.addBlacklist({
      uid: alert.userUid,
      username: alert.username,
      reason: alert.violationDescription
    });
    await store.ignore(alert.alertId);
  } catch {
    // A failed persistence step keeps the alert visible.
  }
}

onMounted(() => {
  if (isReady.value) void store.bootstrap().catch(() => undefined);
});
</script>

<template>
  <section class="mx-auto flex w-full max-w-[1180px] flex-col gap-4">
    <header class="flex flex-wrap items-center gap-2">
      <div class="mr-auto min-w-0">
        <h2 class="app-text m-0 text-lg font-semibold">
          {{ t('moderation.title') }}
        </h2>
        <p class="app-muted mt-1 mb-0 text-sm">
          {{ t('moderation.description') }}
        </p>
      </div>
      <button type="button" class="app-chip" @click="router.push('/moderation/log')">
        <el-icon><Document /></el-icon>
        {{ t('moderation.log') }}
      </button>
      <button
        type="button"
        class="app-btn-primary inline-flex h-8 items-center gap-1.5 px-3.5 text-sm disabled:cursor-not-allowed disabled:opacity-60"
        :disabled="!isReady || !store.rules"
        @click="rulesVisible = true"
      >
        <el-icon><Setting /></el-icon>
        {{ t('moderation.rules') }}
      </button>
    </header>

    <MembershipPausedNotice
      v-if="!isReady"
      :reason="capability"
      @renew="router.push('/membership')"
    />

    <template v-if="isReady">
      <el-alert
        v-if="store.error && !store.summary"
        type="error"
        :closable="false"
        :title="store.error.message"
        show-icon
      >
        <el-button size="small" :icon="Refresh" @click="store.bootstrap()">
          {{ t('common.retry') }}
        </el-button>
      </el-alert>

      <section class="app-card flex flex-wrap items-center gap-4 px-4 py-3">
        <span class="app-text font-medium">{{ t('moderation.autoMute') }}</span>
        <el-switch
          :model-value="store.rules?.isAutoEnabled ?? false"
          :loading="store.pendingOperations.has('setAuto')"
          :disabled="!canMute"
          :aria-label="t('moderation.autoMute')"
          :title="canMute ? '' : t('moderation.credentialUnavailable')"
          @change="requestAuto"
        />
        <span class="app-muted text-sm">
          {{
            t('moderation.muteDurationValue', {
              duration: store.rules?.defaultMuteDuration ?? '-'
            })
          }}
        </span>
      </section>

      <section class="app-card flex items-start gap-2 px-4 py-3" role="note">
        <el-icon class="mt-0.5 shrink-0" :size="16" style="color: var(--app-gold)">
          <WarningFilled />
        </el-icon>
        <p class="app-muted m-0 text-sm">
          {{ t('moderation.riskWarning') }}
        </p>
      </section>
      <el-alert
        v-if="store.summary && !store.summary.isCredentialAvailable"
        type="error"
        :closable="false"
        :title="t('moderation.credentialExpired')"
        show-icon
      />
    </template>

    <section v-if="store.summary" class="app-card flex flex-col overflow-hidden">
      <div class="app-surface-2 app-hairline flex flex-wrap items-center gap-2 border-b px-4 py-2.5">
        <h3 class="app-text m-0 text-sm font-semibold">
          {{ t('moderation.pendingAlerts') }}
        </h3>
      </div>
      <ModerationAlertList
        :alerts="store.alerts"
        :can-mute="canMute"
        :is-expired="isExpired"
        :pending="store.pendingOperations"
        @view="viewAlert"
        @mute="openMute"
        @ignore="ignoreAlert"
        @blacklist="blacklistAlert"
      />
      <div
        class="app-faint app-hairline flex flex-wrap gap-x-6 gap-y-1 border-t px-4 py-2.5 text-xs tabular-nums"
      >
        <span>
          {{ t('moderation.recognizedToday') }}
          <strong class="app-text">{{ store.summary.recognizedToday }}</strong>
        </span>
        <span>
          {{ t('moderation.autoMutedToday') }}
          <strong class="app-text">{{ store.summary.autoMutedToday }}</strong>
        </span>
        <span>
          {{ t('moderation.manualMutedToday') }}
          <strong class="app-text">{{ store.summary.manualMutedToday }}</strong>
        </span>
      </div>
    </section>

    <ModerationRiskDialog
      v-model:visible="riskVisible"
      :pending="store.pendingOperations.has('setAuto')"
      @confirm="confirmRisk"
    />
    <ModerationRulesDialog
      v-model:visible="rulesVisible"
      :rules="store.rules"
      :pending="store.pendingOperations.has('updateRules')"
      :error-message="store.error?.message ?? null"
      @save="saveRules"
    />
    <ModerationUserDrawer v-model:visible="drawerVisible" :alert="selectedAlert" />

    <el-dialog v-model="muteVisible" :title="t('moderation.confirmMute')" width="min(440px, 94vw)">
      <p class="app-muted mt-0 text-sm">
        {{ selectedAlert?.username }}·UID: {{ selectedAlert?.userUid }}
      </p>
      <el-select
        v-model="muteDuration"
        class="w-full"
        :aria-label="t('moderation.muteDurationLabel')"
      >
        <el-option :label="t('moderation.muteDurations.tenMinutes')" value="10m" />
        <el-option :label="t('moderation.muteDurations.oneHour')" value="1h" />
        <el-option :label="t('moderation.muteDurations.permanent')" value="permanent" />
      </el-select>
      <template #footer>
        <div class="flex items-center justify-end gap-2">
          <button
            type="button"
            class="app-chip h-8 px-4"
            :disabled="store.pendingOperations.has(`mute:${selectedAlert?.alertId}`)"
            @click="muteVisible = false"
          >
            {{ t('common.cancel') }}
          </button>
          <button
            type="button"
            class="app-btn-primary inline-flex h-8 items-center gap-1.5 px-4 text-sm disabled:cursor-not-allowed disabled:opacity-60"
            style="background: var(--app-red)"
            :disabled="store.pendingOperations.has(`mute:${selectedAlert?.alertId}`)"
            :aria-busy="store.pendingOperations.has(`mute:${selectedAlert?.alertId}`)"
            @click="confirmMute"
          >
            {{ t('common.confirm') }}
          </button>
        </div>
      </template>
    </el-dialog>
  </section>
</template>
