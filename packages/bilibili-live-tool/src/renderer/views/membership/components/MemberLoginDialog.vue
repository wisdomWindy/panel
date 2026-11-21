<script setup lang="ts">
import { computed, nextTick, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { ArrowLeft, Loading, Refresh, Search } from '@element-plus/icons-vue';
import type { AuthQrStatus, MemberAccount } from '@/platform/desktop';

const props = defineProps<{
  visible: boolean;
  qrStatus: AuthQrStatus | null;
  candidate: MemberAccount | null;
  pending: ReadonlySet<string>;
  errorMessage: string | null;
}>();

const emit = defineEmits<{
  'update:visible': [value: boolean];
  refresh: [];
  cancel: [];
  query: [uid: string];
  bind: [account: MemberAccount];
}>();

const { t } = useI18n();
const manual = ref(false);
const uid = ref('');
const validation = ref<string | null>(null);
const uidInput = ref<{ focus?: () => void } | null>(null);
const canRefresh = computed(
  () => !props.qrStatus || ['expired', 'error'].includes(props.qrStatus.state)
);

watch(
  () => props.visible,
  visible => {
    if (!visible) return;
    manual.value = props.qrStatus?.state === 'error';
    uid.value = '';
    validation.value = null;
  }
);

function showManual(): void {
  manual.value = true;
  validation.value = null;
  void nextTick(() => uidInput.value?.focus?.());
}

function query(): void {
  const normalized = uid.value.trim();
  if (!/^[0-9]+$/.test(normalized)) {
    validation.value = t('membership.qr.uidInvalid');
    return;
  }
  validation.value = null;
  emit('query', normalized);
}

function close(): void {
  emit('cancel');
  emit('update:visible', false);
}
</script>

<template>
  <el-dialog
    :model-value="visible"
    :title="t('membership.login')"
    width="min(440px, calc(100vw - 32px))"
    destroy-on-close
    @close="close"
  >
    <div v-if="!manual" class="flex min-h-72 flex-col items-center justify-center gap-4">
      <img
        v-if="qrStatus && (qrStatus.state === 'pending' || qrStatus.state === 'scanned')"
        :src="`data:image/png;base64,${qrStatus.imageBase64}`"
        :alt="t('membership.qr.alt')"
        class="h-48 w-48 rounded-md border app-hairline p-1"
      >
      <el-empty v-else :image-size="96" :description="t('membership.qr.unavailable')" />
      <p v-if="qrStatus?.state === 'pending'" class="app-muted m-0 text-sm">
        {{ t('membership.qr.waiting') }}
      </p>
      <p v-else-if="qrStatus?.state === 'scanned'" class="app-muted m-0 text-sm">
        {{ t('membership.qr.scanned') }}
      </p>
      <p v-else-if="qrStatus?.state === 'expired'" class="m-0 text-sm" style="color: var(--app-gold)">
        {{ t('membership.qr.expired') }}
      </p>
      <p
        v-if="qrStatus && 'expiresInSeconds' in qrStatus"
        class="app-faint m-0 text-xs tabular-nums"
      >
        {{ t('membership.qr.expiresIn', { count: qrStatus.expiresInSeconds }) }}
      </p>
      <div class="flex flex-wrap justify-center gap-2">
        <button
          v-if="canRefresh"
          type="button"
          class="app-chip disabled:cursor-not-allowed disabled:opacity-50"
          :disabled="pending.has('generateQr')"
          :aria-busy="pending.has('generateQr')"
          @click="emit('refresh')"
        >
          <el-icon v-if="pending.has('generateQr')" class="is-loading">
            <Loading />
          </el-icon>
          <el-icon v-else><Refresh /></el-icon>
          {{ t('membership.qr.refresh') }}
        </button>
        <button
          type="button"
          class="app-btn-primary inline-flex h-8 items-center gap-1.5 px-3.5 text-sm"
          @click="showManual"
        >
          {{ t('membership.qr.manual') }}
        </button>
      </div>
    </div>

    <div v-else class="flex flex-col gap-4">
      <button type="button" class="app-chip self-start" @click="manual = false">
        <el-icon><ArrowLeft /></el-icon>
        {{ t('membership.qr.back') }}
      </button>
      <el-form label-position="top" @submit.prevent="query">
        <el-form-item :label="t('membership.uid')" :error="validation">
          <el-input
            ref="uidInput"
            v-model="uid"
            inputmode="numeric"
            autocomplete="off"
            :prefix-icon="Search"
            @keyup.enter="query"
          />
        </el-form-item>
        <button
          type="button"
          class="app-btn-primary inline-flex h-8 items-center gap-1.5 px-3.5 text-sm disabled:cursor-not-allowed disabled:opacity-60"
          :disabled="pending.has('queryUid')"
          :aria-busy="pending.has('queryUid')"
          @click="query"
        >
          <el-icon v-if="pending.has('queryUid')" class="is-loading"><Loading /></el-icon>
          {{ t('common.search') }}
        </button>
      </el-form>
      <div v-if="candidate" class="app-card flex items-center gap-3 p-3">
        <el-avatar :size="44" :src="candidate.avatarUrl ?? undefined">
          {{ candidate.username.slice(0, 1) }}
        </el-avatar>
        <div class="min-w-0 flex-1">
          <div class="app-text truncate font-medium">
            {{ candidate.username }}
          </div>
          <div class="app-faint text-xs tabular-nums">
            {{ t('membership.uid') }}: {{ candidate.userUid }}
          </div>
        </div>
        <button
          type="button"
          class="app-btn-primary inline-flex h-8 items-center gap-1.5 px-3.5 text-sm disabled:cursor-not-allowed disabled:opacity-60"
          :disabled="pending.has('bindAccount')"
          :aria-busy="pending.has('bindAccount')"
          @click="emit('bind', candidate)"
        >
          <el-icon v-if="pending.has('bindAccount')" class="is-loading"><Loading /></el-icon>
          {{ t('membership.qr.bind') }}
        </button>
      </div>
      <el-alert
        v-if="errorMessage"
        type="error"
        :closable="false"
        :title="errorMessage"
        show-icon
      />
    </div>

    <template #footer>
      <div class="flex items-center justify-end gap-2">
        <button type="button" class="app-chip h-8 px-4" @click="close">
          {{ t('common.cancel') }}
        </button>
      </div>
    </template>
  </el-dialog>
</template>
