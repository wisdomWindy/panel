import { describe, expect, it } from 'vitest';
import { errorKey, isUserCancellation } from './error-text';

describe('error text mapping', () => {
  it('maps known backend codes to translation keys', () => {
    expect(errorKey('RECORD_FFMPEG_MISSING')).toBe('errors.RECORD_FFMPEG_MISSING');
    expect(errorKey('RECORD_DIRECTORY_REQUIRED')).toBe('errors.RECORD_DIRECTORY_REQUIRED');
    expect(errorKey('STORAGE_UNAVAILABLE')).toBe('errors.STORAGE_UNAVAILABLE');
  });

  it('leaves unknown codes to the backend message', () => {
    expect(errorKey('SOME_NEW_CODE')).toBeNull();
    expect(errorKey(null)).toBeNull();
  });

  it('treats a dismissed dialog as a cancellation, not an error', () => {
    expect(isUserCancellation('SYS_DIALOG_CANCELLED')).toBe(true);
    expect(isUserCancellation('RECORD_FFMPEG_MISSING')).toBe(false);
  });
});
