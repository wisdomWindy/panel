<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import { invokeDesktop, listenDesktop, type CaptainEffectPayload } from '@/platform/desktop';
import {
  adaptCaptainEffectPayload,
  adaptCaptainEffectPayloadOrNull
} from '@/modules/media/adapters';
import CaptainEffectSurface from '@/components/media/CaptainEffectSurface.vue';

const payload = ref<CaptainEffectPayload | null>(null);
let dispose: (() => void) | null = null;
let active = true;
let eventRevision = 0;

async function registerTriggerListener(): Promise<void> {
  try {
    const unlisten = await listenDesktop('captain_effect_trigger', next => {
      try {
        const adapted = adaptCaptainEffectPayload(next);
        eventRevision += 1;
        if (active) payload.value = adapted;
      } catch {
        // Ignore malformed events at the desktop boundary.
      }
    });
    if (!active) {
      unlisten();
      return;
    }
    dispose = unlisten;
  } catch {
    // The current payload query still provides the preview's first frame.
  }
}

async function loadCurrentEffect(): Promise<void> {
  if (!active) return;
  const revisionBeforeQuery = eventRevision;
  try {
    const current = adaptCaptainEffectPayloadOrNull(
      await invokeDesktop('captain_effect_current', undefined)
    );
    if (active && eventRevision === revisionBeforeQuery && current) {
      payload.value = current;
    }
  } catch {
    if (active && eventRevision === 0) payload.value = null;
  }
}

onMounted(async () => {
  await registerTriggerListener();
  await loadCurrentEffect();
});

onUnmounted(() => {
  active = false;
  dispose?.();
  dispose = null;
});
</script>

<template>
  <main class="h-screen w-screen overflow-hidden bg-transparent">
    <CaptainEffectSurface :payload="payload" />
  </main>
</template>
