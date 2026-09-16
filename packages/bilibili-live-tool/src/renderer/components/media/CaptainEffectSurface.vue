<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from 'vue';
import type { CaptainEffectPayload } from '@/platform/desktop';

const props = defineProps<{ payload: CaptainEffectPayload | null }>();
const canvas = ref<HTMLCanvasElement | null>(null);
let frame = 0;
let startedAt = 0;
let observer: ResizeObserver | null = null;

const accessibleLabel = computed(() => {
  if (!props.payload) return '';
  const level = { governor: '总督', admiral: '提督', captain: '舰长' }[props.payload.level];
  const action = props.payload.action === 'purchase' ? '首次开通' : '续费';
  return `${props.payload.audience.username} ${action}${level}`;
});

function fitCanvas(target: HTMLCanvasElement): CanvasRenderingContext2D | null {
  const bounds = target.getBoundingClientRect();
  const ratio = Math.max(1, window.devicePixelRatio || 1);
  const width = Math.max(1, Math.round(bounds.width * ratio));
  const height = Math.max(1, Math.round(bounds.height * ratio));
  if (target.width !== width || target.height !== height) {
    target.width = width;
    target.height = height;
  }
  const context = target.getContext('2d');
  context?.setTransform(ratio, 0, 0, ratio, 0, 0);
  return context;
}

function roundedRect(
  context: CanvasRenderingContext2D,
  x: number,
  y: number,
  width: number,
  height: number,
  radius: number
): void {
  context.beginPath();
  context.roundRect(x, y, width, height, radius);
}

function draw(timestamp: number): void {
  const target = canvas.value;
  if (!target) return;
  const context = fitCanvas(target);
  if (!context) return;
  const width = target.clientWidth;
  const height = target.clientHeight;
  context.clearRect(0, 0, width, height);
  const payload = props.payload;
  if (!payload) return;

  const duration = payload.config.durationSeconds * 1000;
  const elapsed = Math.max(0, timestamp - startedAt);
  const reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  const fade = reducedMotion ? 1 : Math.min(1, elapsed / 260) * Math.min(1, (duration - elapsed) / 420);
  if (elapsed >= duration) return;

  context.save();
  context.globalAlpha = Math.max(0, fade);
  const bandWidth = Math.max(1, Math.min(720, width - 32));
  const bandHeight = Math.min(220, Math.max(156, height * 0.58));
  const x = (width - bandWidth) / 2;
  const y = (height - bandHeight) / 2;
  const gradient = context.createLinearGradient(x, y, x + bandWidth, y + bandHeight);
  gradient.addColorStop(0, '#111827');
  gradient.addColorStop(0.55, '#18181b');
  gradient.addColorStop(1, '#09090b');
  roundedRect(context, x, y, bandWidth, bandHeight, 8);
  context.fillStyle = gradient;
  context.fill();
  context.lineWidth = 2;
  context.strokeStyle = payload.config.primaryColor;
  context.stroke();

  const accent = context.createLinearGradient(x, y, x + bandWidth, y);
  accent.addColorStop(0, payload.config.primaryColor);
  accent.addColorStop(1, payload.config.secondaryColor);
  context.fillStyle = accent;
  context.fillRect(x, y, bandWidth, 5);

  context.textAlign = 'center';
  context.fillStyle = payload.config.primaryColor;
  context.font = '600 15px system-ui, sans-serif';
  const level = { governor: '总督', admiral: '提督', captain: '舰长' }[payload.level];
  context.fillText(`${payload.action === 'purchase' ? '首次开通' : '续费'} · ${level}`, width / 2, y + 58);
  context.fillStyle = '#ffffff';
  context.font = '700 30px system-ui, sans-serif';
  context.fillText(payload.audience.username, width / 2, y + 108, bandWidth - 48);
  context.fillStyle = '#d1d5db';
  context.font = '400 14px system-ui, sans-serif';
  context.fillText('欢迎加入直播间', width / 2, y + 144);
  context.restore();

  if (!reducedMotion) frame = requestAnimationFrame(draw);
}

function restart(): void {
  cancelAnimationFrame(frame);
  startedAt = performance.now();
  frame = requestAnimationFrame(draw);
}

watch(() => props.payload, () => {
  void nextTick(restart);
});

onMounted(() => {
  if (canvas.value && 'ResizeObserver' in window) {
    observer = new ResizeObserver(restart);
    observer.observe(canvas.value);
  }
  restart();
});

onUnmounted(() => {
  cancelAnimationFrame(frame);
  observer?.disconnect();
});
</script>

<template>
  <div class="relative h-full min-h-44 w-full overflow-hidden bg-transparent">
    <canvas ref="canvas" class="block h-full w-full" aria-hidden="true" />
    <p class="sr-only" aria-live="polite">
      {{ accessibleLabel }}
    </p>
  </div>
</template>
