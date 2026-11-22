import { describe, expect, it } from 'vitest';
import { adaptQwen3TtsSnapshot } from './adapters';

const snapshot = (instruct: unknown) => ({
  config: {
    isEnabled: true,
    baseUrl: 'http://127.0.0.1:8020',
    voiceId: null,
    language: 'Chinese',
    instruct,
    timeoutMs: 8000,
    fallbackToSystem: true,
    aiVoiceId: null,
    aiInstruct: null
  },
  health: null,
  voiceIds: []
});

describe('qwen3 snapshot adaptation', () => {
  it('accepts an empty instruction, which is the default config', () => {
    // Regression: the page failed on open because "" was validated as malformed.
    expect(adaptQwen3TtsSnapshot(snapshot('')).config.instruct).toBe('');
  });

  it('normalises a missing or null instruction to an empty string', () => {
    // Older records may carry an explicit null.
    expect(adaptQwen3TtsSnapshot(snapshot(null)).config.instruct).toBe('');
    expect(adaptQwen3TtsSnapshot(snapshot(undefined)).config.instruct).toBe('');
  });

  it('keeps a real instruction', () => {
    expect(adaptQwen3TtsSnapshot(snapshot('speak softly')).config.instruct).toBe('speak softly');
  });
});
