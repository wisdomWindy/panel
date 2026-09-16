import { computed, ref } from 'vue';
import zhCn from 'element-plus/dist/locale/zh-cn.mjs';
import en from 'element-plus/dist/locale/en.mjs';

export type AppLocale = 'zh-CN' | 'en-US';

export const localeState = ref<AppLocale>('zh-CN');

export const elementPlusLocale = computed(() => {
  return localeState.value === 'en-US' ? en : zhCn;
});

export function setAppLocale(locale: AppLocale): void {
  localeState.value = locale;
}

export function getStoredLocale(): AppLocale {
  return localeState.value;
}
