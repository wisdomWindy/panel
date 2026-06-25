/**
 * 国际化 (i18n) 初始化模块
 * 基于 vue-i18n 实现多语言支持
 */

import { createI18n, type I18n } from 'vue-i18n';
import zhCN from '../../locales/zh-CN.json';
import enUS from '../../locales/en-US.json';

// 显式类型标注以满足 vue-i18n 的严格类型检查
const messages: Record<string, Record<string, string>> = {
  'zh-CN': zhCN as unknown as Record<string, string>,
  'en-US': enUS as unknown as Record<string, string>
};

// 定义支持的语言
export const SUPPORTED_LOCALES = [
  { code: 'zh-CN', name: '简体中文' },
  { code: 'en-US', name: 'English' }
] as const;

export type SupportedLocale = (typeof SUPPORTED_LOCALES)[number]['code'];

// 创建 i18n 实例
const i18n: I18n = createI18n({
  legacy: false,
  locale: 'zh-CN',
  fallbackLocale: 'zh-CN',
  messages
});

// 切换语言函数
export function setLocale(locale: SupportedLocale): void {
  (i18n.global.locale as unknown as { value: string }).value = locale;
  // 持久化到 localStorage
  localStorage.setItem('app-locale', locale);
}

// 获取当前语言
export function getCurrentLocale(): SupportedLocale {
  const saved = localStorage.getItem('app-locale');
  if (saved === 'zh-CN' || saved === 'en-US') {
    return saved;
  }
  return 'zh-CN';
}

export default i18n;
