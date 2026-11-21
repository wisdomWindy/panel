<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { Bell, Loading, VideoPlay } from '@element-plus/icons-vue';
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
  <section class="mx-auto flex w-full max-w-[1180px] flex-col gap-4">
    <header class="flex flex-wrap items-center gap-2">
      <div class="mr-auto min-w-0">
        <h2 class="app-text m-0 text-lg font-semibold">{{ t('giftReminder.title') }}</h2>
        <p class="app-muted mt-1 mb-0 text-sm">{{ t('giftReminder.description') }}</p>
      </div>
      <button
        type="button"
        class="app-chip disabled:cursor-not-allowed disabled:opacity-50"
        :aria-busy="testPending"
        :disabled="!methodValid || testPending"
        @click="testReminder"
      >
        <el-icon v-if="testPending" class="is-loading"><Loading /></el-icon>
        <el-icon v-else><VideoPlay /></el-icon>
        {{ t('giftReminder.test') }}
      </button>
      <button
        type="button"
        class="app-btn-primary inline-flex h-8 items-center gap-1.5 px-4 text-sm disabled:cursor-not-allowed disabled:opacity-60"
        :aria-busy="savePending"
        :disabled="!methodValid || savePending"
        @click="save"
      >
        <el-icon v-if="savePending" class="is-loading"><Loading /></el-icon>
        <el-icon v-else><Bell /></el-icon>
        {{ t('common.save') }}
      </button>
    </header>

    <el-alert
      v-if="store.error"
      type="error"
      :closable="false"
      :title="store.error.message"
      show-icon
    />

    <div v-loading="store.pending" class="grid grid-cols-1 gap-4 xl:grid-cols-2">
      <section class="app-card p-5" aria-labelledby="gift-reminder-alert-title">
        <div class="flex flex-wrap items-center gap-2">
          <div class="mr-auto min-w-0">
            <h3 id="gift-reminder-alert-title" class="app-text m-0 text-base font-semibold">
              {{ t('giftReminder.sections.alert') }}
            </h3>
            <p class="app-muted mt-1 mb-0 text-sm">{{ t('giftReminder.thresholdHint') }}</p>
          </div>
          <span class="app-muted shrink-0 text-sm">{{ t('giftReminder.enabled') }}</span>
          <el-switch v-model="draft.isEnabled" :aria-label="t('giftReminder.enabled')" />
        </div>

        <div class="mt-4 flex flex-col gap-3">
          <div class="flex min-h-8 items-center justify-between gap-3 border-t app-hairline pt-3">
            <span class="app-muted shrink-0 text-sm">{{ t('giftReminder.threshold') }}</span>
            <el-input-number
              v-model="draft.highValueThresholdMinor"
              :min="0"
              :step="100"
              class="!w-40"
              :aria-label="t('giftReminder.threshold')"
            />
          </div>
          <div class="flex flex-col gap-2 border-t app-hairline pt-3">
            <span class="app-muted text-sm">{{ t('giftReminder.methods') }}</span>
            <div class="flex flex-wrap items-center gap-x-5 gap-y-2">
              <el-checkbox v-model="draft.methods.systemNotification">
                {{ t('giftReminder.systemNotification') }}
              </el-checkbox>
              <el-checkbox v-model="draft.methods.localPopup">
                {{ t('giftReminder.localPopup') }}
              </el-checkbox>
              <el-checkbox v-model="draft.methods.sound">
                {{ t('giftReminder.sound') }}
              </el-checkbox>
            </div>
            <p
              v-if="!methodValid"
              class="m-0 text-xs"
              style="color: var(--app-red)"
              role="alert"
            >
              {{ t('giftReminder.methodRequired') }}
            </p>
          </div>
        </div>
      </section>

      <section class="app-card p-5" aria-labelledby="gift-reminder-behavior-title">
        <h3 id="gift-reminder-behavior-title" class="app-text m-0 text-base font-semibold">
          {{ t('giftReminder.sections.behavior') }}
        </h3>
        <p class="app-muted mt-1 mb-0 text-sm">{{ t('giftReminder.sections.behaviorHint') }}</p>

        <div class="mt-4 flex flex-col gap-3">
          <div class="flex min-h-8 items-center justify-between gap-3 border-t app-hairline pt-3">
            <span class="app-muted shrink-0 text-sm">{{ t('giftReminder.combo') }}</span>
            <el-switch v-model="draft.comboEnabled" :aria-label="t('giftReminder.combo')" />
          </div>
          <div class="flex min-h-8 items-center justify-between gap-3 border-t app-hairline pt-3">
            <span class="app-muted shrink-0 text-sm">{{ t('giftReminder.comboWindow') }}</span>
            <el-segmented
              v-model="draft.comboWindowSeconds"
              :options="[5, 10, 30]"
              :disabled="!draft.comboEnabled"
              :aria-label="t('giftReminder.comboWindow')"
            />
          </div>
          <div class="flex min-h-8 items-center justify-between gap-3 border-t app-hairline pt-3">
            <span class="app-muted shrink-0 text-sm">{{ t('giftReminder.popupPosition') }}</span>
            <el-select
              v-model="draft.popupPosition"
              class="w-40"
              :aria-label="t('giftReminder.popupPosition')"
            >
              <el-option :label="t('captainEffect.positions.topLeft')" value="topLeft" />
              <el-option :label="t('captainEffect.positions.topRight')" value="topRight" />
              <el-option :label="t('captainEffect.positions.bottomLeft')" value="bottomLeft" />
              <el-option :label="t('captainEffect.positions.bottomRight')" value="bottomRight" />
              <el-option :label="t('captainEffect.positions.center')" value="center" />
            </el-select>
          </div>
          <div class="flex min-h-8 items-center justify-between gap-3 border-t app-hairline pt-3">
            <span class="app-muted shrink-0 text-sm">{{ t('giftReminder.displayDuration') }}</span>
            <el-input-number
              v-model="draft.popupDurationSeconds"
              :min="1"
              :max="30"
              class="!w-40"
              :aria-label="t('giftReminder.displayDuration')"
            />
          </div>
        </div>
      </section>
    </div>

    <div class="grid grid-cols-1 gap-4 sm:grid-cols-3">
      <article class="app-card p-4">
        <p class="app-metric-label m-0">{{ t('dashboard.giftRevenue') }}</p>
        <p class="app-metric-value mt-1.5 mb-0">
          {{ formatMoneyMinor(store.notifySnapshot?.totalValueMinor ?? 0) }}
        </p>
      </article>
      <article class="app-card p-4">
        <p class="app-metric-label m-0">{{ t('giftReminder.donors') }}</p>
        <p class="app-metric-value mt-1.5 mb-0 tabular-nums">
          {{ store.notifySnapshot?.donorCount ?? 0 }}
        </p>
      </article>
      <article class="app-card p-4">
        <p class="app-metric-label m-0">{{ t('giftReminder.giftCount') }}</p>
        <p class="app-metric-value mt-1.5 mb-0 tabular-nums">
          {{ store.notifySnapshot?.giftCount ?? 0 }}
        </p>
      </article>
    </div>

    <section class="app-card p-4" aria-labelledby="gift-reminder-recent-title">
      <div class="flex flex-wrap items-baseline gap-2">
        <h3 id="gift-reminder-recent-title" class="app-text m-0 text-base font-semibold">
          {{ t('giftReminder.recent') }}
        </h3>
        <span class="app-faint text-xs">{{ t('giftReminder.recentHint') }}</span>
      </div>
      <el-table
        v-if="store.notifySnapshot?.latestGifts.length"
        :data="store.notifySnapshot.latestGifts"
        row-key="eventId"
        height="300"
        class="mt-3 w-full"
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
    </section>

    <GiftDetailDrawer v-model="drawerVisible" :gift="selectedGift" />
  </section>
</template>
