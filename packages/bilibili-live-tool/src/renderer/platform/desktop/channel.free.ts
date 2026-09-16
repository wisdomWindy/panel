import { readonly, ref } from 'vue';
import type { MemberChannel } from './channel.types';
import type { ShellNavigationItem } from '@/modules/application-shell/navigation';

const unavailable = readonly(ref({
  shellState: 'unavailable',
  account: null,
  entitlement: null,
  capabilities: { ai: 'unavailable', moderation: 'unavailable' },
  serviceErrorCode: null
} as const));

export const memberChannel: MemberChannel = Object.freeze({
  routes: Object.freeze([]),
  toolbarComponent: null,
  navigationMessages: Object.freeze({ 'zh-CN': Object.freeze({}), 'en-US': Object.freeze({}) }),
  deniedRoute: '/',
  buildNavigation: (common: readonly ShellNavigationItem[]) => common,
  useStatus: () => unavailable,
  canAccess: () => true,
  start: () => Promise.resolve(),
  stop: () => undefined
});
