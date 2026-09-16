import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

describe('resolveBuildChannel', () => {
  beforeEach(() => {
    vi.resetModules();
    vi.stubEnv('VITE_BUILD_CHANNEL', 'free');
  });

  afterEach(() => {
    vi.unstubAllEnvs();
  });

  it('accepts the two compile channels', async () => {
    const { resolveBuildChannel } = await import('./build-channel');
    expect(resolveBuildChannel('free')).toBe('free');
    expect(resolveBuildChannel('member')).toBe('member');
  });

  it('rejects missing and runtime-defined channels', async () => {
    const { resolveBuildChannel } = await import('./build-channel');
    expect(() => resolveBuildChannel(undefined)).toThrow(
      'VITE_BUILD_CHANNEL must be either free or member'
    );
    expect(() => resolveBuildChannel('preview')).toThrow();
  });
});
