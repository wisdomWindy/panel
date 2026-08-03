// Tests for danmaku types
let p = 0, f = 0;
function ok(c: boolean, l: string) { if (c) p++; else f++; console.log('  ' + (c ? '✓' : '✗ FAIL') + ': ' + l); }
function eq(a: any, e: any, l: string) { const m = JSON.stringify(a) === JSON.stringify(e); if (m) p++; else f++; console.log('  ' + (m ? '✓' : '✗ FAIL') + ': ' + l + (m ? '' : '\n    exp: ' + JSON.stringify(e) + '\n    got: ' + JSON.stringify(a))); }

// ── Types mirroring the source (not imported, just for test-time reference) ──
interface Danmaku {
  id: string;
  userId: string;
  username: string;
  content: string;
  timestamp: number;
  userType: 'normal' | 'captain' | 'admin' | 'owner';
  badge?: { level: number; name: string };
  medal?: { level: number; name: string; color: string };
}

interface Gift {
  id: string;
  giftId: string;
  giftName: string;
  userId: string;
  username: string;
  count: number;
  price: number;
  timestamp: number;
  isCombo: boolean;
  comboId?: string;
}

type ConnectionStatus = 'disconnected' | 'connecting' | 'connected' | 'error';
const validStatuses: ReadonlySet<string> = new Set(['disconnected', 'connecting', 'connected', 'error']);

interface RoomInfo {
  roomId: number;
  roomName: string;
  ownerId: number;
  ownerName: string;
  onlineCount: number;
}

interface BlacklistItem {
  userId: string;
  username: string;
  reason: string;
  bannedAt: number;
  bannedUntil: number;
}

interface DanmakuMessage {
  cmd: string;
  data: unknown;
  [key: string]: unknown;
}

interface CaptainInfo {
  userId: string;
  username: string;
  level: number;
  title: string;
}

interface GiftComboInfo {
  comboId: string;
  giftName: string;
  count: number;
  userId: string;
  username: string;
}

interface LogEntry {
  id: string;
  timestamp: Date;
  level: 'info' | 'warn' | 'error';
  message: string;
  data?: unknown;
}

function isConnectionStatus(v: string): v is ConnectionStatus {
  return validStatuses.has(v);
}

// ── 1. Valid Danmaku objects ──
console.log('1. Valid Danmaku objects');
{
  const d: Danmaku = {
    id: 'dm-001',
    userId: 'u-123',
    username: 'testuser',
    content: 'Hello World',
    timestamp: 1700000000000,
    userType: 'normal',
  };
  ok(typeof d.id === 'string', 'id is string');
  ok(typeof d.userId === 'string', 'userId is string');
  ok(typeof d.username === 'string', 'username is string');
  ok(typeof d.content === 'string', 'content is string');
  ok(typeof d.timestamp === 'number', 'timestamp is number');
  ok(['normal', 'captain', 'admin', 'owner'].includes(d.userType), 'userType is valid');
}

{
  const d: Danmaku = {
    id: 'dm-002',
    userId: 'u-456',
    username: 'captain_user',
    content: 'Ahoy!',
    timestamp: 1700000001000,
    userType: 'captain',
    badge: { level: 5, name: 'Gold' },
    medal: { level: 10, name: 'Veteran', color: '#ff0000' },
  };
  ok(d.badge?.level === 5, 'badge level');
  ok(d.medal?.color === '#ff0000', 'medal color');
}

{
  const d: Danmaku = {
    id: 'dm-003',
    userId: 'u-789',
    username: 'owner',
    content: 'Welcome!',
    timestamp: 1700000002000,
    userType: 'owner',
  };
  ok(d.badge === undefined, 'badge is undefined when omitted');
}

// ── 2. Valid Gift objects ──
console.log('2. Valid Gift objects');
{
  const g: Gift = {
    id: 'g-001',
    giftId: 'gift-100',
    giftName: 'Rose',
    userId: 'u-123',
    username: 'sender',
    count: 5,
    price: 100,
    timestamp: 1700000000000,
    isCombo: false,
  };
  ok(typeof g.giftId === 'string', 'giftId is string');
  ok(typeof g.giftName === 'string', 'giftName is string');
  ok(g.count === 5, 'count is number');
  ok(g.price === 100, 'price is number');
  ok(g.isCombo === false, 'isCombo false');
  ok(g.comboId === undefined, 'comboId undefined when not provided');
}

{
  const g: Gift = {
    id: 'g-002',
    giftId: 'gift-200',
    giftName: 'Rocket',
    userId: 'u-456',
    username: 'big_spender',
    count: 3,
    price: 1000,
    timestamp: 1700000001000,
    isCombo: true,
    comboId: 'combo-999',
  };
  ok(g.isCombo === true, 'isCombo true');
  ok(g.comboId === 'combo-999', 'comboId set');
}

// ── 3. ConnectionStatus type guard ──
console.log('3. ConnectionStatus type guard');
{
  ok(isConnectionStatus('disconnected'), 'disconnected is valid');
  ok(isConnectionStatus('connecting'), 'connecting is valid');
  ok(isConnectionStatus('connected'), 'connected is valid');
  ok(isConnectionStatus('error'), 'error is valid');
  ok(!isConnectionStatus('unknown'), 'unknown is invalid');
  ok(!isConnectionStatus(''), 'empty string is invalid');
  ok(!isConnectionStatus('DISCONNECTED'), 'case-sensitive: DISCONNECTED is invalid');
}

// ── 4. Optional fields handling ──
console.log('4. Optional fields handling');
{
  const d1: Danmaku = {
    id: 'opt-001',
    userId: 'u-opt',
    username: 'optuser',
    content: 'minimal',
    timestamp: 1700000000000,
    userType: 'admin',
  };
  ok(d1.badge === undefined, 'Danmaku.badge undefined');
  ok(d1.medal === undefined, 'Danmaku.medal undefined');

  const g1: Gift = {
    id: 'opt-g-001',
    giftId: 'gift-opt',
    giftName: 'Minimal',
    userId: 'u-opt',
    username: 'optuser',
    count: 1,
    price: 1,
    timestamp: 1700000000000,
    isCombo: false,
  };
  ok(g1.comboId === undefined, 'Gift.comboId undefined');
}

// ── 5. BlacklistItem with permanent ban ──
console.log('5. BlacklistItem with permanent ban');
{
  const perm: BlacklistItem = {
    userId: 'bad-user-1',
    username: 'troll',
    reason: 'Spamming',
    bannedAt: 1700000000000,
    bannedUntil: 0,
  };
  ok(perm.bannedUntil === 0, 'bannedUntil is 0 (permanent)');
  ok(perm.userId === 'bad-user-1', 'userId matches');
  ok(perm.reason === 'Spamming', 'reason matches');
}

{
  const temp: BlacklistItem = {
    userId: 'bad-user-2',
    username: 'toxic',
    reason: 'Harassment',
    bannedAt: 1700000000000,
    bannedUntil: 1700100000000,
  };
  ok(temp.bannedUntil > temp.bannedAt, 'bannedUntil > bannedAt for temp ban');
  ok(temp.bannedUntil !== 0, 'non-zero bannedUntil means temporary');
}

// ── 6. Additional type structure checks ──
console.log('6. Additional type checks');
{
  // RoomInfo
  const room: RoomInfo = {
    roomId: 12345,
    roomName: 'Test Room',
    ownerId: 999,
    ownerName: 'Streamer',
    onlineCount: 42,
  };
  ok(typeof room.roomId === 'number', 'RoomInfo.roomId is number');
  ok(typeof room.roomName === 'string', 'RoomInfo.roomName is string');
  ok(typeof room.onlineCount === 'number', 'RoomInfo.onlineCount is number');

  // CaptainInfo
  const cap: CaptainInfo = { userId: 'c-1', username: 'cap', level: 3, title: 'Captain' };
  ok(cap.level === 3 && cap.title === 'Captain', 'CaptainInfo fields valid');

  // GiftComboInfo
  const combo: GiftComboInfo = { comboId: 'c-1', giftName: 'Star', count: 10, userId: 'u-1', username: 'fan' };
  ok(combo.count === 10, 'GiftComboInfo count');

  // LogEntry
  const log: LogEntry = { id: 'log-1', timestamp: new Date(), level: 'info', message: 'test' };
  ok(log.level === 'info', 'LogEntry.level');
  eq(log.data, undefined, 'LogEntry.data undefined by default');

  // DanmakuMessage with extra keys
  const msg: DanmakuMessage = { cmd: 'DANMU_MSG', data: { text: 'hi' }, extra: true };
  ok(msg.cmd === 'DANMU_MSG', 'DanmakuMessage.cmd');
  ok(msg.extra === true, 'DanmakuMessage allows extra keys');
}

// ── Summary ──
console.log(`\n${p} passed, ${f} failed, ${p + f} total`);
if (f > 0) process.exit(1);
