<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue';
import { Bell, VideoPlay } from '@element-plus/icons-vue';
import type { GiftEvent, GiftNotifyConfig } from '@/platform/desktop';
import { formatClockTime, formatMoneyMinor } from '@/modules/media/formatters';
import { useMediaGiftsStore } from '@/stores/media/gifts';
import GiftDetailDrawer from '@/components/media/GiftDetailDrawer.vue';

const store = useMediaGiftsStore();
const selectedGift = ref<GiftEvent | null>(null);
const drawerVisible = ref(false);
const draft = reactive<GiftNotifyConfig>({
  isEnabled: false,
  highValueThresholdMinor: 10000,
  methods: { systemNotification: true, localPopup: true, sound: true },
  comboEnabled: true,
  comboWindowSeconds: 10,
  popupPosition: 'topRight',
  popupDurationSeconds: 5
});

const methodValid = computed(() =>
  !draft.isEnabled || Object.values(draft.methods).some(Boolean)
);
const savePending = computed(() => store.pendingOperations.has('saveNotify'));
const testPending = computed(() => store.pendingOperations.has('testNotify'));

function replaceDraft(config: GiftNotifyConfig): void {
  Object.assign(draft, { ...config, methods: { ...config.methods } });
}

async function load(): Promise<void> {
  try {
    await store.load();
    if (store.notifySnapshot) replaceDraft(store.notifySnapshot.config);
  } catch {
    // The stable page error region owns load feedback.
  }
}

async function save(): Promise<void> {
  if (!methodValid.value) return;
  try {
    const value = await store.saveNotify({ ...draft, methods: { ...draft.methods } });
    replaceDraft(value.config);
    ElMessage.success('礼物提醒配置已保存');
  } catch {
    ElMessage.error(store.error?.message ?? '保存失败');
  }
}

async function testReminder(): Promise<void> {
  if (!methodValid.value) return;
  try {
    await store.testNotify({ ...draft, methods: { ...draft.methods } });
  } catch {
    ElMessage.error(store.error?.message ?? '测试提醒失败');
  }
}

function showGift(gift: GiftEvent): void {
  selectedGift.value = gift;
  drawerVisible.value = true;
}

onMounted(load);
</script>

<template>
  <section class="min-w-0 p-[18px] xl:p-6">
    <header class="mb-5 flex flex-wrap items-center gap-3 border-b border-gray-200 pb-4">
      <div class="min-w-56 flex-1">
        <h2 class="m-0 text-xl font-semibold text-gray-900">
          礼物提醒
        </h2>
        <p class="mb-0 mt-1 text-xs text-gray-500">
          本场直播的高价值礼物提醒与统计
        </p>
      </div>
      <el-button
        :icon="VideoPlay"
        :loading="testPending"
        :disabled="!methodValid"
        @click="testReminder"
      >
        测试提醒
      </el-button>
      <el-button
        type="primary"
        :icon="Bell"
        :loading="savePending"
        :disabled="!methodValid"
        @click="save"
      >
        保存
      </el-button>
    </header>

    <el-alert
      v-if="store.error"
      class="mb-4"
      type="error"
      :closable="false"
      :title="store.error.message"
      show-icon
    />

    <div v-loading="store.pending" class="grid min-h-64 grid-cols-1 gap-x-8 border-b border-gray-200 pb-5 lg:grid-cols-2">
      <el-form label-position="top">
        <div class="flex min-h-10 items-center justify-between border-b border-gray-200 pb-3">
          <span class="text-sm font-medium text-gray-800">启用礼物提醒</span>
          <el-switch v-model="draft.isEnabled" />
        </div>
        <el-form-item class="mt-4" label="高价值阈值（分）">
          <el-input-number
            v-model="draft.highValueThresholdMinor"
            :min="0"
            :step="100"
            class="!w-full"
          />
        </el-form-item>
        <el-form-item label="提醒方式" :error="methodValid ? '' : '启用时至少选择一种提醒方式'">
          <el-checkbox v-model="draft.methods.systemNotification" label="系统通知" />
          <el-checkbox v-model="draft.methods.localPopup" label="本地弹窗" />
          <el-checkbox v-model="draft.methods.sound" label="声音" />
        </el-form-item>
      </el-form>

      <el-form label-position="top">
        <el-form-item label="连击合并">
          <el-switch v-model="draft.comboEnabled" />
        </el-form-item>
        <el-form-item label="连击窗口">
          <el-segmented v-model="draft.comboWindowSeconds" :options="[5, 10, 30]" :disabled="!draft.comboEnabled" />
        </el-form-item>
        <el-form-item label="弹窗位置">
          <el-select v-model="draft.popupPosition" class="w-full">
            <el-option label="左上" value="topLeft" />
            <el-option label="右上" value="topRight" />
            <el-option label="左下" value="bottomLeft" />
            <el-option label="右下" value="bottomRight" />
            <el-option label="居中" value="center" />
          </el-select>
        </el-form-item>
        <el-form-item label="显示时长（秒）">
          <el-input-number
            v-model="draft.popupDurationSeconds"
            :min="1"
            :max="30"
            class="!w-full"
          />
        </el-form-item>
      </el-form>
    </div>

    <div class="grid grid-cols-3 border-b border-gray-200">
      <div class="px-4 py-4">
        <span class="block text-xs text-gray-500">礼物总价值</span>
        <strong class="mt-1 block text-xl tabular-nums text-gray-900">{{ formatMoneyMinor(store.notifySnapshot?.totalValueMinor ?? 0) }}</strong>
      </div>
      <div class="border-l border-gray-200 px-4 py-4">
        <span class="block text-xs text-gray-500">送礼人数</span>
        <strong class="mt-1 block text-xl tabular-nums text-gray-900">{{ store.notifySnapshot?.donorCount ?? 0 }}</strong>
      </div>
      <div class="border-l border-gray-200 px-4 py-4">
        <span class="block text-xs text-gray-500">礼物数量</span>
        <strong class="mt-1 block text-xl tabular-nums text-gray-900">{{ store.notifySnapshot?.giftCount ?? 0 }}</strong>
      </div>
    </div>

    <div class="pt-5">
      <h3 class="mb-3 mt-0 text-base font-semibold text-gray-800">
        最近礼物
      </h3>
      <el-table
        v-if="store.notifySnapshot?.latestGifts.length"
        :data="store.notifySnapshot.latestGifts"
        row-key="eventId"
        height="300"
        class="w-full"
        @row-click="showGift"
      >
        <el-table-column
          prop="audience.username"
          label="用户"
          min-width="150"
          show-overflow-tooltip
        />
        <el-table-column
          prop="giftName"
          label="礼物"
          min-width="140"
          show-overflow-tooltip
        />
        <el-table-column
          prop="quantity"
          label="数量"
          width="80"
          align="right"
        />
        <el-table-column label="价值" width="130" align="right">
          <template #default="{ row }">
            {{ formatMoneyMinor(row.valueMinor) }}
          </template>
        </el-table-column>
        <el-table-column label="时间" width="110">
          <template #default="{ row }">
            {{ formatClockTime(row.occurredAt) }}
          </template>
        </el-table-column>
      </el-table>
      <el-empty v-else description="当前会话暂无礼物记录" class="h-[300px]" />
    </div>

    <GiftDetailDrawer v-model="drawerVisible" :gift="selectedGift" />
  </section>
</template>
