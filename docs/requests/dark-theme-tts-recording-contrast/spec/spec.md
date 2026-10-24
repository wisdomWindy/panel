# 规格：语音播报与直播录制深色对比度

## 交付单元

`dark-theme-tts-recording-contrast`

## 目标与范围

为 `/tts` 和 `/recording` 以及其直接可见子组件中未配对的浅色 Tailwind 颜色补充深色变体。仅修复可读性；不改 DOM、布局、文案、数据、状态、路由或交互。

### 范围内文件

- `views/tts/index.vue`
- `components/media/TtsQueuePanel.vue`
- `views/recording/index.vue`
- `components/media/RecordingStatusBand.vue`
- `views/recording/components/RecordingSettingsDialog.vue`
- `components/media/media-pages-contract.test.ts`

### 范围外

TTS/录制业务逻辑、store、IPC、Rust、表单规则、数据格式、录制列表/高光结构、全局主题系统和新设计抽象。

## 来源与 Closed-World 映射

| 规格项 | 标签 | 来源 |
| --- | --- | --- |
| 语音播报同类内容深色可读 | source-backed | U1 |
| 直播录制同类内容深色可读 | source-backed | U1 |
| 直接子组件跟随页面修复 | code-fact-backed | 可见内容分布在现有子组件 |
| 不改浅色与功能 | source-derived | 缺陷仅指向深色可读性 |

Product-content allowlist：无新产品内容；`product-content delta: none`。

## 用户意图合同

- literal：修复语音播报和直播录制的同类深色问题。
- practical：两页所有同类可见文字与分隔层级均清晰，不只修页标题。
- success：主/次文字、状态、数值、警告/错误、边框和快捷键表面在 `.dark` 下可分辨。
- forbidden：全局 CSS 覆盖；改文案/数据/交互；顺手重构。
- acceptable：在现有 utility 后就地增加 `dark:` 变体并扩展契约测试。

## UI / UX 与字段展示合同

- 页标题/关键数值：`dark:text-zinc-100`。
- 区域标题/关键状态：`dark:text-zinc-200`。
- 普通标签/当前队列内容：`dark:text-zinc-300`。
- 辅助描述/计数：`dark:text-zinc-400`。
- 警告：`dark:text-amber-400`；错误：`dark:text-red-400`。
- 主分隔线：`dark:border-zinc-800`；快捷键边框/背景：`dark:border-zinc-700 dark:bg-zinc-900`。
- 业务语义、数值精度、空值、可点击性、hover/focus/active/disabled 均不变。

## 功能完整分解

1. 契约测试遍历五个目标模板，拒绝未配对的 gray/red/amber 文字、gray 边框及 `bg-gray-50`。
2. TTS 页与队列补齐标题、队列内容、计数、警告和分隔线变体。
3. 录制页、状态栏和设置弹窗补齐标题、状态、数值、提示、快捷键与分隔线变体。
4. 两路由在深色主题下检查 computed color 与可见页面。

## 样式约束

仅 Tailwind utility；类名与现有浅色 token 相邻且在模板内可审查。不新增 CSS、预处理器、inline style、语义 class、class constant/map/helper。条件 `:class` 仅保持现有小型状态切换。

## 变化轴与模式决策

- 深度：Level 0。
- 决策：`direct code`，pattern `none`。
- 理由：变更仅是五个现有 DOM owner 的静态主题 token，无选择/适配/创建/编排/副作协调/状态扩散或跨文件行为变化轴。抽取主题 helper/wrapper 反而隐藏本地样式。

## 架构、复用与局部性

- `architecture-design_required: false`。
- 无模块边界、state/data topology、依赖方向、公开 API 或共享抽象变更。
- 不新增/移动 helper、hook、mapper、constant、utility 或 Vue 组件；已有组件边界保持不变。

## 生产代码与专业前端约束

无新类型、函数、空值、异常、缓存、watcher 或副作。保留现有 loading/error/empty、键盘/焦点、disabled、状态 owner、数据生命周期和性能边界。

## 人工评审可通过性

仅允许五个 Vue 文件、一个契约测试及本请求文档的任务 hunk。不允许格式化噪音、重构、命名或生成文件残留。必须提供 RED/GREEN、type/lint/build、两路由运行时和 diff 证据。

## API、TypeScript、函数式与脚手架

- API：不适用。
- TypeScript：测试遵循现有 Vitest/Node 模式，由 `tsconfig.eslint.json` 检查；无新 alias/global/generated type 假设。
- 函数式：不适用，无转换或副作编排。
- scaffold：不适用。

## 边界情况

- 录制的 idle/starting/recording/stopping/error 状态色均需在深色下可读。
- 磁盘低空间和 TTS 回退/引擎不可用警告保持 amber 语义。
- 快捷键表面不应使用浅背景刺眼或丢失边界。
- 无数据/非桌面运行时不影响页面渲染验收。

## 验收标准

1. 五个目标模板中所有 `text-gray-*`、`text-red-*`、`text-amber-*` 行具有对应 `dark:text-*`。
2. `border-gray-200` 具有 `dark:border-zinc-800`，快捷键 `border-gray-300` / `bg-gray-50` 具有 `dark:border-zinc-700` / `dark:bg-zinc-900`。
3. `/tts` 标题与队列文字/计数/警告/分隔线在深色下可读。
4. `/recording` 标题、状态栏、数值、提示、快捷键和弹窗辅助文字在深色下可读。
5. 浅色 utility、DOM、文案、数据、业务逻辑与交互不变。
6. 契约测试、type-check、lint 和 Web build 通过，或仅有明确的既有非阻断 warning。

## 风险

条件 `:class` 中的多状态文字可能遗漏；通过逐行契约测试与运行时样式抽样约束。
