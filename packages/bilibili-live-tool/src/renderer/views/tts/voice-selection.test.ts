import { describe, expect, it } from 'vitest';
import type { TtsVoice } from '../../platform/desktop/contracts';
import { resolvePreferredVoiceId } from './voice-selection';

const voices: readonly TtsVoice[] = [
  {
    voiceId: 'system-zh-huihui',
    displayName: 'Microsoft HuiHui Desktop - Chinese (Simplified)',
    engine: 'system',
    isAvailable: true,
    unavailableReasonCode: null
  },
  {
    voiceId: 'system-en-zira',
    displayName: 'Microsoft Zira Desktop - English (United States)',
    engine: 'system',
    isAvailable: true,
    unavailableReasonCode: null
  }
];

describe('TTS preferred voice selection', () => {
  it('selects the available Microsoft HuiHui voice for an unconfigured system engine', () => {
    expect(resolvePreferredVoiceId('system', null, voices)).toBe('system-zh-huihui');
  });

  it('preserves an existing voice and does not select HuiHui for cloud engines', () => {
    expect(resolvePreferredVoiceId('system', 'saved-voice', voices)).toBe('saved-voice');
    expect(resolvePreferredVoiceId('cloud', null, voices)).toBeNull();
  });

  it('ignores unavailable HuiHui voices', () => {
    expect(
      resolvePreferredVoiceId('system', null, [
        { ...voices[0], isAvailable: false, unavailableReasonCode: 'voice_unavailable' }
      ])
    ).toBeNull();
  });
});
