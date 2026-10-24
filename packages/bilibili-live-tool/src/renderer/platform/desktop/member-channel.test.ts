import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

const source = (path: string) => readFileSync(resolve(process.cwd(), path), 'utf8');

describe('member build channel boundary', () => {
  it('exposes shared AI routes in both channels without leaking membership into free', () => {
    const free = source('src/renderer/platform/desktop/channel.free.ts');
    const member = source('src/renderer/platform/desktop/channel.member.ts');
    const shared = source('src/renderer/platform/desktop/channel.shared.ts');
    expect(shared).toMatch(/path:\s*['"]\/ai['"]/);
    expect(shared).toMatch(/path:\s*['"]\/ai\/log['"]/);
    expect(free).toContain('routes: aiRoutes');
    expect(member).toContain('...aiRoutes');
    expect(free).not.toMatch(/useMembershipStore|useMemberRuntime|views\/membership|views\/moderation|\/membership|Membership/);
    expect(shared).not.toMatch(/memberCapability|useMembershipStore|useMemberRuntime/);
    expect(member).toContain('item(\'membership\', \'/membership\')');
    expect(member).toContain('navigationMessages');
    expect(member).toMatch(/buildNavigation:\s*\([^)]*common[^)]*\)\s*=>/);
  });

  it('selects one channel implementation at build time', () => {
    const vite = source('vite.config.ts');
    expect(vite).toContain('\'@member-channel\'');
    expect(vite).toContain('mode === \'member\'');
    expect(vite).toContain('channel.member.ts');
    expect(vite).toContain('channel.free.ts');
  });
});
