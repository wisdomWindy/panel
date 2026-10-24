import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

const source = (name: string) => readFileSync(fileURLToPath(new URL(name, import.meta.url)), 'utf8');

describe('application shell component contract', () => {
  it('keeps root composition in four stable regions with a compile-time member channel', () => {
    const app = source('../../App.vue');
    expect(app).toContain('AppShellToolbar');
    expect(app).toContain('AppShellNavigation');
    expect(app).toContain('AppShellStatusBar');
    expect(app).toContain('from \'@member-channel\'');
    expect(app).toContain(':member-toolbar-component="memberChannel.toolbarComponent"');
    expect(app).not.toContain('import.meta.env.VITE_BUILD_CHANNEL');
    expect(app).toContain('--shell-toolbar-height');
    expect(app).toContain('--shell-status-height');
    expect(source('./AppShellNavigation.vue')).toContain('--shell-navigation-width,208px');
    expect(app).toContain(':aria-busy="shell.pending"');
    expect(app).not.toContain('v-loading="shell.pending"');
    expect(app).not.toContain('min-w-[560px]');
    expect(app).toContain('notificationOpen.value = !notificationOpen.value');
    expect(app).toContain(':notification-open="notificationOpen"');
    expect(app).toContain('UpdateAvailableDialog');
    expect(app).toContain('release.downloadUpdate()');
    expect(app).toContain('release.installUpdate()');
    expect(app).toContain('release.skipUpdate(');
    expect(app).toContain('ElMessage.error(release.error?.message');
    expect(app).toContain('release.update && release.update.candidate');
    expect(app).not.toContain('invokeDesktop');
  });

  it('keeps the redesigned shell visible, branded and responsive', () => {
    const app = source('../../App.vue');
    const toolbar = source('./AppShellToolbar.vue');
    const navigation = source('./AppShellNavigation.vue');
    const connection = source('../../views/connection/index.vue');
    const main = source('../../main.ts');
    const packageJson = source('../../../../package.json');
    const tailwind = source('../../styles/tailwind.css');
    const tailwindConfig = source('../../../../tailwind.config.js');

    expect(toolbar).toContain('appIconUrl');
    expect(toolbar).toContain(':src="appIconUrl"');
    expect(toolbar).toContain('sr-only');
    expect(toolbar).toContain('xl:not-sr-only');
    expect(toolbar).toContain('<span class="inline-flex">');
    expect(navigation).toContain('w-[72px]');
    expect(navigation).toContain('md:w-[var(--shell-navigation-width,208px)]');
    expect(connection).toContain('dark:text-zinc-100');
    expect(connection).toContain('grid-cols-1');
    expect(connection).toContain('<el-tag\n        class="shrink-0"');
    expect(main).toContain('document.documentElement.classList.toggle(\'dark\', prefersDark)');
    expect(packageJson).toContain('vite --mode free --host 127.0.0.1');
    expect(tailwind).toContain('@custom-variant dark (&:where(.dark, .dark *));');
    expect(tailwindConfig).toContain('500: \'#FB7299\'');
    expect(app).toContain('overflow-hidden bg-white');
    expect(app).toContain('overflow-y-auto bg-white p-3 sm:p-5 dark:bg-zinc-950');
    expect(app).not.toContain('bg-zinc-100');
  });

  it('keeps routed page surfaces from duplicating the shell content padding', () => {
    const app = source('../../App.vue');
    const pages = [
      '../../views/captain-effect/index.vue',
      '../../views/gift-reminder/index.vue',
      '../../views/gift-sound/index.vue'
    ];

    expect(app).toContain('overflow-y-auto bg-white p-3 sm:p-5 dark:bg-zinc-950');
    for (const page of pages) {
      const rootClasses = source(page).match(/<section class="([^"]+)"/)?.[1] ?? '';
      expect(rootClasses, page).not.toMatch(
        /(?:^|\s)(?:[a-z]+:)?p(?:[xytrbl])?-[^\s]+(?:\s|$)/
      );
    }
  });

  it('uses the biliLiveHelper product name across the desktop and renderer surfaces', () => {
    const rootHtml = source('../../../../index.html');
    const router = source('../../router/index.ts');
    const app = source('../../App.vue');
    const zhLocale = source('../../locales/zh-CN.json');
    const enLocale = source('../../locales/en-US.json');
    const tauriConfig = source('../../../../src-tauri/tauri.conf.json');
    const freeConfig = source('../../../../src-tauri/tauri.free.conf.json');

    expect(rootHtml).toContain('<title>biliLiveHelper</title>');
    expect(router).toContain('meta: { titleKey: \'dashboard.title\' }');
    expect(app).toContain('`${t(String(titleKey))} - biliLiveHelper`');
    expect(JSON.parse(zhLocale).shell.productName).toBe('biliLiveHelper');
    expect(JSON.parse(enLocale).shell.productName).toBe('biliLiveHelper');
    expect(JSON.parse(tauriConfig).productName).toBe('biliLiveHelper');
    expect(JSON.parse(tauriConfig).app.windows[0].title).toBe('biliLiveHelper');
    expect(JSON.parse(freeConfig).productName).toBe('biliLiveHelper');
  });

  it('keeps permissions out of templates and renders normalized status values', () => {
    const navigation = source('./AppShellNavigation.vue');
    const status = source('./AppShellStatusBar.vue');
    const toolbar = source('./AppShellToolbar.vue');
    expect(navigation).not.toMatch(/memberStatus|buildChannel/);
    expect(navigation).toContain('aria-current');
    expect(status).toContain('v-for="item in items"');
    expect(status).toContain('tabular-nums');
    expect(toolbar).toContain('<el-popover');
    expect(toolbar).toContain('emit(\'update:notificationOpen\', value)');
    expect(toolbar).toContain('class="h-20"');
    expect(toolbar).not.toMatch(/no notifications|暂无通知|empty-state/);
  });
});
