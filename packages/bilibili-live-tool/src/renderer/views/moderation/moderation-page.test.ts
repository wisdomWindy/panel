import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

const source = (path: string) => readFileSync(resolve(process.cwd(), path), 'utf8');

describe('member moderation page contract', () => {
  it('keeps native effects in stores and renders the fixed risk warning', () => {
    const page = source('src/renderer/views/moderation/index.vue');
    expect(page).not.toMatch(/invokeDesktop|listenDesktop|localStorage|electronAPI/);
    expect(page).toContain('自动禁言依赖本地账号 Cookie 能力');
    expect(page).toContain('非开放平台标准 API');
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
    ])
      expect(dialog).toContain(statement);
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
});
