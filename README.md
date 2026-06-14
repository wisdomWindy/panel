# Bilibili 直播助手

一款集自动化互动、智能回复、特效展示于一体的 Bilibili 直播辅助工具。

## 功能特性

- ✅ 自动弹幕管理（定时/随机发送）
- ✅ 礼物感谢弹幕
- ✅ 上舰特效展示
- 🚧 AI 智能回复（开发中）
- 🚧 语音识别（开发中）
- 🚧 数据看板（开发中）

## 技术栈

- **框架**: Electron 30 + Vue 3 + TypeScript
- **状态管理**: Pinia
- **UI 组件**: Element Plus
- **构建工具**: Vite
- **数据库**: SQLite (better-sqlite3)

## 快速开始

### 安装依赖

```bash
npm install
```

### 开发模式

```bash
npm run dev
```

### 构建

```bash
npm run build
```

## 项目结构

```
bilibili-live-tool/
├── src/
│   ├── main/           # 主进程
│   ├── renderer/       # 渲染进程
│   └── preload/        # 预加载脚本
├── docs/               # 文档
└── resources/          # 资源文件
```

## 开发计划

- [x] 项目初始化
- [x] 基础 UI 框架
- [ ] Bilibili 直播间连接
- [ ] 弹幕接收与发送
- [ ] 礼物感谢功能
- [ ] 上舰特效展示
- [ ] AI 智能回复
- [ ] 性能优化

## 许可证

MIT

## 贡献

欢迎提交 Issue 和 Pull Request！