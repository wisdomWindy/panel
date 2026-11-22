import { describe, expect, it } from 'vitest';
import { THIRD_PARTY_NOTICES, mergeLicenses } from './third-party-notices';

describe('third-party notices', () => {
  it('credits the models and runtimes the app actually uses', () => {
    const names = THIRD_PARTY_NOTICES.map(entry => entry.name).join('|');
    expect(names).toContain('Qwen3-TTS');
    expect(names).toContain('FFmpeg');
    expect(names).toContain('Tauri');
    expect(names).toContain('Element Plus');

    const qwen = THIRD_PARTY_NOTICES.find(entry => entry.name.includes('Qwen3-TTS'));
    expect(qwen?.license).toBe('Apache-2.0');
    const ffmpeg = THIRD_PARTY_NOTICES.find(entry => entry.name.includes('FFmpeg'));
    expect(ffmpeg?.license).toContain('LGPL');
  });

  it('prefers release-provided entries and skips duplicates', () => {
    const merged = mergeLicenses([
      { name: 'Qwen3-TTS（语音合成模型，权重由用户自行下载）', license: 'Apache-2.0', sourceUrl: 'x' },
      { name: 'Custom Runtime', license: 'MIT', sourceUrl: 'y' }
    ]);
    expect(merged[0].name).toBe('Qwen3-TTS（语音合成模型，权重由用户自行下载）');
    expect(merged.filter(entry => entry.name.includes('Qwen3-TTS'))).toHaveLength(1);
    expect(merged.some(entry => entry.name === 'Custom Runtime')).toBe(true);
  });

  it('always has an entry for every notice once merged with nothing', () => {
    expect(mergeLicenses([])).toHaveLength(THIRD_PARTY_NOTICES.length);
  });
});
