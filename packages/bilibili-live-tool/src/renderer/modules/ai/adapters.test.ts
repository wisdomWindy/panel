import { describe, expect, it } from 'vitest';
import { adaptAiLogs, adaptAiModelProfiles, adaptAiVoiceReplySnapshot } from './adapters';

describe('AI adapters', () => {
  it('normalizes and freezes AI audit rows', () => {
    const logs = adaptAiLogs([
      {
        id: 1,
        operationType: 'generate',
        username: null,
        userUid: null,
        originalContent: '问候',
        aiContent: '你好',
        generationStatus: 'completed',
        rejectReason: null,
        roomId: '123',
        createdAt: 1
      }
    ]);
    expect(logs[0]?.aiContent).toBe('你好');
    expect(Object.isFrozen(logs)).toBe(true);
    expect(() => adaptAiLogs([{ id: '1' }])).toThrow(TypeError);
  });

  it('validates model profiles and voice queue snapshots', () => {
    const profiles = adaptAiModelProfiles([{
      profileId: 'profile-1',
      name: 'Local',
      providerKind: 'local',
      baseUrl: 'http://127.0.0.1:11434/v1',
      model: 'qwen',
      reasoningEffort: 'medium',
      maxTokens: 512,
      timeoutSeconds: 30,
      proxyUrl: null,
      isEnabled: true,
      isDefault: true,
      connectionState: 'available',
      isApiKeyConfigured: false,
      updatedAt: 1
    }]);
    const snapshot = adaptAiVoiceReplySnapshot({
      isEnabled: false,
      defaultProfileId: 'profile-1',
      rules: {
        triggerScope: 'questions',
        keywords: [],
        globalCooldownSeconds: 5,
        userCooldownSeconds: 10,
        queueCapacity: 20,
        isDuplicateFilterEnabled: true
      },
      devices: [{
        deviceId: 'device-1',
        displayName: 'Virtual cable',
        isAvailable: true,
        isDefault: false
      }],
      voices: [{
        voiceId: 'gentle-peach',
        displayName: '温柔桃子',
        isAvailable: true,
        isDefault: true
      }],
      queue: {
        state: 'idle',
        current: null,
        pending: [],
        capacity: 20,
        volume: 0.8,
        selectedDeviceId: 'device-1',
        selectedVoiceId: 'gentle-peach',
        isAudioRouteConfirmed: true,
        lastErrorCode: null
      }
    });
    expect(profiles[0]?.isDefault).toBe(true);
    expect(profiles[0]?.reasoningEffort).toBe('medium');
    expect(() => adaptAiModelProfiles([{ ...profiles[0], reasoningEffort: 'media' }]))
      .toThrow(TypeError);
    expect(snapshot.queue.selectedDeviceId).toBe('device-1');
    expect(snapshot.queue.selectedVoiceId).toBe('gentle-peach');
    expect(snapshot.voices[0]?.displayName).toBe('温柔桃子');
    expect(Object.isFrozen(snapshot.voices)).toBe(true);
    expect(Object.isFrozen(snapshot.queue)).toBe(true);
  });
});
