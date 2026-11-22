import { describe, expect, it } from 'vitest';
import { QWEN3_VOICE_PRESETS, mergeVoiceOptions } from './qwen3-voices';

describe('qwen3 voice presets', () => {
  it('offers the upstream CustomVoice speakers when the service is unreachable', () => {
    const options = mergeVoiceOptions([]);
    expect(options).toHaveLength(QWEN3_VOICE_PRESETS.length);
    expect(options.map(option => option.value)).toContain('Serena');
    expect(options.map(option => option.value)).toContain('Vivian');
    expect(options[0].label).toContain('女声');
  });

  it('prefers live service ids and appends the presets', () => {
    const options = mergeVoiceOptions(['Custom_1', 'Serena']);
    expect(options[0]).toEqual({ value: 'Custom_1', label: 'Custom_1' });
    expect(options.filter(option => option.value === 'Serena')).toHaveLength(1);
    expect(options.some(option => option.value === 'Vivian')).toBe(true);
  });

  it('ignores duplicated and blank service ids', () => {
    const options = mergeVoiceOptions(['', 'Serena', 'Serena', '  ']);
    expect(options.filter(option => option.value === 'Serena')).toHaveLength(1);
  });
});
