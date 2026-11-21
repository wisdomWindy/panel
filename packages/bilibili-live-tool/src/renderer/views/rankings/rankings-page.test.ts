import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

const page = readFileSync(fileURLToPath(new URL('./index.vue', import.meta.url)), 'utf8');

describe('rankings page', () => {
  it('renders the three boards side by side on the shared tokens', () => {
    // The design shows gift / danmaku / captain at once instead of behind tabs.
    expect(page).toContain('xl:grid-cols-3');
    expect(page).toContain('v-for="column in columns"');
    expect(page).toContain('app-card');
    expect(page).toContain('app-rank');
    expect(page).not.toContain('el-tabs');
    expect(page).not.toMatch(/dark:bg-zinc-900|dark:text-zinc-100/);
  });

  it('keeps each board as its own empty-state owner', () => {
    // Every column renders its own placeholder, so one empty board does not
    // blank the other two.
    expect(page).toContain('v-if="column.rows.length === 0"');
    expect(page.match(/liveData\.empty/g)).toHaveLength(1);
  });

  it('formats each board with its own unit', () => {
    expect(page).toContain("tab === 'danmu'");
    expect(page).toContain("t('rankings.danmakuUnit'");
    expect(page).toContain('formatMoneyMinor');
  });

  it('resolves every board title from a real locale key', () => {
    // Regression: the board id `danmu` must not be used as a locale key.
    expect(page).toContain("danmu: 'rankings.danmaku'");
    expect(page).not.toContain('t(`rankings.${tab}`)');

    for (const locale of ['zh-CN', 'en-US']) {
      const messages = JSON.parse(
        readFileSync(fileURLToPath(new URL(`../../locales/${locale}.json`, import.meta.url)), 'utf8')
      ) as { rankings: Record<string, string> };
      for (const key of ['gift', 'danmaku', 'captain']) {
        expect(messages.rankings[key], `${locale}.rankings.${key}`).toBeTruthy();
      }
    }
  });
});
