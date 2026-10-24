<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { Delete } from '@element-plus/icons-vue';
import { useLiveFeedStore } from '@/stores/live-data/feed';
import { formatMoneyMinor, formatTimestamp } from '@/modules/live-data/formatters';

const feed = useLiveFeedStore();
const { t } = useI18n();
const gifts = computed(() => feed.gifts);
const totalValueMinor = computed(() =>
  gifts.value.reduce((sum, gift) => sum + gift.valueMinor, 0)
);
const highestValueMinor = computed(() =>
  gifts.value.reduce((max, gift) => Math.max(max, gift.valueMinor), 0)
);

async function clearGifts(): Promise<void> {
  try {
    await ElMessageBox.confirm(t('gift.clearConfirm'), t('gift.clearTitle'), {
      confirmButtonText: t('logs.clear'),
      cancelButtonText: t('common.cancel'),
      type: 'warning'
    });
    feed.clearKind('gift');
    ElMessage.success(t('gift.cleared'));
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
          {{ t('gift.title') }}
        </h2>
        <p class="mb-0 mt-1.5 text-[13px] text-gray-500">
          {{ t('gift.description') }}
        </p>
      </div>
      <el-button :icon="Delete" :disabled="gifts.length === 0" @click="clearGifts">
        {{ t('logs.clear') }}
      </el-button>
    </header>

    <div class="mb-4 grid grid-cols-3 border-y border-gray-200">
      <div class="flex flex-col gap-1.5 px-[18px] py-3.5">
        <span class="text-xs text-gray-500">{{ t('gift.events') }}</span>
        <strong class="text-xl text-gray-900">{{ gifts.length }}</strong>
      </div>
      <div class="flex flex-col gap-1.5 border-l border-gray-200 px-[18px] py-3.5">
        <span class="text-xs text-gray-500">{{ t('dashboard.totalValue') }}</span>
        <strong class="text-xl text-gray-900">{{ formatMoneyMinor(totalValueMinor) }}</strong>
      </div>
      <div class="flex flex-col gap-1.5 border-l border-gray-200 px-[18px] py-3.5">
        <span class="text-xs text-gray-500">{{ t('gift.highestValue') }}</span>
        <strong class="text-xl text-gray-900">{{ formatMoneyMinor(highestValueMinor) }}</strong>
      </div>
    </div>

    <el-table
      v-if="gifts.length"
      :data="gifts"
      row-key="eventId"
      height="calc(100vh - 292px)"
    >
      <el-table-column prop="occurredAt" :label="t('common.time')" width="110">
        <template #default="{ row }">
          {{ formatTimestamp(row.occurredAt) }}
        </template>
      </el-table-column>
      <el-table-column
        prop="audience.username"
        :label="t('gift.user')"
        min-width="160"
        show-overflow-tooltip
      />
      <el-table-column
        prop="giftName"
        :label="t('tts.events.gift')"
        min-width="150"
        show-overflow-tooltip
      />
      <el-table-column
        prop="quantity"
        :label="t('gift.quantity')"
        width="90"
        align="right"
      />
      <el-table-column
        prop="valueMinor"
        :label="t('common.value')"
        width="130"
        align="right"
      >
        <template #default="{ row }">
          {{ formatMoneyMinor(row.valueMinor) }}
        </template>
      </el-table-column>
    </el-table>
    <el-empty v-else :description="t('gift.empty')" />

    <footer class="pt-3 text-right text-xs text-gray-500">
      {{ t('gift.footer', { count: gifts.length }) }}
    </footer>
  </section>
</template>
