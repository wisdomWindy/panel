# Bilibili 直播助手

一款集自动化互动、智能回复、特效展示、直播录制、多语言支持于一体的 Bilibili 直播辅助工具。

## 功能特性

### 已完成功能

- ✅ **直播间连接** - 通过主播身份码连接 Bilibili 直播开放平台
- ✅ **弹幕接收与发送** - 实时接收和发送弹幕
- ✅ **弹幕池管理** - 自动发送弹幕池中的内容
- ✅ **礼物感谢** - 自动感谢送礼物的用户
- ✅ **上舰特效** - 首次上舰、续费上舰特效展示
- ✅ **礼物连击特效** - 礼物连击动画效果
- ✅ **敏感词过滤** - 自动过滤敏感词
- ✅ **黑名单管理** - 用户黑名单功能
- ✅ **角色回复** - 主播、房管、舰长专属回复
- ✅ **数据统计** - 在线人数、弹幕数量、礼物价值统计
- ✅ **特效预览** - 2D Canvas 和 3D WebGL 特效预览
- ✅ **日志监控** - 实时日志查看
- ✅ **自动断开** - 关闭应用时自动断开连接
- ✅ **直播录制** - 全程录制、高光标记、片段提取
- ✅ **更新日志** - 版本更新检测与更新日志查看
- ✅ **多语言支持** - 中文/英文界面切换
- ✅ **AI 智能回复** - LLM 多提供商支持
- ✅ **语音识别** - whisper.cpp 集成
- ✅ **自动禁言与刷屏防护** - 检测刷屏自动禁言
- ✅ **礼物音效系统** - 自定义礼物音效播放

### 开发中功能

- 🚧 无

## 技术栈

- **框架**: Electron 30 + Vue 3 + TypeScript
- **状态管理**: Pinia
- **UI 组件**: Element Plus
- **构建工具**: Vite
- **图表**: ECharts
- **3D 特效**: Three.js
- **样式**: Tailwind CSS
- **录制**: FFmpeg (fluent-ffmpeg)
- **国际化**: vue-i18n
- **TTS**: edge-tts
- **语音识别**: whisper.cpp
- **AI**: LLM 多提供商支持

## 快速开始

### 安装依赖

```bash
pnpm install
```

### 开发模式

```bash
pnpm run dev
```

### 构建可执行文件

```bash
pnpm run build:vite
```

构建完成后，可执行文件位于 `release/win-unpacked/BilibiliLiveTool.exe`

## 项目结构

```
panel/
├── src/
│   ├── main/                    # Electron 主进程
│   │   ├── index.ts             # 应用入口
│   │   ├── bilibili-api.ts      # Bilibili API 封装
│   │   ├── database.ts          # 数据库操作
│   │   ├── ipc-handlers.ts      # IPC 通信处理
│   │   └── modules/
│   │       ├── ai/               # AI 对话模块
│   │       ├── asr/              # 语音识别模块
│   │       ├── gift-sound/       # 礼物音效模块
│   │       └── recording/        # 录制模块
│   │           ├── index.ts             # 模块入口
│   │           ├── ffmpeg-manager.ts    # FFmpeg 进程管理
│   │           ├── recording-engine.ts  # 录制引擎
│   │           ├── highlight-detector.ts # 高光检测
│   │           └── file-manager.ts      # 文件管理
│   ├── preload/                 # 预加载脚本
│   └── renderer/                # Vue 渲染进程
│       ├── components/          # 公共组件
│       │   ├── GiftSoundPreview.vue # 礼物音效预览
│       │   ├── RecordingStatusBar.vue  # 录制状态栏
│       │   ├── RecordingList.vue       # 录制文件列表
│       │   └── HighlightList.vue       # 高光列表
│       ├── locales/             # 国际化语言包
│       │   ├── zh-CN.json       # 中文语言包
│       │   └── en-US.json       # 英文语言包
│       ├── modules/             # 核心模块
│       │   ├── bilibili/        # Bilibili 客户端
│       │   ├── danmaku/         # 弹幕管理
│       │   ├── effects/         # 特效系统
│       │   ├── gift-sound/      # 礼物音效类型定义
│       │   ├── recording/       # 录制类型定义
│       │   └── utils/           # 工具函数
│       ├── services/            # 服务层
│       │   ├── gift-sound-service.ts # 礼物音效服务
│       │   └── recording-service.ts  # 录制服务
│       ├── stores/              # Pinia 状态管理
│       │   ├── ai.ts            # AI 状态管理
│       │   ├── asr.ts           # 语音识别状态管理
│       │   ├── gift-sound.ts    # 礼物音效状态管理
│       │   ├── moderation.ts    # 内容审核状态管理
│       │   ├── recording.ts     # 录制状态管理
│       │   └── tts.ts           # TTS 状态管理
│       ├── views/               # 页面组件
│       │   ├── GiftSoundConfig.vue  # 礼物音效配置
│       │   └── Recording.vue    # 录制管理页面
│       └── router/              # 路由配置
├── docs/                        # 项目文档
├── tests/                       # 测试文件
└── scripts/                     # 构建脚本
```

## 使用说明

1. 从 [Bilibili 互动应用中心](https://play-live.bilibili.com/) 获取主播身份码
2. 启动应用，输入身份码连接
3. 在设置页面配置 AccessKeyId、AccessKeySecret 和 AppID
4. 开始使用各项功能

## 直播录制功能

### 前置条件

- 安装 FFmpeg：`brew install ffmpeg` (macOS) 或从 [FFmpeg 官网](https://ffmpeg.org/) 下载

### 功能说明

- **全程录制**: 支持 MP4/FLV 格式，可选质量（原始/720p/1080p）
- **高光标记**: 
  - 自动标记：弹幕激增、高价值礼物、舰长赠送
  - 手动标记：按 F9 快捷键
- **录制管理**: 文件列表、删除、打开目录
- **状态栏**: 实时显示录制状态、时长、文件大小

### 配置说明

在设置页面的「录制设置」中可以配置：
- 录制格式（MP4/FLV）
- 录制质量（原始/720p/1080p）
- 存储路径
- 自动分段
- 自动清理
- 高光触发阈值

## 多语言支持

### 支持语言

- **中文** (zh-CN) - 默认语言
- **English** (en-US)

### 切换语言

1. 打开设置页面
2. 选择「语言设置」
3. 从下拉菜单选择语言
4. 界面将立即切换到所选语言

### 扩展语言

如需添加新语言：

1. 在 `src/renderer/locales/` 目录创建新的语言文件（如 `ja-JP.json`）
2. 复制 `zh-CN.json` 的结构并翻译所有文本
3. 在 `src/renderer/main.ts` 中导入新语言包
4. 在 `src/renderer/views/Settings.vue` 中添加语言选项

## 配置说明

### Bilibili 直播开放平台配置

- **AccessKeyId**: 开放平台 Access Key ID
- **AccessKeySecret**: 开放平台 Access Key Secret
- **AppID**: 应用 ID

### 弹幕配置

- **最小发送间隔**: 弹幕发送的最小间隔时间
- **自动发送**: 是否启用弹幕池自动发送
- **敏感词过滤**: 是否启用敏感词过滤

### 特效配置

- **渲染模式**: 2D Canvas 或 3D WebGL
- **动画类型**: 淡入淡出、滑动、缩放、旋转、粒子
- **持续时间**: 特效显示时长

### AI 配置

- **AI 提供商**: OpenAI、Claude、Ollama 或自定义
- **API Key**: AI 服务 API Key（加密存储）
- **模型选择**: GPT-4、Claude、本地模型等
- **人设配置**: AI 机器人性格和回复风格

### 语音识别配置

- **识别引擎**: whisper.cpp（推荐）或 Web Speech API
- **模型大小**: tiny、base、small、medium、large
- **语言**: 中文、英文、自动检测

### TTS 配置

- **TTS 引擎**: edge-tts（推荐）或 Web Speech API
- **语音选择**: 18 种中文语音
- **语速调节**: 0.5x - 2.0x
- **音量控制**: 0 - 100%

### 礼物音效配置

- **音效绑定**: 为每个礼物绑定自定义 MP3 音效
- **音量控制**: 独立控制礼物音效音量
- **播放模式**: 单次播放或循环播放
- **冷却时间**: 设置同一礼物音效的最小播放间隔

## 许可证

MIT

## 更新日志

### v2.0.0 (2026-06-17)

- ✨ 新增礼物音效系统功能
- ✨ 新增 AI 智能回复功能
- ✨ 新增语音识别功能（whisper.cpp）
- ✨ 新增自动禁言与刷屏防护功能
- 🌐 完善多语言支持

### v1.5.0 (2026-06-16)

- ✨ 新增弹幕语音播报功能（edge-tts）
- ✨ 新增直播录制功能（全程/高光）
- ✨ 新增多语言支持（中文/英文）

### v1.0.0 (2026-06-03)

- 🎉 初始版本发布
- ✨ 基础直播间连接与弹幕功能
- ✨ 礼物感谢与上舰特效
- ✨ 数据统计与日志监控
