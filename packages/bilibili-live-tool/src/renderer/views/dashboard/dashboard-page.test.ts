import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

const source = (name: string) => readFileSync(fileURLToPath(new URL(name, import.meta.url)), 'utf8');

describe('dashboard page contract', () => {
  it('renders every summary value with readable text on its white card', () => {
    const page = source('./index.vue');
    const valueClasses = [...page.matchAll(/<dd class="([^"]+)"/g)].map(match => match[1]);

    expect(valueClasses).toHaveLength(5);
    expect(valueClasses.every(value => value.includes('text-gray-900'))).toBe(true);
  });

  it('offers a one-click Qwen3-TTS 1.7B model download without bypassing the store', () => {
    const page = source('./index.vue');

    expect(page).toContain("t('dashboard.qwen3Model.title')");
    expect(page).toContain('@click="downloadModel"');
    expect(page).toContain('media.downloadQwen3Model');
    expect(page).toContain('media.loadQwen3Models');
    expect(page).toContain('dashboard.qwen3Model.download');
    expect(page).toContain('dashboard.qwen3Model.serviceDown');
    expect(page).toContain('<el-progress');
    expect(page).not.toMatch(/invokeDesktop|listenDesktop/);
  });

  it('keeps the download model id out of the view layer', () => {
    const page = source('./index.vue');

    // The 1.7B default lives in the service catalogue, not hard-coded in the UI.
    expect(page).not.toContain('Qwen/Qwen3-TTS');
    expect(page).toContain('primaryModel');
  });
});
