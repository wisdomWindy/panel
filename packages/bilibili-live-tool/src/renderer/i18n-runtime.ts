import { computed, ref } from 'vue';
import zhCn from 'element-plus/dist/locale/zh-cn.mjs';
import en from 'element-plus/dist/locale/en.mjs';

export type AppLocale = 'zh-CN' | 'en-US';

const STORAGE_KEY = 'locale';

function normalizeLocale(value: string | null): AppLocale {
  return value === 'en-US' ? 'en-US' : 'zh-CN';
}

export const localeState = ref<AppLocale>(
  normalizeLocale(localStorage.getItem(STORAGE_KEY))
);

export const elementPlusLocale = computed(() => {
  return localeState.value === 'en-US' ? en : zhCn;
});

export function setAppLocale(locale: AppLocale): void {
  localeState.value = locale;
  localStorage.setItem(STORAGE_KEY, locale);
}

export function getStoredLocale(): AppLocale {
  return normalizeLocale(localStorage.getItem(STORAGE_KEY));
}
