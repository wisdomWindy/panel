import { describe, expect, it } from 'vitest';
import { MODEL_PRESETS, mergeModelOptions, normalizeCustomModels } from './model-presets';

describe('ai model presets', () => {
  it('offers mainstream domestic and international models', () => {
    const values = MODEL_PRESETS.map(preset => preset.value);
    for (const expected of ['gpt-4o', 'deepseek-chat', 'qwen-plus', 'glm-4-plus', 'doubao-pro-32k']) {
      expect(values).toContain(expected);
    }
  });

  it('puts the operator ids first and skips presets they already added', () => {
    const options = mergeModelOptions(['my-local-model', 'deepseek-chat']);
    expect(options[0]).toEqual({ value: 'my-local-model', label: 'my-local-model', vendor: '' });
    // deepseek-chat appears once, as the operator's entry.
    expect(options.filter(option => option.value === 'deepseek-chat')).toHaveLength(1);
  });

  it('normalises a stored list by trimming and de-duplicating', () => {
    expect(normalizeCustomModels(['  a  ', 'a', '', 42, 'b'])).toEqual(['a', 'b']);
  });
});
