<script setup lang="ts">
import { computed, reactive, ref, watch } from 'vue';
import { Delete, Headset, Setting, VideoPause, VideoPlay } from '@element-plus/icons-vue';
import type {
  AiModelProfile,
  AiOutputDeviceId,
  AiVoiceId,
  AiVoiceReplyRules,
  AiVoiceReplySnapshot
} from '@/platform/desktop';

const props = defineProps<{
  snapshot: AiVoiceReplySnapshot | null;
  profiles: readonly AiModelProfile[];
  pending: ReadonlySet<string>;
}>();

const emit = defineEmits<{
  openModels: [];
  setEnabled: [isEnabled: boolean, isAudioRouteConfirmed: boolean];
  confirmRoute: [
    deviceId: AiOutputDeviceId,
    voiceId: AiVoiceId,
    volume: number,
    enableAfterSave: boolean
  ];
  testOutput: [deviceId: AiOutputDeviceId, voiceId: AiVoiceId, volume: number];
  updateRules: [rules: AiVoiceReplyRules];
  pause: [isPaused: boolean];
  skip: [];
  clear: [];
}>();

const routeVisible = ref(false);
const rulesVisible = ref(false);
const route = reactive({
  deviceId: null as AiOutputDeviceId | null,
  voiceId: null as AiVoiceId | null,
  volume: 0.8,
  confirmed: false
});
const rules = reactive({
  triggerScope: 'questions' as AiVoiceReplyRules['triggerScope'],
  keywordsText: '',
  globalCooldownSeconds: 5,
  userCooldownSeconds: 10,
  queueCapacity: 20,
  isDuplicateFilterEnabled: true
});

const defaultProfile = computed(() =>
  props.profiles.find(profile => profile.profileId === props.snapshot?.defaultProfileId) ?? null
);
const selectedDevice = computed(() =>
  props.snapshot?.devices.find(device => device.deviceId === props.snapshot?.queue.selectedDeviceId) ?? null
);
const selectedVoice = computed(() =>
  props.snapshot?.voices.find(voice => voice.voiceId === props.snapshot?.queue.selectedVoiceId) ?? null
);
const canEnable = computed(() => Boolean(
  defaultProfile.value?.isEnabled
  && selectedDevice.value?.isAvailable
  && selectedVoice.value?.isAvailable
));
const isDraftRouteAvailable = computed(() => Boolean(
  route.deviceId
  && props.snapshot?.devices.some(device => device.deviceId === route.deviceId && device.isAvailable)
  && route.voiceId
  && props.snapshot.voices.some(voice => voice.voiceId === route.voiceId && voice.isAvailable)
));
const isPaused = computed(() => props.snapshot?.queue.state === 'paused');

watch(
  () => props.snapshot,
  snapshot => {
    if (!snapshot) return;
    route.deviceId = snapshot.queue.selectedDeviceId
      ?? snapshot.devices.find(device => device.isDefault)?.deviceId
      ?? snapshot.devices[0]?.deviceId
      ?? null;
    route.voiceId = snapshot.queue.selectedVoiceId
      ?? snapshot.voices.find(voice => voice.isDefault)?.voiceId
      ?? snapshot.voices[0]?.voiceId
      ?? null;
    route.volume = snapshot.queue.volume;
    route.confirmed = snapshot.queue.isAudioRouteConfirmed;
    Object.assign(rules, {
      ...snapshot.rules,
      keywordsText: snapshot.rules.keywords.join('\n')
    });
  },
  { immediate: true }
);

function toggleEnabled(value: boolean | string | number): void {
  const enabled = Boolean(value);
  if (!enabled) {
    emit('setEnabled', false, false);
    return;
  }
  if (!canEnable.value || !props.snapshot?.queue.isAudioRouteConfirmed) {
    routeVisible.value = true;
    return;
  }
  emit('setEnabled', true, true);
}

function saveRoute(enableAfterSave: boolean): void {
  if (!route.deviceId || !route.voiceId || !isDraftRouteAvailable.value || !route.confirmed) return;
  emit('confirmRoute', route.deviceId, route.voiceId, route.volume, enableAfterSave);
  routeVisible.value = false;
}

function testRoute(): void {
  if (!route.deviceId || !route.voiceId || !isDraftRouteAvailable.value) return;
  emit('testOutput', route.deviceId, route.voiceId, route.volume);
}

function saveRules(): void {
  emit('updateRules', {
    triggerScope: rules.triggerScope,
    keywords: rules.keywordsText
      .split(/\r?\n/)
      .map(value => value.trim())
      .filter(Boolean),
    globalCooldownSeconds: rules.globalCooldownSeconds,
    userCooldownSeconds: rules.userCooldownSeconds,
    queueCapacity: rules.queueCapacity,
    isDuplicateFilterEnabled: rules.isDuplicateFilterEnabled
  });
  rulesVisible.value = false;
}
</script>

<template>
  <section class="flex flex-col gap-4 border-y border-gray-200 bg-white px-4 py-4 dark:border-gray-700 dark:bg-gray-900">
    <div class="flex flex-wrap items-center gap-x-5 gap-y-3">
      <div class="flex items-center gap-3">
        <span class="font-medium">AI 语音回复</span>
        <el-switch
          :model-value="snapshot?.isEnabled ?? false"
          :loading="pending.has('setVoiceReplyEnabled')"
          @change="toggleEnabled"
        />
      </div>
      <div class="min-w-0 text-sm text-gray-500">
        模型：<span class="text-gray-900 dark:text-gray-100">{{ defaultProfile?.name ?? '未配置' }}</span>
      </div>
      <div class="min-w-0 text-sm text-gray-500">
        输出：<span class="text-gray-900 dark:text-gray-100">{{ selectedDevice?.displayName ?? '未选择' }}</span>
      </div>
      <div class="ml-auto flex flex-wrap gap-2">
        <el-button :icon="Headset" @click="routeVisible = true">
          音频路由
        </el-button>
        <el-button :icon="Setting" @click="rulesVisible = true">
          回复规则
        </el-button>
        <el-button @click="emit('openModels')">
          AI 模型配置
        </el-button>
      </div>
    </div>

    <el-alert
      v-if="!defaultProfile"
      type="warning"
      :closable="false"
      title="请先配置并选择默认模型"
      show-icon
    />
    <el-alert
      v-else-if="!selectedDevice"
      type="warning"
      :closable="false"
      title="请先选择并确认 TTS 输出设备"
      show-icon
    />
  </section>

  <section class="mt-5 border-b border-gray-200 pb-5 dark:border-gray-700">
    <div class="mb-3 flex flex-wrap items-center justify-between gap-2">
      <div>
        <h3 class="m-0 text-base font-semibold">
          AI 语音回复队列
        </h3>
        <span class="text-sm text-gray-500">
          待播 {{ snapshot?.queue.pending.length ?? 0 }} / {{ snapshot?.queue.capacity ?? 20 }}
        </span>
      </div>
      <div class="flex flex-wrap gap-2">
        <el-button
          :icon="isPaused ? VideoPlay : VideoPause"
          :disabled="!snapshot"
          @click="emit('pause', !isPaused)"
        >
          {{ isPaused ? '继续' : '暂停' }}
        </el-button>
        <el-button :disabled="!snapshot?.queue.current" @click="emit('skip')">
          跳过当前
        </el-button>
        <el-button
          :icon="Delete"
          :disabled="!snapshot?.queue.pending.length"
          @click="emit('clear')"
        >
          清空队列
        </el-button>
      </div>
    </div>
    <div class="min-h-10 bg-gray-50 px-3 py-2 text-sm dark:bg-gray-950">
      <template v-if="snapshot?.queue.current">
        <span class="mr-2 text-green-600">正在播报</span>{{ snapshot.queue.current.text }}
      </template>
      <span v-else class="text-gray-500">队列空闲</span>
    </div>
  </section>

  <el-dialog v-model="routeVisible" title="AI 音频路由" width="min(560px, 94vw)">
    <el-form label-position="top">
      <el-form-item label="TTS 输出设备" required>
        <el-select v-model="route.deviceId" class="w-full" @change="route.confirmed = false">
          <el-option
            v-for="device in snapshot?.devices ?? []"
            :key="device.deviceId"
            :label="device.displayName"
            :value="device.deviceId"
            :disabled="!device.isAvailable"
          />
        </el-select>
      </el-form-item>
      <el-form-item label="声色" required>
        <el-select v-model="route.voiceId" class="w-full" @change="route.confirmed = false">
          <el-option
            v-for="voice in snapshot?.voices ?? []"
            :key="voice.voiceId"
            :label="voice.displayName"
            :value="voice.voiceId"
            :disabled="!voice.isAvailable"
          />
        </el-select>
      </el-form-item>
      <el-form-item label="音量">
        <el-slider
          v-model="route.volume"
          :min="0"
          :max="1"
          :step="0.05"
        />
      </el-form-item>
      <el-button
        :loading="pending.has('testOutputDevice')"
        :disabled="!isDraftRouteAvailable"
        @click="testRoute"
      >
        测试播报
      </el-button>
      <el-checkbox v-model="route.confirmed" class="ml-4 mt-4">
        我已确认 OBS/直播姬能采集到测试音
      </el-checkbox>
    </el-form>
    <template #footer>
      <el-button @click="routeVisible = false">
        取消
      </el-button>
      <el-button
        type="primary"
        :disabled="!isDraftRouteAvailable || !route.confirmed"
        @click="saveRoute(!(snapshot?.isEnabled ?? false))"
      >
        确认{{ snapshot?.isEnabled ? '' : '并开启' }}
      </el-button>
    </template>
  </el-dialog>

  <el-dialog v-model="rulesVisible" title="AI 语音回复规则" width="min(620px, 94vw)">
    <el-form label-position="top">
      <el-form-item label="触发范围">
        <el-select v-model="rules.triggerScope" class="w-full">
          <el-option label="全部弹幕" value="all" />
          <el-option label="仅提问" value="questions" />
          <el-option label="仅舰长" value="captains" />
          <el-option label="仅关键词" value="keywords" />
          <el-option label="仅高价值用户" value="highValue" />
        </el-select>
      </el-form-item>
      <el-form-item v-if="rules.triggerScope === 'keywords'" label="关键词（每行一个）">
        <el-input v-model="rules.keywordsText" type="textarea" :rows="3" />
      </el-form-item>
      <div class="grid gap-x-4 sm:grid-cols-3">
        <el-form-item label="全局冷却（秒）">
          <el-input-number
            v-model="rules.globalCooldownSeconds"
            :min="1"
            :max="60"
            class="w-full"
          />
        </el-form-item>
        <el-form-item label="单用户冷却（秒）">
          <el-input-number
            v-model="rules.userCooldownSeconds"
            :min="5"
            :max="300"
            class="w-full"
          />
        </el-form-item>
        <el-form-item label="队列上限">
          <el-input-number
            v-model="rules.queueCapacity"
            :min="5"
            :max="100"
            class="w-full"
          />
        </el-form-item>
      </div>
      <el-form-item label="重复/相似拦截">
        <el-switch v-model="rules.isDuplicateFilterEnabled" />
      </el-form-item>
    </el-form>
    <template #footer>
      <el-button @click="rulesVisible = false">
        取消
      </el-button>
      <el-button type="primary" @click="saveRules">
        保存
      </el-button>
    </template>
  </el-dialog>
</template>
