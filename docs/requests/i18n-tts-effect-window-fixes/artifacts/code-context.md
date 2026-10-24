# 代码上下文

## 上下文需求

本缺陷跨 Vue 页面、状态、IPC、Rust domain/application/infrastructure，需要确认完整调用链。bugfix-intake 阶段要求结构分析，code graph 为优先能力。

## 图能力检查

- 状态：missing。
- 检测：仓库文件/脚本搜索及 `Get-Command codegraph, ckg, codemap` 均未发现可用工具。
- 迭代：bugfix-intake iteration 1。

## 安装或引导记录

- attempted: no。
- method: 仓库和团队说明中未声明安全、兼容 Vue/Rust 混合项目的 graph bootstrap 来源。
- result: not_needed。
- next step: 使用定向 `rg`、直接 import/caller 阅读以及编译/测试闭包验证。

## 回退记录

- fallback used: yes。
- 方法：按功能链定向搜索 Vue、TypeScript 和 Rust 符号及调用方。
- 原因：无可用 code graph 或安全 bootstrap。
- 置信度：核心链路均有静态类型和双端契约测试，可通过全量 TS/Rust 测试与桌面构建补足。

## 相关入口

- 语言：`LanguageSettings.vue -> application-shell store -> projection -> i18n-runtime`。
- TTS：`views/tts/index.vue -> media/tts store -> tts_save/tts_get -> Rust TtsRuntime`。
- 特效：`views/captain-effect/index.vue -> media/effects store -> captain_effect_* -> EffectRuntime -> EffectWindowPort`。

## 变更功能链

- i18n：现有 locale 状态已驱动导航，但业务页面多数仍为中文字面量；变更归页面/组件的 `useI18n` 消费层与 locale 资源所有权。
- TTS：默认 Rust config 为 `selected_voice_id=None`，页面直接复制该值；重置仅 `applyConfig`，未调用 store.save 且无成功消息。
- 特效：`CaptainEffectConfig.position` 已持久化并传到 payload，但 `EffectWorker` 创建 `EffectWindowSpec` 时只给固定 `560×280`；`TauriEffectWindowPort` 仅设置 inner_size，未设置 position。
- 测试文件是适配目标，不是保留旧行为的生产所有者。

## 关键符号和模块

- `setAppLocale`, `useI18n`, locale JSON。
- `TtsConfig`, `useMediaTtsStore`, `handleReset`。
- `CaptainEffectConfig`, `EffectWindowSpec`, `EffectRuntime`, `TauriEffectWindowPort`。

## 依赖和副作用边界

- locale 状态由 shell projection 单向更新，页面只读取。
- TTS 保存和反馈由页面编排，持久化仍归 desktop command。
- 尺寸/位置属于 media config；屏幕坐标计算与窗口副作用归 infrastructure port。

## 影响范围

free renderer 页面/组件、locale JSON、media TS contract/adapters/tests、Rust media model/rules/state/DTO/IPC/effects/window/tests。

## 后续检查

- 扫描免费版可达页面残留的硬编码中文界面文案。
- 验证旧 JSON 配置缺少新宽高字段时的兼容反序列化。
- 在 Windows 桌面运行时验证五种位置和尺寸。

