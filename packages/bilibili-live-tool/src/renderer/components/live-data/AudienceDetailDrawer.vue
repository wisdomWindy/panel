<script setup lang="ts">
import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import {
  invokeDesktop,
  type AudienceDetail,
  type RoomId,
  type UserUid
} from '@/platform/desktop';
import { formatCount, formatMoneyMinor } from '@/modules/live-data/formatters';
import { useLiveSafetyStore } from '@/stores/live-data/safety';
import AudienceIdentityView from './AudienceIdentityView.vue';

const props = defineProps<{ visible: boolean; roomId: RoomId | null; userUid: UserUid | null }>();
const emit = defineEmits<{ close: []; blacklisted: [uid: UserUid] }>();
const safety = useLiveSafetyStore();
const { t } = useI18n();
const detail = ref<AudienceDetail | null>(null);
const loading = ref(false);
const error = ref<string | null>(null);

watch(
  () => [props.visible, props.roomId, props.userUid] as const,
  async ([visible, roomId, userUid]) => {
    detail.value = null;
    error.value = null;
    if (!visible || !roomId || !userUid) return;
    loading.value = true;
    try {
      detail.value = await invokeDesktop('audience_detail_query', { roomId, userUid });
    } catch (value) {
      error.value = (value as { message?: string }).message ?? t('common.operationFailed');
    } finally {
      loading.value = false;
    }
  },
  { immediate: true }
);

async function addToBlacklist(): Promise<void> {
  if (!detail.value) return;
  await safety.addBlacklist({
    uid: detail.value.identity.userUid,
    username: detail.value.identity.username,
    reason: null
  });
  emit('blacklisted', detail.value.identity.userUid);
  emit('close');
}
</script>

<template>
  <el-drawer
    :model-value="visible"
    :title="t('liveData.audienceDetails')"
    size="min(420px, 92vw)"
    @close="$emit('close')"
  >
    <el-skeleton v-if="loading" :rows="5" animated />
    <el-alert
      v-else-if="error"
      :title="error"
      type="error"
      :closable="false"
    />
    <div v-else-if="detail" class="space-y-5">
      <AudienceIdentityView :identity="detail.identity" />
      <dl class="grid grid-cols-[120px_1fr] gap-y-3 border-y border-gray-200 py-4 text-sm">
        <dt class="text-gray-500">
          UID
        </dt>
        <dd class="m-0 text-gray-900">
          {{ detail.identity.userUid }}
        </dd>
        <dt class="text-gray-500">
          {{ t('liveData.sessionDanmaku') }}
        </dt>
        <dd class="m-0 tabular-nums text-gray-900">
          {{ formatCount(detail.sessionDanmuCount) }}
        </dd>
        <dt class="text-gray-500">
          {{ t('liveData.giftTotal') }}
        </dt>
        <dd class="m-0 tabular-nums text-gray-900">
          {{ formatMoneyMinor(detail.giftValueMinor) }}
        </dd>
        <dt class="text-gray-500">
          {{ t('liveData.blacklistStatus') }}
        </dt>
        <dd class="m-0 text-gray-900">
          {{ detail.isBlacklisted ? t('liveData.blacklisted') : t('liveData.notBlacklisted') }}
        </dd>
      </dl>
      <el-button
        v-if="!detail.isBlacklisted"
        type="danger"
        :loading="safety.mutationPending"
        @click="addToBlacklist"
      >
        {{ t('liveData.addToBlacklist') }}
      </el-button>
    </div>
  </el-drawer>
</template>
