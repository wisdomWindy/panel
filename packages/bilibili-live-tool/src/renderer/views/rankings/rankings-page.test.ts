import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

const page = readFileSync(fileURLToPath(new URL('./index.vue', import.meta.url)), 'utf8');

describe('rankings page', () => {
  it('provides a coordinated dark surface for the page chrome', () => {
    expect(page).toContain('dark:bg-zinc-900');
    expect(page).toContain('dark:border-zinc-800');
    expect(page).toContain('dark:text-zinc-100');
    expect(page).toContain('dark:text-zinc-400');
  });

  it('keeps the table as the single empty ranking owner', () => {
    expect(page).toContain(':empty-text="t(\'liveData.empty\')"');
    expect(page.match(/liveData\.empty/g)).toHaveLength(1);
    expect(page).not.toContain('rows.length === 0');
  });
});
