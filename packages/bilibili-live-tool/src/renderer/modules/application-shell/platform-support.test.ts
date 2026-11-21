import { describe, expect, it } from 'vitest';
import {
  detectHostPlatform,
  isFullyVerifiedPlatform
} from './platform-support';

describe('platform support matrix', () => {
  it('recognises the three desktop targets from the user agent', () => {
    expect(
      detectHostPlatform(
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'
      )
    ).toBe('windows');
    expect(
      detectHostPlatform('Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)')
    ).toBe('macos');
    expect(detectHostPlatform('Mozilla/5.0 (X11; Linux x86_64)')).toBe('linux');
    expect(detectHostPlatform('something-else')).toBe('unknown');
  });

  it('only treats Windows as fully verified', () => {
    // PRD 13: other targets must not be presented as fully supported.
    expect(isFullyVerifiedPlatform('windows')).toBe(true);
    expect(isFullyVerifiedPlatform('macos')).toBe(false);
    expect(isFullyVerifiedPlatform('linux')).toBe(false);
    expect(isFullyVerifiedPlatform('unknown')).toBe(false);
  });
});
