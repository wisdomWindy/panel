<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { AudienceIdentity } from '@/platform/desktop';

defineProps<{ identity: AudienceIdentity; interactive?: boolean }>();
defineEmits<{ select: [] }>();

const { t } = useI18n();

/** PRD 9.2 identity palette, shared with the shell tokens. */
const TONES: Readonly<Record<string, string>> = {
  admin: 'admin',
  governor: 'governor',
  admiral: 'admiral',
  captain: 'captain',
  vip: 'vip'
};
</script>

<template>
  <div class="flex min-w-0 items-center gap-2">
    <div class="flex shrink-0 items-center gap-1">
      <span
        v-for="tag in identity.tags"
        :key="tag"
        class="app-badge"
        :class="`app-badge--${TONES[tag] ?? 'plain'}`"
      >
        {{ t(`liveData.tags.${tag}`) }}
      </span>
    </div>
    <button
      v-if="interactive"
      type="button"
      class="app-text min-w-0 truncate font-medium hover:underline focus-visible:outline-2 focus-visible:outline-[var(--app-accent)]"
      @click="$emit('select')"
    >
      {{ identity.username }}
    </button>
    <span v-else class="app-text min-w-0 truncate font-medium">{{ identity.username }}</span>
    <span v-if="identity.userLevel !== null" class="app-faint shrink-0 text-xs">
      Lv.{{ identity.userLevel }}
    </span>
    <el-tooltip v-if="identity.medal" :content="identity.medal.name" placement="top">
      <span class="app-badge shrink-0 app-badge--vip">
        {{ identity.medal.name }}
        <template v-if="identity.medal.level !== null">{{ identity.medal.level }}</template>
      </span>
    </el-tooltip>
  </div>
</template>
