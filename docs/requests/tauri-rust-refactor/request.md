# Tauri / Rust 全量重构请求

## 请求标识

- `request_id`: `tauri-rust-refactor`
- 类型：PRD 驱动的跨框架、跨语言、跨模块重构
- 来源：`docs/requirements/Bilibili 直播助手 - 产品需求文档 (PRD).md`
- 来源版本：2026-09-14 最终定稿（以工作区当前未提交版本为只读事实源）

## 业务摘要

将现有 Electron + Vue 桌面应用迁移为 Tauri + Rust + Vue WebView + Sidecar 架构，在不降低现有免费能力的前提下，落实只读直播监听、本地优先、安全 IPC、SQLite 持久化、双安装包、会员模块按需下载、故障隔离和跨平台发布。

## 初始目标

1. 移除 Electron 主进程、preload 和渲染层对 `window.electronAPI` 的运行时依赖，使用 Tauri v2 Command / Event 作为桌面边界。
2. 将 WebSocket、过滤/风控、统计、TTS、录制、AI、数据库、插件、会员与模块生命周期的核心能力迁到 Rust。
3. 保留并重构 Vue + TypeScript 前端，使其符合 PRD 的四区布局、20 个功能和免费/会员可见性规则。
4. 产出两个构建通道：免费版二进制不注册或编译会员能力，会员版按登录、付费、激活、到期状态加载会员模块。
5. 建立可验证的安全、迁移、降级、合规和性能基线。

## 初始 Done Signal

- 默认桌面开发与构建命令由 Tauri 驱动，仓库生产依赖与生产源码不再依赖 Electron。
- Rust crate 可通过格式、静态检查与测试，Vue/TypeScript 可通过类型检查、lint、单元测试与生产构建。
- 免费版与会员版构建配置均可生成，免费版不存在会员入口及会员 Command 注册。
- PRD 20 个功能均有可运行实现或被明确标为依赖外部服务契约的受控降级；只读边界、路径白名单、敏感字段保护与 Capability 隔离有测试证据。
- 原有本地配置/统计迁移路径可验证，失败不会破坏原数据。
- 逐模块 verification、review 与最终集成验收全部通过。

## 触发条件

用户明确要求根据指定 PRD 重构当前项目；当前 PRD 已定稿并明确 Tauri/Rust 目标架构。

## 初始上下文来源

- PRD：`docs/requirements/Bilibili 直播助手 - 产品需求文档 (PRD).md`
- 现有前端：`packages/bilibili-live-tool/src/renderer/`
- 现有 Electron 主进程与 preload：`packages/bilibili-live-tool/src/main/`、`packages/bilibili-live-tool/src/preload/`
- 构建和依赖：根 `package.json`、`pnpm-workspace.yaml`、`pnpm-lock.yaml`、应用 `package.json`、`vite.config.ts`
- 现有测试：`tests/`
- 现有实现记录：`docs/implementation/`、`docs/requests/`
- 外部脚手架事实源：Tauri v2 官方“为现有前端单独初始化后端”流程

## 人工交接点

完成全部模块实现、验证与审查后交接可运行应用、验证结果和仍需部署方提供的外部服务配置。只有会改变产品语义且无法从 PRD、代码或官方契约恢复的决策才在上游阶段阻塞。

## 影响范围

- 桌面运行时与构建工具链
- Rust 应用核心、Tauri Commands/Events/Capabilities、Sidecar
- Vue 页面、路由、状态、服务适配层与样式
- SQLite schema、旧数据迁移、安全存储与保留策略
- 免费/会员构建通道、更新与模块分发
- 单元、集成、端到端、性能与安全验证

## 参与模块

公共底座、连接与事件、风控、提醒与音效、数据统计、TTS、录制、AI/ASR、会员与模块、配置/更新/插件/i18n、应用壳层与 UI、迁移/安全/发布。

## Scaffold 敏感性

- `greenfield_scaffold_sensitive=true`
- 触发来源：PRD 明确“将原应用重构为 Tauri 框架 + Rust 核心 + 前端 WebView + Sidecar”，会新增可独立构建的 Rust/Tauri 运行面。
- 项目类型：现有 pnpm monorepo 中的跨平台桌面应用；保留现有 Vue + TypeScript + Vite + Pinia + Vue Router + Element Plus/Tailwind 前端。
- 候选：Tauri v2 官方 CLI 对现有前端执行 backend initialization；仓库内无同类型 Tauri sibling app 或生成器。
- 初始选择倾向：复用现有 Vue/Vite 应用，通过官方 Tauri CLI 初始化 `src-tauri`，不另建重复前端项目。最终选择在 spec 前完成并记录。

## 非目标

- 不增加任何发送弹幕按钮、Command、自动任务或变相发送能力。
- 不把免费版变成会员导流入口。
- 不在缺少服务端地址、OAuth、支付、模块签名/清单等部署契约时伪造生产服务。
- 不修改用户当前未提交的 PRD 内容。
