# Bilibili 直播助手 - 产品需求文档 (PRD) v2.0

**版本号**: v2.0
**创建日期**: 2026-06-03
**文档状态**: v2.0 初稿
**最后更新**: 2026-06-16
**基于版本**: v1.0（已完成 78%）

---

## 版本概述

v2.0 版本在 v1.0 基础上新增以下功能：

| 功能 | 优先级 | 状态 |
|------|--------|------|
| 礼物音效系统 | Should Have | ❌ 待实现 |

---

## v2.0 新增功能

### 2.1 礼物音效系统

**功能描述**: 支持使用者为指定礼物设置自定义音效（MP3 格式），当直播间投送对应礼物时自动播放音效，增强直播互动体验和氛围感。

#### 2.1.1 核心功能

- **礼物音效绑定**: 支持为每个礼物单独绑定一个 MP3 音效文件。
- **音效播放**: 收到对应礼物时自动播放绑定的音效。
- **音量控制**: 支持独立控制礼物音效音量，不影响其他音频。
- **音效预览**: 支持在设置页面预览音效效果。
- **批量管理**: 支持批量导入/导出音效配置。

#### 2.1.2 音效配置

- **礼物选择**: 从 Bilibili 礼物列表中选择需要绑定音效的礼物。
- **音效文件**: 支持上传本地 MP3 文件作为礼物音效。
- **音效时长**: 支持设置音效播放时长（截取前 N 秒或完整播放）。
- **播放模式**: 支持单次播放或循环播放（直到礼物连击结束）。
- **冷却时间**: 设置同一礼物音效的最小播放间隔，避免连击时音效过于频繁。

#### 2.1.3 高级功能

- **音效队列**: 多个礼物同时到达时，音效按队列顺序播放，避免混音。
- **音效优先级**: 高价值礼物音效可设置更高优先级，优先播放。
- **音效淡入淡出**: 支持音效淡入淡出效果，避免突然的音量变化。
- **静音时段**: 支持设置静音时段（如夜间自动关闭音效）。

#### 2.1.4 用户交互

- **快捷开关**: 支持一键开启/关闭礼物音效。
- **音效测试**: 在设置页面点击即可测试音效效果。
- **音效库**: 支持管理本地音效文件库。

#### 2.1.5 功能规格

**输入**:
- 礼物事件流（来自 Bilibili WebSocket）
- 音效配置（礼物-音效映射、音量、播放模式）
- 音效文件（本地 MP3 文件）

**输出**:
- 音效音频输出（通过系统音频设备）
- 音效播放日志

**前置条件**:
- 已连接 Bilibili 直播间
- 已配置礼物音效映射
- 音效文件存在且可访问
- 系统音频设备正常

**后置条件**:
- 音效已播放
- 播放日志已记录

**异常处理**:
- 音效文件缺失：跳过播放，记录警告日志
- 音效文件损坏：跳过播放，记录错误日志
- 音频设备异常：跳过播放，提示用户检查设备
- 音效队列满：丢弃最旧的音效，优先播放新音效

**验收标准**:
- 音效触发延迟 < 500ms（从礼物到达到音效播放）
- 音效播放与礼物事件同步误差 < 100ms
- 支持同时处理 10+ 个礼物/秒的音效队列
- 音效播放不影响其他音频（弹幕播报、背景音乐等）
- 音效文件大小支持 1KB ~ 10MB

#### 2.1.6 数据模型

```sql
-- 礼物音效配置表
CREATE TABLE gift_sounds (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    gift_id TEXT NOT NULL,           -- 礼物 ID
    gift_name TEXT NOT NULL,         -- 礼物名称
    sound_file TEXT NOT NULL,        -- 音效文件路径
    sound_name TEXT,                 -- 音效名称
    volume REAL DEFAULT 0.8,         -- 音量 (0-1)
    duration INTEGER DEFAULT 0,      -- 播放时长（毫秒），0=完整播放
    play_mode TEXT DEFAULT 'once',   -- 'once'（单次）, 'loop'（循环）
    cooldown INTEGER DEFAULT 1000,   -- 冷却时间（毫秒）
    priority INTEGER DEFAULT 0,      -- 优先级，数字越大优先级越高
    enabled BOOLEAN DEFAULT TRUE,    -- 是否启用
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 音效播放日志表
CREATE TABLE sound_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    gift_id TEXT NOT NULL,
    gift_name TEXT NOT NULL,
    username TEXT,
    sound_file TEXT NOT NULL,
    played_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

#### 2.1.7 API 设计

```typescript
// 礼物音效配置
interface GiftSoundConfig {
  giftId: string;
  giftName: string;
  soundFile: string;
  soundName: string;
  volume: number;  // 0-1
  duration: number;  // 毫秒，0=完整播放
  playMode: 'once' | 'loop';
  cooldown: number;  // 毫秒
  priority: number;
  enabled: boolean;
}

// 音效播放请求
interface PlaySoundRequest {
  giftId: string;
  giftName: string;
  username: string;
  count: number;
}

// 音效播放结果
interface PlaySoundResult {
  success: boolean;
  soundFile?: string;
  error?: string;
}

// 添加礼物音效配置
function addGiftSound(config: GiftSoundConfig): Promise<void>;

// 更新礼物音效配置
function updateGiftSound(giftId: string, config: Partial<GiftSoundConfig>): Promise<void>;

// 删除礼物音效配置
function deleteGiftSound(giftId: string): Promise<void>;

// 获取礼物音效配置
function getGiftSound(giftId: string): Promise<GiftSoundConfig | null>;

// 获取所有礼物音效配置
function getAllGiftSounds(): Promise<GiftSoundConfig[]>;

// 播放礼物音效
function playGiftSound(request: PlaySoundRequest): Promise<PlaySoundResult>;

// 停止所有音效
function stopAllSounds(): Promise<void>;

// 预览音效
function previewSound(soundFile: string, volume: number): Promise<void>;

// 导入音效配置
function importGiftSounds(configs: GiftSoundConfig[]): Promise<void>;

// 导出音效配置
function exportGiftSounds(): Promise<GiftSoundConfig[]>;
```

#### 2.1.8 技术实现建议

- **音频播放**: 使用 Web Audio API 或 Howler.js 实现音效播放。
- **音效队列**: 使用优先级队列管理多个音效的播放顺序。
- **文件管理**: 音效文件存储在用户指定目录，配置中保存相对路径。
- **音频格式**: 支持 MP3 格式，后续可扩展 WAV、OGG 等格式。
- **音量混合**: 使用 AudioContext 的 GainNode 实现独立音量控制。

---

## v1.0 功能回顾

### 已完成功能

| 功能 | 状态 | 说明 |
|------|------|------|
| 直播间连接与弹幕接收 | ✅ | 通过主播身份码连接 |
| 自动弹幕发送 | ✅ | 定时/随机/触发词 |
| 礼物感谢弹幕 | ✅ | 自动感谢送礼用户 |
| 分级用户互动配置 | ✅ | 舰长/房管/粉丝差异化 |
| 上舰特效展示 | ✅ | WebGL/Canvas 动画 |
| 实时数据看板 | ✅ | 在线人数/弹幕/礼物统计 |
| 黑名单管理 | ✅ | 跨直播间共享 |
| 配置文件保存与加载 | ✅ | JSON 格式导入导出 |
| 敏感词过滤 | ✅ | 自动过滤敏感词 |
| 更新日志与版本管理 | ✅ | 版本检测与更新日志 |
| 基础 UI 界面 | ✅ | Element Plus |
| 插件系统 | ✅ | 第三方插件扩展 |
| 直播录制 | ✅ | 全程/高光录制 |
| 多语言支持 | ✅ | 中文/英文 |
| 弹幕语音播报 | ✅ | edge-tts 集成 |
| 语音识别 | ✅ | whisper.cpp 集成 |
| AI 对话生成 | ✅ | LLM 多提供商支持 |

### 待实现功能

| 功能 | 优先级 | 说明 |
|------|--------|------|
| 自动禁言与刷屏防护 | Must Have | v1.0 遗留 |

---

## 3. 非功能性需求

### 3.1 性能要求

| 指标 | 要求 | 测量方法 |
|------|------|----------|
| 礼物音效触发延迟 | < 500ms | 从礼物到达到音效播放的时间差 |
| 音效队列处理能力 | ≥ 10 个/秒 | 压力测试 |
| CPU 占用增加 | < 5% | 音效播放时的额外 CPU 占用 |
| 内存占用增加 | < 50MB | 音效缓存占用 |

### 3.2 兼容性要求

| 类别 | 要求 |
|------|------|
| 音频格式 | MP3（必须）、WAV/OGG（可选） |
| 音效文件大小 | 1KB ~ 10MB |
| 系统音频 | 支持系统默认音频设备 |

---

## 4. 用户故事

| ID | 用户故事 | 优先级 | 验收标准 | 状态 |
|----|----------|--------|----------|------|
| US-V2-001 | 作为主播，我想要为指定礼物设置音效，以便在收到礼物时播放音效增强互动 | Should | 音效触发延迟 < 500ms，音效播放正常 | ❌ |
| US-V2-002 | 作为主播，我想要控制礼物音效音量，以便不影响其他音频 | Should | 音量可独立调节，范围 0-1 | ❌ |
| US-V2-003 | 作为主播，我想要预览音效效果，以便确认配置正确 | Should | 点击即可预览，播放正常 | ❌ |
| US-V2-004 | 作为主播，我想要批量管理音效配置，以便快速配置多个礼物 | Could | 支持导入/导出 JSON 配置 | ❌ |

---

## 5. 技术架构

### 5.1 模块结构

```
src/
├── main/
│   └── modules/
│       └── gift-sound/        # 礼物音效主进程模块
│           ├── index.ts       # 模块入口
│           ├── audio-player.ts # 音频播放器
│           └── sound-queue.ts  # 音效队列管理
├── renderer/
│   ├── modules/
│   │   └── gift-sound/        # 礼物音效类型定义
│   │       └── types.ts
│   ├── services/
│   │   └── gift-sound-service.ts # 礼物音效服务
│   ├── stores/
│   │   └── gift-sound.ts      # 礼物音效状态管理
│   └── components/
│       └── GiftSoundConfig.vue # 礼物音效配置组件
```

### 5.2 数据流

```
礼物事件 → 音效配置查询 → 音效队列 → 音频播放器 → 系统音频输出
```

---

## 6. 风险评估

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| 音效文件缺失 | 无法播放音效 | 跳过播放，记录日志 |
| 音效队列过长 | 音效延迟 | 优先级队列 + 丢弃策略 |
| 音频设备冲突 | 无法播放 | 独立 AudioContext |
| 大文件加载慢 | 播放延迟 | 预加载 + 缓存 |

---

## 7. 里程碑

| 阶段 | 功能 | 预计时间 |
|------|------|----------|
| Phase 1 | 基础音效播放 | 1 周 |
| Phase 2 | 音效队列和优先级 | 1 周 |
| Phase 3 | 配置页面和预览 | 1 周 |
| Phase 4 | 测试和优化 | 1 周 |
