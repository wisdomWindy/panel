import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

const read = (relative: string) => readFileSync(new URL(relative, import.meta.url), 'utf8');

describe('release pages', () => {
  it('keeps update operations behind the native store', () => {
    const page = read('./index.vue');
    expect(page).not.toMatch(/window\.open|fetch\(|localStorage|electronAPI|invokeDesktop/);
    expect(page).toContain('store.checkUpdate()');
    expect(page).toContain('store.downloadUpdate()');
    expect(page).toContain('store.installUpdate()');
    expect(page).toContain('store.skipUpdate(');
  });

  it('renders the version history and update prompt as tokenised cards', () => {
    const page = read('./index.vue');
    const template = page.slice(page.indexOf('<template>'));
    expect(template).toContain('app-card');
    expect(template).toContain('app-btn-primary');
    expect(template).toContain('app-muted');
    expect(template).toContain('app-faint');
    expect(template).toContain('store.pending.has(\'checkUpdate\')');
    expect(template).toContain('t(\'update.versionHistory\')');
    expect(template).toContain('store.update.candidate.releaseNotes');
    expect(template).toContain('UpdateAvailableDialog');
    expect(template).not.toMatch(/dark:|bg-white|text-gray-\d|border-gray-\d/);
    expect(template).not.toMatch(/[\u4e00-\u9fff]/);
  });

  it('prevents every ordinary dismissal path for forced updates', () => {
    const dialog = read('./UpdateAvailableDialog.vue');
    expect(dialog).toContain(':show-close="!isForced"');
    expect(dialog).toContain(':close-on-click-modal="!isForced"');
    expect(dialog).toContain(':close-on-press-escape="!isForced"');
    expect(dialog.match(/v-if="!isForced"/g)).toHaveLength(2);
    expect(dialog).toContain('snapshot.status === \'downloaded\'');
  });
});
