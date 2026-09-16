<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import { listenDesktop, type CaptainEffectPayload } from '@/platform/desktop';
import CaptainEffectSurface from '@/components/media/CaptainEffectSurface.vue';

const payload = ref<CaptainEffectPayload | null>(null);
let dispose: (() => void) | null = null;
let active = true;

onMounted(async () => {
  try {
    const unlisten = await listenDesktop('captain_effect_trigger', next => {
      if (active) payload.value = next;
    });
    if (active) dispose = unlisten;
    else unlisten();
  } catch {
    payload.value = null;
  }
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
