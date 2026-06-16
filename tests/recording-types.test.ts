import { 
  DEFAULT_RECORDING_CONFIG, 
  DEFAULT_HOTKEYS 
} from '../src/renderer/modules/recording/types';
import type { 
  RecordingConfig, 
  HighlightConfig,
  RecordingOptions,
  RecordingSession,
  Highlight,
  Recording
} from '../src/renderer/modules/recording/types';

let p = 0, f = 0;
function ok(c: boolean, l: string) { if (c) p++; else f++; console.log('  ' + (c ? '✓' : '✗ FAIL') + ': ' + l); }
function eq(a: any, e: any, l: string) { const m = JSON.stringify(a) === JSON.stringify(e); if (m) p++; else f++; console.log('  ' + (m ? '✓' : '✗ FAIL') + ': ' + l + (m ? '' : '\n    exp: ' + JSON.stringify(e) + '\n    got: ' + JSON.stringify(a))); }

// ------- 类型定义测试 -------

console.log('=== DEFAULT_RECORDING_CONFIG: 默认值 ===');
{
  eq(DEFAULT_RECORDING_CONFIG.enabled, false, '默认禁用');
  eq(DEFAULT_RECORDING_CONFIG.autoStart, false, '默认不自动开始');
  eq(DEFAULT_RECORDING_CONFIG.format, 'mp4', '默认格式mp4');
  eq(DEFAULT_RECORDING_CONFIG.quality, 'original', '默认原始质量');
  eq(DEFAULT_RECORDING_CONFIG.segmentSize, 0, '默认不分段');
  eq(DEFAULT_RECORDING_CONFIG.segmentDuration, 0, '默认不分段时长');
  eq(DEFAULT_RECORDING_CONFIG.maxStorageSize, 0, '默认不限制存储');
  eq(DEFAULT_RECORDING_CONFIG.autoCleanup, false, '默认不自动清理');
  eq(DEFAULT_RECORDING_CONFIG.cleanupDays, 30, '默认清理30天');
  eq(DEFAULT_RECORDING_CONFIG.highlightEnabled, true, '默认启用高光');
}

console.log('=== DEFAULT_RECORDING_CONFIG: 高光配置默认值 ===');
{
  const hl = DEFAULT_RECORDING_CONFIG.highlightConfig;
  eq(hl.danmakuThreshold, 30, '弹幕阈值30');
  eq(hl.danmakuWindow, 10, '弹幕窗口10秒');
  eq(hl.giftValueThreshold, 100, '礼物阈值100');
  eq(hl.captainTrigger, true, '舰长触发改');
  eq(hl.clipDuration, 30, '片段时长30秒');
  eq(hl.beforeSeconds, 15, '高光前15秒');
  eq(hl.afterSeconds, 15, '高光后15秒');
  eq(hl.cooldownSeconds, 10, '冷却10秒');
}

console.log('=== DEFAULT_HOTKEYS: 快捷键默认值 ===');
{
  eq(DEFAULT_HOTKEYS.toggleRecording, 'CommandOrControl+Shift+R', '录制快捷键');
  eq(DEFAULT_HOTKEYS.markHighlight, 'F9', '高光快捷键');
}

console.log('=== RecordingConfig: 类型兼容性 ===');
{
  const config: RecordingConfig = {
    enabled: true,
    autoStart: true,
    format: 'flv',
    quality: '1080p',
    segmentSize: 2048,
    segmentDuration: 3600,
    outputPath: '/test/path',
    maxStorageSize: 50,
    autoCleanup: true,
    cleanupDays: 7,
    highlightEnabled: true,
    highlightConfig: {
      danmakuThreshold: 50,
      danmakuWindow: 15,
      giftValueThreshold: 200,
      captainTrigger: false,
      clipDuration: 60,
      beforeSeconds: 30,
      afterSeconds: 30,
      cooldownSeconds: 5
    }
  };
  
  eq(config.format, 'flv', '格式flv');
  eq(config.quality, '1080p', '质量1080p');
  eq(config.segmentSize, 2048, '分段大小2048');
  eq(config.highlightConfig.danmakuThreshold, 50, '弹幕阈值50');
}

console.log('=== HighlightConfig: 类型兼容性 ===');
{
  const config: HighlightConfig = {
    danmakuThreshold: 100,
    danmakuWindow: 5,
    giftValueThreshold: 500,
    captainTrigger: true,
    clipDuration: 45,
    beforeSeconds: 20,
    afterSeconds: 25,
    cooldownSeconds: 15
  };
  
  eq(config.danmakuThreshold, 100, '弹幕阈值100');
  eq(config.cooldownSeconds, 15, '冷却15秒');
}

console.log('=== RecordingOptions: 类型兼容性 ===');
{
  const options: RecordingOptions = {
    roomId: '12345',
    roomName: '测试直播间',
    format: 'mp4',
    quality: '720p',
    segmentSize: 1024,
    outputPath: '/output',
    highlightEnabled: true,
    highlightConfig: DEFAULT_RECORDING_CONFIG.highlightConfig
  };
  
  eq(options.roomId, '12345', '房间ID');
  eq(options.format, 'mp4', '格式mp4');
  eq(options.quality, '720p', '质量720p');
}

console.log('=== RecordingSession: 类型兼容性 ===');
{
  const session: RecordingSession = {
    id: 'session_123',
    roomId: '12345',
    roomName: '测试房间',
    status: 'recording',
    filePath: '/recordings/test.mp4',
    fileName: 'test.mp4',
    fileSize: 1024000,
    duration: 300,
    format: 'mp4',
    quality: 'original',
    startedAt: new Date(),
    highlightCount: 5
  };
  
  eq(session.id, 'session_123', '会话ID');
  eq(session.status, 'recording', '状态recording');
  eq(session.duration, 300, '时长300秒');
  eq(session.highlightCount, 5, '高光数量5');
}

console.log('=== Highlight: 类型兼容性 ===');
{
  const highlight: Highlight = {
    id: 'highlight_123',
    recordingId: 'session_123',
    roomId: '12345',
    timestamp: 150,
    reason: 'danmaku_surge',
    description: '弹幕激增',
    clipPath: '/clips/highlight.mp4',
    clipDuration: 30,
    createdAt: new Date()
  };
  
  eq(highlight.reason, 'danmaku_surge', '原因弹幕激增');
  eq(highlight.timestamp, 150, '时间戳150秒');
  eq(highlight.clipDuration, 30, '片段时长30秒');
}

console.log('=== Recording: 类型兼容性 ===');
{
  const recording: Recording = {
    id: 'recording_123',
    roomId: '12345',
    roomName: '测试房间',
    filePath: '/recordings/test.mp4',
    fileName: 'test.mp4',
    fileSize: 1024000,
    duration: 3600,
    format: 'mp4',
    quality: 'original',
    status: 'completed',
    isHighlight: false,
    startedAt: new Date(),
    endedAt: new Date(),
    createdAt: new Date()
  };
  
  eq(recording.status, 'completed', '状态completed');
  eq(recording.isHighlight, false, '非高光录制');
  eq(recording.duration, 3600, '时长3600秒');
}

console.log('=== Recording: 高光录制类型 ===');
{
  const highlightRecording: Recording = {
    id: 'highlight_rec_123',
    roomId: '12345',
    roomName: '测试房间',
    filePath: '/recordings/highlight.mp4',
    fileName: 'highlight.mp4',
    fileSize: 512000,
    duration: 30,
    format: 'mp4',
    quality: 'original',
    status: 'completed',
    isHighlight: true,
    highlightReason: 'danmaku_surge',
    startedAt: new Date(),
    endedAt: new Date(),
    createdAt: new Date()
  };
  
  eq(highlightRecording.isHighlight, true, '是高光录制');
  eq(highlightRecording.highlightReason, 'danmaku_surge', '高光原因');
}

console.log('');
console.log(`--- ${p} passed, ${f} failed, ${p + f} total ---`);
