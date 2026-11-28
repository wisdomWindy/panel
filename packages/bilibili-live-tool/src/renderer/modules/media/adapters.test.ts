import { describe, expect, it } from 'vitest';
import type {
  CaptainEffectPayload,
  CaptainEffectSnapshot,
  GiftNotifySnapshot,
  RecordingConfig,
  TtsSnapshot
} from '@/platform/desktop';
import {
  adaptCaptainEffectPayloadOrNull,
  adaptCaptainEffectSnapshot,
  adaptGiftNotifySnapshot,
  adaptRecordingConfig,
  adaptTtsSnapshot,
  MalformedMediaPayloadError
} from './adapters';

const captainSnapshot = (): CaptainEffectSnapshot => ({
  config: {
    presetId: 'gold',
    durationSeconds: 5,
    position: 'topRight',
    windowWidth: 560,
    windowHeight: 280,
    primaryColor: '#FFD700',
    secondaryColor: '#FFA500',
    firstSound: null,
    renewalSound: null,
    comboSound: null,
    volume: 0.7,
    comboEnabled: true,
    comboWindowSeconds: 10
  },
  presets: [
    {
      presetId: 'gold',
      displayName: 'Gold',
      primaryColor: '#FFD700',
      secondaryColor: '#FFA500'
    }
  ],
  updatedAt: 1
});

describe('media adapters', () => {
  it('preserves the nullable gift image URL at the desktop boundary', () => {
    const base = {
      kind: 'gift',
      eventId: 'event-1',
      roomId: '100',
      occurredAt: 1,
      audience: {
        userUid: '2',
        username: 'viewer',
        userLevel: null,
        medal: null,
        tags: []
      },
      giftId: 'gift-1',
      giftName: '小花花',
      quantity: 1,
      valueMinor: 100
    };
    const snapshot = {
      config: {
        isEnabled: true,
        highValueThresholdMinor: 100,
        methods: { systemNotification: true, localPopup: true, sound: true },
        comboEnabled: true,
        comboWindowSeconds: 10,
        popupPosition: 'center',
        popupDurationSeconds: 5
      },
      totalValueMinor: 100,
      donorCount: 1,
      giftCount: 1,
      latestGifts: [{ ...base, giftIconUrl: 'https://i0.hdslb.com/bfs/live/gift.png' }],
      updatedAt: 1
    };

    expect(adaptGiftNotifySnapshot(snapshot).latestGifts[0]?.giftIconUrl).toBe(
      'https://i0.hdslb.com/bfs/live/gift.png'
    );
    expect(
      adaptGiftNotifySnapshot({
        ...snapshot,
        latestGifts: [{ ...base, giftIconUrl: null }]
      }).latestGifts[0]?.giftIconUrl
    ).toBeNull();
  });

  it('preserves a missing active captain effect and validates a present payload', () => {
    const payload: CaptainEffectPayload = {
      eventId: 'preview-active' as never,
      audience: {
        userUid: 'preview-user' as never,
        username: '用户A',
        userLevel: null,
        medal: null,
        tags: ['captain']
      },
      action: 'purchase',
      level: 'captain',
      config: captainSnapshot().config,
      isPreview: true
    };

    expect(adaptCaptainEffectPayloadOrNull(null)).toBeNull();
    expect(adaptCaptainEffectPayloadOrNull(payload)).toEqual(payload);
    expect(() => adaptCaptainEffectPayloadOrNull({ ...payload, action: 'unknown' })).toThrow(
      MalformedMediaPayloadError
    );
  });

  it('returns a frozen captain snapshot without sharing nested data', () => {
    const source = captainSnapshot();
    const result = adaptCaptainEffectSnapshot(source);
    expect(result).toEqual(source);
    expect(result).not.toBe(source);
    expect(result.config).not.toBe(source.config);
    expect(Object.isFrozen(result)).toBe(true);
    expect(Object.isFrozen(result.presets)).toBe(true);
  });

  it('rejects captain effect window sizes outside the supported range', () => {
    expect(() =>
      adaptCaptainEffectSnapshot({
        ...captainSnapshot(),
        config: { ...captainSnapshot().config, windowWidth: 319 }
      })
    ).toThrow(MalformedMediaPayloadError);
    expect(() =>
      adaptCaptainEffectSnapshot({
        ...captainSnapshot(),
        config: { ...captainSnapshot().config, windowHeight: 1081 }
      })
    ).toThrow(MalformedMediaPayloadError);
  });

  it('rejects malformed captain preset colors at the desktop boundary', () => {
    expect(() =>
      adaptCaptainEffectSnapshot({
        ...captainSnapshot(),
        presets: [
          {
            ...captainSnapshot().presets[0],
            primaryColor: 'gold'
          }
        ]
      })
    ).toThrow(MalformedMediaPayloadError);
  });

  it('rejects malformed nested gift DTOs and keeps room IDs closed', () => {
    const value: GiftNotifySnapshot = {
      config: {
        isEnabled: true,
        highValueThresholdMinor: 100,
        methods: { systemNotification: true, localPopup: true, sound: false },
        comboEnabled: true,
        comboWindowSeconds: 10,
        popupPosition: 'topRight',
        popupDurationSeconds: 5
      },
      totalValueMinor: 100,
      donorCount: 1,
      giftCount: 1,
      latestGifts: [
        {
          kind: 'gift',
          eventId: 'event-1' as never,
          roomId: '' as never,
          occurredAt: 1,
          audience: {
            userUid: '2' as never,
            username: 'viewer',
            userLevel: null,
            medal: null,
            tags: []
          },
          giftId: 'gift-1' as never,
          giftName: 'gift',
          giftIconUrl: null,
          quantity: 1,
          valueMinor: 100
        }
      ],
      updatedAt: 1
    };
    expect(() => adaptGiftNotifySnapshot(value)).toThrow(MalformedMediaPayloadError);
  });

  it('rejects unknown TTS and recording enums', () => {
    const tts = {
      config: {
        isEnabled: true,
        selectedEngine: 'browser',
        selectedVoiceId: null,
        rate: 1,
        volume: 0.8,
        eventKinds: ['danmu'],
        identityKinds: ['ordinary'],
        templates: {
          danmu: '{用户}说：{内容}',
          gift: '{用户}送出{礼物}×{数量}',
          captain: '{用户}开通{等级}'
        }
      },
      voices: [],
      status: {
        state: 'idle',
        currentText: null,
        pendingCount: 0,
        activeEngine: 'system',
        fallbackReasonCode: null
      },
      updatedAt: 1
    } as unknown as TtsSnapshot;
    expect(() => adaptTtsSnapshot(tts)).toThrow(MalformedMediaPayloadError);

    const recording = {
      format: 'avi',
      quality: 'original',
      directory: null,
      segmentationEnabled: false,
      segmentDurationSeconds: 1800,
      autoRecord: false,
      danmakuThreshold: 10,
      highlightBeforeSeconds: 30,
      highlightAfterSeconds: 30,
      quotaBytes: 1,
      waterlineBytes: 1,
      ffmpegSource: 'bundled',
      shortcut: 'F9'
    } as unknown as RecordingConfig;
    expect(() => adaptRecordingConfig(recording)).toThrow(MalformedMediaPayloadError);
  });

  it('accepts every FFmpeg source the settings dialog can persist', () => {
    const base = {
      format: 'mp4',
      quality: 'original',
      directory: null,
      segmentationEnabled: false,
      segmentDurationSeconds: 1800,
      autoRecord: false,
      danmakuThreshold: 10,
      highlightBeforeSeconds: 30,
      highlightAfterSeconds: 30,
      quotaBytes: 1,
      waterlineBytes: 1,
      shortcut: 'F9'
    };

    for (const ffmpegSource of ['bundled', 'system', 'installed', 'local'] as const) {
      const adapted = adaptRecordingConfig({ ...base, ffmpegSource });
      expect(adapted.ffmpegSource).toBe(ffmpegSource);
    }
  });
});
