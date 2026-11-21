import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

const source = (path: string) => readFileSync(resolve(process.cwd(), path), 'utf8');

const page = 'src/renderer/views/ai/index.vue';
const voice = 'src/renderer/views/ai/components/AiVoiceReplyControl.vue';
const models = 'src/renderer/views/ai/components/AiModelProfilesDialog.vue';
const persona = 'src/renderer/views/ai/components/AiPersonaConfigDialog.vue';

function localeKeys(value: Record<string, unknown>, prefix = ''): string[] {
  return Object.entries(value).flatMap(([key, child]) => {
    const path = prefix ? `${prefix}.${key}` : key;
    return typeof child === 'string'
      ? [path]
      : localeKeys(child as Record<string, unknown>, path);
  });
}

describe('shared AI page contract', () => {
  it('keeps native effects in stores and shows only automatic generation results', () => {
    const pageSource = source(page);
    expect(pageSource).not.toMatch(/invokeDesktop|listenDesktop|localStorage|electronAPI/);
    expect(pageSource).not.toMatch(/AiContextList|ai-prompt|@click="generate"|@click="regenerate"/);
    expect(pageSource).not.toContain('当前上下文');
    expect(pageSource).not.toContain('重新生成');
    // The local-only disclaimer is translated, so the copy lives in the zh-CN resource.
    expect(pageSource).toContain("t('ai.result.localOnly')");
    const zh = JSON.parse(source('src/renderer/locales/zh-CN.json')) as {
      ai: { result: { localOnly: string } };
    };
    expect(zh.ai.result.localOnly).toContain('仅本地预览，不发送');
    expect(pageSource).toContain('@click="copyResult"');
    expect(pageSource).not.toMatch(/发送弹幕|sendDanmaku|danmaku_send/);
  });

  it('uses focused config component boundaries', () => {
    const pageSource = source(page);
    for (const component of [
      'AiVoiceReplyControl',
      'AiModelProfilesDialog',
      'AiPersonaConfigDialog'
    ]) {
      expect(pageSource).toContain(component);
    }
    expect(pageSource).not.toMatch(/useMembershipStore|MembershipPausedNotice|memberCapability/);
    const config = source(persona);
    for (const style of ['concise', 'natural', 'detailed']) {
      expect(config).toContain(`value="${style}"`);
    }
    expect(config).toContain('draft.maxSentences');
    expect(config).not.toMatch(/localStorage|invokeDesktop/);
  });

  it('keeps model secrets and voice routing behind typed store actions', () => {
    const modelsSource = source(models);
    const voiceSource = source(voice);
    expect(modelsSource).toContain('apiKeyReplacement');
    expect(modelsSource).not.toMatch(/localStorage|invokeDesktop/);
    expect(modelsSource).not.toContain('代理 URL');
    expect(modelsSource).not.toContain('draft.proxyUrl');
    expect(modelsSource).toContain('proxyUrl: null');
    expect(modelsSource).toContain("t('ai.models.editor.reasoningEffort')");
    expect(modelsSource).not.toContain('label="温度"');
    expect(modelsSource).not.toContain('draft.temperature');
    for (const effort of ['none', 'minimal', 'low', 'medium', 'high', 'xhigh', 'max']) {
      expect(modelsSource).toContain(`value="${effort}"`);
    }
    for (const [key, sourcePath] of [
      ['ai.voice.title', voice],
      ['ai.voice.routeGuide', voice],
      ['ai.voice.testPlayback', voice],
      ['ai.queue.skip', voice],
      ['ai.queue.clear', voice]
    ] as const) {
      expect(source(sourcePath)).toContain(key);
    }
    expect(voiceSource).toContain("t('ai.voice.route.voice')");
    expect(voiceSource).toContain('voice.displayName');
    expect(voiceSource).toContain('voice.voiceId');
    expect(voiceSource).toContain('route.voiceId');
    expect(voiceSource).toContain('selectedVoiceId');
    expect(voiceSource).toContain('isDraftRouteAvailable');
    expect(voiceSource).toContain(':disabled="!isDraftRouteAvailable"');
    expect(voiceSource).toContain('snapshot?.voices');
    expect(voiceSource).toContain('grid gap-4 xl:grid-cols-[minmax(0,1.35fr)_minmax(0,1fr)]');
    expect(voiceSource).toContain('v-if="!defaultProfile"');
    expect(voiceSource).toContain('v-else-if="!selectedDevice"');
    expect(voiceSource).not.toMatch(/localStorage|invokeDesktop|发送弹幕/);
  });

  it('keeps the AI audit route read-only and free of ASR remnants', () => {
    const pageSource = source(page);
    const contract = source('src/renderer/platform/desktop/contracts/ai.ts');
    const log = source('src/renderer/views/ai/log.vue');
    expect(pageSource).not.toMatch(/asr|Asr/);
    expect(contract).not.toMatch(/asr_|AsrStatus|AsrModel/);
    expect(log).not.toMatch(/invokeDesktop|listenDesktop|localStorage|electronAPI/);
    expect(log).toMatch(/store\s*\.\s*loadLogs/);
  });

  it('requires the PRD 12.2 data disclosure before enabling voice reply', () => {
    const pageSource = source(page);
    // Enabling must ask first and only then pass the acknowledgement.
    expect(pageSource).toContain("t('ai.messages.dataDisclosure')");
    expect(pageSource).toContain('ElMessageBox.confirm');
    expect(pageSource).toContain('store.setVoiceReplyEnabled(isEnabled, confirmed, isEnabled)');

    for (const locale of ['zh-CN', 'en-US']) {
      const messages = JSON.parse(source(`src/renderer/locales/${locale}.json`)) as {
        ai: { messages: Record<string, string> };
      };
      for (const key of ['dataDisclosure', 'dataDisclosureTitle', 'dataDisclosureConfirm']) {
        expect(messages.ai.messages[key], `${locale}.ai.messages.${key}`).toBeTruthy();
      }
    }
  });

  it('shows the context that was actually sent with the generation', () => {
    const pageSource = source(page);
    // PRD 9.13 sketch: context, input and result are all visible locally.
    expect(pageSource).toContain('contextUsed');
    expect(pageSource).toContain("t('ai.result.context')");
    expect(pageSource).toContain('v-if="contextUsed.length > 0"');

    for (const locale of ['zh-CN', 'en-US']) {
      const messages = JSON.parse(source(`src/renderer/locales/${locale}.json`)) as {
        ai: { result: Record<string, string> };
      };
      for (const key of ['context', 'contextHint']) {
        expect(messages.ai.result[key], `${locale}.ai.result.${key}`).toBeTruthy();
      }
    }
  });

  it('renders the AI workspace on the shared design tokens only', () => {    for (const path of [page, voice, models, persona]) {
      const content = source(path);
      expect(content, path).not.toMatch(/dark:|bg-white|text-gray-|border-gray-/);
      const template = content.slice(content.indexOf('<template>'));
      expect(template, path).not.toMatch(/[\u4e00-\u9fff]/);
    }

    const surfaces = [page, voice, models, persona].map(source).join('\n');
    for (const token of [
      'app-card',
      'app-text',
      'app-muted',
      'app-faint',
      'app-btn-primary',
      'app-chip',
      'app-icon-btn'
    ]) {
      expect(surfaces).toContain(token);
    }

    // Every AI-page copy key is mirrored between Chinese and English.
    const zh = JSON.parse(source('src/renderer/locales/zh-CN.json')) as { ai: Record<string, unknown> };
    const en = JSON.parse(source('src/renderer/locales/en-US.json')) as { ai: Record<string, unknown> };
    expect(localeKeys(en.ai).sort()).toEqual(localeKeys(zh.ai).sort());
  });
});
