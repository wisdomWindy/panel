<script setup lang="ts">
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

const currency = (value: MembershipOrder) => new Intl.NumberFormat(undefined, {
  style: 'currency', currency: value.currency
}).format(value.amountMinor / 100);
const dateTime = (value: number) => new Intl.DateTimeFormat(undefined, {
  dateStyle: 'medium', timeStyle: 'short'
}).format(value);
</script>

<template>
  <el-dialog
    :model-value="visible"
    title="订单记录"
    width="min(760px, calc(100vw - 32px))"
    @update:model-value="$emit('update:visible', $event)"
  >
    <el-table v-if="orders.length" :data="orders" class="w-full">
      <el-table-column
        prop="orderId"
        label="订单号"
        min-width="180"
        show-overflow-tooltip
      />
      <el-table-column prop="planLabel" label="套餐" width="100" />
      <el-table-column label="金额" width="120">
        <template #default="scope">
          {{ currency(scope.row) }}
        </template>
      </el-table-column>
      <el-table-column label="创建时间" min-width="170">
        <template #default="scope">
          {{ dateTime(scope.row.createdAt) }}
        </template>
      </el-table-column>
      <el-table-column prop="status" label="状态" width="100" />
      <el-table-column label="操作" width="90" fixed="right">
        <template #default="scope">
          <el-button
            v-if="scope.row.status === 'pending'"
            link
            type="danger"
            :loading="pending.has(`cancelOrder:${scope.row.orderId}`)"
            @click="$emit('cancel', scope.row.orderId)"
          >
            取消
          </el-button>
        </template>
      </el-table-column>
    </el-table>
  </el-dialog>
</template>
