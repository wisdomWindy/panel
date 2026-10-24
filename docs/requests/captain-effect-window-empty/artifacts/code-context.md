# 代码上下文

## 上下文需求

根因跨 Rust 队列、窗口创建、Tauri 事件和 Vue 首帧监听，bugfix-intake 需要结构分析；code graph 为优先能力。

## 图能力检查

- 状态：missing。
- 检测：仓库 graph 文件搜索与 `Get-Command codegraph, ckg, codemap, scip` 均未找到可用工具。
- 迭代：bugfix-intake iteration 1。

## 安装或引导记录

- attempted: no。
- 原因：仓库和团队说明未提供可安全用于 Vue/Rust 混合项目的 graph bootstrap；本仓库相邻已完成请求也记录为不可用。
- 处置：按 policy fallback 使用定向符号搜索、调用方阅读和双端测试闭包。

## 回退记录

- fallback used: yes。
- 方法：定向 `rg` 追踪命令、队列、event sink、窗口入口和 Canvas 的数据依赖。

## 相关入口

- 预览：`captain_effect_preview -> EffectRuntime::preview -> EffectWorker::run -> EffectWindowPort`。
- 直播：`MediaState::consume -> effects.enqueue -> publish_captain_effect`。
- 窗口：`EffectWindowRoot.onMounted -> listenDesktop -> CaptainEffectSurface`。

## 变更功能链与根因

- 预览仅入队并显示窗口，没有调用 `publish_captain_effect`，因此 payload 始终为空。
- 直播路径先入队再立即发布事件；worker 异步创建窗口，事件通常早于目标窗口及监听器存在。
- `CaptainEffectSurface.draw` 在 `props.payload` 为空时返回并保持透明，这是空窗口的直接渲染结果。
- 根因是瞬时事件被错误地当作新窗口首帧状态源，而不是 Canvas 尺寸或模板配置。

## 关键符号和模块

`EffectRuntime`, `EffectQueueState`, `TauriMediaEventSink`, `captain_effect_preview`, `EffectWindowRoot`, `adaptCaptainEffectPayload`。

## 依赖和副作用边界

Rust runtime 拥有当前活动特效；IPC 只做 DTO 转换；窗口组件应先挂事件监听，再读取 runtime 当前状态；Canvas 只消费有效 payload。

## 影响范围

局限于 captain-effect 活动状态读取命令及其前后端契约，不扩散到礼物、TTS 或录制。

## 后续检查

- 测试活动 payload 在预览与真实事件中的内容和 `isPreview`。
- 测试窗口根组件先监听后读取当前状态。
- 验证命令注册、TypeScript contract、Rust tests 和 desktop build。
