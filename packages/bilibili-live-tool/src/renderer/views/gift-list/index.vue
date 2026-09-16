<script setup lang="ts">
import { computed } from 'vue';
import { Delete } from '@element-plus/icons-vue';
import { useLiveFeedStore } from '@/stores/live-data/feed';
import { formatMoneyMinor, formatTimestamp } from '@/modules/live-data/formatters';

const feed = useLiveFeedStore();
const gifts = computed(() => feed.gifts);
const totalValueMinor = computed(() =>
  gifts.value.reduce((sum, gift) => sum + gift.valueMinor, 0)
);
const highestValueMinor = computed(() =>
  gifts.value.reduce((max, gift) => Math.max(max, gift.valueMinor), 0)
);

async function clearGifts(): Promise<void> {
  try {
    await ElMessageBox.confirm('清空后仅移除当前会话中的礼物记录，是否继续？', '清空礼物记录', {
      confirmButtonText: '清空',
      cancelButtonText: '取消',
      type: 'warning'
    });
    feed.clearKind('gift');
    ElMessage.success('礼物记录已清空');
  } catch {
    // Cancellation does not need user feedback.
  }
}
</script>

<template>
  <section class="min-w-0 p-[18px] xl:p-6">
    <header class="mb-5 flex items-start justify-between gap-4">
      <div>
        <h2 class="m-0 text-[22px] text-gray-800">
          礼物列表
        </h2>
        <p class="mb-0 mt-1.5 text-[13px] text-gray-500">
          当前直播会话中由 Rust 接收并通过风控的礼物事件
        </p>
      </div>
      <el-button :icon="Delete" :disabled="gifts.length === 0" @click="clearGifts">
        清空
      </el-button>
    </header>

    <div class="mb-4 grid grid-cols-3 border-y border-gray-200">
      <div class="flex flex-col gap-1.5 px-[18px] py-3.5">
        <span class="text-xs text-gray-500">礼物事件</span>
        <strong class="text-xl text-gray-900">{{ gifts.length }}</strong>
      </div>
      <div class="flex flex-col gap-1.5 border-l border-gray-200 px-[18px] py-3.5">
        <span class="text-xs text-gray-500">总价值</span>
        <strong class="text-xl text-gray-900">{{ formatMoneyMinor(totalValueMinor) }}</strong>
      </div>
      <div class="flex flex-col gap-1.5 border-l border-gray-200 px-[18px] py-3.5">
        <span class="text-xs text-gray-500">单次最高</span>
        <strong class="text-xl text-gray-900">{{ formatMoneyMinor(highestValueMinor) }}</strong>
      </div>
    </div>

    <el-table
      v-if="gifts.length"
      :data="gifts"
      row-key="eventId"
      height="calc(100vh - 292px)"
    >
      <el-table-column prop="occurredAt" label="时间" width="110">
        <template #default="{ row }">
          {{ formatTimestamp(row.occurredAt) }}
        </template>
      </el-table-column>
      <el-table-column
        prop="audience.username"
        label="用户"
        min-width="160"
        show-overflow-tooltip
      />
      <el-table-column
        prop="giftName"
        label="礼物"
        min-width="150"
        show-overflow-tooltip
      />
      <el-table-column
        prop="quantity"
        label="数量"
        width="90"
        align="right"
      />
      <el-table-column
        prop="valueMinor"
        label="价值"
        width="130"
        align="right"
      >
        <template #default="{ row }">
          {{ formatMoneyMinor(row.valueMinor) }}
        </template>
      </el-table-column>
    </el-table>
    <el-empty v-else description="当前会话暂无礼物记录" />

    <footer class="pt-3 text-right text-xs text-gray-500">
      共 {{ gifts.length }} 条礼物事件
    </footer>
  </section>
</template>
