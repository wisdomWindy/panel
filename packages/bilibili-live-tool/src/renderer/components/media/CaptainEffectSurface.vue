<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import type { CaptainEffectPayload } from '@/platform/desktop';

const props = defineProps<{ payload: CaptainEffectPayload | null }>();
const { t } = useI18n();
const canvas = ref<HTMLCanvasElement | null>(null);
let frame = 0;
let startedAt = 0;
let observer: ResizeObserver | null = null;

type CaptainDecoration =
  | 'classic'
  | 'sparkles'
  | 'stars'
  | 'petals'
  | 'orbit'
  | 'stream'
  | 'flames'
  | 'crystals'
  | 'neon'
  | 'aurora'
  | 'rays'
  | 'crown';

const CAPTAIN_DECORATIONS: Readonly<Record<string, CaptainDecoration>> = {
  'captain-default': 'classic',
  'golden-premium': 'sparkles',
  'starlight-blue': 'stars',
  'cherry-blossom': 'petals',
  'violet-galaxy': 'orbit',
  'emerald-stream': 'stream',
  'crimson-celebration': 'flames',
  'silver-frost': 'crystals',
  'neon-party': 'neon',
  'aurora-dream': 'aurora',
  'sunset-flare': 'rays',
  'midnight-crown': 'crown'
};

const accessibleLabel = computed(() => {
  if (!props.payload) return '';
  return t('captainEffect.accessibleAnnouncement', {
    username: props.payload.audience.username,
    action: t(`captainEffect.actions.${props.payload.action}`),
    level: t(`liveData.tags.${props.payload.level}`)
  });
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

function diamond(context: CanvasRenderingContext2D, x: number, y: number, radius: number): void {
  context.beginPath();
  context.moveTo(x, y - radius);
  context.lineTo(x + radius, y);
  context.lineTo(x, y + radius);
  context.lineTo(x - radius, y);
  context.closePath();
}

function drawDecorations(
  context: CanvasRenderingContext2D,
  decoration: CaptainDecoration,
  x: number,
  y: number,
  width: number,
  height: number,
  primaryColor: string,
  secondaryColor: string,
  progress: number
): void {
  context.save();
  roundedRect(context, x, y, width, height, 8);
  context.clip();
  context.globalAlpha *= 0.72;
  context.lineWidth = 2;

  if (decoration === 'classic') {
    context.fillStyle = primaryColor;
    for (let index = 0; index < 8; index += 1) {
      context.beginPath();
      context.arc(x + 24 + index * 18, y + height - 22, 3, 0, Math.PI * 2);
      context.fill();
    }
  } else if (decoration === 'sparkles' || decoration === 'stars') {
    const count = decoration === 'sparkles' ? 11 : 16;
    context.fillStyle = decoration === 'sparkles' ? secondaryColor : primaryColor;
    for (let index = 0; index < count; index += 1) {
      const angle = progress * Math.PI * 2 + index * 2.4;
      const px = x + 24 + ((index * 67) % Math.max(1, width - 48));
      const py = y + 20 + ((index * 41) % Math.max(1, height - 40));
      const radius = 3 + (index % 3);
      context.save();
      context.translate(px, py);
      context.rotate(angle);
      diamond(context, 0, 0, radius);
      context.fill();
      context.restore();
    }
  } else if (decoration === 'petals') {
    context.fillStyle = secondaryColor;
    for (let index = 0; index < 12; index += 1) {
      const px = x + ((index * 73 + progress * 80) % width);
      const py = y + 16 + ((index * 37 + progress * height) % (height - 32));
      context.beginPath();
      context.ellipse(px, py, 7, 3, index + progress, 0, Math.PI * 2);
      context.fill();
    }
  } else if (decoration === 'orbit') {
    context.strokeStyle = primaryColor;
    context.save();
    context.translate(x + width / 2, y + height / 2);
    context.rotate(progress * Math.PI);
    for (let index = 0; index < 3; index += 1) {
      context.beginPath();
      context.ellipse(
        0,
        0,
        width * (0.22 + index * 0.1),
        24 + index * 14,
        index * 0.55,
        0,
        Math.PI * 2
      );
      context.stroke();
    }
    context.restore();
  } else if (decoration === 'stream' || decoration === 'neon') {
    context.strokeStyle = decoration === 'stream' ? primaryColor : secondaryColor;
    const spacing = decoration === 'stream' ? 34 : 48;
    for (let offset = -height; offset < width + height; offset += spacing) {
      context.beginPath();
      context.moveTo(x + offset + progress * spacing, y + height);
      context.lineTo(x + offset + height + progress * spacing, y);
      context.stroke();
    }
  } else if (decoration === 'flames') {
    const flame = context.createLinearGradient(x, y + height, x, y + height - 64);
    flame.addColorStop(0, secondaryColor);
    flame.addColorStop(1, primaryColor);
    context.fillStyle = flame;
    for (let index = 0; index < 12; index += 1) {
      const px = x + index * (width / 11);
      const peak = 28 + ((index * 17) % 38) + Math.sin(progress * Math.PI * 2 + index) * 7;
      context.beginPath();
      context.moveTo(px - 18, y + height);
      context.quadraticCurveTo(px, y + height - peak, px + 18, y + height);
      context.fill();
    }
  } else if (decoration === 'crystals') {
    context.strokeStyle = primaryColor;
    for (let index = 0; index < 10; index += 1) {
      const px = x + 20 + ((index * 79) % Math.max(1, width - 40));
      const py = y + 22 + ((index * 47) % Math.max(1, height - 44));
      diamond(context, px, py, 7 + (index % 4));
      context.stroke();
    }
  } else if (decoration === 'aurora') {
    context.strokeStyle = secondaryColor;
    context.lineWidth = 9;
    for (let index = 0; index < 3; index += 1) {
      const baseline = y + 38 + index * 54;
      context.beginPath();
      context.moveTo(x - 10, baseline);
      context.bezierCurveTo(
        x + width * 0.28,
        baseline - 48 + progress * 18,
        x + width * 0.68,
        baseline + 48 - progress * 18,
        x + width + 10,
        baseline - 10
      );
      context.stroke();
    }
  } else if (decoration === 'rays') {
    context.strokeStyle = primaryColor;
    context.lineWidth = 5;
    const centerX = x + width / 2;
    const centerY = y + height;
    for (let index = 0; index < 15; index += 1) {
      const angle = Math.PI + (index / 14) * Math.PI;
      context.beginPath();
      context.moveTo(centerX, centerY);
      context.lineTo(
        centerX + Math.cos(angle) * width * 0.62,
        centerY + Math.sin(angle) * width * 0.62
      );
      context.stroke();
    }
  } else {
    const centerX = x + width / 2;
    context.fillStyle = primaryColor;
    context.beginPath();
    context.moveTo(centerX - 58, y + 42);
    context.lineTo(centerX - 44, y + 16);
    context.lineTo(centerX - 18, y + 34);
    context.lineTo(centerX, y + 10);
    context.lineTo(centerX + 18, y + 34);
    context.lineTo(centerX + 44, y + 16);
    context.lineTo(centerX + 58, y + 42);
    context.closePath();
    context.fill();
  }
  context.restore();
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
  const fade = reducedMotion
    ? 1
    : Math.min(1, elapsed / 260) * Math.min(1, (duration - elapsed) / 420);
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

  const decoration = CAPTAIN_DECORATIONS[payload.config.presetId] ?? 'classic';
  drawDecorations(
    context,
    decoration,
    x,
    y,
    bandWidth,
    bandHeight,
    payload.config.primaryColor,
    payload.config.secondaryColor,
    reducedMotion ? 0.5 : elapsed / duration
  );

  context.textAlign = 'center';
  context.fillStyle = payload.config.primaryColor;
  context.font = '600 15px system-ui, sans-serif';
  const level = t(`liveData.tags.${payload.level}`);
  context.fillText(
    `${t(`captainEffect.actions.${payload.action}`)} · ${level}`,
    width / 2,
    y + 58
  );
  context.fillStyle = '#ffffff';
  context.font = '700 30px system-ui, sans-serif';
  context.fillText(payload.audience.username, width / 2, y + 108, bandWidth - 48);
  context.restore();

  if (!reducedMotion) frame = requestAnimationFrame(draw);
}

function restart(): void {
  cancelAnimationFrame(frame);
  startedAt = performance.now();
  frame = requestAnimationFrame(draw);
}

watch(
  () => props.payload,
  () => {
    void nextTick(restart);
  }
);

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
