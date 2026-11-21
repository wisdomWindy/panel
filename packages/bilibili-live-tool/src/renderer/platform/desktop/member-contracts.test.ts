import { describe, expect, it } from 'vitest';
import { MEMBER_COMMAND_NAMES } from './contracts/member';
import { AI_COMMAND_NAMES } from './contracts/ai';

describe('member desktop contract', () => {
  it('exposes only the approved command surface', () => {
    expect(MEMBER_COMMAND_NAMES).toHaveLength(19);
    expect(MEMBER_COMMAND_NAMES[0]).toBe('member_status');
    expect(MEMBER_COMMAND_NAMES.at(-1)).toBe('moderate_mark_false_positive');
    expect(MEMBER_COMMAND_NAMES.some(command => command.includes('send'))).toBe(false);
  });

  it('keeps the shared AI command surface separate from membership', () => {
    expect(AI_COMMAND_NAMES).toHaveLength(19);
    expect(AI_COMMAND_NAMES[0]).toBe('ai_config_get');
    expect(AI_COMMAND_NAMES.at(-1)).toBe('ai_log_query');
    for (const command of [
      'ai_model_profile_save',
      'ai_voice_reply_set_enabled',
      'ai_tts_set_output_device',
      'ai_tts_clear_queue'
    ]) {
      expect(AI_COMMAND_NAMES).toContain(command);
    }
    expect(AI_COMMAND_NAMES.some(command => command.includes('asr'))).toBe(false);
    expect(AI_COMMAND_NAMES.some(command => command.includes('send'))).toBe(false);
  });

  it('does not duplicate a command name', () => {
    expect(new Set(MEMBER_COMMAND_NAMES).size).toBe(MEMBER_COMMAND_NAMES.length);
  });
});
