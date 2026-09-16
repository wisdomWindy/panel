import { describe, expect, it } from 'vitest';
import { formatBytes, formatClockTime, formatDuration, formatMoneyMinor } from './formatters';

describe('media formatters', () => {
  it('keeps money in integer minor units', () => {
    expect(formatMoneyMinor(1234)).toContain('12.34');
    expect(formatMoneyMinor(1.5)).toBe('--');
  });

  it('formats duration and binary bytes with explicit invalid fallbacks', () => {
    expect(formatDuration(5025)).toBe('01:23:45');
    expect(formatDuration(-1)).toBe('--');
    expect(formatBytes(2.3 * 1024 ** 3)).toBe('2.3 GiB');
    expect(formatBytes(-1)).toBe('--');
  });

  it('formats valid local clock time and rejects invalid epochs', () => {
    expect(formatClockTime(0)).not.toBe('--');
    expect(formatClockTime(Number.NaN)).toBe('--');
  });
});
