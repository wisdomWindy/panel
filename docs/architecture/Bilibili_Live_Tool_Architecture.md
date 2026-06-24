# Bilibili 直播助手 - 技术架构设计文档

**版本**: v1.1
**创建日期**: 2026-06-03
**更新日期**: 2026-06-04
**文档状态**: 更新版（反映实际实现）

---

## 目录

1. [技术选型](#1-技术选型)
2. [系统架构](#2-系统架构)
3. [目录结构](#3-目录结构)
4. [模块设计](#4-模块设计)
5. [数据流设计](#5-数据流设计)
6. [进程间通信](#6-进程间通信)
7. [技术风险与应对](#7-技术风险与应对)
8. [性能优化策略](#8-性能优化策略)

---

## 1. 技术选型

### 1.1 框架选型对比

#### 1.1.1 桌面应用框架：Electron vs Tauri

| 维度 | Electron | Tauri | 选择 |
|------|----------|-------|------|
| **跨平台支持** | Windows/macOS/Linux | Windows/macOS/Linux | 平局 |
| **包体积** | ~150MB | ~10MB | Tauri ✅ |
| **内存占用** | 较高 (~200MB+) | 较低 (~50MB) | Tauri ✅ |
| **开发效率** | 高（成熟生态） | 中（较新） | Electron ✅ |
| **社区支持** | 非常活跃 | 成长中 | Electron ✅ |
| **原生能力** | 较弱 | 强（Rust 后端） | Tauri ✅ |
| **学习曲线** | 平缓（Node.js） | 陡峭（Rust） | Electron ✅ |
| **成熟度** | 非常成熟 | 较新 | Electron ✅ |

**最终选择**: **Electron**

**理由**:
1. 开发团队熟悉 Node.js/JavaScript，学习成本低
2. 生态成熟，第三方库丰富
3. 调试工具完善，开发效率高
4. 社区支持强大，遇到问题容易解决
5. 对于 MVP 阶段，开发效率比包体积更重要

#### 1.1.2 前端框架：Vue 3 vs React

| 维度 | Vue 3 | React | 选择 |
|------|-------|-------|------|
| **学习曲线** | 平缓 | 陡峭 | Vue 3 ✅ |
| **开发效率** | 高 | 中 | Vue 3 ✅ |
| **性能** | 优秀 | 优秀 | 平局 |
| **生态系统** | 丰富 | 非常丰富 | React 稍优 |
| **TypeScript 支持** | 优秀 | 优秀 | 平局 |
| **组件化** | 强 | 强 | 平局 |
| **文档质量** | 优秀 | 优秀 | 平局 |

**最终选择**: **Vue 3 + TypeScript**

**理由**:
1. 组合式 API (Composition API) 更符合直觉
2. 模板语法更接近 HTML，易于理解
3. 官方提供完整工具链（Vite、Pinia、Vue Router）
4. 性能与 React 相当，但代码更简洁

### 1.2 核心技术栈

```json
{
  "桌面框架": {
    "Electron": "^30.0.0"
  },
  "前端框架": {
    "Vue": "^3.4.0"
  },
  "开发语言": {
    "TypeScript": "^5.3.0"
  },
  "构建工具": {
    "Vite": "^5.0.0",
    "vite-plugin-electron": "用于 Electron 主进程/预加载脚本构建",
    "vite-plugin-electron-renderer": "用于渲染进程 Node.js API 支持"
  },
  "UI 组件库": {
    "Element Plus": "^2.5.0"
  },
  "CSS 框架": {
    "Tailwind CSS": "^4.0.0 (通过 @tailwindcss/postcss 插件)"
  },
  "状态管理": {
    "Pinia": "^2.1.0"
  },
  "路由": {
    "vue-router": "^4.2.5"
  },
  "数据可视化": {
    "ECharts": "^6.1.0"
  },
  "3D 渲染": {
    "Three.js": "^0.184.0"
  },
  "网络通信": {
    "axios": "^1.6.0 (HTTP 客户端)",
    "WebSocket": "浏览器原生 WebSocket API (连接 Bilibili 直播间)",
    "pako": "^2.1.0 (Brotli/gzip 消息解压)"
  },
  "配置存储": {
    "electron-store": "^11.0.2 (主进程持久化配置)"
  },
  "日志系统": {
    "winston": "^3.19.0"
  },
  "加密": {
    "crypto-js": "^4.2.0"
  },
  "工具库": {
    "dayjs": "^1.11.21",
    "lodash": "^4.18.1",
    "@vueuse/core": "^10.7.0"
  }
}
```

### 1.3 依赖说明

#### 生产依赖

| 包名 | 用途 | 说明 |
|------|------|------|
| electron | 桌面应用框架 | 核心依赖 |
| vue | 前端框架 | 渲染进程 UI |
| vue-router | 路由 | 页面导航 (Hash 模式) |
| pinia | 状态管理 | 全局状态管理 |
| element-plus | UI 组件库 | 快速构建界面 |
| echarts | 数据可视化 | Dashboard 图表展示 |
| three | 3D 渲染引擎 | WebGL 特效渲染 |
| pako | 压缩库 | 解压 Bilibili 直播协议消息 |
| axios | HTTP 客户端 | 调用 AI API |
| electron-store | 配置存储 | 主进程持久化配置 |
| winston | 日志系统 | 结构化日志 |
| crypto-js | 加密库 | API Key 加密 |
| dayjs | 日期工具 | 时间格式化 |
| lodash | 工具函数 | 通用工具 |
| @vueuse/core | Vue 组合式工具集 | 响应式工具函数 |

#### 开发依赖

| 包名 | 用途 | 说明 |
|------|------|------|
| vite | 构建工具 | 快速开发 |
| typescript | TypeScript 编译器 | 类型安全 |
| @vitejs/plugin-vue | Vite Vue 插件 | Vue SFC 编译 |
| vite-plugin-electron | Electron 构建插件 | 主进程/预加载脚本构建 |
| vite-plugin-electron-renderer | Electron 渲染插件 | 渲染进程 Node.js 集成 |
| @tailwindcss/postcss | Tailwind CSS PostCSS 插件 | CSS 框架 (v4) |
| tailwindcss | Tailwind CSS | 原子化 CSS 框架 |
| autoprefixer | CSS 前缀自动添加 | 浏览器兼容性 |
| @vue/tsconfig | Vue TypeScript 配置 | tsconfig 基础配置 |
| @vue/typescript-plugin | Vue TypeScript 插件 | IDE 支持 |
| eslint | 代码检查 | 代码质量 |
| @typescript-eslint/eslint-plugin | TS ESLint 插件 | TypeScript 规则 |
| @typescript-eslint/parser | TS ESLint 解析器 | TypeScript 解析 |
| prettier | 代码格式化 | 代码风格 |
| postcss | CSS 后处理器 | Tailwind CSS 依赖 |

---

## 2. 系统架构

### 2.1 整体架构图

```
┌─────────────────────────────────────────────────────────────┐
│                    Electron 应用                              │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌───────────────────────────────────────────────────────┐   │
│  │                    主进程                               │   │
│  │  ┌───────────┐  ┌───────────┐  ┌───────────────────┐  │   │
│  │  │窗口管理器   │  │生命周期管理 │  │安全隔离           │  │   │
│  │  │BrowserWndw│  │App Lifecycle│  │contextIsolation  │  │   │
│  │  └───────────┘  └───────────┘  └───────────────────┘  │   │
│  └───────────────────────────────────────────────────────┘   │
│                           │                                   │
│                  IPC (contextBridge)                          │
│                  electronAPI.invoke/on/off                    │
│                           │                                   │
│  ┌───────────────────────────────────────────────────────┐   │
│  │                    渲染进程                               │   │
│  │  ┌───────────┐  ┌───────────┐  ┌───────────────────┐  │   │
│  │  │Vue 应用     │  │Pinia Store│  │Vue Router         │  │   │
│  │  │App.vue    │  │config     │  │Hash 模式           │  │   │
│  │  │           │  │danmaku    │  │8 个懒加载路由       │  │   │
│  │  │           │  │room       │  │                    │  │   │
│  │  └───────────┘  └───────────┘  └───────────────────┘  │   │
│  │  ┌─────────────────────────────────────────────────┐   │   │
│  │  │              Views (页面组件)                      │   │   │
│  │  │  ┌──────────┐ ┌──────────┐ ┌────────────────┐    │   │   │
│  │  │  │Dashboard │ │Connection│ │DanmakuList      │    │   │   │
│  │  │  │(ECharts) │ │(连接管理) │ │(弹幕列表)       │    │   │   │
│  │  │  └──────────┘ └──────────┘ └────────────────┘    │   │   │
│  │  │  ┌──────────┐ ┌──────────┐ ┌────────────────┐    │   │   │
│  │  │  │DanmakuPool│ │GiftList  │ │Blacklist       │    │   │   │
│  │  │  │(弹幕池)   │ │(礼物列表) │ │(黑名单)        │    │   │   │
│  │  │  └──────────┘ └──────────┘ └────────────────┘    │   │   │
│  │  │  ┌──────────┐ ┌──────────┐                       │   │   │
│  │  │  │Settings  │ │Logs      │                       │   │   │
│  │  │  │(设置)    │ │(日志)    │                       │   │   │
│  │  │  └──────────┘ └──────────┘                       │   │   │
│  │  └─────────────────────────────────────────────────┘   │   │   │
│  │  ┌─────────────────────────────────────────────────┐   │   │   │
│  │  │              Services (服务层)                      │   │   │
│  │  │  ┌───────────────────────────────────────────┐  │   │   │
│  │  │  │ danmaku-service.ts (弹幕服务)                │  │   │   │
│  │  │  │ database-service.ts (数据库服务)             │  │   │   │
│  │  │  └───────────────────────────────────────────┘  │   │   │
│  │  └─────────────────────────────────────────────────┘   │   │   │
│  │  ┌─────────────────────────────────────────────────┐   │   │   │
│  │  │              Modules (业务逻辑模块)                 │   │   │
│  │  │  ┌──────────┐ ┌──────────┐ ┌────────────────┐    │   │   │
│  │  │  │danmaku/  │ │bilibili/ │ │effects/        │    │   │   │
│  │  │  │manager   │ │client    │ │effect-manager  │    │   │   │
│  │  │  │sender    │ │protocol  │ │canvas-renderer │    │   │   │
│  │  │  │parser    │ │types     │ │webgl-renderer  │    │   │   │
│  │  │  │blacklist │ │          │ │sound-manager   │    │   │   │
│  │  │  │types     │ │          │ │types           │    │   │   │
│  │  │  └──────────┘ └──────────┘ └────────────────┘    │   │   │
│  │  │  ┌──────────┐ ┌──────────────────────────────┐   │   │   │
│  │  │  │ai/       │ │utils/                        │   │   │   │
│  │  │  │ai-manager│ │sensitive-words.ts            │   │   │   │
│  │  │  │ai-service│ │variable-replacer.ts          │   │   │   │
│  │  │  │asr-srvce │ │logger.ts                     │   │   │   │
│  │  │  │types     │ │                              │   │   │   │
│  │  │  └──────────┘ └──────────────────────────────┘   │   │   │
│  │  └─────────────────────────────────────────────────┘   │   │   │
│  └───────────────────────────────────────────────────────┘   │
│                                                               │
└─────────────────────────────────────────────────────────────┘
         │                                    │
         │ WebSocket (原生)                    │ HTTP/HTTPS (axios)
         │                                    │
    ┌────▼─────────┐                  ┌──────▼──────────┐
    │ Bilibili     │                  │ 第三方 AI 服务    │
    │ 直播间       │                  │ (ASR / LLM)     │
    └──────────────┘                  └─────────────────┘
```

### 2.2 进程职责划分

#### 2.2.1 主进程 (src/main/index.ts)

**职责**:
1. 窗口生命周期管理（创建、显示、关闭）
2. 开发/生产环境切换
3. 安全隔离（contextIsolation: true, nodeIntegration: false）
4. macOS Dock 行为处理

**当前实现状态**: MVP 阶段，主进程仅包含基础窗口管理。后续迭代将逐步添加数据库操作、系统托盘、全局快捷键、自动更新等功能。

#### 2.2.2 预加载脚本 (src/preload/index.ts)

**职责**:
1. 通过 `contextBridge.exposeInMainWorld` 安全暴露 API
2. 提供 `electronAPI.invoke(channel, ...args)` 用于渲染进程调用主进程
3. 提供 `electronAPI.on(channel, callback)` 用于监听主进程事件
4. 提供 `electronAPI.off(channel, callback)` 用于移除事件监听

#### 2.2.3 渲染进程 (src/renderer/)

**职责**:
1. Vue 3 应用 UI 渲染与交互
2. 通过 Pinia Store 管理全局状态
3. 通过 Vue Router (Hash 模式) 管理 8 个页面路由
4. 通过 Services 层调用模块业务逻辑
5. Modules 层包含所有核心业务逻辑：
   - **danmaku/**: 弹幕管理、发送、解析、黑名单
   - **bilibili/**: Bilibili 直播协议 WebSocket 客户端
   - **effects/**: Canvas 2D + Three.js WebGL 特效渲染、音效管理
   - **ai/**: AI 管理、AI 服务调用、ASR 语音识别
   - **utils/**: 敏感词过滤、变量替换、日志工具

### 2.3 架构原则

1. **单一职责原则**: 每个模块只负责一个功能
2. **关注点分离**: Views 层、Services 层、Modules 层分离
3. **事件驱动**: 使用 EventEmitter 模式进行模块间通信
4. **类型安全**: 全面使用 TypeScript，每个模块有独立的 types.ts
5. **配置驱动**: 通过 Pinia configStore 管理所有配置，持久化到 localStorage
6. **渐进增强**: MVP 阶段先实现核心功能，后续迭代逐步完善

---

## 3. 目录结构

```
bilibili-live-tool/
├── scripts/                    # 脚本工具
│   ├── dev.js                  # 开发脚本
│   └── build.js                # 构建脚本
├── resources/                  # 资源文件（预留，当前为空）
├── public/                     # 公共静态资源
├── src/                        # 源代码
│   ├── main/                   # 主进程
│   │   ├── index.ts            # 主进程入口（窗口管理 + 生命周期）
│   │   ├── bilibili-api.ts     # Bilibili API 封装
│   │   ├── database.ts         # 数据库操作
│   │   ├── ipc-handlers.ts     # IPC 通信处理
│   │   └── modules/            # 主进程模块
│   │       ├── ai/             # AI 对话模块
│   │       ├── asr/            # 语音识别模块
│   │       ├── gift-sound/     # 礼物音效模块
│   │       └── recording/      # 录制模块
│   │           ├── index.ts            # 模块入口
│   │           ├── ffmpeg-manager.ts   # FFmpeg 进程管理
│   │           ├── recording-engine.ts # 录制引擎
│   │           ├── highlight-detector.ts # 高光检测
│   │           └── file-manager.ts     # 文件管理
│   ├── preload/                # 预加载脚本
│   │   └── index.ts            # contextBridge API 暴露
│   └── renderer/               # 渲染进程
│       ├── index.html          # HTML 入口
│       ├── main.ts             # Vue 应用入口
│       ├── App.vue             # 根组件（Element Plus 布局 + 路由视图）
│       ├── router/             # 路由配置
│       │   └── index.ts        # 懒加载路由
│       ├── stores/             # Pinia 状态管理
│       │   ├── ai.ts           # AI 状态
│       │   ├── asr.ts          # 语音识别状态
│       │   ├── config.ts       # 配置状态
│       │   ├── danmaku.ts      # 弹幕状态
│       │   ├── gift-sound.ts   # 礼物音效状态
│       │   ├── moderation.ts   # 审核状态
│       │   ├── plugin.ts       # 插件状态
│       │   ├── recording.ts    # 录制状态
│       │   ├── room.ts         # 房间状态
│       │   ├── tts.ts          # TTS 状态
│       │   └── update.ts       # 更新状态
│       ├── services/           # 服务层
│       │   ├── asr-service.ts      # ASR 服务
│       │   ├── tts-service.ts      # TTS 服务
│       │   ├── llm-service.ts      # LLM 服务
│       │   ├── recording-service.ts # 录制服务
│       │   ├── danmaku-service.ts  # 弹幕服务
│       │   ├── gift-sound-service.ts # 礼物音效服务
│       │   ├── database-service.ts # 数据库服务
│       │   ├── moderation-service.ts # 内容审核服务
│       │   └── update-service.ts   # 更新服务
│       ├── engines/            # TTS/ASR 引擎层 (Strategy 模式)
│       │   ├── tts-engine.ts       # TTS 引擎接口
│       │   ├── asr-engine.ts       # ASR 引擎接口
│       │   ├── edge-tts-engine.ts  # Edge TTS 实现
│       │   ├── web-speech-tts-engine.ts # Web Speech TTS 实现
│       │   ├── web-speech-asr-engine.ts # Web Speech ASR 实现
│       │   ├── whisper-asr-engine.ts # Whisper ASR 实现
│       │   └── engine-factory.ts   # 引擎工厂
│       ├── composables/        # Vue 组合式函数
│       │   ├── useAudioRecording.ts    # 音频录制
│       │   ├── useBroadcastQueue.ts    # 播报队列
│       │   ├── useChatContext.ts       # 聊天上下文
│       │   ├── useReplyLogic.ts        # 回复判断逻辑
│       │   └── useWebSpeechRecognition.ts # Web Speech 识别
│       ├── utils/              # 公共工具函数
│       │   ├── format.ts       # 格式化工具
│       │   ├── ipc.ts          # IPC 通信封装
│       │   └── ui.ts           # UI 工具函数
│       ├── plugins/            # 插件系统
│       │   ├── types.ts        # 插件类型定义
│       │   ├── plugin-api.ts   # 插件 API
│       │   ├── plugin-loader.ts # 插件加载器
│       │   ├── plugin-manager.ts # 插件管理器
│       │   └── plugin-sandbox.ts # 插件沙箱
│       ├── modules/            # 业务逻辑模块
│       │   ├── bilibili/       # Bilibili 协议
│       │   │   ├── client.ts   # WebSocket 客户端
│       │   │   ├── protocol.ts # 协议解析
│       │   │   ├── api.ts      # 开放平台 API
│       │   │   └── types.ts    # 类型定义
│       │   ├── danmaku/        # 弹幕模块
│       │   │   ├── manager.ts  # 弹幕管理器 (EventEmitter)
│       │   │   ├── sender.ts   # 弹幕发送器
│       │   │   ├── parser.ts   # 弹幕解析器
│       │   │   ├── blacklist.ts # 黑名单管理
│       │   │   └── types.ts    # 类型定义
│       │   ├── effects/        # 特效模块
│       │   │   ├── effect-manager.ts  # 特效管理器
│       │   │   ├── canvas-renderer.ts # Canvas 2D 渲染器
│       │   │   ├── webgl-renderer.ts  # Three.js WebGL 渲染器
│       │   │   ├── sound-manager.ts   # 音效管理器
│       │   │   └── types.ts    # 类型定义
│       │   ├── ai/             # AI 类型定义
│       │   ├── asr/            # ASR 类型定义
│       │   ├── tts/            # TTS 类型定义
│       │   ├── recording/      # 录制类型定义
│       │   ├── gift-sound/     # 礼物音效类型定义
│       │   └── utils/           # 工具模块
│       │       ├── sensitive-words.ts  # 敏感词过滤
│       │       └── variable-replacer.ts # 变量替换
│       ├── views/              # 页面模块 (每个模块独立文件夹)
│       │   ├── dashboard/      # 仪表盘
│       │   │   ├── index.vue
│       │   │   └── components/
│       │   │       ├── StatCard.vue
│       │   │       └── Leaderboard.vue
│       │   ├── connection/     # 连接配置
│       │   ├── danmaku-list/   # 弹幕列表
│       │   ├── danmaku-pool/   # 弹幕池管理
│       │   ├── settings/       # 设置
│       │   │   ├── index.vue
│       │   │   └── components/ # 设置子组件 (14 个)
│       │   ├── recording/      # 录制管理
│       │   │   ├── index.vue
│       │   │   └── components/
│       │   │       ├── RecordingList.vue
│       │   │       └── HighlightList.vue
│       │   ├── gift-list/      # 礼物列表
│       │   ├── gift-sound/     # 礼物音效配置
│       │   ├── effect-preview/ # 特效预览
│       │   ├── blacklist/      # 黑名单管理
│       │   ├── logs/           # 日志监控
│       │   ├── plugin-manager/ # 插件管理
│       │   ├── update-log/     # 更新日志
│       │   └── login/          # 登录
│       ├── components/         # 全局共享组件
│       │   ├── RecordingStatusBar.vue # 录制状态栏
│       │   └── WhatsNewModal.vue # 新版本特性弹窗
│       ├── locales/            # 国际化语言包
│       │   ├── zh-CN.json      # 中文语言包
│       │   └── en-US.json      # 英文语言包
│       └── typings/            # 类型声明
│           ├── whisper-node.d.ts
│           ├── edge-tts.d.ts
│           └── element-plus-locale.d.ts
├── docs/                       # 项目文档
├── tests/                      # 测试文件
└── scripts/                    # 构建脚本
```

---

## 4. 模块设计

### 4.1 弹幕管理模块 (src/renderer/modules/danmaku/)

**职责**:
1. 管理弹幕生命周期（接收、解析、存储、发送）
2. 弹幕池管理（定时/随机/触发词弹幕）
3. 自动发送弹幕（变量替换 + 敏感词过滤）
4. 触发词回复匹配
5. 黑名单过滤

**核心类**:

```typescript
// src/renderer/modules/danmaku/manager.ts
export class DanmakuManager extends EventEmitter {
  private client: BilibiliClient;
  private sender: DanmakuSender;
  private parser: DanmakuParser;
  private blacklist: BlacklistManager;
  
  // 连接直播间
  async connect(roomId: number): Promise<void>;
  
  // 断开连接
  disconnect(): void;
  
  // 手动发送弹幕
  async send(content: string): Promise<void>;
  
  // 添加、移除弹幕池项
  addPoolItem(item: DanmakuPoolItem): void;
  removePoolItem(id: string): void;
}
```

**相关类**:
- `DanmakuSender`: 弹幕发送器（频率限制、敏感词过滤、变量替换）
- `DanmakuParser`: 弹幕解析器（消息解码、字段提取、类型判断）
- `BlacklistManager`: 黑名单管理（添加、移除、检查）

**数据流**:
```
WebSocket 消息 → BilibiliClient → DanmakuParser → DanmakuManager
                                                      ├── 触发词匹配 → DanmakuSender → WebSocket
                                                      ├── 弹幕池定时发送 → DanmakuSender → WebSocket
                                                      └── 黑名单过滤
```

### 4.2 Bilibili 协议模块 (src/renderer/modules/bilibili/)

**职责**:
1. 管理 WebSocket 连接（原生 WebSocket API）
2. Bilibili 直播协议编解码
3. 心跳保活
4. 自动重连（指数退避）
5. pako 解压 Brotli/gzip 消息

**核心类**:

```typescript
// src/renderer/modules/bilibili/client.ts
export class BilibiliClient extends EventEmitter {
  private ws: WebSocket | null;
  private roomId: number;
  private heartbeatTimer: NodeJS.Timeout | null;
  
  // 连接指定直播间
  async connect(roomId: number): Promise<void>;
  
  // 断开连接
  disconnect(): void;
  
  // 发送消息
  send(data: object): void;
  
  // 获取直播间信息
  async getRoomInfo(roomId: number): Promise<RoomInfo>;
  
  // 获取真实房间号
  async getRealRoomId(roomId: number): Promise<number>;
}
```

**协议处理** (protocol.ts):
- 消息打包/解包
- pako 解压
- 操作码处理（认证响应、心跳响应、消息通知等）

### 4.3 特效渲染模块 (src/renderer/modules/effects/)

**职责**:
1. 特效队列管理
2. Canvas 2D 渲染（文字弹幕、简单粒子）
3. Three.js WebGL 渲染（3D 特效、粒子系统）
4. 音效播放
5. 性能控制（限制并发特效数量）

**核心类**:

```typescript
// src/renderer/modules/effects/effect-manager.ts
export class EffectManager {
  private canvasRenderer: CanvasRenderer;
  private webglRenderer: WebGLRenderer;
  private soundManager: SoundManager;
  
  // 添加特效
  addEffect(effect: Effect): void;
  
  // 移除特效
  removeEffect(id: string): void;
  
  // 清空所有特效
  clearAll(): void;
}
```

**渲染器类型**:
- **CanvasRenderer**: 基于 Canvas 2D API，适合文字特效、简单动画
- **WebGLRenderer**: 基于 Three.js，适合 3D 粒子特效、复杂动画
- **SoundManager**: 音效播放（Web Audio API），支持淡入淡出

**特效数据类型**:
```typescript
// src/renderer/modules/effects/types.ts
interface Effect {
  id: string;
  type: 'text' | 'particle' | '3d';
  duration: number;
  position: { x: number; y: number };
  config: EffectConfig;
}
```

### 4.4 AI 模块 (src/renderer/modules/ai/)

**职责**:
1. LLM 回复生成（通过 axios 调用 AI API）
2. ASR 语音识别（将直播间语音转为文本）
3. AI 回复管理（上下文管理、回复策略）

**核心类**:

```typescript
// src/renderer/modules/ai/ai-manager.ts
export class AIManager {
  private aiService: AIService;
  private asrService: ASRService;
  
  // 生成 AI 回复
  async generateReply(context: string[]): Promise<string>;
  
  // 开始语音识别
  async startASR(): Promise<void>;
  
  // 停止语音识别
  stopASR(): void;
}
```

### 4.5 Pinia Store 设计

**configStore** (src/renderer/stores/config.ts):
```typescript
interface AppConfig {
  // 直播间配置
  autoReconnect: boolean;
  reconnectInterval: number;
  maxReconnectAttempts: number;
  heartbeatInterval: number;
  
  // 弹幕发送配置
  minSendInterval: number;      // 最小发送间隔 (ms)
  enableAutoSend: boolean;      // 启用自动发送
  autoSendInterval: number;     // 自动发送间隔 (ms)
  enableTriggerReply: boolean;  // 启用触发词回复
  enableGiftThank: boolean;     // 启用礼物感谢
  
  // 敏感词过滤
  enableSensitiveWordFilter: boolean;
  sensitiveWords: string[];
  replaceChar: string;
  
  // UI 配置
  theme: 'light' | 'dark' | 'auto';
  language: 'zh-CN' | 'en-US';
  fontSize: number;
  enableNotifications: boolean;
  
  // 日志配置
  enableLog: boolean;
  logLevel: 'debug' | 'info' | 'warn' | 'error';
  maxLogLines: number;
}
```

**danmakuStore** (src/renderer/stores/danmaku.ts):
- 弹幕列表（接收的弹幕）
- 弹幕池项管理
- 发送队列状态

**roomStore** (src/renderer/stores/room.ts):
- 直播间连接状态
- 房间信息
- 在线人数

---

## 5. 数据流设计

### 5.1 弹幕数据流

```
Bilibili 直播间
    │ WebSocket (原生)
    │ 二进制消息 (Brotli/gzip 压缩)
    ↓
┌─────────────────┐
│ BilibiliClient  │
│ - 连接管理       │
│ - pako 解压      │
│ - 心跳保活       │
│ - 自动重连       │
└─────────────────┘
    │ 解压后 JSON
    ↓
┌─────────────────┐
│ DanmakuParser   │
│ - 协议字段提取   │
│ - 消息类型判断   │
│ - Danmaku/Gift 对象构造 │
└─────────────────┘
    │ Danmaku 对象
    ↓
┌─────────────────┐
│ DanmakuManager  │
│ - 触发词匹配     │
│ - 黑名单检查     │
│ - 弹幕池管理     │
└─────────────────┘
    │ 匹配结果
    ├──────────────┬──────────────┐
    ↓              ↓              ↓
┌───────────┐ ┌───────────┐ ┌───────────┐
│自动回复    │ │特效触发    │ │日志记录    │
│DanmakuSndr│ │EffectMgr  │ │Logger     │
└───────────┘ └───────────┘ └───────────┘
```

### 5.2 配置数据流

```
用户操作 (Settings.vue)
    │
    ↓
┌─────────────────┐
│ configStore      │
│ (Pinia)          │
│ - updateConfig() │
│ - 立即持久化     │
└─────────────────┘
    │ localStorage
    │ key: 'bilibili-live-config'
    ↓
┌─────────────────┐
│ localStorage    │
│ (浏览器存储)    │
└─────────────────┘
```

### 5.3 页面路由结构

```
/                   → Dashboard.vue     (主界面，ECharts 图表)
/connection         → Connection.vue    (直播间连接)
/danmaku-list       → DanmakuList.vue   (弹幕列表)
/danmaku-pool       → DanmakuPool.vue   (弹幕池管理)
/gift-list          → GiftList.vue      (礼物列表)
/blacklist          → Blacklist.vue     (黑名单管理)
/settings           → Settings.vue      (应用设置)
/logs               → Logs.vue          (日志查看)
```

---

## 6. 进程间通信 (IPC)

### 6.1 预加载脚本 API

```typescript
// src/preload/index.ts
contextBridge.exposeInMainWorld('electronAPI', {
  invoke: (channel: string, ...args: any[]) => 
    ipcRenderer.invoke(channel, ...args),
  on: (channel: string, callback: (...args: any[]) => void) => 
    ipcRenderer.on(channel, (event, ...args) => callback(...args)),
  off: (channel: string, callback: (...args: any[]) => void) => 
    ipcRenderer.removeListener(channel, callback)
});
```

### 6.2 安全策略

- **contextIsolation**: `true` — 渲染进程与主进程完全隔离
- **nodeIntegration**: `false` — 渲染进程无法直接访问 Node.js API
- **sandbox**: 默认 `true` — 沙箱模式
- **preload**: 通过预加载脚本安全暴露有限 API

### 6.3 当前 IPC 使用状态

MVP 阶段：目前主进程没有注册 IPC handlers，渲染进程主要通过以下方式运行：
- 渲染进程内 Modules 直接处理所有业务逻辑
- 使用 `localStorage` 进行配置持久化
- WebSocket 连接和 AI API 调用均在渲染进程内完成

后续迭代计划：将数据库操作、文件系统访问等需要 Node.js 能力的功能移至主进程，通过 IPC 暴露给渲染进程。

---

## 7. 技术风险与应对

### 7.1 WebSocket 连接稳定性

**风险**: Bilibili 直播协议变更或网络波动导致连接中断

**应对方案**:
1. 心跳机制，定期检测连接状态（Bilibili 协议要求每 30 秒发送心跳）
2. 自动重连机制，指数退避策略
3. 连接状态通过 roomStore 实时反映到 UI
4. pako 库处理多种压缩格式（Brotli/gzip）

### 7.2 自动弹幕风控

**风险**: 自动弹幕频率过高导致账号被限流或封禁

**应对方案**:
1. 严格设置发送间隔（通过 configStore.minSendInterval 配置）
2. DanmakuSender 内置频率限制器
3. 敏感词过滤（sensitive-words.ts）避免违禁内容
4. 变量替换增加随机性

### 7.3 AI 延迟问题

**风险**: 语音识别和 LLM 生成存在延迟，导致互动不及时

**应对方案**:
1. 使用流式 API（如支持）
2. 设置超时机制，超时使用预设回复
3. 异步处理，不阻塞弹幕接收

### 7.4 特效渲染性能

**风险**: WebGL 特效和大量弹幕渲染占用过多 GPU

**应对方案**:
1. 限制并发特效数量
2. Canvas 2D 和 WebGL 分场景使用
3. requestAnimationFrame 优化渲染循环
4. 低性能模式下关闭 WebGL 特效，仅用 Canvas

---

## 8. 性能优化策略

### 8.1 渲染性能优化

1. **懒加载路由**: 8 个页面组件均使用 `() => import()` 动态导入
2. **Canvas/WebGL 分层**: 简单文字用 Canvas 2D，复杂粒子用 Three.js
3. **动画优化**: Tailwind CSS 自定义动画 (fade-in, slide-up, pulse-slow)
4. **ECharts 优化**: 使用按需引入减少包体积

### 8.2 构建优化

1. **Vite**: 开发环境秒级热更新
2. **代码分割**: 路由级别的动态 import
3. **Tree Shaking**: ES Module 原生支持

### 8.3 内存优化

1. **弹幕池上限**: 限制内存中弹幕数量
2. **特效清理**: 过期特效自动移除
3. **localStorage 限制**: 日志和配置有上限控制

---

## 附录

### A. 参考文档

- [Electron 官方文档](https://www.electronjs.org/docs)
- [Vue 3 官方文档](https://vuejs.org/)
- [Pinia 官方文档](https://pinia.vuejs.org/)
- [Tailwind CSS 文档](https://tailwindcss.com/)
- [Three.js 文档](https://threejs.org/)
- [ECharts 文档](https://echarts.apache.org/)
- [Bilibili 直播协议](https://github.com/lovelyyoshino/Bilibili-Live-API)

### B. 术语表

- **IPC**: Inter-Process Communication (进程间通信)
- **MVP**: Minimum Viable Product (最小可行性产品)
- **SFC**: Single File Component (Vue 单文件组件)
- **FPS**: Frames Per Second (每秒帧数)
- **LLM**: Large Language Model (大语言模型)
- **ASR**: Automatic Speech Recognition (自动语音识别)
- **Brotli**: Google 开发的无损压缩算法

### C. 版本历史

| 版本 | 日期 | 作者 | 说明 |
|------|------|------|------|
| v1.0 | 2026-06-03 | Cline | 初始版本 |
| v1.1 | 2026-06-04 | Cline | 根据实际项目实现更新技术栈、目录结构、模块设计 |
| v1.2 | 2026-06-24 | Cline | 更新目录结构：新增 engines/、composables/、utils/ 层；Views 模块化；Strategy 模式 |

---

**文档结束**