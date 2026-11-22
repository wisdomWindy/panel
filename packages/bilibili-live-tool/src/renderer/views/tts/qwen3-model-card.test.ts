import { describe, expect, it } from 'vitest';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

const source = (path: string) =>
  readFileSync(fileURLToPath(new URL(path, import.meta.url)), 'utf8');

describe('speech model handling', () => {
  it('keeps the model download out of the UI, because it is automatic', () => {
    const ttsPage = source('./index.vue');

    // The runtime and the weights are prepared when 播报 is enabled or when the
    // stored configuration already has it on, so the page needs no card, button
    // or progress bar for them.
    expect(ttsPage).not.toContain('Qwen3ModelCard');
    expect(ttsPage).not.toContain('downloadQwen3Model');
    expect(ttsPage).not.toContain('tts_qwen3_download_model');
  });

  it('still reports preparation progress while it runs', () => {
    const ttsPage = source('./index.vue');

    expect(ttsPage).toContain('SpeechPrepareDialog');
    expect(ttsPage).toContain('ensureSpeechReady');
  });
});
