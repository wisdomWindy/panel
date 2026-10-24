# 请求

## 请求标识

`captain-effect-window-empty`

## 来源

当前 Codex 会话中的直接用户反馈：“特效窗口里面怎么什么都没有，特效呢”。

## 业务摘要

上舰特效窗口能够打开，但窗口内容为空，用户无法看到所选模板对应的特效。

## 目标

修复上舰特效窗口的数据交付链，使预览和真实上舰触发时窗口都能获得当前活动特效并完成绘制。

## 初始完成信号

- 点击上舰特效预览后，窗口能够绘制当前配置的特效。
- 首次创建窗口时不依赖可能丢失的瞬时事件。
- 真实上舰触发沿用同一可靠数据交付路径。
- 覆盖事件先于窗口监听建立的回归场景。

## 触发条件

用户在上舰特效页面触发预览，或应用收到真实上舰事件。

## 初始上下文

- 用户缺陷原文。
- `EffectRuntime -> EffectWindowPort -> TauriMediaEventSink -> EffectWindowRoot -> CaptainEffectSurface` 代码链。
- 已完成请求 `captain-effect-preset-catalog` 与 `i18n-tts-effect-window-fixes` 的相关契约。

## 人工交接点

完成自动测试、类型检查、Rust 测试和桌面构建后交付用户复测。

## 受影响区域

`bilibili-live-tool` 上舰特效运行时、桌面 IPC 契约及特效窗口根组件。

## 参与模块

Rust media application/IPC、Tauri 命令注册、renderer desktop contract/adapter、`EffectWindowRoot.vue`。

## 用户意图合同

- stated request：特效窗口不应为空。
- practical goal：窗口打开时能稳定看到当前上舰特效，而不是偶发或仅后续触发可见。
- success criteria：首次预览与真实上舰均能获取活动 payload 并绘制。
- forbidden interpretations：只增加延时重发、只改背景让空窗口看起来不空、只修第二次触发。
- acceptable approaches：在监听建立后同步读取当前活动特效，事件继续承担实时更新。

## 流程说明

这是已有功能的单条跨层缺陷，使用非拆分 bugfix 流程；不需要 PRD requirement-analysis/requirement-splitting，也不改变页面布局。

## 2026-09-23 Semantic Reopen

- source: U2 in `artifacts/semantic-reopen-2026-09-23.md`.
- goal: keep the feature as a captain purchase/renewal effect and remove the contradictory live-room-entry sentence from its Canvas.
- done signal: the effect still displays purchase/renewal action, captain level, and username, but contains no “欢迎加入直播间”/entry semantic.
- forbidden interpretation: renaming the page as an entry effect, changing event routing, or replacing the sentence with unconfirmed promotional copy.
