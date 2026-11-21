/**
 * 路由配置
 */

import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router';
import { memberChannel } from '@member-channel';

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    redirect: '/live'
  },
  {
    path: '/live',
    name: 'LiveRoom',
    component: () => import('@/views/live-room/index.vue'),
    meta: { titleKey: 'liveRoom.title' }
  },
  {
    path: '/dashboard',
    name: 'Dashboard',
    component: () => import('@/views/dashboard/index.vue'),
    meta: { titleKey: 'dashboard.title' }
  },
  {
    path: '/rankings',
    name: 'Rankings',
    component: () => import('@/views/rankings/index.vue'),
    meta: { titleKey: 'rankings.title' }
  },
  {
    path: '/connection',
    name: 'Connection',
    component: () => import('@/views/connection/index.vue'),
    meta: { titleKey: 'connection.title' }
  },
  {
    path: '/danmaku-list',
    name: 'DanmakuList',
    component: () => import('@/views/danmaku-list/index.vue'),
    meta: { titleKey: 'danmakuPanel.title' }
  },
  {
    path: '/gift-list',
    name: 'GiftList',
    component: () => import('@/views/gift-list/index.vue'),
    meta: { titleKey: 'gift.title' }
  },
  {
    path: '/settings',
    name: 'Settings',
    component: () => import('@/views/settings/index.vue'),
    meta: { titleKey: 'settings.title' }
  },
  {
    path: '/logs',
    name: 'Logs',
    component: () => import('@/views/logs/index.vue'),
    meta: { titleKey: 'logs.title' }
  },
  {
    path: '/sensitive-words',
    name: 'SensitiveWords',
    component: () => import('@/views/sensitive-words/index.vue'),
    meta: { titleKey: 'sensitiveWords.title' }
  },
  {
    path: '/blacklist',
    name: 'Blacklist',
    component: () => import('@/views/blacklist/index.vue'),
    meta: { titleKey: 'blacklist.title' }
  },
  {
    path: '/captain-effect',
    name: 'CaptainEffect',
    component: () => import('@/views/captain-effect/index.vue'),
    meta: { titleKey: 'captainEffect.title' }
  },
  {
    path: '/gift-reminder',
    name: 'GiftReminder',
    component: () => import('@/views/gift-reminder/index.vue'),
    meta: { titleKey: 'giftReminder.title' }
  },
  {
    path: '/update-log',
    name: 'UpdateLog',
    component: () => import('@/views/update-log/index.vue'),
    meta: { titleKey: 'update.title' }
  },
  {
    path: '/tts',
    name: 'Tts',
    component: () => import('@/views/tts/index.vue'),
    meta: { titleKey: 'tts.title' }
  },
  {
    path: '/recording',
    name: 'Recording',
    component: () => import('@/views/recording/index.vue'),
    meta: { titleKey: 'recording.title' }
  },
  {
    path: '/gift-sound',
    name: 'GiftSound',
    component: () => import('@/views/gift-sound/index.vue'),
    meta: { titleKey: 'giftSound.title' }
  },
  ...memberChannel.routes
];

const router = createRouter({
  history: createWebHashHistory(),
  routes
});

// 路由守卫
router.beforeEach((to, _from, next) => {
  if (!memberChannel.canAccess(to.path)) {
    next(memberChannel.deniedRoute);
    return;
  }
  next();
});

export default router;
