import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

const source = (name: string) => readFileSync(fileURLToPath(new URL(name, import.meta.url)), 'utf8');
const templateOf = (page: string) => page.slice(page.indexOf('<template>'));

function namespaceKeys(messages: Record<string, unknown>, namespace: string): string[] {
  const walk = (value: unknown, prefix: string): string[] => {
    if (typeof value === 'string') return [prefix];
    return Object.entries(value as Record<string, unknown>).flatMap(([key, child]) =>
      walk(child, `${prefix}.${key}`)
    );
  };

  return Object.entries(messages[namespace] as Record<string, unknown>).flatMap(([key, child]) =>
    walk(child, `${namespace}.${key}`)
  );
}

describe('system settings shell contract', () => {
  it('keeps the application-shell categories closed and delegates each owner', () => {
    const page = source('./index.vue');
    expect(page).toContain('ConfigManagementPanel');
    expect(page).toContain('LanguageSettings');
    expect(page).toContain('InterfaceSettingsPanel');
    expect(page).not.toMatch(/LogSettings|UpdateSettings|ASRSettings|AISettings|ModerationSettings/);
    expect(page).not.toMatch(/useConfigStore|createObjectURL|new Blob|importPlaceholder/);
  });

  it('uses native configuration actions and a second reset confirmation', () => {
    const panel = source('./components/ConfigManagementPanel.vue');
    expect(panel).toContain('loadConfigOverview');
    expect(panel).toContain('openConfigDirectory');
    expect(panel).toContain('exportConfig');
    expect(panel).toContain('importConfig');
    expect(panel).toContain('resetConfig');
    expect(panel).toContain('result.importedScopes');
    expect(panel).toContain('result.resetScopes');
    expect(panel).toContain('Promise.allSettled');
    expect(panel).toContain('second: \'2-digit\'');
    expect(panel).toContain('selected.has(\'sensitiveWords\')');
    expect(panel).toContain('selected.has(\'blacklist\')');
    expect(panel).toContain('selected.has(\'giftSounds\')');
    expect(panel).toContain('selected.has(\'recording\')');
    expect(panel).toContain('ElMessageBox.confirm');
    expect(panel).not.toMatch(/createObjectURL|new Blob|JSON\.parse|type="textarea"/);
  });

  it('keeps locale and interface persistence in the application shell store', () => {
    const language = source('./components/LanguageSettings.vue');
    const panel = source('./components/InterfaceSettingsPanel.vue');
    const layout = source('./components/LayoutEditorDialog.vue');
    expect(language).toContain('useApplicationShellStore');
    expect(language).toContain('setLocaleAuto');
    expect(language).toContain('saveLocale');
    expect(language).toContain('draftLocale.value = shell.locale.locale');
    expect(language).not.toContain('useConfigStore');
    expect(panel).toContain('previewPreferences');
    expect(panel).toContain('saveUi');
    expect(panel).toContain('resetUi');
    expect(panel).toContain('LayoutEditorDialog');
    expect(panel).toContain('theme: \'system\'');
    expect(panel).toContain('navigationWidth: 208');
    expect(layout).toContain('role="separator"');
    expect(layout).toContain('@pointerdown');
    expect(layout).toContain('setPointerCapture');
    expect(layout).toContain('ArrowRight');
    expect(layout).toContain('--layout-preview-navigation');
  });

  it('exposes a deduplicated native update check from the about section', () => {
    const page = source('./index.vue');
    expect(page).toContain('release.checkUpdate()');
    expect(page).toContain('release.pending.has(\'checkUpdate\')');
    expect(page).toContain('t(\'update.noUpdate\')');
    expect(page).toContain('t(\'update.hasUpdate\')');
    expect(page).not.toContain('settings.aboutInfo.channel');
    expect(page).not.toContain('release.about?.channel');
    expect(page).not.toContain('invokeDesktop');
  });

  it('renders the settings shell as tokenised cards with a local category rail', () => {
    const page = source('./index.vue');
    const template = templateOf(page);
    expect(template).toContain('app-card');
    expect(template).toContain('app-text');
    expect(template).toContain('app-muted');
    expect(template).toContain('app-faint');
    expect(template).toContain('app-btn-primary');
    expect(template).toContain('role="tablist"');
    expect(template).toContain('aria-selected');
    expect(template).toContain('t(`settings.categories.${category.id}`)');
    expect(template).toContain('t(`settings.categoryHints.${activeCategory}`)');
    expect(template).not.toMatch(/dark:|bg-white|text-gray-\d|border-gray-\d/);
    expect(template).not.toMatch(/[\u4e00-\u9fff]/);
  });

  it('keeps the settings, logs, and update namespaces complete and localised', () => {
    const zh = JSON.parse(source('../../locales/zh-CN.json')) as Record<string, unknown>;
    const en = JSON.parse(source('../../locales/en-US.json')) as Record<string, unknown>;

    for (const namespace of ['settings', 'logs', 'update']) {
      expect(namespaceKeys(en, namespace).sort()).toEqual(namespaceKeys(zh, namespace).sort());
    }
    for (const key of [
      'settings.pageHint',
      'settings.categoryHints.about',
      'logs.pageHint',
      'logs.console',
      'logs.count',
      'update.pageHint',
      'update.versionHistory',
      'update.forced'
    ]) {
      const [namespace, ...rest] = key.split('.');
      expect(namespaceKeys(zh, namespace)).toContain(`${namespace}.${rest.join('.')}`);
      expect(namespaceKeys(en, namespace)).toContain(`${namespace}.${rest.join('.')}`);
    }
  });
});
