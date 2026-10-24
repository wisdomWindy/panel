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
  root.style.setProperty('--el-color-primary-rgb', [
    value.accentColor.slice(1, 3),
    value.accentColor.slice(3, 5),
    value.accentColor.slice(5, 7)
  ].map(channel => Number.parseInt(channel, 16)).join(', '));
  root.style.setProperty('--el-color-primary-light-3', `color-mix(in srgb, ${value.accentColor} 70%, white)`);
  root.style.setProperty('--el-color-primary-light-5', `color-mix(in srgb, ${value.accentColor} 50%, white)`);
  root.style.setProperty('--el-color-primary-light-7', `color-mix(in srgb, ${value.accentColor} 30%, white)`);
  root.style.setProperty('--el-color-primary-light-8', `color-mix(in srgb, ${value.accentColor} 20%, white)`);
  root.style.setProperty('--el-color-primary-light-9', `color-mix(in srgb, ${value.accentColor} 10%, white)`);
  root.style.setProperty('--el-color-primary-dark-2', `color-mix(in srgb, ${value.accentColor} 80%, black)`);
  root.style.setProperty('--shell-opacity', value.opacity.toFixed(2));
  root.style.setProperty('--shell-font-size', { small: '12px', medium: '14px', large: '16px' }[value.fontSize]);
  const layout = value.customLayout;
  root.style.setProperty('--shell-navigation-width', `${layout?.navigationWidth ?? 208}px`);
  root.style.setProperty('--shell-toolbar-height', `${layout?.toolbarHeight ?? 56}px`);
  root.style.setProperty('--shell-status-height', `${layout?.statusHeight ?? 32}px`);
}
