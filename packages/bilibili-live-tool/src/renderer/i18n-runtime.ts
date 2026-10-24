import { computed, ref } from 'vue';
import type { CompileError, MessageCompiler, MessageContext } from 'vue-i18n';
import zhCn from 'element-plus/dist/locale/zh-cn.mjs';
import en from 'element-plus/dist/locale/en.mjs';

export type AppLocale = 'zh-CN' | 'en-US';

export const localeState = ref<AppLocale>('zh-CN');

const NAMED_PLACEHOLDER_PATTERN = /\{([A-Za-z][A-Za-z0-9_]*)\}/g;
const UNSUPPORTED_MESSAGE_RESOURCE_ERROR_CODE = 0;

export const cspSafeMessageCompiler: MessageCompiler = (message, { key, onError }) => {
  if (typeof message !== 'string') {
    const error: CompileError = Object.assign(
      new SyntaxError(`Unsupported locale message resource for ${key}`),
      { code: UNSUPPORTED_MESSAGE_RESOURCE_ERROR_CODE }
    );
    onError?.(error);
    return () => key;
  }

  return (context: MessageContext) => {
    return message.replace(NAMED_PLACEHOLDER_PATTERN, (_placeholder, name: string) => {
      return String(context.named(name));
    });
  };
};

export const elementPlusLocale = computed(() => {
  return localeState.value === 'en-US' ? en : zhCn;
});

export function setAppLocale(locale: AppLocale): void {
  localeState.value = locale;
}

export function getStoredLocale(): AppLocale {
  return localeState.value;
}
