import { DanmakuParser, ParsedDanmaku } from '../src/renderer/modules/danmaku/parser';
import { DanmakuMessage } from '../src/renderer/modules/bilibili/types';

let p = 0, f = 0;
function ok(c: boolean, l: string) { if (c) p++; else f++; console.log('  ' + (c ? '✓' : '✗ FAIL') + ': ' + l); }
function eq(a: any, e: any, l: string) { const m = JSON.stringify(a) === JSON.stringify(e); if (m) p++; else f++; console.log('  ' + (m ? '✓' : '✗ FAIL') + ': ' + l + (m ? '' : '\n    exp: ' + JSON.stringify(e) + '\n    got: ' + JSON.stringify(a))); }

// Build a DanmakuMessage with the correct info array layout.
function makeMsg(opts: any = {}): DanmakuMessage {
  const ts = opts.timestamp ?? 1700000000000;
  const uid = opts.uid ?? 12345;
  const info: any[] = new Array(13).fill(null);
  info[0] = [0, 0, opts.roomId ?? 1000, 0, ts, 0, 0, '0', 0, 0, null, null, null, null, null];
  info[1] = opts.content ?? 'test';
  info[2] = [uid, opts.username ?? 'testuser', opts.isVip ?? 0, opts.isSvip ?? 0, 0, 0, 0, 0, null, null, null, null, null];
  info[3] = opts.medal !== undefined ? opts.medal : null;
  info[4] = opts.badge !== undefined ? opts.badge : null;
  info[7] = opts.perms !== undefined ? opts.perms : null;
  return { cmd: 'DANMU_MSG', info } as unknown as DanmakuMessage;
}

// Build a minimal ParsedDanmaku directly for filter/statistics tests.
function pd(overrides: any = {}): ParsedDanmaku {
  return {
    id: overrides.id ?? '1',
    userId: overrides.userId ?? '1',
    username: overrides.username ?? 'u',
    content: overrides.content ?? '',
    timestamp: overrides.timestamp ?? 0,
    userType: overrides.userType ?? 'normal',
    badge: overrides.badge,
    medal: overrides.medal,
    isVip: overrides.isVip ?? false,
    isSvip: overrides.isSvip ?? false,
    isCaptain: overrides.isCaptain ?? false,
    isAdmin: overrides.isAdmin ?? false,
    isOwner: overrides.isOwner ?? false,
  };
}

// ------- parse tests -------

console.log('=== parse: normal user ===');
{
  const msg = makeMsg({
    content: 'hello', username: 'alice', uid: 100, isVip: 1, isSvip: 0,
    badge: [10, 'badgeA'],
    medal: [5, 'fanA', 15, 0xffffff]
  });
  const r = DanmakuParser.parse(msg, 1000, 999);
  eq(r.content, 'hello', 'content');
  eq(r.username, 'alice', 'username');
  eq(r.userId, '100', 'userId');
  eq(r.userType, 'normal', 'userType normal');
  ok(r.isVip, 'isVip true');
  ok(!r.isSvip, 'isSvip false');
  ok(!r.isCaptain, 'isCaptain false');
  ok(!r.isAdmin, 'isAdmin false');
  ok(!r.isOwner, 'isOwner false');
  eq(r.timestamp, 1700000000000, 'timestamp');
  eq(r.badge, { level: 10, name: 'badgeA' }, 'badge');
  eq(r.medal, { level: 5, name: 'fanA', color: '#00000f' }, 'medal');
  eq(r.roomRank, 5, 'roomRank');
}

console.log('=== parse: captain (medal[2] >= 20) ===');
{
  const msg = makeMsg({ uid: 200, username: 'captainX',
    medal: [21, 'captMedal', 25, 0xff0000]
  });
  const r = DanmakuParser.parse(msg, 1000, 999);
  eq(r.userType, 'captain', 'userType captain');
  ok(!r.isCaptain, 'isCaptain flag is always false in parse');
  eq(r.medal, { level: 21, name: 'captMedal', color: '#000019' }, 'medal');
}

console.log('=== parse: admin (perms at info[7] contains 1) ===');
{
  const msg = makeMsg({ uid: 300, username: 'adminUser', perms: [0, 1, 0] });
  const r = DanmakuParser.parse(msg, 1000, 999);
  eq(r.userType, 'admin', 'userType admin');
  ok(!r.isOwner, 'not owner');
}

console.log('=== parse: admin (perms contains 2) ===');
{
  const msg = makeMsg({ uid: 301, username: 'admin2', perms: [2] });
  const r = DanmakuParser.parse(msg, 1000, 999);
  eq(r.userType, 'admin', 'userType admin (perm=2)');
}

console.log('=== parse: owner ===');
{
  const msg = makeMsg({ uid: 42, username: 'theOwner' });
  const r = DanmakuParser.parse(msg, 1000, 42);
  eq(r.userType, 'owner', 'userType owner');
  ok(r.isOwner, 'isOwner true');
  eq(r.username, 'theOwner', 'username');
}

console.log('=== parse: owner takes priority over admin ===');
{
  const msg = makeMsg({ uid: 77, username: 'ownerAdmin', perms: [1] });
  const r = DanmakuParser.parse(msg, 1000, 77);
  eq(r.userType, 'owner', 'owner overrides admin');
}

console.log('=== parse: admin takes priority over captain ===');
{
  const msg = makeMsg({ uid: 400, username: 'adminCap',
    medal: [21, 'capMedal', 25, 0xff0000],
    perms: [1]
  });
  const r = DanmakuParser.parse(msg, 1000, 999);
  eq(r.userType, 'admin', 'admin overrides captain');
}

console.log('=== parse: no medal/badge -> undefined extras ===');
{
  const msg = makeMsg({ content: 'bare' });
  const r = DanmakuParser.parse(msg, 1000, 999);
  eq(r.medal, undefined, 'medal undefined');
  eq(r.roomRank, undefined, 'roomRank undefined');
  eq(r.badge, undefined, 'badge undefined');
}

console.log('=== parse: empty medal array -> no roomRank, no medal ===');
{
  const msg = makeMsg({ medal: [] });
  const r = DanmakuParser.parse(msg, 1000, 999);
  eq(r.medal, undefined, 'medal undefined');
  eq(r.roomRank, undefined, 'roomRank undefined');
}

console.log('=== parse: isVip/isSvip flags ===');
{
  const msg = makeMsg({ isVip: 1, isSvip: 1 });
  const r = DanmakuParser.parse(msg, 1000, 999);
  ok(r.isVip, 'isVip true');
  ok(r.isSvip, 'isSvip true');
}

console.log('=== parse: medal length 3 -> captain via [2]>=20, no medal object ===');
{
  const msg = makeMsg({ uid: 500, username: 'cap3', medal: [30, 'capMedal', 30] });
  const r = DanmakuParser.parse(msg, 1000, 999);
  eq(r.userType, 'captain', 'captain via 3-element medal');
  eq(r.medal, undefined, 'medal undefined (needs 4 elems)');
  eq(r.roomRank, 30, 'roomRank from medal[0]');
}

// ------- parseBatch tests -------

console.log('=== parseBatch ===');
{
  const msgs = [
    makeMsg({ uid: 1, content: 'a' }),
    makeMsg({ uid: 2, content: 'b', perms: [1] }),
    makeMsg({ uid: 42, content: 'c' })
  ];
  const res = DanmakuParser.parseBatch(msgs, 1000, 42);
  eq(res.length, 3, 'batch length');
  eq(res[0].userType, 'normal', 'batch[0] normal');
  eq(res[1].userType, 'admin', 'batch[1] admin');
  eq(res[2].userType, 'owner', 'batch[2] owner');
  eq(res[2].content, 'c', 'batch[2] content');
}

// ------- filter tests -------

console.log('=== filter: userType ===');
{
  const danmakus = [
    pd({ id: '1', content: 'a', userType: 'normal' }),
    pd({ id: '2', content: 'b', userType: 'admin' }),
    pd({ id: '3', content: 'c', userType: 'owner' }),
    pd({ id: '4', content: 'd', userType: 'captain' }),
  ];
  const r = DanmakuParser.filter(danmakus, { userType: ['admin', 'owner'] });
  eq(r.length, 2, 'filter admin+owner count');
  eq(r.map(d => d.content), ['b', 'c'], 'filter admin+owner content');
}

console.log('=== filter: keywords ===');
{
  const danmakus = [
    pd({ id: '1', content: 'hello world' }),
    pd({ id: '2', content: 'foo bar' }),
    pd({ id: '3', content: 'hello foo' }),
  ];
  const r = DanmakuParser.filter(danmakus, { keywords: ['hello'] });
  eq(r.length, 2, 'keyword hello count');
  eq(r.map(d => d.content), ['hello world', 'hello foo'], 'keyword hello content');
}

console.log('=== filter: keywords empty array returns all ===');
{
  const danmakus = [pd({ id: '1', content: 'a' }), pd({ id: '2', content: 'b' })];
  const r = DanmakuParser.filter(danmakus, { keywords: [] });
  eq(r.length, 2, 'empty keywords returns all');
}

console.log('=== filter: excludeKeywords ===');
{
  const danmakus = [
    pd({ id: '1', content: 'hello spam' }),
    pd({ id: '2', content: 'hello world' }),
  ];
  const r = DanmakuParser.filter(danmakus, { excludeKeywords: ['spam'] });
  eq(r.length, 1, 'exclude spam count');
  eq(r[0].content, 'hello world', 'exclude spam remaining');
}

console.log('=== filter: excludeKeywords empty array returns all ===');
{
  const danmakus = [pd({ id: '1', content: 'a' }), pd({ id: '2', content: 'b' })];
  const r = DanmakuParser.filter(danmakus, { excludeKeywords: [] });
  eq(r.length, 2, 'empty exclude returns all');
}

console.log('=== filter: minLevel (badge level) ===');
{
  const danmakus = [
    pd({ id: '1', content: 'a', badge: { level: 3, name: 'low' } }),
    pd({ id: '2', content: 'b', badge: { level: 10, name: 'mid' } }),
    pd({ id: '3', content: 'c', badge: { level: 25, name: 'high' } }),
  ];
  const r = DanmakuParser.filter(danmakus, { minLevel: 10 });
  eq(r.length, 2, 'minLevel 10 count');
  eq(r.map(d => d.badge?.name), ['mid', 'high'], 'minLevel 10 names');
}

console.log('=== filter: minLevel passes users with no badge (code only checks if badge exists and < minLevel) ===');
{
  const danmakus = [
    pd({ id: '1', content: 'a' }),
    pd({ id: '2', content: 'b', badge: { level: 5, name: 'x' } }),
  ];
  const r = DanmakuParser.filter(danmakus, { minLevel: 3 });
  eq(r.length, 2, 'no-badge users pass minLevel check');
  eq(r[0].content, 'a', 'no badge retained');
  eq(r[1].content, 'b', 'badged also retained');
}

console.log('=== filter: combined filters ===');
{
  const danmakus = [
    pd({ id: '1', content: 'hello admin msg', userType: 'admin', badge: { level: 15, name: 'b' } }),
    pd({ id: '2', content: 'hello normal msg', userType: 'normal', badge: { level: 15, name: 'b' } }),
    pd({ id: '3', content: 'hello admin spam', userType: 'admin', badge: { level: 15, name: 'b' } }),
    pd({ id: '4', content: 'hello admin msg', userType: 'admin', badge: { level: 5, name: 'b' } }),
  ];
  const r = DanmakuParser.filter(danmakus, {
    userType: ['admin'],
    minLevel: 10,
    keywords: ['msg'],
    excludeKeywords: ['spam']
  });
  eq(r.length, 1, 'combined filters count');
  eq(r[0].id, '1', 'combined filters result id');
}

// ------- statistics tests -------

console.log('=== statistics ===');
{
  // Build timestamps for specific local hours (using setHours so it works regardless of timezone).
  function tsAtHour(h: number): number {
    const d = new Date('2023-11-14T00:00:00');
    d.setHours(h, 0, 0, 0);
    return d.getTime();
  }
  const ts5a = tsAtHour(5);
  const ts5b = tsAtHour(5) + 60000;
  const ts8a = tsAtHour(8);
  const ts8b = tsAtHour(8) + 60000;
  const ts12 = tsAtHour(12);

  const danmakus: ParsedDanmaku[] = [
    pd({ id: '1', username: 'alice', content: 'a', timestamp: ts5a, userType: 'normal' }),
    pd({ id: '2', username: 'alice', content: 'b', timestamp: ts5b, userType: 'normal' }),
    pd({ id: '3', username: 'bob', content: 'c', timestamp: ts8a, userType: 'captain' }),
    pd({ id: '4', username: 'carol', content: 'd', timestamp: ts8b, userType: 'admin' }),
    pd({ id: '5', username: 'dave', content: 'e', timestamp: ts12, userType: 'owner', isOwner: true }),
  ];
  const s = DanmakuParser.statistics(danmakus);
  eq(s.total, 5, 'total');
  eq(s.byUserType, { normal: 2, captain: 1, admin: 1, owner: 1 }, 'byUserType');
  eq(s.byHour, { '5': 2, '8': 2, '12': 1 }, 'byHour');
  eq(s.topUsers.length, 4, 'topUsers length');
  eq(s.topUsers[0], { username: 'alice', count: 2 }, 'top user alice');
  eq(s.topUsers[1].count, 1, 'other users count 1');
}

console.log('=== statistics: empty array ===');
{
  const s = DanmakuParser.statistics([]);
  eq(s.total, 0, 'zero total');
  eq(s.byUserType, {}, 'empty byUserType');
  eq(s.byHour, {}, 'empty byHour');
  eq(s.topUsers, [], 'empty topUsers');
}

console.log('');
console.log(`--- ${p} passed, ${f} failed, ${p + f} total ---`);
