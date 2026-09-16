import { describe, expect, it } from 'vitest';
import { adaptPluginSnapshot, adaptUpdateSnapshot } from './adapters';

describe('release adapters', () => {
  it('accepts a strict unavailable update snapshot', () => {
    expect(adaptUpdateSnapshot({
      currentVersion: '0.1.0', channel: 'free', status: 'unavailable',
      candidate: null, progressPercent: null, errorCode: 'update_provider_unavailable'
    }).status).toBe('unavailable');
  });

  it('rejects invalid update progress', () => {
    expect(() => adaptUpdateSnapshot({
      currentVersion: '0.1.0', channel: 'free', status: 'downloading',
      candidate: null, progressPercent: 101, errorCode: null
    })).toThrow();
  });

  it('rejects unknown plugin permissions', () => {
    expect(() => adaptPluginSnapshot({
      id: 'demo', name: 'Demo', version: '1.0.0', status: 'enabled',
      permissions: ['filesystem'], errorCode: null
    })).toThrow();
  });
});
