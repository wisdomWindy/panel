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

  it('no longer hosts the speech model card', () => {
    const page = source('./index.vue');

    // The model download belongs to the speech feature, not the dashboard.
    expect(page).not.toMatch(/qwen3|Qwen3/);
    expect(page).not.toMatch(/invokeDesktop|listenDesktop/);
  });
});
