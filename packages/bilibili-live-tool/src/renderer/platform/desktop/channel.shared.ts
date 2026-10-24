import type { RouteRecordRaw } from 'vue-router';

export const aiRoutes: readonly RouteRecordRaw[] = Object.freeze([
  {
    path: '/ai',
    name: 'AiAssistant',
    component: () => import('@/views/ai/index.vue'),
    meta: { title: 'AI 助手' }
  },
  {
    path: '/ai/log',
    name: 'AiLog',
    component: () => import('@/views/ai/log.vue'),
    meta: { title: 'AI 记录' }
  }
]);

export const aiNavigationMessages = Object.freeze({
  'zh-CN': Object.freeze({ ai: 'AI 助手' }),
  'en-US': Object.freeze({ ai: 'AI assistant' })
});
