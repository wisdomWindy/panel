import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

const source = (path: string) => readFileSync(resolve(process.cwd(), path), 'utf8');

describe('member build channel boundary', () => {
  it('keeps the free channel empty and independent from member stores and routes', () => {
    const free = source('src/renderer/platform/desktop/channel.free.ts');
    const member = source('src/renderer/platform/desktop/channel.member.ts');
    expect(free).toContain('routes: Object.freeze([])');
    expect(free).not.toMatch(/useMembershipStore|useMemberRuntime|views\/membership|views\/ai|views\/moderation|\/membership|AI assistant|Membership/);
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
