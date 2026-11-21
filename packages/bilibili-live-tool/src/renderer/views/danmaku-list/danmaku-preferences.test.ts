import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

const source = (name: string) =>
  readFileSync(fileURLToPath(new URL(name, import.meta.url)), 'utf8');

describe('danmaku display preferences', () => {
  it('applies the four interface settings the panel used to ignore', () => {
    const page = source('./index.vue');

    // The effective preference is the previewed one, so the settings panel
    // previews live.
    expect(page).toContain('shell.previewUi ?? shell.savedUi');
    for (const key of [
      'maxDanmakuEntries',
      'showDanmakuTime',
      'showDanmakuLevel',
      'showDanmakuMedal'
    ]) {
      expect(page, key).toContain(key);
    }
    // The cap is applied to what is rendered, not only to the store buffer.
    expect(page).toContain('slice(-limit)');
    expect(page).toContain('v-for="event in shownEvents"');
    expect(page).toContain('v-if="showTime"');
  });

  it('lets the identity view hide level and medal independently', () => {
    const view = source('../../components/live-data/AudienceIdentityView.vue');

    expect(view).toContain('showLevel?: boolean');
    expect(view).toContain('showMedal?: boolean');
    // Defaults keep other call sites unchanged.
    expect(view).toContain('showLevel: true');
    expect(view).toContain('showMedal: true');
    expect(view).toContain('props.showLevel');
    expect(view).toContain('props.showMedal');
  });
});
