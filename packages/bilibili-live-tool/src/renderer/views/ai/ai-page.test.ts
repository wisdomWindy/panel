import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

const source = (path: string) => readFileSync(resolve(process.cwd(), path), 'utf8');

describe('member AI page contract', () => {
  it('keeps native effects in stores and generation explicitly user driven', () => {
    const page = source('src/renderer/views/ai/index.vue');
    expect(page).not.toMatch(/invokeDesktop|listenDesktop|localStorage|electronAPI/);
    expect(page).toContain('@click="generate"');
    expect(page).not.toMatch(/watch(?:Effect)?\s*\([^)]*generate/);
    expect(page).toContain('仅本地预览，不发送');
    expect(page).not.toMatch(/发送弹幕|sendDanmaku|danmaku_send/);
  });

  it('uses focused config, context, and ASR component boundaries', () => {
    const page = source('src/renderer/views/ai/index.vue');
    for (const component of ['AiPersonaConfigDialog', 'AiContextList', 'AsrModelDownloadDialog']) {
      expect(page).toContain(component);
    }
    const config = source('src/renderer/views/ai/components/AiPersonaConfigDialog.vue');
    for (const style of ['short', 'humorous', 'formal', 'casual']) {
      expect(config).toContain(`value="${style}"`);
    }
    expect(config).not.toMatch(/localStorage|invokeDesktop/);
  });

  it('keeps unavailable ASR fail closed and provides a separate audit route', () => {
    const asr = source('src/renderer/views/ai/components/AsrModelDownloadDialog.vue');
    const log = source('src/renderer/views/ai/log.vue');
    expect(asr).toContain('模型资源暂不可用');
    expect(asr).toContain(':disabled="!manifest || !accepted"');
    expect(log).not.toMatch(/invokeDesktop|listenDesktop|localStorage|electronAPI/);
    expect(log).toMatch(/store\s*\.\s*loadLogs/);
  });
});
