<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { Document, Refresh, Setting } from '@element-plus/icons-vue';
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
    ElMessage.success('规则已保存');
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
  <div class="mx-auto w-full max-w-6xl">
    <header class="mb-5 flex flex-wrap items-center justify-between gap-3">
      <div>
        <h2 class="m-0 text-xl font-semibold text-gray-900 dark:text-gray-100">
          刷屏防护
        </h2>
        <p class="mb-0 mt-1 text-sm text-gray-500">
          本地规则检测与待处理告警
        </p>
      </div>
      <div class="flex gap-2">
        <el-button :icon="Document" @click="router.push('/moderation/log')">
          日志
        </el-button>
        <el-button
          :icon="Setting"
          :disabled="!isReady || !store.rules"
          @click="rulesVisible = true"
        >
          规则配置
        </el-button>
      </div>
    </header>

    <MembershipPausedNotice
      v-if="!isReady"
      :reason="capability"
      @renew="router.push('/membership')"
    />

    <template v-if="isReady">
      <el-alert
        v-if="store.error && !store.summary"
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
        <span class="font-medium">自动禁言</span>
        <el-switch
          :model-value="store.rules?.isAutoEnabled ?? false"
          :loading="store.pendingOperations.has('setAuto')"
          :disabled="!canMute"
          :title="canMute ? '' : '本地账号凭证不可用'"
          @change="requestAuto"
        />
        <span class="text-sm text-gray-500">
          禁言时长：{{ store.rules?.defaultMuteDuration ?? '-' }}
        </span>
      </section>

      <el-alert
        class="my-4"
        type="warning"
        :closable="false"
        title="自动禁言依赖本地账号 Cookie 能力，非开放平台标准 API；不承诺可用性，接口变动随时下线。首次启用需风险确认。"
        show-icon
      />
      <el-alert
        v-if="store.summary && !store.summary.isCredentialAvailable"
        class="mb-4"
        type="error"
        :closable="false"
        title="本地账号凭证失效，请重新登录"
        show-icon
      />
    </template>

    <section v-if="store.summary" class="mt-5">
      <h3 class="mb-2 mt-0 text-base font-semibold">
        待处理告警
      </h3>
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
        class="mt-5 flex flex-wrap gap-x-6 gap-y-2 border-y border-gray-200 py-3 text-sm tabular-nums dark:border-gray-700"
      >
        <span>
          今日识别：
          <strong>{{ store.summary.recognizedToday }}</strong>
        </span>
        <span>
          自动禁言：
          <strong>{{ store.summary.autoMutedToday }}</strong>
        </span>
        <span>
          手动禁言：
          <strong>{{ store.summary.manualMutedToday }}</strong>
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

    <el-dialog v-model="muteVisible" title="确认禁言" width="min(440px, 94vw)">
      <p class="mt-0 text-sm">
        {{ selectedAlert?.username }}·UID: {{ selectedAlert?.userUid }}
      </p>
      <el-select v-model="muteDuration" class="w-full" aria-label="禁言时长">
        <el-option label="10 分钟" value="10m" />
        <el-option label="1 小时" value="1h" />
        <el-option label="永久" value="permanent" />
      </el-select>
      <template #footer>
        <el-button
          :disabled="store.pendingOperations.has(`mute:${selectedAlert?.alertId}`)"
          @click="muteVisible = false"
        >
          取消
        </el-button>
        <el-button
          type="danger"
          :loading="store.pendingOperations.has(`mute:${selectedAlert?.alertId}`)"
          @click="confirmMute"
        >
          确认
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>
