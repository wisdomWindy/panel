/**
 * 路由配置
 */

import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router';
import { memberChannel } from '@member-channel';

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    redirect: '/connection'
  },
  {
    path: '/dashboard',
    name: 'Dashboard',
    component: () => import('@/views/dashboard/index.vue'),
    meta: { title: '仪表盘' }
  },
  {
    path: '/rankings',
    name: 'Rankings',
    component: () => import('@/views/rankings/index.vue'),
    meta: { title: '排行榜' }
  },
  {
    path: '/connection',
    name: 'Connection',
    component: () => import('@/views/connection/index.vue'),
    meta: { title: '连接配置' }
  },
  {
    path: '/danmaku-list',
    name: 'DanmakuList',
    component: () => import('@/views/danmaku-list/index.vue'),
    meta: { title: '弹幕列表' }
  },
  {
    path: '/gift-list',
    name: 'GiftList',
    component: () => import('@/views/gift-list/index.vue'),
    meta: { title: '礼物列表' }
  },
  {
    path: '/settings',
    name: 'Settings',
    component: () => import('@/views/settings/index.vue'),
    meta: { title: '设置' }
  },
  {
    path: '/logs',
    name: 'Logs',
    component: () => import('@/views/logs/index.vue'),
    meta: { title: '日志监控' }
  },
  {
    path: '/sensitive-words',
    name: 'SensitiveWords',
    component: () => import('@/views/sensitive-words/index.vue'),
    meta: { title: '敏感词管理' }
  },
  {
    path: '/blacklist',
    name: 'Blacklist',
    component: () => import('@/views/blacklist/index.vue'),
    meta: { title: '黑名单管理' }
  },
  {
    path: '/captain-effect',
    name: 'CaptainEffect',
    component: () => import('@/views/captain-effect/index.vue'),
    meta: { title: '上舰特效' }
  },
  {
    path: '/gift-reminder',
    name: 'GiftReminder',
    component: () => import('@/views/gift-reminder/index.vue'),
    meta: { title: '礼物提醒' }
  },
  {
    path: '/update-log',
    name: 'UpdateLog',
    component: () => import('@/views/update-log/index.vue'),
    meta: { title: '更新日志' }
  },
  {
    path: '/plugins',
    name: 'PluginManager',
    component: () => import('@/views/plugin-manager/index.vue'),
    meta: { title: '插件管理' }
  },
  {
    path: '/tts',
    name: 'Tts',
    component: () => import('@/views/tts/index.vue'),
    meta: { title: '语音播报' }
  },
  {
    path: '/recording',
    name: 'Recording',
    component: () => import('@/views/recording/index.vue'),
    meta: { title: '录制管理' }
  },
  {
    path: '/gift-sound',
    name: 'GiftSound',
    component: () => import('@/views/gift-sound/index.vue'),
    meta: { title: '礼物音效配置' }
  },
  ...memberChannel.routes
];

const router = createRouter({
  history: createWebHashHistory(),
  routes
});

// 路由守卫
router.beforeEach((to, _from, next) => {
  // 设置页面标题
  if (to.meta.title) {
    document.title = `${to.meta.title} - biliLiveHelper`;
  }
  if (!memberChannel.canAccess(to.path)) {
    next(memberChannel.deniedRoute);
    return;
  }
  next();
});

export default router;
