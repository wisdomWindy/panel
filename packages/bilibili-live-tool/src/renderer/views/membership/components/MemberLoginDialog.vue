<script setup lang="ts">
import { computed, nextTick, ref, watch } from 'vue';
import { ArrowLeft, Refresh, Search } from '@element-plus/icons-vue';
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

const manual = ref(false);
const uid = ref('');
const validation = ref<string | null>(null);
const uidInput = ref<{ focus?: () => void } | null>(null);
const canRefresh = computed(() => !props.qrStatus || ['expired', 'error'].includes(props.qrStatus.state));

watch(() => props.visible, visible => {
  if (!visible) return;
  manual.value = props.qrStatus?.state === 'error';
  uid.value = '';
  validation.value = null;
});

function showManual(): void {
  manual.value = true;
  validation.value = null;
  void nextTick(() => uidInput.value?.focus?.());
}

function query(): void {
  const normalized = uid.value.trim();
  if (!/^[0-9]+$/.test(normalized)) {
    validation.value = 'UID 无效或查询失败';
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
    title="登录"
    width="min(440px, calc(100vw - 32px))"
    destroy-on-close
    @close="close"
  >
    <div v-if="!manual" class="flex min-h-72 flex-col items-center justify-center gap-4">
      <img
        v-if="qrStatus && (qrStatus.state === 'pending' || qrStatus.state === 'scanned')"
        :src="`data:image/png;base64,${qrStatus.imageBase64}`"
        alt="B 站登录二维码"
        class="h-48 w-48 border border-gray-200 bg-white p-2"
      >
      <el-empty v-else :image-size="96" description="扫码登录暂不可用" />
      <p v-if="qrStatus?.state === 'pending'" class="m-0 text-sm text-gray-600">
        等待扫码
      </p>
      <p v-else-if="qrStatus?.state === 'scanned'" class="m-0 text-sm text-gray-600">
        等待确认
      </p>
      <p v-else-if="qrStatus?.state === 'expired'" class="m-0 text-sm text-amber-700">
        二维码已失效
      </p>
      <p v-if="qrStatus && 'expiresInSeconds' in qrStatus" class="m-0 text-xs text-gray-500">
        有效期 {{ qrStatus.expiresInSeconds }} 秒
      </p>
      <div class="flex flex-wrap justify-center gap-2">
        <el-button
          v-if="canRefresh"
          :icon="Refresh"
          :loading="pending.has('generateQr')"
          @click="emit('refresh')"
        >
          刷新二维码
        </el-button>
        <el-button type="primary" plain @click="showManual">
          手动输入 UID
        </el-button>
      </div>
    </div>

    <div v-else class="space-y-4">
      <el-button link :icon="ArrowLeft" @click="manual = false">
        返回扫码
      </el-button>
      <el-form label-position="top" @submit.prevent="query">
        <el-form-item label="UID" :error="validation">
          <el-input
            ref="uidInput"
            v-model="uid"
            inputmode="numeric"
            autocomplete="off"
            :prefix-icon="Search"
            @keyup.enter="query"
          />
        </el-form-item>
        <el-button type="primary" :loading="pending.has('queryUid')" @click="query">
          查询
        </el-button>
      </el-form>
      <div v-if="candidate" class="flex items-center gap-3 border-t border-gray-200 pt-4">
        <el-avatar :size="44" :src="candidate.avatarUrl ?? undefined">
          {{ candidate.username.slice(0, 1) }}
        </el-avatar>
        <div class="min-w-0 flex-1">
          <div class="truncate font-medium">
            {{ candidate.username }}
          </div>
          <div class="text-xs text-gray-500">
            UID: {{ candidate.userUid }}
          </div>
        </div>
        <el-button type="primary" :loading="pending.has('bindAccount')" @click="emit('bind', candidate)">
          确认绑定
        </el-button>
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
      <el-button @click="close">
        取消
      </el-button>
    </template>
  </el-dialog>
</template>
