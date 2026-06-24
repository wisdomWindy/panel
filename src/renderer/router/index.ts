/**
 * 路由配置
 */

import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router';

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
    path: '/connection',
    name: 'Connection',
    component: () => import('@/views/connection/index.vue'),
    meta: { title: '连接配置' }
  },
  {
    path: '/danmaku-pool',
    name: 'DanmakuPool',
    component: () => import('@/views/danmaku-pool/index.vue'),
    meta: { title: '弹幕池管理' }
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
    path: '/blacklist',
    name: 'Blacklist',
    component: () => import('@/views/blacklist/index.vue'),
    meta: { title: '黑名单管理' }
  },
  {
    path: '/effect-preview',
    name: 'EffectPreview',
    component: () => import('@/views/effect-preview/index.vue'),
    meta: { title: '特效预览' }
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
  }
];

const router = createRouter({
  history: createWebHashHistory(),
  routes
});

// 路由守卫
router.beforeEach((to, _from, next) => {
  // 设置页面标题
  if (to.meta.title) {
    document.title = `${to.meta.title} - Bilibili 直播助手`;
  }
  next();
});

export default router;
