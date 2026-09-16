import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

const source = (path: string) => readFileSync(resolve(process.cwd(), path), 'utf8');

describe('interaction media page contracts', () => {
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
    expect(recording).toContain('RecordingStatusBand');
    expect(recording).toContain('RecordingSettingsDialog');
    expect(`${recording}\n${recordingList}\n${highlightList}`).not.toMatch(
      /pauseRecording|resumeRecording|openRecordingFolder|electronAPI|createElement\('a'\)|\.download\s*=/
    );
  });

  it('keeps the restricted effect surface to one listener and a complete canvas lifecycle', () => {
    const root = source('src/renderer/views/captain-effect/EffectWindowRoot.vue');
    const surface = source('src/renderer/components/media/CaptainEffectSurface.vue');
    expect(root.match(/listenDesktop\('captain_effect_trigger'/g)).toHaveLength(1);
    expect(root).not.toContain('invokeDesktop');
    expect(surface).toContain('requestAnimationFrame');
    expect(surface).toContain('cancelAnimationFrame');
    expect(surface).toContain('ResizeObserver');
    expect(surface).toContain('devicePixelRatio');
    expect(surface).toContain('prefers-reduced-motion');
  });

  it('exposes the approved reminder routes and navigation labels', () => {
    const router = source('src/renderer/router/index.ts');
    const navigation = source('src/renderer/modules/application-shell/navigation.ts');
    const locale = source('src/renderer/locales/zh-CN.json');
    expect(router).toContain('path: \'/captain-effect\'');
    expect(router).toContain('path: \'/gift-reminder\'');
    expect(router).toContain('path: \'/gift-sound\'');
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
