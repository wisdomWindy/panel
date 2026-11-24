import { describe, expect, it } from 'vitest';
import {
  MEDIA_COMMAND_NAMES,
  MEDIA_EVENT_NAMES,
  type CaptainEffectConfig,
  type CommandRequestMap,
  type CommandResultMap,
  type EventPayloadMap,
  type RecordingStatus,
  type TtsStatus
} from './contracts';

describe('interaction media desktop contracts', () => {
  it('keeps the approved media surface closed', () => {
    expect(MEDIA_COMMAND_NAMES).toHaveLength(48);
    expect(MEDIA_COMMAND_NAMES[0]).toBe('captain_effect_get');
    expect(MEDIA_COMMAND_NAMES[1]).toBe('captain_effect_current');
    expect(MEDIA_COMMAND_NAMES[20]).toBe('ffmpeg_install');
    expect(MEDIA_COMMAND_NAMES[36]).toBe('highlight_delete');
    expect(MEDIA_COMMAND_NAMES[41]).toBe('tts_qwen3_environment_status');
    expect(MEDIA_COMMAND_NAMES[43]).toBe('tts_qwen3_start');
    expect(MEDIA_COMMAND_NAMES.at(-1)).toBe('media_cleanup');
    expect(MEDIA_COMMAND_NAMES.filter(name => name.startsWith('tts_qwen3_'))).toEqual([
      'tts_qwen3_get',
      'tts_qwen3_save',
      'tts_qwen3_test',
      'tts_qwen3_models',
      'tts_qwen3_environment_status',
      'tts_qwen3_prepare_environment',
      'tts_qwen3_start',
      'tts_qwen3_download_model'
    ]);
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
      windowWidth: 560,
      windowHeight: 280,
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
    const current: CommandResultMap['captain_effect_current'] = null;
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
      lastErrorCode: null,
      tasks: []
    };
    const event: EventPayloadMap['tts_status_changed'] = tts;

    expect(request.config.durationSeconds).toBe(5);
    expect(current).toBeNull();
    expect(request.config.windowWidth).toBe(560);
    expect(request.config.windowHeight).toBe(280);
    expect(event.currentText).toBeNull();
    expect(JSON.stringify({ config, tts, recording })).not.toMatch(/rawPath|filePath/);
  });
});
