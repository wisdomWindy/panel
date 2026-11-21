import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

const source = (name: string) => readFileSync(fileURLToPath(new URL(name, import.meta.url)), 'utf8');

describe('live-room overview page contract', () => {
  it('composes the PRD 8.1 landing screen from room data, metrics and recent events', () => {
    const page = source('./index.vue');

    // Room identity + read-only promise (PRD 2.3 / 9.1).
    expect(page).toContain("t('liveRoom.readOnly')");
    expect(page).toContain('room.snapshot.roomName');
    expect(page).toContain('room.snapshot.roomId');

    // Four metric cards fed by the analytics snapshot.
    expect(page).toContain('metricCards');
    expect(page).toContain("t('liveRoom.metrics.danmu')");
    expect(page).toContain("t('liveRoom.metrics.gift')");
    expect(page).toContain("t('liveRoom.metrics.captain')");
    expect(page).toContain("t('liveRoom.metrics.online')");

    // Recent-event rail, newest first.
    expect(page).toContain('...feed.events].reverse()');
    expect(page).toContain('recentEvents');
  });

  it('uses the shared identity badge tones instead of ad-hoc colours', () => {
    const page = source('./index.vue');

    for (const tone of ['admin', 'governor', 'admiral', 'captain', 'vip', 'plain']) {
      expect(page).toContain(`'${tone}'`);
    }
    expect(page).toContain('app-badge');
    // The badge palette lives in one stylesheet, not in the page.
    expect(page).not.toMatch(/#[0-9a-fA-F]{6}/);
  });

  it('renders every user-facing string through i18n', () => {
    const page = source('./index.vue');

    expect(page).not.toMatch(/[\u4e00-\u9fff]/);
    expect(page).toContain("useI18n");
  });
});
