import { describe, expect, it } from 'vitest';
import { MEMBER_COMMAND_NAMES } from './contracts/member';

describe('member desktop contract', () => {
  it('exposes only the approved command surface', () => {
    expect(MEMBER_COMMAND_NAMES).toHaveLength(28);
    expect(MEMBER_COMMAND_NAMES[0]).toBe('member_status');
    expect(MEMBER_COMMAND_NAMES.at(-1)).toBe('moderate_mark_false_positive');
    expect(MEMBER_COMMAND_NAMES.some(command => command.includes('send'))).toBe(false);
  });

  it('does not duplicate a command name', () => {
    expect(new Set(MEMBER_COMMAND_NAMES).size).toBe(MEMBER_COMMAND_NAMES.length);
  });
});
