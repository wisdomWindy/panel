import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

const source = (name: string) => readFileSync(fileURLToPath(new URL(name, import.meta.url)), 'utf8');

describe('system settings shell contract', () => {
  it('keeps the application-shell categories closed and delegates each owner', () => {
    const page = source('./index.vue');
    expect(page).toContain('ConfigManagementPanel');
    expect(page).toContain('LanguageSettings');
    expect(page).toContain('InterfaceSettingsPanel');
    expect(page).not.toMatch(/LogSettings|UpdateSettings|PluginSettings|ASRSettings|AISettings|ModerationSettings/);
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
    expect(panel).toContain('navigationWidth: 208');
    expect(layout).toContain('role="separator"');
    expect(layout).toContain('@pointerdown');
    expect(layout).toContain('setPointerCapture');
    expect(layout).toContain('ArrowRight');
    expect(layout).toContain('--layout-preview-navigation');
  });
});
