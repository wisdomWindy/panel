# 直播录制功能 - 功能规格

## 背景与目标

Bilibili 直播助手当前缺少直播流录制能力。主播无法在应用内录制直播内容进行存档和二次剪辑，也无法自动标记和提取高光片段。本功能旨在：

1. 提供全程直播流录制能力（MP4/FLV 格式，可选质量）
2. 支持按文件大小或时长自动分段录制
3. 在热度激增（弹幕刷屏、高价值礼物、舰长赠送）时自动标记高光时刻
4. 支持快捷键手动打点标记高光时刻
5. 自动提取高光片段并独立保存
6. 在主界面状态栏实时显示录制状态
7. 提供录制文件管理页面（列表、预览、删除、导出）

## 范围内

- 录制管理页面（`Recording.vue`）
- 录制状态栏组件（`RecordingStatusBar.vue`）
- 录制文件列表组件（`RecordingList.vue`）
- 高光时刻列表组件（`HighlightList.vue`）
- 录制服务（`recording-service.ts`）
- 录制状态管理（`stores/recording.ts`）
- 主进程录制模块（`src/main/modules/recording/`）
- 设置页面新增录制配置项
- 主框架新增录制状态栏
- 数据库新增 recordings/highlights 数据表
- FFmpeg 集成（fluent-ffmpeg）

## 范围外

- 直播流推流功能（仅录制，不推流）
- 视频编辑/剪辑功能（仅提取片段，不编辑）
- 云端存储录制文件
- 录制文件上传到 B 站（仅导出到本地）
- 硬件加速编码（MVP 阶段使用软件编码）
- 多路直播流同时录制
- 录制文件的音轨分离

## 用户流程

### 流程 1：开始全程录制

```
用户连接直播间 → 状态栏显示录制按钮
  → 用户点击「开始录制」按钮
  → 系统获取直播流 URL
  → 调用 FFmpeg 开始录制
  → 状态栏显示录制状态（红点 + 时长 + 文件大小）
  → 录制文件写入指定存储路径
  → 用户可随时暂停/恢复/停止录制
```

### 流程 2：自动分段录制

```
录制进行中 → 文件大小达到分段阈值（如 2GB）
  → FFmpeg 在关键帧处结束当前分段
  → 自动开始新的分段文件
  → 录制不中断，状态栏持续更新
  → 分段文件按序号命名（_001, _002, ...）
```

### 流程 3：自动高光标记（弹幕激增）

```
录制进行中 → 监听弹幕事件
  → 弹幕数量在 N 秒内超过阈值（默认 30 条/10秒）
  → 系统自动标记当前时间为高光时刻
  → 记录高光原因（danmaku_surge）
  → 异步提取高光前后各 30 秒的片段
  → 高光片段保存到独立文件
  → 高光列表实时更新
```

### 流程 4：自动高光标记（礼物/舰长）

```
录制进行中 → 监听礼物和舰长事件
  → 收到高价值礼物（价值 >= 阈值）或舰长赠送
  → 系统自动标记当前时间为高光时刻
  → 记录高光原因（high_value_gift / captain）
  → 异步提取高光片段
  → 高光列表实时更新
```

### 流程 5：手动高光打点

```
录制进行中 → 用户按下 F9 快捷键
  → 系统标记当前时间为高光时刻
  → 记录高光原因（manual）
  → 异步提取高光片段
  → 状态栏短暂提示「高光已标记」
```

### 流程 6：录制文件管理

```
用户打开录制管理页面
  → 加载录制文件列表（时间倒序）
  → 显示文件名、时长、大小、录制时间、状态
  → 用户可点击播放预览
  → 用户可重命名/删除/打开目录
  → 用户可查看高光片段列表
  → 用户可导出录制文件
```

## 页面与模块设计

### Recording.vue

**路由**: `/recording`
**菜单入口**: 侧边栏新增「录制管理」菜单项

**页面结构**:
- 顶部操作区：开始/暂停/停止录制按钮 + 录制配置快捷入口
- 高光标签页：高光时刻列表（时间、原因、描述、操作）
- 录制文件标签页：录制文件列表（文件名、时长、大小、状态、操作）
- 底部状态区：当前录制状态、时长、文件大小

**数据来源**:
- 录制列表：`recordings` 数据表
- 高光列表：`highlights` 数据表
- 录制状态：Pinia store（`stores/recording.ts`）

### RecordingStatusBar.vue

**位置**: `App.vue` 底部状态栏

**显示内容**:
- 录制状态图标（灰色=未录制，红色闪烁=录制中，黄色=暂停）
- 录制时长（HH:MM:SS 格式）
- 当前文件大小（自动转换单位 KB/MB/GB）
- 快捷操作按钮（开始/暂停/停止）

**Props**:
- `status`: `'idle' | 'recording' | 'paused'`
- `duration`: number（秒）
- `fileSize`: number（字节）

### RecordingList.vue

**Props**:
- `recordings`: Recording[]
- `loading`: boolean

**表格列**:
- 文件名
- 录制时长
- 文件大小
- 录制时间
- 状态（recording/paused/completed/failed）
- 操作（播放/重命名/删除/打开目录/导出）

### HighlightList.vue

**Props**:
- `highlights`: Highlight[]
- `loading`: boolean

**表格列**:
- 时间戳（相对于录制开始的秒数）
- 触发原因（弹幕激增/高价值礼物/舰长/手动）
- 描述
- 片段时长
- 操作（播放/删除/打开文件）

### recording-service.ts

**职责**:
- 管理 FFmpeg 录制进程
- 控制录制开始/暂停/恢复/停止
- 监听录制状态变化
- 高光标记和片段提取
- 录制文件元数据管理

**接口**:
```typescript
interface RecordingService {
  // 录制控制
  startRecording(options: RecordingOptions): Promise<RecordingSession>;
  pauseRecording(sessionId: string): Promise<void>;
  resumeRecording(sessionId: string): Promise<void>;
  stopRecording(sessionId: string): Promise<RecordingResult>;

  // 高光管理
  markHighlight(sessionId: string, reason: string, description?: string): Promise<Highlight>;
  generateHighlightClip(highlightId: string): Promise<string>;

  // 录制查询
  getRecordingStatus(sessionId: string): Promise<RecordingStatus>;
  getRecordings(options?: QueryOptions): Promise<Recording[]>;
  getHighlights(recordingId?: string): Promise<Highlight[]>;

  // 文件操作
  deleteRecording(recordingId: string): Promise<void>;
  deleteHighlight(highlightId: string): Promise<void>;
  openRecordingFolder(recordingId: string): Promise<void>;
  getRecordingPath(recordingId: string): Promise<string>;
}

interface RecordingOptions {
  roomId: string;
  format: 'mp4' | 'flv';
  quality: 'original' | '720p' | '1080p';
  segmentSize?: number;  // MB，0 表示不分段
  segmentDuration?: number;  // 秒，0 表示不分段
  outputPath: string;
  highlightEnabled: boolean;
  highlightConfig?: HighlightConfig;
}

interface RecordingSession {
  id: string;
  roomId: string;
  status: 'recording' | 'paused' | 'stopped';
  startedAt: Date;
  filePath: string;
  currentSize: number;
  currentDuration: number;
}

interface RecordingResult {
  sessionId: string;
  filePath: string;
  fileSize: number;
  duration: number;
  highlightCount: number;
}

interface Highlight {
  id: string;
  recordingId: string;
  roomId: string;
  timestamp: number;
  reason: 'danmaku_surge' | 'high_value_gift' | 'captain' | 'manual';
  description: string;
  clipPath?: string;
  clipDuration: number;
  createdAt: Date;
}

interface HighlightConfig {
  danmakuThreshold: number;  // 弹幕数量阈值
  danmakuWindow: number;     // 弹幕统计窗口（秒）
  giftValueThreshold: number;  // 礼物价值阈值
  captainTrigger: boolean;   // 舰长触发
  clipDuration: number;      // 高光片段时长（秒）
  beforeSeconds: number;     // 高光前录制秒数
  afterSeconds: number;      // 高光后录制秒数
}

interface RecordingStatus {
  sessionId: string;
  status: 'recording' | 'paused' | 'stopped' | 'completed' | 'failed';
  currentSize: number;
  currentDuration: number;
  highlightCount: number;
}

interface QueryOptions {
  roomId?: string;
  status?: string;
  startDate?: Date;
  endDate?: Date;
  limit?: number;
  offset?: number;
  sortBy?: string;
  sortOrder?: 'asc' | 'desc';
}
```

### stores/recording.ts

**职责**: 管理录制相关的状态

**State**:
```typescript
interface RecordingState {
  currentSession: RecordingSession | null;
  status: 'idle' | 'recording' | 'paused';
  duration: number;  // 秒
  fileSize: number;  // 字节
  highlightCount: number;
  recordings: Recording[];
  highlights: Highlight[];
  isLoading: boolean;
  config: RecordingConfig;
}
```

**Actions**:
- `startRecording(options)`: 开始录制
- `pauseRecording()`: 暂停录制
- `resumeRecording()`: 恢复录制
- `stopRecording()`: 停止录制
- `markHighlight(reason)`: 手动标记高光
- `fetchRecordings()`: 获取录制列表
- `fetchHighlights(recordingId?)`: 获取高光列表
- `deleteRecording(id)`: 删除录制
- `deleteHighlight(id)`: 删除高光
- `updateConfig(config)`: 更新录制配置

### 主进程录制模块（src/main/modules/recording/）

**文件结构**:
```
src/main/modules/recording/
├── index.ts              # 模块入口，注册 IPC 处理器
├── ffmpeg-manager.ts     # FFmpeg 进程管理
├── recording-engine.ts   # 录制引擎（调用 FFmpeg）
├── highlight-detector.ts # 高光检测器
└── file-manager.ts       # 录制文件管理
```

**ffmpeg-manager.ts**:
- 检测系统 FFmpeg 是否可用
- 管理 FFmpeg 子进程生命周期
- 处理 FFmpeg 输出和错误流
- 支持暂停/恢复录制（通过向 FFmpeg 发送信号）

**recording-engine.ts**:
- 封装 fluent-ffmpeg 调用
- 支持 MP4（H.264+AAC）和 FLV 格式
- 支持质量转码（720p/1080p）
- 支持分段录制（按大小/时长）
- 环形缓冲区处理网络波动

**highlight-detector.ts**:
- 监听弹幕/礼物/舰长事件
- 统计弹幕频率（滑动窗口）
- 检测高价值礼物
- 检测舰长赠送事件
- 触发高光标记

**file-manager.ts**:
- 录制文件路径管理
- 文件元数据读取
- 文件删除和清理
- 存储空间检查

## API 与数据契约

### 数据库表结构

#### recordings 表

```sql
CREATE TABLE recordings (
    id TEXT PRIMARY KEY,
    room_id TEXT NOT NULL,
    room_name TEXT,
    file_path TEXT NOT NULL,
    file_name TEXT NOT NULL,
    file_size INTEGER DEFAULT 0,
    duration INTEGER DEFAULT 0,
    format TEXT DEFAULT 'mp4',
    quality TEXT DEFAULT 'original',
    status TEXT DEFAULT 'recording',
    is_highlight BOOLEAN DEFAULT FALSE,
    highlight_reason TEXT,
    started_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    ended_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_recordings_room_id ON recordings(room_id);
CREATE INDEX idx_recordings_started_at ON recordings(started_at);
CREATE INDEX idx_recordings_status ON recordings(status);
```

#### highlights 表

```sql
CREATE TABLE highlights (
    id TEXT PRIMARY KEY,
    recording_id TEXT,
    room_id TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    reason TEXT NOT NULL,
    description TEXT,
    clip_path TEXT,
    clip_duration INTEGER DEFAULT 30,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (recording_id) REFERENCES recordings(id)
);

CREATE INDEX idx_highlights_recording_id ON highlights(recording_id);
CREATE INDEX idx_highlights_room_id ON highlights(room_id);
```

### IPC 通道

```typescript
// 录制控制
ipcMain.handle('recording:start', (event, options: RecordingOptions) => {...})
ipcMain.handle('recording:pause', (event, sessionId: string) => {...})
ipcMain.handle('recording:resume', (event, sessionId: string) => {...})
ipcMain.handle('recording:stop', (event, sessionId: string) => {...})
ipcMain.handle('recording:status', (event, sessionId: string) => {...})

// 高光管理
ipcMain.handle('recording:markHighlight', (event, sessionId: string, reason: string) => {...})
ipcMain.handle('recording:getHighlights', (event, recordingId?: string) => {...})
ipcMain.handle('recording:deleteHighlight', (event, highlightId: string) => {...})

// 录制文件管理
ipcMain.handle('recording:getRecordings', (event, options?: QueryOptions) => {...})
ipcMain.handle('recording:deleteRecording', (event, recordingId: string) => {...})
ipcMain.handle('recording:openFolder', (event, recordingId: string) => {...})
ipcMain.handle('recording:getPath', (event, recordingId: string) => {...})

// 配置
ipcMain.handle('recording:getConfig', () => {...})
ipcMain.handle('recording:setConfig', (event, config: Partial<RecordingConfig>) => {...})

// 事件推送（主进程 → 渲染进程）
ipcMain.on('recording:statusChanged', (event, status: RecordingStatus) => {...})
ipcMain.on('recording:highlightMarked', (event, highlight: Highlight) => {...})
ipcMain.on('recording:error', (event, error: string) => {...})
```

### 录制配置

```typescript
interface RecordingConfig {
  enabled: boolean;
  autoStart: boolean;
  format: 'mp4' | 'flv';
  quality: 'original' | '720p' | '1080p';
  segmentSize: number;  // MB，0 表示不分段
  segmentDuration: number;  // 秒，0 表示不分段
  outputPath: string;
  maxStorageSize: number;  // GB，0 表示不限制
  autoCleanup: boolean;
  cleanupDays: number;
  highlightEnabled: boolean;
  highlightConfig: {
    danmakuThreshold: number;
    danmakuWindow: number;
    giftValueThreshold: number;
    captainTrigger: boolean;
    clipDuration: number;
    beforeSeconds: number;
    afterSeconds: number;
  };
}
```

### 快捷键配置

```typescript
// 默认快捷键
const DEFAULT_HOTKEYS = {
  toggleRecording: 'CommandOrControl+Shift+R',
  markHighlight: 'F9'
};
```

## 边界情况

1. **FFmpeg 未安装**: 启动时检测 FFmpeg 可用性，不可用时显示提示并禁用录制功能
2. **直播流中断**: 自动重连，录制暂停，重连成功后继续录制，不丢帧
3. **磁盘空间不足**: 录制前检查可用空间，空间不足时停止录制并提示用户
4. **录制文件损坏**: 记录错误日志，标记状态为 failed，支持删除
5. **网络波动**: 使用缓冲机制平滑录制，避免卡顿
6. **分段边界丢帧**: 在关键帧处分段，确保分段无缝衔接
7. **高光标记过于频繁**: 设置冷却时间（默认 10 秒），避免连续标记
8. **高光片段提取失败**: 记录错误日志，保留高光记录但 clipPath 为空
9. **录制过程中应用退出**: 优雅停止录制，保存元数据
10. **多个录制会话冲突**: 同一时间只允许一个录制会话
11. **存储路径不存在**: 自动创建目录，创建失败时提示用户
12. **直播流格式不支持**: 降级到原始格式录制，记录警告日志

## 验收标准

| 编号 | 验收条件 | 可测试性 |
|------|----------|----------|
| AC-01 | 连接直播间后可点击「开始录制」启动录制 | 手动验证 |
| AC-02 | 录制状态栏显示录制状态（红点 + 时长 + 文件大小） | 自动化测试 |
| AC-03 | 支持暂停/恢复录制 | 手动验证 |
| AC-04 | 点击「停止录制」生成完整录制文件 | 手动验证 + 文件检查 |
| AC-05 | 录制文件可使用主流播放器正常播放（VLC、系统播放器） | 手动验证 |
| AC-06 | 设置页面可配置录制格式（MP4/FLV） | 自动化测试 |
| AC-07 | 设置页面可配置录制质量（原始/720p/1080p） | 自动化测试 |
| AC-08 | 设置页面可配置分段规则（按大小/按时长） | 自动化测试 |
| AC-09 | 设置页面可配置存储路径 | 手动验证 |
| AC-10 | 设置页面可配置最大存储空间和自动清理策略 | 自动化测试 |
| AC-11 | 弹幕数量激增时自动标记高光时刻（30条/10秒） | 自动化测试 |
| AC-12 | 收到高价值礼物时自动标记高光时刻 | 自动化测试 |
| AC-13 | 收到舰长赠送时自动标记高光时刻 | 自动化测试 |
| AC-14 | 按 F9 快捷键手动标记高光时刻 | 手动验证 |
| AC-15 | 高光片段可独立保存为 MP4 文件 | 自动化测试 + 文件检查 |
| AC-16 | 录制列表按时间倒序展示所有录制文件 | 自动化测试 |
| AC-17 | 录制列表显示文件名、时长、大小、录制时间 | 自动化测试 |
| AC-18 | 支持在应用内预览录制文件 | 手动验证 |
| AC-19 | 支持重命名、删除、打开文件所在目录 | 手动验证 |
| AC-20 | 支持导出录制文件 | 手动验证 |
| AC-21 | 录制启动延迟 < 3 秒 | 性能测试 |
| AC-22 | 录制过程中 CPU 占用增加 < 15% | 性能测试 |
| AC-23 | 录制过程中内存占用增加 < 200MB | 性能测试 |
| AC-24 | 分段录制无丢帧或卡顿 | 手动验证 + 文件检查 |
| AC-25 | FFmpeg 未安装时显示友好提示 | 手动验证 |
| AC-26 | 磁盘空间不足时停止录制并提示 | 自动化测试 |
| AC-27 | 录制过程中应用退出时优雅停止录制 | 手动验证 |

## 风险

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| FFmpeg 未预装 | 录制功能不可用 | 启动时检测 + 引导用户安装 |
| 直播流 URL 获取失败 | 无法开始录制 | 复用现有连接模块的流获取逻辑 |
| 大文件写入性能 | 录制卡顿 | 使用流式写入 + 关键帧分段 |
| 高光检测误判 | 无意义的高光标记 | 可配置阈值 + 冷却时间 |
| 磁盘空间耗尽 | 录制中断 | 预检查 + 自动清理 + 提示 |
| FFmpeg 进程崩溃 | 录制数据丢失 | 进程监控 + 优雅降级 |
| 快捷键冲突 | 用户体验差 | 支持自定义快捷键 |
| 录制文件编码不兼容 | 无法播放 | 使用通用编码（H.264+AAC） |
