import { describe, expect, it } from 'vitest';
import { adaptUpdateSnapshot } from './adapters';

describe('release adapters', () => {
  it('accepts a strict unavailable update snapshot', () => {
    expect(adaptUpdateSnapshot({
      currentVersion: '0.1.0', channel: 'free', status: 'unavailable',
      candidate: null, progressPercent: null, errorCode: 'UPDATE_PROVIDER_UNAVAILABLE'
    }).status).toBe('unavailable');
  });

  it('rejects invalid update progress', () => {
    expect(() => adaptUpdateSnapshot({
      currentVersion: '0.1.0', channel: 'free', status: 'downloading',
      candidate: null, progressPercent: 101, errorCode: null
    })).toThrow();
  });
});
