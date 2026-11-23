import { describe, expect, it } from 'vitest';
import { retriesSpeechInstall } from './speech-retry';

describe('speech retry routing', () => {
  it('reinstalls the runtime for install and service failures', () => {
    for (const code of [
      'TTS_ENVIRONMENT_INCOMPLETE',
      'TTS_SERVICE_SCRIPT_MISSING',
      'TTS_SERVICE_PYTHON_MISSING',
      'TTS_SERVICE_START_FAILED',
      'qwen3_tts_unreachable'
    ]) {
      expect(retriesSpeechInstall(code)).toBe(true);
    }
  });

  it('falls back to a plain refresh for unrelated errors', () => {
    expect(retriesSpeechInstall('ROOM_OFFLINE')).toBe(false);
    expect(retriesSpeechInstall('RECORD_FFMPEG_MISSING')).toBe(false);
    expect(retriesSpeechInstall(null)).toBe(false);
    expect(retriesSpeechInstall(undefined)).toBe(false);
  });
});
