import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

const source = (path: string) => readFileSync(resolve(process.cwd(), path), 'utf8');
const templateOf = (page: string) => page.slice(page.indexOf('<template>'));

const zh = JSON.parse(source('src/renderer/locales/zh-CN.json')) as {
  moderation: {
    riskWarning: string;
    risk: Record<string, string>;
  };
};

describe('member moderation page contract', () => {
  it('keeps native effects in stores and renders the fixed risk warning', () => {
    const page = source('src/renderer/views/moderation/index.vue');
    expect(page).not.toMatch(/invokeDesktop|listenDesktop|localStorage|electronAPI/);
    // The fixed warning copy lives in the moderation namespace and is rendered
    // through t(), so the page itself carries no hard-coded Chinese.
    expect(zh.moderation.riskWarning).toContain('自动禁言依赖本地账号 Cookie 能力');
    expect(zh.moderation.riskWarning).toContain('非开放平台标准 API');
    expect(page).toContain("t('moderation.riskWarning')");
    expect(page).toContain('ModerationRiskDialog');
    expect(page).toContain('ModerationRulesDialog');
    expect(page).toContain('ModerationAlertList');
  });

  it('requires explicit acknowledgement of all four source risk statements', () => {
    const dialog = source('src/renderer/views/moderation/components/ModerationRiskDialog.vue');
    for (const statement of [
      '本功能依赖本地账号 Cookie 能力，非 B 站开放平台标准 API。',
      '不承诺可用性，接口变动随时下线。',
      '使用本功能可能导致账号被平台风控。',
      '请确保你已了解并自行承担相关风险。'
    ]) {
      expect(Object.values(zh.moderation.risk)).toContain(statement);
      expect(dialog).toContain('moderation.risk.');
    }
    expect(dialog).toContain(':disabled="!accepted"');
  });

  it('uses only source-backed mute durations and a separate audit route', () => {
    const page = source('src/renderer/views/moderation/index.vue');
    const log = source('src/renderer/views/moderation/log.vue');
    for (const duration of ['10m', '1h', 'permanent'])
      expect(page).toContain(`value="${duration}"`);
    expect(log).not.toMatch(/invokeDesktop|listenDesktop|localStorage|electronAPI/);
    expect(log).toMatch(/store\s*\.\s*loadLogs/);
    expect(log).toContain('store.markFalsePositive');
  });

  it('renders both moderation pages as token-driven surfaces without dark: literals', () => {
    const pages = [
      source('src/renderer/views/moderation/index.vue'),
      source('src/renderer/views/moderation/log.vue'),
      source('src/renderer/views/moderation/components/ModerationAlertList.vue'),
      source('src/renderer/views/moderation/components/ModerationRiskDialog.vue'),
      source('src/renderer/views/moderation/components/ModerationUserDrawer.vue')
    ].map(templateOf);

    for (const template of pages) {
      expect(template).toContain('app-muted');
      expect(template).not.toMatch(
        /dark:|bg-white|text-gray-|border-gray-|divide-gray-|bg-zinc-|text-zinc-|border-zinc-/
      );
      // Copy is translated, never inlined in the template.
      expect(template).not.toMatch(/[\u4e00-\u9fff]/);
    }

    const page = pages[0];
    expect(page).toContain('app-card');
    expect(page).toContain('app-text');
    expect(page).toContain('app-faint');
    expect(page).toContain('app-btn-primary');
    expect(pages[2]).toContain('app-chip');
    expect(pages[2]).toContain('app-badge');
  });
});
