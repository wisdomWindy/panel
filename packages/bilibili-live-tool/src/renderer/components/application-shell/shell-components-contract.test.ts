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
    expect(app).toContain('--shell-navigation-width');
    expect(app).toContain('--shell-status-height');
    expect(app).toContain('--shell-navigation-width,208px');
    expect(app).toContain('v-loading="shell.pending"');
    expect(app).toContain('notificationOpen.value = !notificationOpen.value');
    expect(app).toContain(':notification-open="notificationOpen"');
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
