<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { AudienceIdentity } from '@/platform/desktop';

defineProps<{ identity: AudienceIdentity; interactive?: boolean }>();
defineEmits<{ select: [] }>();

const { t } = useI18n();
</script>

<template>
  <div class="flex min-w-0 items-center gap-2">
    <div class="flex shrink-0 items-center gap-1">
      <el-tag
        v-for="tag in identity.tags"
        :key="tag"
        size="small"
        effect="plain"
        :type="
          tag === 'admin'
            ? 'success'
            : tag === 'admiral'
              ? 'primary'
              : tag === 'vip'
                ? 'warning'
                : 'danger'
        "
      >
        {{ t(`liveData.tags.${tag}`) }}
      </el-tag>
    </div>
    <button
      v-if="interactive"
      type="button"
      class="min-w-0 truncate font-medium text-pink-600 hover:underline focus-visible:outline-2 active:text-pink-700"
      @click="$emit('select')"
    >
      {{ identity.username }}
    </button>
    <span v-else class="min-w-0 truncate font-medium text-gray-900">{{ identity.username }}</span>
    <span v-if="identity.userLevel !== null" class="shrink-0 text-xs text-gray-500">
      Lv.{{ identity.userLevel }}
    </span>
    <el-tooltip v-if="identity.medal" :content="identity.medal.name" placement="top">
      <span
        class="shrink-0 border border-amber-300 bg-amber-50 px-1.5 py-0.5 text-xs text-amber-800"
      >
        {{ identity.medal.name }}
        <template v-if="identity.medal.level !== null">{{ identity.medal.level }}</template>
      </span>
    </el-tooltip>
  </div>
</template>
