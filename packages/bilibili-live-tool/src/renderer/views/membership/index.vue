<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { Clock, CreditCard, Refresh, Setting, SwitchButton, User } from '@element-plus/icons-vue';
import type { MemberAccount, ModuleId, OrderId, UserUid } from '@/platform/desktop';
import { useMembershipStore } from '@/stores/membership';
import MemberLoginDialog from './components/MemberLoginDialog.vue';
import MembershipModuleDialog from './components/MembershipModuleDialog.vue';
import MembershipOrderDialog from './components/MembershipOrderDialog.vue';
import MembershipPlanGrid from './components/MembershipPlanGrid.vue';

const store = useMembershipStore();
const loginVisible = ref(false);
const moduleVisible = ref(false);
const ordersVisible = ref(false);
const candidate = ref<MemberAccount | null>(null);

const account = computed(() => store.status.account);
const entitlement = computed(() => store.status.entitlement);
const isInitial = computed(() => store.pendingOperations.has('bootstrap') && store.status.shellState === 'unavailable');
const statusLabel = computed(() => ({
  unavailable: '服务不可用',
  loggedOut: '未登录',
  unpaid: '未开通',
  active: '会员有效',
  expired: '已到期'
}[store.status.shellState]));
const statusType = computed(() => store.status.shellState === 'active' ? 'success' : store.status.shellState === 'expired' ? 'danger' : 'info');

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
    ElMessage.success('登录成功');
  } catch {
    // Stable store error is rendered in the dialog.
  }
}

async function logout(): Promise<void> {
  try {
    await store.logout();
    ElMessage.success('已退出登录');
  } catch (error) {
    ElMessage.error((error as { message?: string }).message ?? '退出登录失败');
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
    ElMessage.error((error as { message?: string }).message ?? '模块清理失败');
  }
}

async function cancelOrder(orderId: OrderId): Promise<void> {
  try {
    await store.cancelOrder(orderId);
  } catch (error) {
    ElMessage.error((error as { message?: string }).message ?? '订单取消失败');
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
  <div class="mx-auto w-full max-w-5xl">
    <header class="mb-5 flex flex-wrap items-center justify-between gap-3">
      <div>
        <h2 class="m-0 text-xl font-semibold text-gray-900 dark:text-gray-100">
          会员中心
        </h2>
        <p class="mb-0 mt-1 text-sm text-gray-500">
          账号、会员状态与已下载模块
        </p>
      </div>
      <el-tag :type="statusType" effect="plain" size="large">
        {{ statusLabel }}
      </el-tag>
    </header>

    <el-skeleton v-if="isInitial" :rows="8" animated />
    <template v-else>
      <el-alert
        v-if="store.error && store.status.shellState === 'unavailable'"
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
        v-if="store.status.shellState === 'loggedOut'"
        class="flex min-h-80 flex-col items-center justify-center border-y border-gray-200 bg-white px-6 text-center dark:border-gray-700 dark:bg-gray-900"
      >
        <el-icon :size="38" class="text-gray-400">
          <User />
        </el-icon>
        <h3 class="mb-2 mt-4 text-lg font-semibold">
          登录
        </h3>
        <p class="mb-5 mt-0 text-sm text-gray-500">
          扫码登录不可用时可手动输入 UID
        </p>
        <el-button type="primary" :icon="User" @click="openLogin">
          登录
        </el-button>
      </section>

      <template v-else-if="account">
        <section class="flex flex-wrap items-center gap-4 border-y border-gray-200 bg-white px-5 py-4 dark:border-gray-700 dark:bg-gray-900">
          <el-avatar :size="52" :src="account.avatarUrl ?? undefined">
            {{ account.username.slice(0, 1) }}
          </el-avatar>
          <div class="min-w-0 flex-1">
            <div class="truncate text-base font-semibold">
              {{ account.username }}
            </div>
            <div class="mt-1 text-sm text-gray-500">
              UID: {{ account.userUid }}
            </div>
          </div>
          <el-button :icon="SwitchButton" :loading="store.pendingOperations.has('logout')" @click="logout">
            退出登录
          </el-button>
        </section>

        <el-alert
          v-if="store.status.shellState === 'expired'"
          class="mt-4"
          type="warning"
          :closable="false"
          title="会员功能已暂停，免费版功能不受影响。已下载模块保留，续费后立即恢复。"
          show-icon
        />

        <section v-if="entitlement" class="mt-5 grid gap-4 border-b border-gray-200 pb-5 sm:grid-cols-3 dark:border-gray-700">
          <div>
            <div class="text-xs text-gray-500">
              套餐
            </div>
            <div class="mt-1 font-medium">
              {{ entitlement.planId }}
            </div>
          </div>
          <div>
            <div class="text-xs text-gray-500">
              到期时间
            </div>
            <div class="mt-1 flex items-center gap-1 font-medium">
              <el-icon><Clock /></el-icon>{{ dateOnly(entitlement.expiresAt) }}
            </div>
          </div>
          <div>
            <div class="text-xs text-gray-500">
              剩余天数
            </div>
            <div class="mt-1 font-medium">
              {{ entitlement.remainingDays }} 天
            </div>
          </div>
        </section>

        <div class="mt-5 flex flex-wrap gap-2">
          <el-button type="primary" :icon="CreditCard" disabled>
            续费
          </el-button>
          <el-button :icon="Setting" @click="openModules">
            管理模块
          </el-button>
          <el-button :icon="CreditCard" @click="openOrders">
            订单记录
          </el-button>
        </div>

        <MembershipPlanGrid
          v-if="store.status.shellState === 'unpaid' || store.status.shellState === 'expired'"
          class="mt-6"
          :pending="store.pendingOperations.has('purchase')"
          :error-message="store.error?.message ?? null"
          @retry="store.bootstrap()"
        />

        <section v-if="store.modules.length" class="mt-6">
          <h3 class="mb-3 mt-0 text-base font-semibold">
            已安装模块
          </h3>
          <div class="divide-y divide-gray-200 border-y border-gray-200 dark:divide-gray-700 dark:border-gray-700">
            <div v-for="item in store.modules" :key="item.moduleId" class="flex items-center gap-3 py-3">
              <div class="min-w-0 flex-1 truncate font-medium">
                {{ item.name }}
              </div>
              <span class="text-sm text-gray-500">v{{ item.version }}</span>
              <el-tag :type="item.status === 'installed' ? 'success' : item.status === 'failed' ? 'danger' : 'info'">
                {{ item.status }}
              </el-tag>
            </div>
          </div>
        </section>
      </template>

      <el-alert
        v-else-if="store.status.shellState === 'unavailable' && !store.error"
        type="warning"
        :closable="false"
        title="会员服务暂不可用，请稍后重试"
        show-icon
      >
        <el-button size="small" :icon="Refresh" @click="store.bootstrap()">
          重试
        </el-button>
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
  </div>
</template>
