import { describe, expect, it } from 'vitest';
import {
  MEDIA_COMMAND_NAMES,
  MEDIA_EVENT_NAMES,
  type CaptainEffectConfig,
  type CommandRequestMap,
  type EventPayloadMap,
  type RecordingStatus,
  type TtsStatus
} from './contracts';

describe('interaction media desktop contracts', () => {
  it('keeps the approved media surface closed', () => {
    expect(MEDIA_COMMAND_NAMES).toHaveLength(34);
    expect(MEDIA_COMMAND_NAMES[0]).toBe('captain_effect_get');
    expect(MEDIA_COMMAND_NAMES.at(-1)).toBe('highlight_delete');
    expect(MEDIA_EVENT_NAMES).toEqual([
      'captain_effect_trigger',
      'gift_alert_trigger',
      'gift_notify_updated',
      'tts_status_changed',
      'recording_status_changed',
      'recording_completed',
      'highlight_created'
    ]);
  });

  it('uses explicit units, nullable absence, and no authoritative paths', () => {
    const config: CaptainEffectConfig = {
      presetId: 'gold',
      durationSeconds: 5,
      position: 'center',
      primaryColor: '#FFD700',
      secondaryColor: '#FFFFFF',
      firstSound: null,
      renewalSound: null,
      comboSound: null,
      volume: 1,
      comboEnabled: true,
      comboWindowSeconds: 10
    };
    const request: CommandRequestMap['captain_effect_save'] = { config };
    const tts: TtsStatus = {
      state: 'idle',
      currentText: null,
      pendingCount: 0,
      activeEngine: 'system',
      fallbackReasonCode: null
    };
    const recording: RecordingStatus = {
      state: 'idle',
      sessionId: null,
      roomId: null,
      elapsedSeconds: 0,
      fileSizeBytes: 0,
      diskRemainingBytes: 0,
      highlightCount: 0,
      lastErrorCode: null
    };
    const event: EventPayloadMap['tts_status_changed'] = tts;

    expect(request.config.durationSeconds).toBe(5);
    expect(event.currentText).toBeNull();
    expect(JSON.stringify({ config, tts, recording })).not.toMatch(/rawPath|filePath/);
  });
});
