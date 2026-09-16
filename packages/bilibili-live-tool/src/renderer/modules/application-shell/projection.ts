import type { LocalePreferences, UiPreferences } from '@/platform/desktop';
import { setAppLocale } from '@/i18n-runtime';

export function applyLocaleProjection(value: LocalePreferences): void {
  setAppLocale(value.locale);
  document.documentElement.lang = value.locale;
}

export function applyUiProjection(value: UiPreferences): void {
  const root = document.documentElement;
  const prefersDark = window.matchMedia?.('(prefers-color-scheme: dark)').matches ?? false;
  const isDark = value.theme === 'dark' || (value.theme === 'system' && prefersDark);
  root.classList.toggle('dark', isDark);
  root.style.colorScheme = isDark ? 'dark' : 'light';
  root.dataset.theme = value.theme;
  root.style.setProperty('--shell-accent', value.accentColor);
  root.style.setProperty('--el-color-primary', value.accentColor);
  root.style.setProperty('--shell-opacity', value.opacity.toFixed(2));
  root.style.setProperty('--shell-font-size', { small: '12px', medium: '14px', large: '16px' }[value.fontSize]);
  const layout = value.customLayout;
  root.style.setProperty('--shell-navigation-width', `${layout?.navigationWidth ?? 208}px`);
  root.style.setProperty('--shell-toolbar-height', `${layout?.toolbarHeight ?? 56}px`);
  root.style.setProperty('--shell-status-height', `${layout?.statusHeight ?? 32}px`);
}
