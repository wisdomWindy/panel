import { describe, expect, it } from 'vitest';
import { reactive } from 'vue';
import {
  SHELL_COMMAND_NAMES,
  SHELL_EVENT_NAMES,
  type UiPreferences
} from '@/platform/desktop/contracts/shell';
import {
  adaptLocalePreferences,
  adaptConfigOverview,
  adaptSystemMetrics,
  adaptUiPreferences,
  cloneUiPreferences
} from './adapters';
import { buildNavigationTree } from './navigation';
import { buildStatusBarView, connectionErrorMessageKey, normalizeRoomInput } from './view-model';

const validUi = (): UiPreferences => ({
  theme: 'dark',
  accentColor: '#FB7299',
  opacity: 0.95,
  fontSize: 'medium',
  layoutMode: 'default',
  customLayout: null,
  showDanmakuLevel: true,
  showDanmakuMedal: true,
  showDanmakuTime: true,
  maxDanmakuEntries: 1000
});

describe('application shell contracts', () => {
  it('freezes exactly thirteen commands and three events', () => {
    expect(SHELL_COMMAND_NAMES).toHaveLength(13);
    expect(SHELL_EVENT_NAMES).toEqual([
      'shell_status_changed',
      'locale_changed',
      'ui_preferences_changed'
    ]);
  });

  it('validates complete locale, UI and metric snapshots', () => {
    expect(adaptLocalePreferences({ locale: 'en-US', followSystem: false })).toEqual({
      locale: 'en-US',
      followSystem: false
    });
    expect(adaptUiPreferences(validUi())).toEqual(validUi());
    expect(adaptSystemMetrics({ memoryBytes: 0, cpuPercent: 0, observedAt: 1 })).toEqual({
      memoryBytes: 0,
      cpuPercent: 0,
      observedAt: 1
    });
    expect(() => adaptUiPreferences({ ...validUi(), opacity: Number.NaN })).toThrow();
    expect(() => adaptSystemMetrics({ memoryBytes: -1, cpuPercent: 101, observedAt: 1 })).toThrow();
    expect(() =>
      adaptConfigOverview({
        displayPath: 'config',
        lastBackupAt: -1,
        scopes: []
      })
    ).toThrow('lastBackupAt must be non-negative');
  });

  it('migrates only the legacy default accent to bilibili pink', () => {
    expect(adaptUiPreferences({ ...validUi(), accentColor: '#FF6699' }).accentColor).toBe(
      '#FB7299'
    );
    expect(adaptUiPreferences({ ...validUi(), accentColor: '#12ABEF' }).accentColor).toBe(
      '#12ABEF'
    );
  });

  it('clones reactive UI preferences into a detached plain snapshot', () => {
    const source = reactive({
      ...validUi(),
      layoutMode: 'custom' as const,
      customLayout: {
        version: 1 as const,
        navigationWidth: 208,
        toolbarHeight: 56,
        statusHeight: 32
      }
    });

    const cloned = cloneUiPreferences(source);

    expect(cloned).toEqual(source);
    expect(cloned).not.toBe(source);
    expect(cloned.customLayout).not.toBe(source.customLayout);
  });

  it('normalizes identity codes without accepting controls or non-ASCII text', () => {
    expect(normalizeRoomInput(' ABC123 ')).toEqual({ ok: true, value: 'ABC123' });
    expect(normalizeRoomInput('   ')).toEqual({ ok: false, code: 'roomRequired' });
    expect(normalizeRoomInput('12\n34')).toEqual({ ok: false, code: 'roomInvalid' });
    expect(normalizeRoomInput('１２３')).toEqual({ ok: false, code: 'roomInvalid' });
  });

  it('maps asynchronous open-live failures to actionable messages', () => {
    expect(connectionErrorMessageKey('missing_open_live_credentials')).toBe(
      'openLiveConfigMissing'
    );
    expect(connectionErrorMessageKey('open_live_auth_failed')).toBe('openLiveAuthFailed');
    expect(connectionErrorMessageKey('open_live_network_error')).toBe('openLiveNetworkFailed');
    expect(connectionErrorMessageKey('something_new')).toBe('operationFailed');
    expect(connectionErrorMessageKey(null)).toBeNull();
  });

  it('builds the exact common tree with shared AI and without member-only routes', () => {
    const free = buildNavigationTree();
    expect(free.map(item => item.id)).toEqual([
      'liveRoom',
      'danmaku',
      'dashboard',
      'rankings',
      'risk',
      'reminders',
      'tts',
      'recording',
      'ai',
      'settings'
    ]);
    // PRD 8.1: the first entry opens the live-room overview, not the raw form.
    expect(free[0].route).toBe('/live');
    const freeTokens = free.flatMap(item => [
      item.id,
      item.labelKey,
      item.route ?? '',
      ...(item.children ?? []).flatMap(child => [child.id, child.labelKey, child.route ?? ''])
    ]);
    expect(freeTokens).toContain('ai');
    expect(freeTokens).not.toContain('membership');
    expect(freeTokens).not.toContain('moderation');
    expect(freeTokens.join('|')).not.toContain('/membership');
  });

  it('projects seven stable status fields with field-specific formats', () => {
    const view = buildStatusBarView({
      room: {
        status: 'connected',
        roomId: '100' as never,
        roomName: null,
        onlineCount: 1234,
        reconnectAttempt: 0,
        lastErrorCode: null
      },
      acceptedDanmakuCount: 8,
      recordingState: 'recording',
      metrics: { memoryBytes: 1_572_864, cpuPercent: 12.34, observedAt: 1 },
      locale: 'en-US'
    });
    expect(view).toHaveLength(7);
    expect(view.map(item => item.value)).toEqual([
      'connected',
      '100',
      '8 msg/s',
      '1,234',
      'recording',
      '1.5 MiB',
      '12.3%'
    ]);
  });
});
