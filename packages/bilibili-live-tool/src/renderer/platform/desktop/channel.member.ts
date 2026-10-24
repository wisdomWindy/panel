import { storeToRefs } from 'pinia';
import type { MemberChannel } from './channel.types';
import type { ShellNavigationItem } from '@/modules/application-shell/navigation';
import { useMemberRuntime } from '@/composables/useMemberRuntime';
import { useMembershipStore } from '@/stores/membership';
import MemberToolbarControl from '@/components/member/MemberToolbarControl.vue';
import { aiNavigationMessages, aiRoutes } from './channel.shared';

const item = (id: string, route: string, isDisabled = false): ShellNavigationItem => ({
  id,
  labelKey: `shell.navigation.${id}`,
  route,
  isDisabled
});

export const memberChannel: MemberChannel = Object.freeze({
  routes: Object.freeze([
    ...aiRoutes,
    {
      path: '/membership',
      name: 'Membership',
      component: () => import('@/views/membership/index.vue'),
      meta: { title: '会员中心' }
    },
    {
      path: '/moderation',
      name: 'Moderation',
      component: () => import('@/views/moderation/index.vue'),
      meta: { title: '刷屏防护', memberCapability: 'moderation' }
    },
    {
      path: '/moderation/log',
      name: 'ModerationLog',
      component: () => import('@/views/moderation/log.vue'),
      meta: { title: '禁言日志', memberCapability: 'moderation' }
    }
  ]),
  toolbarComponent: MemberToolbarControl,
  navigationMessages: Object.freeze({
    'zh-CN': Object.freeze({ ...aiNavigationMessages['zh-CN'], moderation: '禁言防护', membership: '会员中心' }),
    'en-US': Object.freeze({ ...aiNavigationMessages['en-US'], moderation: 'Moderation', membership: 'Membership' })
  }),
  deniedRoute: '/membership',
  buildNavigation: (common: readonly ShellNavigationItem[]) => {
    const status = useMembershipStore().status;
    const hasModeration = status.capabilities.moderation === 'ready' || status.capabilities.moderation === 'expired';
    const isExpired = status.shellState === 'expired';
    const risk = common.find(entry => entry.id === 'risk');
    const withRisk = common.map(entry => entry.id === 'risk' && hasModeration
      ? Object.freeze({
        ...entry,
        children: Object.freeze([
          ...(risk?.children ?? []),
          item('moderation', '/moderation', isExpired)
        ])
      })
      : entry);
    return Object.freeze([
      ...withRisk.slice(0, -1),
      item('membership', '/membership'),
      withRisk.at(-1) as ShellNavigationItem
    ]);
  },
  useStatus: () => storeToRefs(useMembershipStore()).status,
  canAccess: (path: string) => {
    const status = useMembershipStore().status;
    if (path === '/membership') return true;
    if (path === '/moderation' || path.startsWith('/moderation/')) {
      return status.capabilities.moderation === 'ready';
    }
    return true;
  },
  start: () => useMemberRuntime().start(),
  stop: () => useMemberRuntime().stop()
});
