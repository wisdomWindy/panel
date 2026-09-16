import { describe, expect, it } from 'vitest';
import { validateFileNameStem, validateHexColor, validateTtsTemplate, validateVolume } from './validation';

describe('media validation', () => {
  it('validates bounded volume and six-digit colors', () => {
    expect(validateVolume(0).valid).toBe(true);
    expect(validateVolume(1).valid).toBe(true);
    expect(validateVolume(Number.NaN).valid).toBe(false);
    expect(validateVolume(1.01).valid).toBe(false);
    expect(validateHexColor('#12aBcF').valid).toBe(true);
    expect(validateHexColor('#fff').valid).toBe(false);
  });

  it('allows only approved template variables and preserves meaningful spaces', () => {
    expect(validateTtsTemplate(' {用户}说：{内容} ', 'danmu').valid).toBe(true);
    expect(validateTtsTemplate('{用户}\n{内容}', 'danmu').valid).toBe(false);
    expect(validateTtsTemplate('{用户}{礼物}', 'danmu').valid).toBe(false);
    expect(validateTtsTemplate('   ', 'gift').valid).toBe(false);
  });

  it('rejects empty names and path separators', () => {
    expect(validateFileNameStem(' stream-01 ').valid).toBe(true);
    expect(validateFileNameStem('folder/file').valid).toBe(false);
    expect(validateFileNameStem('   ').valid).toBe(false);
  });
});
