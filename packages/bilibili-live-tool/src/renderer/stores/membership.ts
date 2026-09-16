import { computed, ref } from 'vue';
import { defineStore } from 'pinia';
import {
  invokeDesktop,
  normalizeDesktopError,
  type AuthQrStatus,
  type DesktopError,
  type MemberAccount,
  type MemberModule,
  type MemberStatus,
  type MembershipOrder,
  type ModuleId,
  type OrderId,
  type UserUid
} from '@/platform/desktop';
import {
  adaptAuthQrStatus,
  adaptMemberAccount,
  adaptMemberModules,
  adaptMemberStatus,
  adaptMembershipOrder,
  adaptMembershipOrders
} from '@/modules/member/adapters';

const INITIAL_STATUS: MemberStatus = Object.freeze({
  shellState: 'unavailable',
  account: null,
  entitlement: null,
  capabilities: Object.freeze({ ai: 'unavailable', moderation: 'unavailable' }),
  serviceErrorCode: null
});

export const useMembershipStore = defineStore('membership', () => {
  const status = ref<MemberStatus>(INITIAL_STATUS);
  const modules = ref<readonly MemberModule[]>([]);
  const orders = ref<readonly MembershipOrder[]>([]);
  const qrStatus = ref<AuthQrStatus | null>(null);
  const pendingOperations = ref<ReadonlySet<string>>(new Set());
  const error = ref<DesktopError | null>(null);
  const operations = new Map<string, Promise<unknown>>();
  let generation = 0;

  const shellState = computed(() => status.value.shellState);
  const isActive = computed(() => shellState.value === 'active');

  function run<T>(key: string, effect: () => Promise<T>): Promise<T> {
    const existing = operations.get(key) as Promise<T> | undefined;
    if (existing) return existing;
    const localGeneration = generation;
    error.value = null;
    pendingOperations.value = new Set([...pendingOperations.value, key]);
    const current = effect()
      .catch(reason => {
        const mapped = normalizeDesktopError(reason);
        if (localGeneration === generation) error.value = mapped;
        throw mapped;
      })
      .finally(() => {
        if (operations.get(key) === current) operations.delete(key);
        if (localGeneration === generation) {
          const next = new Set(pendingOperations.value);
          next.delete(key);
          pendingOperations.value = next;
        }
      });
    operations.set(key, current);
    return current;
  }

  function bootstrap(): Promise<MemberStatus> {
    return run('bootstrap', async () => {
      const next = adaptMemberStatus(await invokeDesktop('member_status', undefined));
      status.value = next;
      return next;
    });
  }

  function generateQr(): Promise<AuthQrStatus> {
    return run('generateQr', async () => {
      const next = adaptAuthQrStatus(await invokeDesktop('auth_qrcode_generate', undefined));
      qrStatus.value = next;
      return next;
    });
  }

  function cancelQr(): Promise<void> {
    const current = qrStatus.value;
    if (!current) return Promise.resolve();
    return run('cancelQr', async () => {
      await invokeDesktop('auth_qrcode_cancel', { sessionId: current.sessionId });
      qrStatus.value = null;
    });
  }

  function queryUid(userUid: UserUid): Promise<MemberAccount> {
    return run('queryUid', async () => adaptMemberAccount(
      await invokeDesktop('auth_query_uid', { userUid })
    ));
  }

  function bindAccount(account: MemberAccount): Promise<MemberStatus> {
    return run('bindAccount', async () => {
      const next = adaptMemberStatus(await invokeDesktop('auth_bind_uid', { account }));
      status.value = next;
      return next;
    });
  }

  function logout(): Promise<MemberStatus> {
    return run('logout', async () => {
      const next = adaptMemberStatus(await invokeDesktop('auth_logout', undefined));
      status.value = next;
      modules.value = Object.freeze([]);
      orders.value = Object.freeze([]);
      return next;
    });
  }

  function purchase(planId: string): Promise<MembershipOrder> {
    return run('purchase', async () => {
      const order = adaptMembershipOrder(await invokeDesktop('membership_purchase', { planId }));
      orders.value = Object.freeze([order, ...orders.value.filter(item => item.orderId !== order.orderId)]);
      return order;
    });
  }

  function cancelOrder(orderId: OrderId): Promise<MembershipOrder> {
    return run(`cancelOrder:${orderId}`, async () => {
      const order = adaptMembershipOrder(await invokeDesktop('membership_cancel_order', { orderId }));
      orders.value = Object.freeze(orders.value.map(item => item.orderId === order.orderId ? order : item));
      return order;
    });
  }

  function loadOrders(): Promise<readonly MembershipOrder[]> {
    return run('orders', async () => {
      const next = adaptMembershipOrders(await invokeDesktop('membership_orders', undefined));
      orders.value = next;
      return next;
    });
  }

  function loadModules(): Promise<readonly MemberModule[]> {
    return run('modules', async () => {
      const next = adaptMemberModules(await invokeDesktop('module_list', undefined));
      modules.value = next;
      return next;
    });
  }

  function cleanupModule(moduleId: ModuleId): Promise<void> {
    return run(`cleanup:${moduleId}`, async () => {
      const result = await invokeDesktop('module_cleanup', { moduleId });
      modules.value = adaptMemberModules(result.modules);
    });
  }

  function replaceStatus(value: unknown): void {
    status.value = adaptMemberStatus(value);
  }

  function replaceQrStatus(value: unknown): void {
    qrStatus.value = adaptAuthQrStatus(value);
  }

  function replaceModules(value: unknown): void {
    modules.value = adaptMemberModules(value);
  }

  function acceptModule(value: unknown): void {
    const module = adaptMemberModules([value])[0];
    if (!module) return;
    modules.value = Object.freeze([module, ...modules.value.filter(item => item.moduleId !== module.moduleId)]);
  }

  function dispose(): void {
    generation += 1;
    operations.clear();
    pendingOperations.value = new Set();
    error.value = null;
  }

  return {
    status,
    modules,
    orders,
    qrStatus,
    pendingOperations,
    error,
    shellState,
    isActive,
    bootstrap,
    generateQr,
    cancelQr,
    queryUid,
    bindAccount,
    logout,
    purchase,
    cancelOrder,
    loadOrders,
    loadModules,
    cleanupModule,
    replaceStatus,
    replaceQrStatus,
    replaceModules,
    acceptModule,
    dispose
  };
});
