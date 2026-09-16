import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

const read = (relative: string) => readFileSync(new URL(relative, import.meta.url), 'utf8');

describe('release pages', () => {
  it('keeps update operations behind the native store', () => {
    const page = read('../update-log/index.vue');
    expect(page).not.toMatch(/window\.open|fetch\(|localStorage|electronAPI|invokeDesktop/);
    expect(page).toContain('store.checkUpdate()');
    expect(page).toContain('store.downloadUpdate()');
  });

  it('accepts only native-selected plugin packages', () => {
    const page = read('./index.vue');
    const dialog = read('./PluginInstallDialog.vue');
    expect(page + dialog).not.toMatch(/\.js|new Function|eval\(|electronAPI|type="file"/);
    expect(page).toContain('store.selectPluginInstall()');
    expect(page).toContain('ElMessageBox.confirm');
  });

  it('hides defer and skip actions for forced updates', () => {
    const dialog = read('../update-log/UpdateAvailableDialog.vue');
    expect(dialog.match(/!snapshot\.candidate\?\.isForced/g)).toHaveLength(2);
    expect(dialog).toContain('snapshot.status === \'downloaded\'');
  });
});
