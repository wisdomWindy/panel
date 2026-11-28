import { existsSync, readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

const source = (path: string) => readFileSync(resolve(process.cwd(), path), 'utf8');

describe('interaction media page contracts', () => {
  it('pairs TTS and recording surfaces with readable dark-theme variants', () => {
    const paths = [
      'src/renderer/views/tts/index.vue',
      'src/renderer/components/media/TtsQueuePanel.vue',
      'src/renderer/views/recording/index.vue',
      'src/renderer/views/recording/components/RecordingSettingsDialog.vue'
    ];

    for (const path of paths) {
      const lines = source(path).split(/\r?\n/);
      expect(
        lines.filter(line => /text-gray-\d+/.test(line) && !/dark:text-/.test(line)),
        path
      ).toEqual([]);
      expect(
        lines.filter(line => /text-red-\d+/.test(line) && !/dark:text-red-/.test(line)),
        path
      ).toEqual([]);
      expect(
        lines.filter(line => /text-amber-\d+/.test(line) && !/dark:text-amber-/.test(line)),
        path
      ).toEqual([]);
      expect(
        lines.filter(
          line => /border-gray-200/.test(line) && !/dark:border-zinc-800/.test(line)
        ),
        path
      ).toEqual([]);
      expect(
        lines.filter(
          line => /border-gray-300/.test(line) && !/dark:border-zinc-700/.test(line)
        ),
        path
      ).toEqual([]);
      expect(
        lines.filter(line => /bg-gray-50/.test(line) && !/dark:bg-zinc-900/.test(line)),
        path
      ).toEqual([]);
    }
  });

  it('renders reminder pages as token-driven surfaces without per-page dark variants', () => {
    const pages = [
      'src/renderer/views/captain-effect/index.vue',
      'src/renderer/views/gift-reminder/index.vue',
      'src/renderer/views/gift-sound/index.vue'
    ];
    const tablePath = 'src/renderer/components/media/GiftSoundTable.vue';

    for (const path of [...pages, tablePath]) {
      const content = source(path);
      expect(content, path).not.toContain('dark:');
      expect(content, path).not.toMatch(
        /\bbg-white\b|\btext-gray-\d+|\bborder-gray-\d+|\bdivide-gray-\d+|\bbg-gray-\d+/
      );
    }

    for (const path of pages) {
      const content = source(path);
      expect(content, path).toContain('app-card');
      expect(content, path).toContain('app-muted');
      expect(content, path).toContain('app-text');
      expect(content, path).toContain('app-btn-primary');
      expect(content, path).toContain('app-chip');
    }

    expect(source(tablePath)).toContain('app-faint');
    expect(source(tablePath)).toContain('app-chip');
  });

  it('keeps page components behind stores and removes browser/Electron media paths', () => {
    const pages = [
      'src/renderer/views/captain-effect/index.vue',
      'src/renderer/views/gift-reminder/index.vue',
      'src/renderer/views/gift-sound/index.vue',
      'src/renderer/views/tts/index.vue',
      'src/renderer/views/recording/index.vue'
    ].map(source).join('\n');
    expect(pages).not.toMatch(/invokeDesktop|listenDesktop|electronAPI|new Audio|new Blob|createObjectURL/);
    expect(pages).not.toMatch(/播放模式|冷却时间|优先级|渲染模式|测试数据/);
  });

  it('keeps TTS and recording controls inside the approved surface', () => {
    const tts = source('src/renderer/views/tts/index.vue');
    const recording = source('src/renderer/views/recording/index.vue');
    const recordingList = source('src/renderer/views/recording/components/RecordingList.vue');
    const highlightList = source('src/renderer/views/recording/components/HighlightList.vue');
    expect(tts).toContain('TtsQueuePanel');
    expect(tts).toContain('validateTtsTemplate');
    // The redesigned TTS page is token-driven: app-card sections and one primary action.
    expect(tts).toContain('app-card');
    expect(tts).toContain('app-btn-primary');
    expect(tts).toContain('app-muted');
    expect(tts).not.toContain('dark:');
    // The recording status surface is now the token-driven control card itself.
    expect(recording).toContain('app-btn-primary');
    expect(recording).toContain('app-metric');
    expect(recording).toContain("t('recording.recordingStatus')");
    expect(recording).toContain('RecordingSettingsDialog');
    // 打开文件夹 is an approved surface now: it opens the recording folder through
    // its own IPC command, not through an Electron-era shell API.
    expect(`${recording}\n${recordingList}\n${highlightList}`).not.toMatch(
      /pauseRecording|resumeRecording|electronAPI|createElement\('a'\)|\.download\s*=/
    );
    expect(recording).toContain('openRecordingFolder');
  });

  it('keeps one engine selector that reveals that engine settings in place', () => {
    const tts = source('src/renderer/views/tts/index.vue');

    // Both engines are choices of the same dropdown...
    expect(tts).toMatch(/t\('tts\.systemEngine'\)/);
    expect(tts).toMatch(/t\('tts\.cloudEngine'\)/);
    expect(tts).toContain('handleEngineChange');
    // ...and the Qwen3 options live in that card instead of a second panel.
    expect(tts).not.toContain('tts-qwen3-title');
    expect(tts).toMatch(/v-if="draft\.selectedEngine === 'cloud'"/);
    expect(tts).toMatch(/t\('tts\.qwen3\.language'\)/);
    expect(tts).toMatch(/t\('tts\.qwen3\.instruct'\)/);
    // System-only controls are hidden for the Qwen3 engine.
    expect(tts).toMatch(/v-if="draft\.selectedEngine === 'system'"/);
    expect(tts).toMatch(/t\('tts\.rate'\)/);
  });

  it('offers an in-app cleanup of redundant recording leftovers', () => {
    const recording = source('src/renderer/views/recording/index.vue');
    const store = source('src/renderer/stores/media/recording.ts');
    expect(recording).toContain('handleCleanup');
    expect(recording).toContain("t('recording.cleanup')");
    expect(store).toContain('cleanupLeftovers');
  });
  it('lets one click start another concurrent recording task', () => {
    const recording = source('src/renderer/views/recording/index.vue');
    // The primary action always adds a task; stopping lives on each task row.
    expect(recording).toContain("t('recording.startRecording')");
    expect(recording).toContain('handleStopAll');
    expect(recording).toContain('stopOneTask');
    expect(recording).toContain('markTask');
    expect(recording).toContain("t('recording.tasks')");
    expect(recording).toContain('taskStateText');
  });
  it('opens the recording settings while recording instead of silently ignoring the click', () => {
    const recording = source('src/renderer/views/recording/index.vue');
    const dialog = source('src/renderer/views/recording/components/RecordingSettingsDialog.vue');
    // The button must stay clickable; the dialog explains why it is read-only.
    expect(recording).toContain(':locked="isActive"');
    expect(recording).not.toContain(':disabled="isActive"\n              @click="openSettings"');
    expect(dialog).toContain('locked?: boolean');
    expect(dialog).toContain("t('recording.settingsLocked')");
    expect(dialog).toContain('pointer-events-none opacity-60');
  });
  it('recovers the active effect after registering one listener and keeps a complete canvas lifecycle', () => {
    const root = source('src/renderer/views/captain-effect/EffectWindowRoot.vue');
    const surface = source('src/renderer/components/media/CaptainEffectSurface.vue');
    expect(root.match(/listenDesktop\('captain_effect_trigger'/g)).toHaveLength(1);
    expect(root.match(/invokeDesktop\('captain_effect_current'/g)).toHaveLength(1);
    expect(root.indexOf('await listenDesktop(\'captain_effect_trigger\'')).toBeLessThan(
      root.indexOf('invokeDesktop(\'captain_effect_current\'')
    );
    expect(root).toContain('eventRevision');
    expect(surface).toContain('requestAnimationFrame');
    expect(surface).toContain('cancelAnimationFrame');
    expect(surface).toContain('ResizeObserver');
    expect(surface).toContain('devicePixelRatio');
    expect(surface).toContain('prefers-reduced-motion');
    expect(surface).toContain('CAPTAIN_DECORATIONS');
    expect(surface).toContain('t(`captainEffect.actions.${payload.action}`)');
    expect(surface).toContain('payload.audience.username');
    expect(surface).toContain('t(`liveData.tags.${payload.level}`)');
    expect(surface).not.toContain('captainEffect.welcome');
    for (const mapping of [
      '\'captain-default\': \'classic\'',
      '\'golden-premium\': \'sparkles\'',
      '\'starlight-blue\': \'stars\'',
      '\'cherry-blossom\': \'petals\'',
      '\'violet-galaxy\': \'orbit\'',
      '\'emerald-stream\': \'stream\'',
      '\'crimson-celebration\': \'flames\'',
      '\'silver-frost\': \'crystals\'',
      '\'neon-party\': \'neon\'',
      '\'aurora-dream\': \'aurora\'',
      '\'sunset-flare\': \'rays\'',
      '\'midnight-crown\': \'crown\''
    ]) {
      expect(surface).toContain(mapping);
    }
  });

  it('applies built-in captain preset colors from the selected catalog item', () => {
    const page = source('src/renderer/views/captain-effect/index.vue');
    expect(page).toContain('function handlePresetChange');
    expect(page).toContain('@change="handlePresetChange"');
    expect(page).toContain('draft.primaryColor = preset.primaryColor');
    expect(page).toContain('draft.secondaryColor = preset.secondaryColor');
    expect(page).not.toContain('captainEffect.materialsUnavailable');
  });

  it('renders gift alerts in the configured local-popup position with timer cleanup', () => {
    const app = source('src/renderer/App.vue');
    const overlayPath = resolve(
      process.cwd(),
      'src/renderer/components/media/GiftAlertOverlay.vue'
    );
    const overlay = existsSync(overlayPath) ? readFileSync(overlayPath, 'utf8') : '';

    expect(app).toContain('GiftAlertOverlay');
    expect(app).toContain(':payload="gifts.latestAlert"');
    expect(overlay).toContain('topLeft');
    expect(overlay).toContain('topRight');
    expect(overlay).toContain('bottomLeft');
    expect(overlay).toContain('bottomRight');
    expect(overlay).toContain('center');
    expect(overlay).toContain('translate-x-[-50%]');
    expect(overlay).toContain('translate-y-[-50%]');
    expect(overlay).toContain('clearTimeout');
    expect(overlay).toContain('popupDurationSeconds * 1_000');
    expect(overlay).toContain('<img');
    expect(overlay).toContain('giftIconUrl');
    expect(overlay).toContain('@error');
    expect(overlay).toContain('imageLoadFailed');
    expect(overlay).toContain('giftName');
    // The popup is token-driven: one palette, no per-page dark: literals.
    expect(overlay).toContain('app-card');
    expect(overlay).toContain('app-text');
    expect(overlay).toContain('app-surface-2');
    expect(overlay).not.toContain('dark:');
  });

  it('exposes the approved reminder routes and navigation labels', () => {
    const router = source('src/renderer/router/index.ts');
    const navigation = source('src/renderer/modules/application-shell/navigation.ts');
    const locale = source('src/renderer/locales/zh-CN.json');
    expect(router).toContain("path: '/reminders'");
    expect(router).toContain("redirect: '/reminders/gift'");
    expect(router).toContain("redirect: '/reminders/captain'");
    expect(router).toContain("redirect: '/reminders/sound'");
    for (const page of ['gift-reminder', 'captain-effect', 'gift-sound']) {
      expect(router).toContain(`component: () => import('@/views/${page}/index.vue')`);
    }
    expect(router).toContain("path: 'gift'");
    expect(router).toContain("path: 'captain'");
    expect(router).toContain("path: 'sound'");
    expect(router).toContain('path: \'/tts\'');
    expect(router).toContain('path: \'/recording\'');
    expect(locale).toContain('提醒配置');
    expect(locale).toContain('上舰特效');
    expect(locale).toContain('礼物提醒');
    expect(locale).toContain('礼物音效');
    expect(locale).toContain('语音播报');
    expect(navigation).toContain('item(\'recording\', \'/recording\')');
  });
});
