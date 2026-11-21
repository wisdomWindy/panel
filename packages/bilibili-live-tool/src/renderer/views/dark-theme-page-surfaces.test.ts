import { createSSRApp, defineComponent, h, type Component } from 'vue';
import { renderToString } from 'vue/server-renderer';
import { createI18n } from 'vue-i18n';
import { createPinia, setActivePinia, type Pinia } from 'pinia';
import { ID_INJECTION_KEY, ZINDEX_INJECTION_KEY } from 'element-plus';
import { beforeEach, describe, expect, it } from 'vitest';
import zhCN from '@/locales/zh-CN.json';
import type { EventId, LiveEvent, RoomId, StatSnapshot, UserUid } from '@/platform/desktop';
import { useLiveAnalyticsStore } from '@/stores/live-data/analytics';
import { useLiveFeedStore } from '@/stores/live-data/feed';
import DanmakuPage from './danmaku-list/index.vue';
import DashboardPage from './dashboard/index.vue';
import SensitiveWordsPage from './sensitive-words/index.vue';
import BlacklistPage from './blacklist/index.vue';
import TrendChart from '@/components/live-data/TrendChart.vue';

function createTestI18n() {
  return createI18n({
    legacy: false,
    locale: 'zh-CN',
    missingWarn: false,
    fallbackWarn: false,
    messages: { 'zh-CN': zhCN }
  });
}

async function renderPage(component: Component, prepare?: (pinia: Pinia) => void): Promise<string> {
  const pinia = createPinia();
  setActivePinia(pinia);
  prepare?.(pinia);
  const app = createSSRApp(component);
  app.use(pinia);
  app.use(createTestI18n());
  app.provide(ID_INJECTION_KEY, { prefix: 1024, current: 0 });
  app.provide(ZINDEX_INJECTION_KEY, { current: 0 });
  app.component(
    'RouterLink',
    defineComponent({
      setup(_, { slots }) {
        return () => h('a', slots.default?.());
      }
    })
  );
  return renderToString(app);
}

function classTokens(html: string): string[] {
  return [...html.matchAll(/\sclass="([^"]*)"/g)].flatMap(match => match[1].split(/\s+/));
}

function expectSharedPageSurface(html: string): void {
  const tokens = classTokens(html);
  // The risk pages are token-driven cards: one palette, no per-page dark: literals.
  expect(tokens).toEqual(
    expect.arrayContaining(['app-card', 'app-text', 'app-faint', 'app-chip', 'app-btn-primary'])
  );
  expect(tokens.some(token => token.startsWith('dark:'))).toBe(false);
  expect(html).not.toContain('dark:');
  expect(html).not.toContain('bg-white');
  expect(html).not.toContain('text-gray-');
  expect(html).not.toContain('border-gray-');
}

describe('dark theme page surfaces', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
  });

  it('renders the danmaku panel as one readable dark surface', async () => {
    const html = await renderPage(DanmakuPage, () => {
      const event: LiveEvent = {
        kind: 'danmu',
        eventId: 'event-1' as EventId,
        roomId: '1' as RoomId,
        occurredAt: 0,
        audience: {
          userUid: '1000' as UserUid,
          username: 'viewer',
          userLevel: null,
          medal: null,
          tags: []
        },
        content: 'message'
      };
      useLiveFeedStore().accept(event);
    });

    // The redesigned panel is token-driven: one palette, no per-page dark: literals.
    const tokens = classTokens(html);
    expect(tokens).toEqual(
      expect.arrayContaining(['app-card', 'app-chip', 'app-text', 'app-faint'])
    );
    expect(tokens.some(token => token.startsWith('dark:'))).toBe(false);
    expect(html).toContain('身份图例');
    expect(html).toContain('viewer');
  });

  it('renders dashboard metrics and chart framing with dark surface tokens', async () => {
    const statistics: StatSnapshot = {
      range: { kind: 'liveSession' },
      metrics: {
        onlineCount: 1,
        danmuCount: 2,
        danmuRate: 3,
        giftValueMinor: 4,
        captainCount: 5,
        filteredCount: 6
      },
      trendPoints: [],
      generatedAt: 0,
      isHistoricalDataAvailable: true,
      isStale: false
    };
    const html = await renderPage(DashboardPage, () => {
      useLiveAnalyticsStore().replaceStatistics(statistics);
    });
    const tokens = classTokens(html);

    // Metric cards and the model card share the token palette.
    expect(tokens.filter(token => token === 'app-metric')).toHaveLength(5);
    expect(tokens).toContain('app-metric-value');
    expect(tokens).toContain('app-card');
    expect(tokens).toContain('app-btn-primary');
    // The Qwen3-TTS model card shows the "service offline" hint when the
    // catalogue cannot be loaded.
    expect(html).toContain('Qwen3-TTS 1.7B');
    expect(html).toContain('本地语音服务未启动');
  });

  it.each([
    ['sensitive words', SensitiveWordsPage],
    ['blacklist', BlacklistPage]
  ])('renders the %s page as one token-driven card surface', async (_, page) => {
    expectSharedPageSurface(await renderPage(page));
  });

  it('renders empty trend chart framing with the shared card tokens', async () => {
    const app = createSSRApp(TrendChart, {
      title: 'trend',
      points: [],
      metric: 'onlineCount',
      color: '#2563eb'
    });
    app.use(createTestI18n());
    const tokens = classTokens(await renderToString(app));

    expect(tokens).toEqual(expect.arrayContaining(['app-card', 'app-text', 'app-faint']));
    expect(tokens.some(token => token.startsWith('dark:'))).toBe(false);
  });
});
