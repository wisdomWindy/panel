import { readonly, ref } from 'vue';
import type { MemberChannel } from './channel.types';
import type { ShellNavigationItem } from '@/modules/application-shell/navigation';
import { aiNavigationMessages, aiRoutes } from './channel.shared';

const unavailable = readonly(ref({
  shellState: 'unavailable',
  account: null,
  entitlement: null,
  capabilities: { moderation: 'unavailable' },
  serviceErrorCode: null
} as const));

export const memberChannel: MemberChannel = Object.freeze({
  routes: aiRoutes,
  toolbarComponent: null,
  navigationMessages: aiNavigationMessages,
  deniedRoute: '/',
  buildNavigation: (common: readonly ShellNavigationItem[]) => common,
  useStatus: () => unavailable,
  canAccess: () => true,
  start: () => Promise.resolve(),
  stop: () => undefined
});
