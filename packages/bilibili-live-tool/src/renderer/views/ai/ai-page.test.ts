import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

const source = (path: string) => readFileSync(resolve(process.cwd(), path), 'utf8');

describe('shared AI page contract', () => {
  it('keeps native effects in stores and shows only automatic generation results', () => {
    const page = source('src/renderer/views/ai/index.vue');
    expect(page).not.toMatch(/invokeDesktop|listenDesktop|localStorage|electronAPI/);
    expect(page).not.toMatch(/AiContextList|ai-prompt|@click="generate"|@click="regenerate"/);
    expect(page).not.toContain('当前上下文');
    expect(page).not.toContain('重新生成');
    expect(page).toContain('仅本地预览，不发送');
    expect(page).toContain('@click="copyResult"');
    expect(page).not.toMatch(/发送弹幕|sendDanmaku|danmaku_send/);
  });

  it('uses focused config component boundaries', () => {
    const page = source('src/renderer/views/ai/index.vue');
    for (const component of [
      'AiVoiceReplyControl',
      'AiModelProfilesDialog',
      'AiPersonaConfigDialog'
    ]) {
      expect(page).toContain(component);
    }
    expect(page).not.toMatch(/useMembershipStore|MembershipPausedNotice|memberCapability/);
    const config = source('src/renderer/views/ai/components/AiPersonaConfigDialog.vue');
    for (const style of ['concise', 'natural', 'detailed']) {
      expect(config).toContain(`value="${style}"`);
    }
    expect(config).toContain('draft.maxSentences');
    expect(config).not.toMatch(/localStorage|invokeDesktop/);
  });

  it('keeps model secrets and voice routing behind typed store actions', () => {
    const models = source('src/renderer/views/ai/components/AiModelProfilesDialog.vue');
    const voice = source('src/renderer/views/ai/components/AiVoiceReplyControl.vue');
    expect(models).toContain('apiKeyReplacement');
    expect(models).not.toMatch(/localStorage|invokeDesktop/);
    expect(models).not.toContain('代理 URL');
    expect(models).not.toContain('draft.proxyUrl');
    expect(models).toContain('proxyUrl: null');
    expect(models).toContain('label="推理强度"');
    expect(models).not.toContain('label="温度"');
    expect(models).not.toContain('draft.temperature');
    for (const effort of ['none', 'minimal', 'low', 'medium', 'high', 'xhigh', 'max']) {
      expect(models).toContain(`value="${effort}"`);
    }
    for (const control of ['AI 语音回复', '音频路由', '测试播报', '跳过当前', '清空队列']) {
      expect(voice).toContain(control);
    }
    expect(voice).toContain('label="声色"');
    expect(voice).toContain('voice.displayName');
    expect(voice).toContain('voice.voiceId');
    expect(voice).toContain('route.voiceId');
    expect(voice).toContain('selectedVoiceId');
    expect(voice).toContain('isDraftRouteAvailable');
    expect(voice).toContain(':disabled="!isDraftRouteAvailable"');
    expect(voice).toContain('snapshot?.voices');
    expect(voice).toContain('flex flex-col gap-4 border-y');
    expect(voice).toContain('v-if="!defaultProfile"');
    expect(voice).toContain('v-else-if="!selectedDevice"');
    expect(voice).not.toMatch(/localStorage|invokeDesktop|发送弹幕/);
  });

  it('keeps the AI audit route read-only and free of ASR remnants', () => {
    const page = source('src/renderer/views/ai/index.vue');
    const contract = source('src/renderer/platform/desktop/contracts/ai.ts');
    const log = source('src/renderer/views/ai/log.vue');
    expect(page).not.toMatch(/asr|Asr/);
    expect(contract).not.toMatch(/asr_|AsrStatus|AsrModel/);
    expect(log).not.toMatch(/invokeDesktop|listenDesktop|localStorage|electronAPI/);
    expect(log).toMatch(/store\s*\.\s*loadLogs/);
  });
});
