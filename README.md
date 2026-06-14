# Bilibili 直播助手

一款集自动化互动、智能回复、特效展示于一体的 Bilibili 直播辅助工具。

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

### 开发中功能

- 🚧 AI 智能回复
- 🚧 语音识别

## 技术栈

- **框架**: Electron 30 + Vue 3 + TypeScript
- **状态管理**: Pinia
- **UI 组件**: Element Plus
- **构建工具**: Vite
- **图表**: ECharts
- **3D 特效**: Three.js
- **样式**: Tailwind CSS

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
│   │   └── ipc-handlers.ts      # IPC 通信处理
│   ├── preload/                 # 预加载脚本
│   └── renderer/                # Vue 渲染进程
│       ├── components/          # 公共组件
│       ├── modules/             # 核心模块
│       │   ├── bilibili/        # Bilibili 客户端
│       │   ├── danmaku/         # 弹幕管理
│       │   ├── effects/         # 特效系统
│       │   └── utils/           # 工具函数
│       ├── services/            # 服务层
│       ├── stores/              # Pinia 状态管理
│       ├── views/               # 页面组件
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

## 许可证

MIT
