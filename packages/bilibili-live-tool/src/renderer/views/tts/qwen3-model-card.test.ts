import { describe, expect, it } from 'vitest';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

const source = (path: string) =>
  readFileSync(fileURLToPath(new URL(path, import.meta.url)), 'utf8');

describe('qwen3 model card', () => {
  it('lives next to the speech settings instead of the dashboard', () => {
    const ttsPage = source('../tts/index.vue');
    const dashboard = source('../dashboard/index.vue');

    expect(ttsPage).toContain('Qwen3ModelCard');
    expect(dashboard).not.toContain('qwen3');
  });

  it('downloads through the store and reports progress', () => {
    const card = source('../../components/media/Qwen3ModelCard.vue');

    expect(card).toContain("t('tts.qwen3Model.title')");
    expect(card).toContain('@click="downloadModel"');
    expect(card).toContain('media.downloadQwen3Model');
    expect(card).toContain('media.loadQwen3Models');
    expect(card).toContain('tts.qwen3Model.serviceDown');
    expect(card).toContain('<el-progress');
    expect(card).not.toMatch(/invokeDesktop|listenDesktop/);
  });

  it('keeps the default model id in the service catalogue, not the view', () => {
    const card = source('../../components/media/Qwen3ModelCard.vue');

    expect(card).not.toContain('Qwen/Qwen3-TTS');
    expect(card).toContain('primaryModel');
  });
});
