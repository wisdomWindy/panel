import { describe, expect, it } from 'vitest';
import {
  normalizeSensitiveWord,
  sensitiveWordError,
  userUidError,
  usernameError
} from './validation';

describe('live data form validation', () => {
  it('normalizes control characters before enforcing the sensitive-word boundary', () => {
    expect(normalizeSensitiveWord(' \n规则\u0000 ')).toBe('规则');
    expect(sensitiveWordError('\n')).not.toBe('');
    expect(sensitiveWordError('词'.repeat(100))).toBe('');
    expect(sensitiveWordError('词'.repeat(101))).not.toBe('');
  });

  it('accepts only numeric 1-20 digit UIDs and bounded usernames', () => {
    expect(userUidError('123')).toBe('');
    expect(userUidError('12a')).not.toBe('');
    expect(userUidError('1'.repeat(21))).not.toBe('');
    expect(usernameError('主播')).toBe('');
    expect(usernameError('')).not.toBe('');
  });
});
