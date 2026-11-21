<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Loading } from '@element-plus/icons-vue';
import type { MembershipOrder, OrderId } from '@/platform/desktop';

defineProps<{
  visible: boolean;
  orders: readonly MembershipOrder[];
  pending: ReadonlySet<string>;
}>();
defineEmits<{
  'update:visible': [value: boolean];
  cancel: [orderId: OrderId];
}>();

const { t } = useI18n();
const headerCellStyle = {
  background: 'transparent',
  color: 'var(--app-text-3)',
  fontWeight: '500'
};

const currency = (value: MembershipOrder) =>
  new Intl.NumberFormat(undefined, {
    style: 'currency',
    currency: value.currency
  }).format(value.amountMinor / 100);
const dateTime = (value: number) =>
  new Intl.DateTimeFormat(undefined, {
    dateStyle: 'medium',
    timeStyle: 'short'
  }).format(value);

function statusStyle(status: MembershipOrder['status']): Record<string, string> {
  if (status === 'paid')
    return {
      background: 'color-mix(in srgb, var(--app-green) 20%, transparent)',
      color: 'var(--app-green)'
    };
  if (status === 'pending')
    return {
      background: 'color-mix(in srgb, var(--app-gold) 20%, transparent)',
      color: 'var(--app-gold)'
    };
  if (status === 'failed' || status === 'timedOut')
    return {
      background: 'color-mix(in srgb, var(--app-red) 20%, transparent)',
      color: 'var(--app-red)'
    };
  return { background: 'var(--app-panel-2)', color: 'var(--app-text-3)' };
}
</script>

<template>
  <el-dialog
    :model-value="visible"
    :title="t('membership.orderDialog.title')"
    width="min(760px, calc(100vw - 32px))"
    @update:model-value="$emit('update:visible', $event)"
  >
    <div class="app-card overflow-hidden">
      <el-table
        v-if="orders.length"
        :data="orders"
        class="w-full"
        :header-cell-style="headerCellStyle"
      >
        <el-table-column
          prop="orderId"
          :label="t('membership.orderDialog.id')"
          min-width="180"
          show-overflow-tooltip
        />
        <el-table-column
          prop="planLabel"
          :label="t('membership.orderDialog.plan')"
          width="110"
        />
        <el-table-column :label="t('membership.orderDialog.amount')" width="120">
          <template #default="scope">
            <span class="app-text tabular-nums">{{ currency(scope.row) }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('membership.orderDialog.createdAt')" min-width="170">
          <template #default="scope">
            <span class="app-muted tabular-nums">{{ dateTime(scope.row.createdAt) }}</span>
          </template>
        </el-table-column>
        <el-table-column :label="t('common.status')" width="140">
          <template #default="scope">
            <span class="app-badge" :style="statusStyle(scope.row.status)">
              {{ t(`membership.orderStatus.${scope.row.status}`) }}
            </span>
          </template>
        </el-table-column>
        <el-table-column :label="t('common.actions')" width="100" fixed="right">
          <template #default="scope">
            <button
              v-if="scope.row.status === 'pending'"
              type="button"
              class="app-chip h-7 px-2.5 text-xs disabled:cursor-not-allowed disabled:opacity-50"
              style="color: var(--app-red); border-color: color-mix(in srgb, var(--app-red) 45%, transparent)"
              :disabled="pending.has(`cancelOrder:${scope.row.orderId}`)"
              :aria-busy="pending.has(`cancelOrder:${scope.row.orderId}`)"
              @click="$emit('cancel', scope.row.orderId)"
            >
              <el-icon
                v-if="pending.has(`cancelOrder:${scope.row.orderId}`)"
                class="is-loading"
              >
                <Loading />
              </el-icon>
              {{ t('common.cancel') }}
            </button>
          </template>
        </el-table-column>
      </el-table>
      <el-empty v-else :image-size="72" :description="t('common.noData')" />
    </div>
  </el-dialog>
</template>
