import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

const read = (relative: string) => readFileSync(new URL(relative, import.meta.url), 'utf8');

describe('release dialog', () => {
  it('prevents every ordinary dismissal path for forced updates', () => {
    const dialog = read('./UpdateAvailableDialog.vue');
    expect(dialog).toContain(':show-close="!isForced"');
    expect(dialog).toContain(':close-on-click-modal="!isForced"');
    expect(dialog).toContain(':close-on-press-escape="!isForced"');
    expect(dialog.match(/v-if="!isForced"/g)).toHaveLength(2);
    expect(dialog).toContain('snapshot.status === \'downloaded\'');
  });
});
