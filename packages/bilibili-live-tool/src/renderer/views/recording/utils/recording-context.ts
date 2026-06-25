export interface RecordingHighlightConfigShape {
  danmakuThreshold: number;
  danmakuWindow: number;
  giftValueThreshold: number;
  captainTrigger: boolean;
  clipDuration: number;
  beforeSeconds: number;
  afterSeconds: number;
  cooldownSeconds: number;
}

export interface RecordingConfigShape {
  format: 'mp4' | 'flv';
  quality: 'original' | '720p' | '1080p';
  segmentSize?: number;
  segmentDuration?: number;
  outputPath: string;
  highlightEnabled: boolean;
  highlightConfig: RecordingHighlightConfigShape;
}

export interface RecordingOptionsShape {
  roomId: string;
  roomName?: string;
  format: 'mp4' | 'flv';
  quality: 'original' | '720p' | '1080p';
  segmentSize?: number;
  segmentDuration?: number;
  outputPath: string;
  highlightEnabled: boolean;
  highlightConfig: RecordingHighlightConfigShape;
}

export interface RecordingRoomContext {
  roomId: string;
  roomName?: string;
}

export function canStartRecording(
  room: RecordingRoomContext | null
): { ok: true } | { ok: false; message: string } {
  if (!room?.roomId) {
    return {
      ok: false,
      message: '当前未连接直播间，无法开始录制'
    };
  }

  return { ok: true };
}

export function buildRecordingStartOptions(
  room: RecordingRoomContext,
  config: RecordingConfigShape
): RecordingOptionsShape {
  return {
    roomId: room.roomId,
    roomName: room.roomName,
    format: config.format,
    quality: config.quality,
    segmentSize: config.segmentSize,
    segmentDuration: config.segmentDuration,
    outputPath: config.outputPath || '',
    highlightEnabled: config.highlightEnabled,
    highlightConfig: config.highlightConfig
  };
}
