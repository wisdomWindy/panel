<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import {
  Clock,
  CreditCard,
  Loading,
  Refresh,
  Setting,
  SwitchButton,
  User,
  WarningFilled
} from '@element-plus/icons-vue';
import type {
  MemberAccount,
  MemberModule,
  ModuleId,
  OrderId,
  UserUid
} from '@/platform/desktop';
import { useMembershipStore } from '@/stores/membership';
import MemberLoginDialog from './components/MemberLoginDialog.vue';
import MembershipModuleDialog from './components/MembershipModuleDialog.vue';
import MembershipOrderDialog from './components/MembershipOrderDialog.vue';
import MembershipPlanGrid from './components/MembershipPlanGrid.vue';

const { t } = useI18n();
const store = useMembershipStore();
const loginVisible = ref(false);
const moduleVisible = ref(false);
const ordersVisible = ref(false);
const candidate = ref<MemberAccount | null>(null);

const account = computed(() => store.status.account);
const entitlement = computed(() => store.status.entitlement);
const isInitial = computed(
  () => store.pendingOperations.has('bootstrap') && store.status.shellState === 'unavailable'
);
const statusLabel = computed(() => t(`membership.shellState.${store.status.shellState}`));
const statusTone = computed(() =>
  store.status.shellState === 'active'
    ? 'app-badge--captain'
    : store.status.shellState === 'expired'
      ? 'app-badge--vip'
      : 'app-badge--plain'
);

function moduleBadge(status: MemberModule['status']): string | undefined {
  return status === 'installed' || status === 'failed' ? undefined : 'app-badge--plain';
}

function moduleStyle(status: MemberModule['status']): Record<string, string> | undefined {
  if (status === 'installed')
    return {
      background: 'color-mix(in srgb, var(--app-green) 20%, transparent)',
      color: 'var(--app-green)'
    };
  if (status === 'failed')
    return {
      background: 'color-mix(in srgb, var(--app-red) 20%, transparent)',
      color: 'var(--app-red)'
    };
  return undefined;
}

async function openLogin(): Promise<void> {
  candidate.value = null;
  loginVisible.value = true;
  try {
    await store.generateQr();
  } catch {
    // The dialog owns the source-backed manual UID fallback.
  }
}

async function refreshQr(): Promise<void> {
  try {
    await store.generateQr();
  } catch {
    // The dialog switches to the approved manual UID fallback.
  }
}

async function cancelQr(): Promise<void> {
  await store.cancelQr().catch(() => undefined);
}

async function queryUid(value: string): Promise<void> {
  candidate.value = null;
  try {
    candidate.value = await store.queryUid(value as UserUid);
  } catch {
    // Stable store error is rendered in the dialog.
  }
}

async function bind(accountValue: MemberAccount): Promise<void> {
  try {
    await store.bindAccount(accountValue);
    loginVisible.value = false;
    candidate.value = null;
    ElMessage.success(t('membership.messages.loginSuccess'));
  } catch {
    // Stable store error is rendered in the dialog.
  }
}

async function logout(): Promise<void> {
  try {
    await store.logout();
    ElMessage.success(t('membership.messages.logoutSuccess'));
  } catch (error) {
    ElMessage.error(
      (error as { message?: string }).message ?? t('membership.messages.logoutFailed')
    );
  }
}

async function openOrders(): Promise<void> {
  ordersVisible.value = true;
  try {
    await store.loadOrders();
  } catch {
    // Store error remains visible on the page.
  }
}

async function openModules(): Promise<void> {
  moduleVisible.value = true;
  try {
    await store.loadModules();
  } catch {
    // Store error remains visible on the page.
  }
}

async function cleanup(moduleId: ModuleId): Promise<void> {
  try {
    await store.cleanupModule(moduleId);
  } catch (error) {
    ElMessage.error(
      (error as { message?: string }).message ?? t('membership.messages.cleanupFailed')
    );
  }
}

async function cancelOrder(orderId: OrderId): Promise<void> {
  try {
    await store.cancelOrder(orderId);
  } catch (error) {
    ElMessage.error(
      (error as { message?: string }).message ?? t('membership.messages.orderCancelFailed')
    );
  }
}

function dateOnly(value: number): string {
  return new Intl.DateTimeFormat(undefined, { dateStyle: 'medium' }).format(value);
}

onMounted(() => {
  if (store.status.shellState === 'unavailable') void store.bootstrap().catch(() => undefined);
});
</script>

<template>
  <section class="mx-auto flex w-full max-w-[1180px] flex-col gap-4">
    <header class="flex flex-wrap items-center gap-3">
      <div class="mr-auto min-w-0">
        <h2 class="app-text m-0 text-lg font-semibold">
          {{ t('membership.title') }}
        </h2>
        <p class="app-muted mt-1 mb-0 text-sm">
          {{ t('membership.description') }}
        </p>
      </div>
      <span class="app-badge" :class="statusTone">{{ statusLabel }}</span>
    </header>

    <el-skeleton v-if="isInitial" :rows="8" animated />
    <template v-else>
      <el-alert
        v-if="store.error && store.status.shellState === 'unavailable'"
        type="error"
        :closable="false"
        :title="store.error.message"
        show-icon
      >
        <button
          type="button"
          class="app-chip h-7 px-2.5 text-xs"
          @click="store.bootstrap()"
        >
          <el-icon><Refresh /></el-icon>
          {{ t('common.retry') }}
        </button>
      </el-alert>

      <section
        v-if="store.status.shellState === 'loggedOut'"
        class="app-card flex min-h-80 flex-col items-center justify-center px-6 text-center"
      >
        <el-icon :size="38" class="app-faint">
          <User />
        </el-icon>
        <h3 class="app-text mb-2 mt-4 text-lg font-semibold">
          {{ t('membership.login') }}
        </h3>
        <p class="app-muted mb-5 mt-0 text-sm">
          {{ t('membership.loginHint') }}
        </p>
        <button
          type="button"
          class="app-btn-primary inline-flex h-8 items-center gap-1.5 px-4 text-sm"
          @click="openLogin"
        >
          <el-icon><User /></el-icon>
          {{ t('membership.login') }}
        </button>
      </section>

      <template v-else-if="account">
        <section class="app-card flex flex-wrap items-center gap-4 px-5 py-4">
          <el-avatar :size="52" :src="account.avatarUrl ?? undefined">
            {{ account.username.slice(0, 1) }}
          </el-avatar>
          <div class="min-w-0 flex-1">
            <div class="app-text truncate text-base font-semibold">
              {{ account.username }}
            </div>
            <div class="app-faint mt-1 text-xs tabular-nums">
              {{ t('membership.uid') }}: {{ account.userUid }}
            </div>
          </div>
          <button
            type="button"
            class="app-chip disabled:cursor-not-allowed disabled:opacity-50"
            :disabled="store.pendingOperations.has('logout')"
            :aria-busy="store.pendingOperations.has('logout')"
            @click="logout"
          >
            <el-icon v-if="store.pendingOperations.has('logout')" class="is-loading">
              <Loading />
            </el-icon>
            <el-icon v-else><SwitchButton /></el-icon>
            {{ t('membership.logout') }}
          </button>
        </section>

        <section
          v-if="store.status.shellState === 'expired'"
          class="app-card flex items-start gap-2 px-4 py-3"
          role="note"
        >
          <el-icon class="mt-0.5 shrink-0" :size="16" style="color: var(--app-gold)">
            <WarningFilled />
          </el-icon>
          <p class="app-muted m-0 text-sm">
            {{ t('membership.expiredNotice') }}
          </p>
        </section>

        <section v-if="entitlement" class="grid gap-3 sm:grid-cols-3">
          <article class="app-card p-4">
            <p class="app-metric-label m-0">
              {{ t('membership.plan') }}
            </p>
            <p class="app-text mt-1.5 mb-0 text-base font-semibold">
              {{ entitlement.planId }}
            </p>
          </article>
          <article class="app-card p-4">
            <p class="app-metric-label m-0">
              {{ t('membership.expiresAt') }}
            </p>
            <p class="app-text mt-1.5 mb-0 flex items-center gap-1 text-base font-semibold">
              <el-icon><Clock /></el-icon>{{ dateOnly(entitlement.expiresAt) }}
            </p>
          </article>
          <article class="app-card p-4">
            <p class="app-metric-label m-0">
              {{ t('membership.remainingDays') }}
            </p>
            <p class="app-text mt-1.5 mb-0 text-base font-semibold tabular-nums">
              {{ t('membership.days', { count: entitlement.remainingDays }) }}
            </p>
          </article>
        </section>

        <div class="flex flex-wrap items-center gap-2">
          <button
            type="button"
            class="app-btn-primary inline-flex h-8 items-center gap-1.5 px-3.5 text-sm disabled:cursor-not-allowed disabled:opacity-60"
            disabled
          >
            <el-icon><CreditCard /></el-icon>
            {{ t('membership.renew') }}
          </button>
          <button type="button" class="app-chip" @click="openModules">
            <el-icon><Setting /></el-icon>
            {{ t('membership.manageModules') }}
          </button>
          <button type="button" class="app-chip" @click="openOrders">
            <el-icon><CreditCard /></el-icon>
            {{ t('membership.orders') }}
          </button>
        </div>

        <MembershipPlanGrid
          v-if="store.status.shellState === 'unpaid' || store.status.shellState === 'expired'"
          :pending="store.pendingOperations.has('purchase')"
          :error-message="store.error?.message ?? null"
          @retry="store.bootstrap()"
        />

        <section v-if="store.modules.length" class="app-card flex flex-col overflow-hidden">
          <div class="app-surface-2 app-hairline border-b px-4 py-2.5">
            <h3 class="app-text m-0 text-sm font-semibold">
              {{ t('membership.installedModules') }}
            </h3>
          </div>
          <div
            v-for="item in store.modules"
            :key="item.moduleId"
            class="app-hairline flex items-center gap-3 border-b px-4 py-3 last:border-b-0"
          >
            <div class="app-text min-w-0 flex-1 truncate font-medium">
              {{ item.name }}
            </div>
            <span class="app-faint text-xs tabular-nums">v{{ item.version }}</span>
            <span class="app-badge" :class="moduleBadge(item.status)" :style="moduleStyle(item.status)">
              {{ t(`membership.moduleStatus.${item.status}`) }}
            </span>
          </div>
        </section>
      </template>

      <el-alert
        v-else-if="store.status.shellState === 'unavailable' && !store.error"
        type="warning"
        :closable="false"
        :title="t('membership.serviceUnavailable')"
        show-icon
      >
        <button type="button" class="app-chip h-7 px-2.5 text-xs" @click="store.bootstrap()">
          <el-icon><Refresh /></el-icon>
          {{ t('common.retry') }}
        </button>
      </el-alert>
    </template>

    <MemberLoginDialog
      v-model:visible="loginVisible"
      :qr-status="store.qrStatus"
      :candidate="candidate"
      :pending="store.pendingOperations"
      :error-message="store.error?.message ?? null"
      @refresh="refreshQr"
      @cancel="cancelQr"
      @query="queryUid"
      @bind="bind"
    />
    <MembershipModuleDialog
      v-model:visible="moduleVisible"
      :modules="store.modules"
      :pending="store.pendingOperations"
      @cleanup="cleanup"
    />
    <MembershipOrderDialog
      v-model:visible="ordersVisible"
      :orders="store.orders"
      :pending="store.pendingOperations"
      @cancel="cancelOrder"
    />
  </section>
</template>
