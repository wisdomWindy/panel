<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick, reactive } from 'vue';
import { createEffectManager, EffectManagerImpl } from '../../modules/effects/effect-manager';
import { EffectType, AnimationType, EffectConfig, EffectTriggerData } from '../../modules/effects/types';
import { WebGLRenderer } from '../../modules/effects/webgl-renderer';

const canvas2dRef = ref<HTMLCanvasElement>();
const canvas3dRef = ref<HTMLCanvasElement>();
const isReady = ref(false);
const initError = ref('');

let effectManager: EffectManagerImpl | null = null;
let webglRenderer: WebGLRenderer | null = null;

const form = reactive({
  effectType: EffectType.CAPTAIN_FIRST,
  textTemplate: '感谢 {username} 首次开通 {level}！🎉',
  animationType: AnimationType.SCALE,
  primaryColor: '#FFD700',
  secondaryColor: '#FFA500',
  duration: 5000,
  testUsername: '测试用户',
  testCaptainLevel: 3,
  testGiftName: '舰长',
  testComboCount: 5,
  renderMode: 'canvas' as 'canvas' | 'webgl'
});

const effectTypeOptions = [
  { label: '首次上舰', value: EffectType.CAPTAIN_FIRST },
  { label: '续费上舰', value: EffectType.CAPTAIN_RENEW },
  { label: '礼物连击', value: EffectType.GIFT_COMBO }
];

const animationTypeOptions = [
  { label: '淡入淡出', value: AnimationType.FADE },
  { label: '滑动', value: AnimationType.SLIDE },
  { label: '缩放', value: AnimationType.SCALE },
  { label: '旋转', value: AnimationType.ROTATE },
  { label: '粒子', value: AnimationType.PARTICLE }
];

function handleEffectTypeChange() {
  switch (form.effectType) {
    case EffectType.CAPTAIN_FIRST:
      form.textTemplate = '感谢 {username} 首次开通 {level}！🎉';
      break;
    case EffectType.CAPTAIN_RENEW:
      form.textTemplate = '感谢 {username} 续费 {level}！🎊';
      break;
    case EffectType.GIFT_COMBO:
      form.textTemplate = '{username} 连击 {count} 次 {giftName}！💥';
      break;
  }
}

function buildEffectConfig(): EffectConfig {
  return {
    id: `preview_${Date.now()}`,
    type: form.effectType,
    templateName: '预览特效',
    textTemplate: form.textTemplate,
    animationType: form.animationType,
    primaryColor: form.primaryColor,
    secondaryColor: form.secondaryColor,
    duration: form.duration,
    enabled: true,
    renderMode: form.renderMode,
    createdAt: new Date(),
    updatedAt: new Date()
  };
}

function buildTriggerData(): EffectTriggerData {
  const triggerData: EffectTriggerData = {
    type: form.effectType,
    timestamp: Date.now()
  };

  if (form.effectType === EffectType.CAPTAIN_FIRST || form.effectType === EffectType.CAPTAIN_RENEW) {
    triggerData.captainInfo = {
      userId: 'test_001',
      username: form.testUsername,
      level: form.testCaptainLevel,
      giftName: form.testGiftName,
      giftId: '1',
      count: 1,
      timestamp: Date.now(),
      isFirstTime: form.effectType === EffectType.CAPTAIN_FIRST,
      isRenewal: form.effectType === EffectType.CAPTAIN_RENEW
    };
  } else {
    triggerData.comboInfo = {
      userId: 'test_001',
      username: form.testUsername,
      giftId: '1',
      giftName: form.testGiftName,
      count: form.testComboCount,
      comboId: 'test_combo',
      timestamp: Date.now()
    };
  }

  return triggerData;
}

async function handlePreview() {
  const config = buildEffectConfig();
  const triggerData = buildTriggerData();

  if (form.renderMode === 'webgl') {
    if (!webglRenderer) {
      ElMessage.warning('3D 渲染器未就绪');
      return;
    }
    webglRenderer.addEffect(triggerData, config);
    ElMessage.success('3D 特效已触发预览');
  } else {
    if (!effectManager) {
      ElMessage.warning('2D 渲染器未就绪');
      return;
    }
    effectManager.addEffectConfig(config);
    effectManager.addEffect(triggerData);
    ElMessage.success('2D 特效已触发预览');
  }
}

function handleClear() {
  effectManager?.clearAll();
  webglRenderer?.clear();
  ElMessage.info('已清除所有特效');
}

function initWebGL() {
  if (!canvas3dRef.value) return;
  try {
    webglRenderer = new WebGLRenderer(canvas3dRef.value);
    // 覆盖全屏样式，让 3D Canvas 留在预览容器内
    canvas3dRef.value.style.position = 'relative';
    canvas3dRef.value.style.top = '';
    canvas3dRef.value.style.left = '';
    canvas3dRef.value.style.pointerEvents = 'auto';
    canvas3dRef.value.style.zIndex = '0';
  } catch (err) {
    console.warn('WebGL 初始化失败，3D 特效将不可用:', err);
  }
}

onMounted(async () => {
  await nextTick();
  initWebGL();

  if (!canvas2dRef.value) {
    initError.value = 'Canvas 元素未找到';
    return;
  }

  try {
    effectManager = createEffectManager(canvas2dRef.value);
    await effectManager.initialize();
    isReady.value = true;
  } catch (err) {
    initError.value = `特效管理器初始化失败: ${err}`;
  }
});

onUnmounted(() => {
  webglRenderer?.dispose();
});
</script>

<template>
  <div class="p-8">
    <el-row :gutter="20">
      <el-col :span="8">
        <el-card>
          <template #header>
            <span>特效参数配置</span>
          </template>

          <el-form :model="form" label-width="100px">
            <el-form-item label="特效类型">
              <el-select v-model="form.effectType" class="w-full" @change="handleEffectTypeChange">
                <el-option
                  v-for="opt in effectTypeOptions"
                  :key="opt.value"
                  :label="opt.label"
                  :value="opt.value"
                />
              </el-select>
            </el-form-item>

            <el-form-item label="文字模板">
              <el-input v-model="form.textTemplate" type="textarea" :rows="3" />
            </el-form-item>

            <el-row :gutter="10">
              <el-col :span="12">
                <el-form-item label="主色">
                  <el-color-picker v-model="form.primaryColor" />
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item label="副色">
                  <el-color-picker v-model="form.secondaryColor" />
                </el-form-item>
              </el-col>
            </el-row>

            <el-form-item label="渲染模式">
              <el-radio-group v-model="form.renderMode">
                <el-radio-button label="canvas">
                  2D Canvas
                </el-radio-button>
                <el-radio-button label="webgl">
                  3D WebGL
                </el-radio-button>
              </el-radio-group>
            </el-form-item>

            <el-form-item label="持续时间">
              <el-input-number
                v-model="form.duration"
                :min="1000"
                :max="30000"
                :step="1000"
              />
              <span class="ml-2">毫秒</span>
            </el-form-item>

            <el-divider v-if="form.renderMode === 'canvas'" content-position="left">
              2D 动画设置
            </el-divider>
            <el-divider v-else content-position="left">
              3D 模型动画
            </el-divider>

            <el-form-item v-if="form.renderMode === 'canvas'" label="动画类型">
              <el-select v-model="form.animationType" class="w-full">
                <el-option
                  v-for="opt in animationTypeOptions"
                  :key="opt.value"
                  :label="opt.label"
                  :value="opt.value"
                />
              </el-select>
            </el-form-item>

            <el-divider content-position="left">
              测试数据
            </el-divider>

            <el-form-item label="用户名">
              <el-input v-model="form.testUsername" placeholder="虚拟用户名" />
            </el-form-item>

            <el-form-item
              v-if="form.effectType === 'captain_first' || form.effectType === 'captain_renew'"
              label="舰长等级"
            >
              <el-select v-model="form.testCaptainLevel" class="w-full">
                <el-option label="督察 (Lv1)" :value="1" />
                <el-option label="提督 (Lv2)" :value="2" />
                <el-option label="总督 (Lv3)" :value="3" />
              </el-select>
            </el-form-item>

            <el-form-item v-if="form.effectType === 'gift_combo'" label="连击次数">
              <el-input-number v-model="form.testComboCount" :min="1" :max="999" />
            </el-form-item>

            <el-form-item label="礼物名">
              <el-input v-model="form.testGiftName" placeholder="虚拟礼物名" />
            </el-form-item>

            <div class="flex gap-2">
              <el-button type="primary" :disabled="!isReady" @click="handlePreview">
                预览特效
              </el-button>
              <el-button :disabled="!isReady" @click="handleClear">
                清除
              </el-button>
            </div>
          </el-form>
        </el-card>
      </el-col>

      <el-col :span="16">
        <el-card>
          <template #header>
            <span>特效预览区</span>
          </template>

          <div
            v-if="initError"
            class="flex items-center justify-center h-[600px] bg-gray-100 rounded-lg"
          >
            <el-result icon="error" :title="'初始化失败'" :sub-title="initError" />
          </div>

          <div
            v-else-if="!isReady"
            class="flex items-center justify-center h-[600px] bg-gray-100 rounded-lg"
          >
            <el-icon class="text-4xl text-gray-400 animate-spin">
              <svg viewBox="0 0 1024 1024" width="1em" height="1em">
                <path d="M512 64a448 448 0 1 0 448 448A448 448 0 0 0 512 64zm0 832a384 384 0 1 1 384-384 384 384 0 0 1-384 384z" fill="currentColor" />
              </svg>
            </el-icon>
            <span class="ml-3 text-gray-500">正在初始化渲染器...</span>
          </div>

          <canvas
            v-show="isReady && form.renderMode === 'canvas'"
            ref="canvas2dRef"
            class="w-full h-[600px] bg-gray-900 rounded-lg border border-gray-700 overflow-hidden"
          />
          <canvas
            v-show="isReady && form.renderMode === 'webgl'"
            ref="canvas3dRef"
            class="w-full h-[600px] bg-gray-900 rounded-lg border border-gray-700 overflow-hidden"
          />
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>
