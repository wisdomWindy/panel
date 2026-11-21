import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

const source = (name: string) => readFileSync(fileURLToPath(new URL(name, import.meta.url)), 'utf8');

describe('dashboard page contract', () => {
  it('renders every summary value through the shared metric card', () => {
    const page = source('./index.vue');
    const cards = [...page.matchAll(/class="app-metric"/g)];

    // Five metrics: online, danmaku, rate, gift value, captains.
    expect(cards).toHaveLength(1);
    expect(page).toContain('v-for="card in metricCards"');
    expect(page).toContain('class="app-metric-value');
    expect(page).toContain("t('dashboard.onlineCount')");
    expect(page).toContain("t('dashboard.danmakuCount')");
    expect(page).toContain("t('dashboard.danmakuRate')");
    expect(page).toContain("t('dashboard.giftRevenue')");
    expect(page).toContain("t('dashboard.captainCount')");
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
