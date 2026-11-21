<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { ModerationAlert } from '@/platform/desktop';

defineProps<{
  alerts: readonly ModerationAlert[];
  canMute: boolean;
  isExpired: boolean;
  pending: ReadonlySet<string>;
}>();

defineEmits<{
  view: [alert: ModerationAlert];
  mute: [alert: ModerationAlert];
  ignore: [alert: ModerationAlert];
  blacklist: [alert: ModerationAlert];
}>();

const { t } = useI18n();

function dateTime(value: number): string {
  return new Intl.DateTimeFormat(undefined, { dateStyle: 'short', timeStyle: 'medium' }).format(
    value
  );
}

function statusBadge(status: ModerationAlert['status']): string {
  return status === 'pending' || status === 'executing' ? 'app-badge--captain' : 'app-badge--plain';
}

function statusStyle(status: ModerationAlert['status']): Record<string, string> | undefined {
  if (status === 'muted')
    return {
      background: 'color-mix(in srgb, var(--app-green) 20%, transparent)',
      color: 'var(--app-green)'
    };
  if (status === 'failed')
    return {
      background: 'color-mix(in srgb, var(--app-red) 20%, transparent)',
      color: 'var(--app-red)'
    };
  return undefined;
}
</script>

<template>
  <div>
    <article
      v-for="alert in alerts"
      :key="alert.alertId"
      class="app-hairline grid gap-3 border-b px-4 py-3 last:border-b-0 md:grid-cols-[minmax(0,1fr)_auto]"
    >
      <div class="min-w-0">
        <div class="flex flex-wrap items-center gap-x-3 gap-y-1">
          <strong class="app-text break-words">{{ alert.username }}</strong>
          <span class="app-faint text-xs tabular-nums">UID: {{ alert.userUid }}</span>
          <span class="app-badge" :class="statusBadge(alert.status)" :style="statusStyle(alert.status)">
            {{ t(`moderation.status.${alert.status}`) }}
          </span>
          <span class="app-faint text-xs">{{ dateTime(alert.occurredAt) }}</span>
        </div>
        <p class="app-muted mb-1 mt-2 text-sm">
          {{ alert.violationDescription }}
        </p>
        <p class="app-faint m-0 break-words text-sm">
          {{ t('moderation.contentExample', { content: alert.contentExample }) }}
        </p>
        <p v-if="alert.errorCode" class="m-0 mt-1 text-sm" style="color: var(--app-red)">
          {{ alert.errorCode }}
        </p>
      </div>
      <div class="flex flex-wrap items-center gap-2 md:justify-end">
        <button
          type="button"
          class="app-chip h-7 px-2.5 text-xs"
          @click="$emit('view', alert)"
        >
          {{ t('moderation.view') }}
        </button>
        <button
          v-if="!isExpired"
          type="button"
          class="app-chip h-7 px-2.5 text-xs disabled:cursor-not-allowed disabled:opacity-50"
          style="color: var(--app-red); border-color: color-mix(in srgb, var(--app-red) 45%, transparent)"
          :disabled="!canMute || pending.has(`mute:${alert.alertId}`)"
          :aria-label="t('moderation.mute')"
          :aria-busy="pending.has(`mute:${alert.alertId}`)"
          :title="canMute ? '' : t('moderation.credentialUnavailable')"
          @click="$emit('mute', alert)"
        >
          {{ t('moderation.mute') }}
        </button>
        <button
          v-else
          type="button"
          class="app-chip h-7 px-2.5 text-xs"
          @click="$emit('blacklist', alert)"
        >
          {{ t('moderation.blacklist') }}
        </button>
        <button
          type="button"
          class="app-chip h-7 px-2.5 text-xs disabled:cursor-not-allowed disabled:opacity-50"
          :disabled="pending.has(`ignore:${alert.alertId}`)"
          :aria-busy="pending.has(`ignore:${alert.alertId}`)"
          @click="$emit('ignore', alert)"
        >
          {{ t('moderation.ignore') }}
        </button>
      </div>
    </article>
    <p v-if="alerts.length === 0" class="app-faint m-0 px-4 py-8 text-center text-sm">
      {{ t('common.noData') }}
    </p>
  </div>
</template>
