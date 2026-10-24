import { describe, expect, it } from 'vitest';
import { adaptMemberStatus, adaptModerationSummary } from './adapters';

describe('member adapters', () => {
  it('freezes a valid member projection without inventing optional data', () => {
    const status = adaptMemberStatus({
      shellState: 'active',
      account: {
        userUid: '123',
        username: '主播',
        avatarUrl: null,
        loggedInAt: 1
      },
      entitlement: {
        status: 'active',
        planId: 'annual',
        expiresAt: 2,
        remainingDays: 0,
        lastVerifiedAt: 1,
        isOfflineValid: true
      },
      capabilities: { moderation: 'missingModule' },
      serviceErrorCode: null
    });

    expect(status.shellState).toBe('active');
    expect(status.account?.userUid).toBe('123');
    expect(Object.isFrozen(status)).toBe(true);
    expect(Object.isFrozen(status.capabilities)).toBe(true);
  });

  it('rejects unknown enums, invalid progress, and unsafe moderation ranges', () => {
    expect(() =>
      adaptMemberStatus({
        shellState: 'paid',
        account: null,
        entitlement: null,
        capabilities: { moderation: 'ready' },
        serviceErrorCode: null
      })
    ).toThrow(TypeError);

    expect(() =>
      adaptModerationSummary({
        rules: {
          frequencyWindowSeconds: 10,
          frequencyThreshold: 5,
          repeatWindowSeconds: 60,
          repeatThreshold: 3,
          similarityWindowSeconds: 60,
          similarityThreshold: 5,
          similarityRatio: 1.1,
          defaultMuteDuration: '1h',
          isRiskConfirmed: false,
          isAutoEnabled: false
        },
        alerts: [],
        recognizedToday: 0,
        autoMutedToday: 0,
        manualMutedToday: 0,
        isCredentialAvailable: false
      })
    ).toThrow(TypeError);
  });

});
