import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

const read = (name: string) => readFileSync(fileURLToPath(new URL(name, import.meta.url)), 'utf8');
const page = read('./index.vue');
const template = page.slice(page.indexOf('<template>'));
const zh = JSON.parse(read('../../locales/zh-CN.json')) as Record<string, unknown>;
const en = JSON.parse(read('../../locales/en-US.json')) as Record<string, unknown>;

function hasKey(messages: Record<string, unknown>, key: string): boolean {
  let cursor: unknown = messages;
  for (const part of key.split('.')) {
    if (cursor === null || typeof cursor !== 'object') return false;
    cursor = (cursor as Record<string, unknown>)[part];
  }
  return typeof cursor === 'string';
}

describe('logs page contract', () => {
  it('keeps filtering, keyword search, highlighting, clearing, and export', () => {
    expect(page).toContain('useConfigStore');
    expect(page).toContain('filterLevel');
    expect(page).toContain('filterKeyword');
    expect(page).toContain('autoScroll');
    expect(page).toContain('showHighlights');
    expect(page).toContain('handleClear');
    expect(page).toContain('handleExport');
    expect(page).toContain('handleMarkHighlight');
    expect(page).toContain('createObjectURL');
  });

  it('renders as tokenised surfaces instead of legacy palette literals', () => {
    expect(template).toContain('app-card');
    expect(template).toContain('app-chip');
    expect(template).toContain('app-btn-primary');
    expect(template).toContain('app-muted');
    expect(template).toContain('app-faint');
    expect(template).toContain('app-surface-2');
    expect(template).not.toMatch(/dark:|bg-white|text-gray-\d|border-gray-\d/);
    expect(template).not.toMatch(/[\u4e00-\u9fff]/);
  });

  it('routes every literal message key through both locales', () => {
    const keys = [...page.matchAll(/(?<![A-Za-z0-9_$])t\('([A-Za-z0-9_.]+)'\)/g)].map(match => match[1]);
    expect(keys.length).toBeGreaterThan(0);
    for (const key of keys) {
      expect(hasKey(zh, key), `zh-CN is missing ${key}`).toBe(true);
      expect(hasKey(en, key), `en-US is missing ${key}`).toBe(true);
    }
  });
});
