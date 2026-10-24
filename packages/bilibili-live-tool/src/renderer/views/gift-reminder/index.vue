<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { Bell, VideoPlay } from '@element-plus/icons-vue';
import type { GiftEvent, GiftNotifyConfig } from '@/platform/desktop';
import { formatClockTime, formatMoneyMinor } from '@/modules/media/formatters';
import { useMediaGiftsStore } from '@/stores/media/gifts';
import GiftDetailDrawer from '@/components/media/GiftDetailDrawer.vue';

const store = useMediaGiftsStore();
const { t } = useI18n();
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

const methodValid = computed(
  () => !draft.isEnabled || Object.values(draft.methods).some(Boolean)
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
    ElMessage.success(t('giftReminder.saved'));
  } catch {
    ElMessage.error(store.error?.message ?? t('common.saveFailed'));
  }
}

async function testReminder(): Promise<void> {
  if (!methodValid.value) return;
  try {
    await store.testNotify({ ...draft, methods: { ...draft.methods } });
  } catch {
    ElMessage.error(store.error?.message ?? t('giftReminder.testFailed'));
  }
}

function showGift(gift: GiftEvent): void {
  selectedGift.value = gift;
  drawerVisible.value = true;
}

onMounted(load);
</script>

<template>
  <section class="min-w-0">
    <header class="mb-5 flex flex-wrap items-center gap-3 border-b border-gray-200 pb-4 dark:border-zinc-800">
      <div class="min-w-56 flex-1">
        <h2 class="m-0 text-xl font-semibold text-gray-900 dark:text-zinc-100">
          {{ t('giftReminder.title') }}
        </h2>
        <p class="mb-0 mt-1 text-xs text-gray-500 dark:text-zinc-400">
          {{ t('giftReminder.description') }}
        </p>
      </div>
      <el-button
        :icon="VideoPlay"
        :loading="testPending"
        :disabled="!methodValid"
        @click="testReminder"
      >
        {{ t('giftReminder.test') }}
      </el-button>
      <el-button
        type="primary"
        :icon="Bell"
        :loading="savePending"
        :disabled="!methodValid"
        @click="save"
      >
        {{ t('common.save') }}
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

    <div
      v-loading="store.pending"
      class="grid min-h-64 grid-cols-1 gap-x-8 border-b border-gray-200 pb-5 dark:border-zinc-800 lg:grid-cols-2"
    >
      <el-form label-position="top">
        <div class="flex min-h-10 items-center justify-between border-b border-gray-200 pb-3 dark:border-zinc-800">
          <span class="text-sm font-medium text-gray-800 dark:text-zinc-200">{{ t('giftReminder.enabled') }}</span>
          <el-switch v-model="draft.isEnabled" />
        </div>
        <el-form-item class="mt-4" :label="t('giftReminder.threshold')">
          <el-input-number
            v-model="draft.highValueThresholdMinor"
            :min="0"
            :step="100"
            class="!w-full"
          />
        </el-form-item>
        <el-form-item
          :label="t('giftReminder.methods')"
          :error="methodValid ? '' : t('giftReminder.methodRequired')"
        >
          <el-checkbox
            v-model="draft.methods.systemNotification"
            :label="t('giftReminder.systemNotification')"
          />
          <el-checkbox v-model="draft.methods.localPopup" :label="t('giftReminder.localPopup')" />
          <el-checkbox v-model="draft.methods.sound" :label="t('giftReminder.sound')" />
        </el-form-item>
      </el-form>

      <el-form label-position="top">
        <el-form-item :label="t('giftReminder.combo')">
          <el-switch v-model="draft.comboEnabled" />
        </el-form-item>
        <el-form-item :label="t('giftReminder.comboWindow')">
          <el-segmented
            v-model="draft.comboWindowSeconds"
            :options="[5, 10, 30]"
            :disabled="!draft.comboEnabled"
          />
        </el-form-item>
        <el-form-item :label="t('giftReminder.popupPosition')">
          <el-select v-model="draft.popupPosition" class="w-full">
            <el-option :label="t('captainEffect.positions.topLeft')" value="topLeft" />
            <el-option :label="t('captainEffect.positions.topRight')" value="topRight" />
            <el-option :label="t('captainEffect.positions.bottomLeft')" value="bottomLeft" />
            <el-option :label="t('captainEffect.positions.bottomRight')" value="bottomRight" />
            <el-option :label="t('captainEffect.positions.center')" value="center" />
          </el-select>
        </el-form-item>
        <el-form-item :label="t('giftReminder.displayDuration')">
          <el-input-number
            v-model="draft.popupDurationSeconds"
            :min="1"
            :max="30"
            class="!w-full"
          />
        </el-form-item>
      </el-form>
    </div>

    <div class="grid grid-cols-3 border-b border-gray-200 dark:border-zinc-800">
      <div class="px-4 py-4">
        <span class="block text-xs text-gray-500 dark:text-zinc-400">{{ t('dashboard.giftRevenue') }}</span>
        <strong class="mt-1 block text-xl tabular-nums text-gray-900 dark:text-zinc-100">
          {{ formatMoneyMinor(store.notifySnapshot?.totalValueMinor ?? 0) }}
        </strong>
      </div>
      <div class="border-l border-gray-200 px-4 py-4 dark:border-zinc-800">
        <span class="block text-xs text-gray-500 dark:text-zinc-400">{{ t('giftReminder.donors') }}</span>
        <strong class="mt-1 block text-xl tabular-nums text-gray-900 dark:text-zinc-100">
          {{ store.notifySnapshot?.donorCount ?? 0 }}
        </strong>
      </div>
      <div class="border-l border-gray-200 px-4 py-4 dark:border-zinc-800">
        <span class="block text-xs text-gray-500 dark:text-zinc-400">{{ t('giftReminder.giftCount') }}</span>
        <strong class="mt-1 block text-xl tabular-nums text-gray-900 dark:text-zinc-100">
          {{ store.notifySnapshot?.giftCount ?? 0 }}
        </strong>
      </div>
    </div>

    <div class="pt-5">
      <h3 class="mb-3 mt-0 text-base font-semibold text-gray-800 dark:text-zinc-200">
        {{ t('giftReminder.recent') }}
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
          :label="t('gift.user')"
          min-width="150"
          show-overflow-tooltip
        />
        <el-table-column
          prop="giftName"
          :label="t('tts.events.gift')"
          min-width="140"
          show-overflow-tooltip
        />
        <el-table-column
          prop="quantity"
          :label="t('gift.quantity')"
          width="80"
          align="right"
        />
        <el-table-column :label="t('common.value')" width="130" align="right">
          <template #default="{ row }">
            {{ formatMoneyMinor(row.valueMinor) }}
          </template>
        </el-table-column>
        <el-table-column :label="t('common.time')" width="110">
          <template #default="{ row }">
            {{ formatClockTime(row.occurredAt) }}
          </template>
        </el-table-column>
      </el-table>
      <el-empty v-else :description="t('gift.empty')" class="h-[300px]" />
    </div>

    <GiftDetailDrawer v-model="drawerVisible" :gift="selectedGift" />
  </section>
</template>
