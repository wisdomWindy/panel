# 规格：安装版应用壳层显示不完整

## 交付单元标识

`installed-shell-content-clipped`

## 背景和目标

安装版启动后，默认 `vue-i18n` 消息编译与 Tauri CSP 冲突，导致应用壳层局部渲染失败。目标是在不放宽 CSP 的前提下，恢复现有壳层和路由内容。

## 范围内

- 为主渲染器的 `createI18n` 配置 CSP 可用的消息编译器。
- 支持当前 locale 资源中的字面文本和 `{name}` 命名占位符。
- 用单元测试和生产构建运行时检查覆盖根因。

## 范围外

- 不放宽 `script-src`，不加入 `'unsafe-eval'`。
- 不改变翻译文案、布局、路由、后端 API 或会员模式。
- 不处理本次诊断中另行发现的 Tauri ACL 拒绝；它们不是本界面空白的根因。

## 触发与启动条件

用户启动打包后的 Tauri 应用，WebView 按 `tauri.conf.json` 中的严格 CSP 加载生产资源。

## 需求拆分摘要

不适用：S1 本地单链路缺陷，无 PRD 模块拆分。

## 来源锚定与可追溯性

| 规格项 | 标签 | 依据 | 下游义务 |
| --- | --- | --- | --- |
| 完整显示既有界面 | source-backed | 用户文字与截图 | 运行时验证壳层四区域 |
| 保持严格 CSP | code-fact-backed | `tauri.conf.json` 现有安全边界 | 不使用 `unsafe-eval` |
| 定制消息编译器 | code-fact-backed | CDP 捕获到 `new Function` CSP `EvalError`；`MessageCompiler` 公开契约 | 为 `createI18n` 注入纯函数编译器 |
| 只支持字面与命名占位符 | code-fact-backed | 两份 locale JSON 全量扫描 | 测试资源语法边界 |

闭世界允许列表：只恢复当前翻译资源、顶部工具栏、左侧导航、当前路由内容和底部状态栏的现有行为；列表外的文案、控件、状态、权限、导航、API 语义均禁止变更。

## 工作流效率配置

`S1-local`，compact 工件；下游仅读取 i18n 启动链、CSP 配置、locale 资源与相关测试。若发现需要改动资源语法、CSP 或引入新构建插件，必须升级返回 spec。

## 自愈完成标准

要求需求一致性、locale 语法、类型、lint、单测、生产构建、CSP 运行时和壳层 DOM 全部通过；无未处理翻译异常、无调试代码、无无关 diff。完成时记录 `self-healing loop compliance: pass`、`self-healing loop assessment: pass`、剩余阻塞 `none`。

## 用户意图合同

- 字面请求：运行后界面如截图所示。
- 实际目标：安装版可正常使用，而不是只消除空白颜色。
- 成功标准：已有界面区域完整、翻译正常、无 CSP `EvalError`。
- 禁止解读：不得通过隐藏组件、删除翻译或放宽 CSP 表面上“填满”页面。
- 可接受方法：定制无 eval 编译器或等价的构建期预编译，但不得扩大产品范围。

## 用户流程

启动应用 -> locale 消息在严格 CSP 下被解析 -> 壳层四区域完整渲染 -> 用户可继续连接直播间。

## 页面与模块设计

不适用：只恢复已有 `App.vue` 壳层组合，不改变页面设计。

## UI / UX 设计合同

保持现有阅读顺序和可见状态：工具栏 -> 导航与路由内容 -> 状态栏。本次不新增交互或可见文案。

## 前端样式约束

不适用：不修改样式。

## 字段显示语义和交互视觉状态

不适用：所有可见字段与状态保持现有 locale 与组件合同。

## 功能完整行为分解

1. 消息编译器接收字符串时，返回不使用动态代码执行的消息函数。
2. 消息函数原样返回字面片段，并通过 `MessageContext.named` 替换 `{identifier}`。
3. 非字符串 AST 输入必须调用 `onError`，并以 key 作为可诊断回退，不得静默当作正常文案。
4. 主 `createI18n` 显式注入该编译器。
5. 打包后首屏翻译不再抛出 CSP `EvalError`，壳层区域正常渲染。

## 专业前端工程约束

状态所有权、路由、异步请求和组件交互均不改变。渲染边界只增加确定性的 locale 编译，不增加响应式状态、watcher 或包体层。

## 前端架构设计路由输入

`architecture-design_required: false`。修复不改变模块边界、依赖方向、状态拓扑或公开 API。

## 生产代码质量约束

使用 `vue-i18n` 导出的 `MessageCompiler` 类型；不使用 `any`。编译函数为纯函数，输入不可变；非字符串明确报错。不引入缓存、类或微优化。

## 人工评审可读性约束

允许改动 `i18n-runtime.ts`、`main.ts`、聚焦测试与本请求工件。禁止 locale 文案、CSP、页面样式和无关格式化变更。每个 diff hunk 必须映射到消息编译、启动注入或回归测试。

## 函数式编程约束

消息分段和命名替换为确定性纯转换；只从参数和 `MessageContext` 读取值，不读写全局状态。副作用仅限 `onError` 边界。拒绝引入函数工具库。

## 架构复用与共享所有权约束

已有 `i18n-runtime.ts` 是当前 locale 运行时所有者，编译器应就近由其导出并由 `main.ts` 消费。无等价生产逻辑需要抽象或迁移。

## 就近性与文件抽取约束

不新建单函数 utility 文件。编译器放入既有 `i18n-runtime.ts`；其独立测试文件是确定性安全边界的聚焦证据。

## Vue 组件抽取约束

不适用：不新增、拆分或变更 Vue SFC 边界。

## 设计约束

保持严格 CSP，保持现有 i18n 使用方式与 locale 数据结构。

## 项目引导与脚手架决策

不适用：非 greenfield。

## 变化轴与模式决策

- 问题：第三方默认消息编译器与宿主 CSP 不兼容。
- 候选信号：存在一个明确的第三方适配边界，但没有多实现、选择或复杂创建需求。
- 深度：Level 1 local lightweight shape。
- 决策：`adapt lightweight pattern`，最接近 Adapter，以一个类型化纯函数适配 `MessageCompiler` 契约。
- 拒绝方案：放宽 CSP 会扩大安全面；新增构建插件与依赖对当前仅一种资源语法过重。
- 复核信号：若 locale 引入复数、链接或 ICU 语法，当前轻量适配将不再适用，需回到构建期预编译方案。

## 代码上下文与影响假设

参见 `artifacts/code-context.md`。核心假设已由代码事实验证：当前 locale 语法是字面文本加命名占位符。

## API 与数据合同

无后端 API 变更。前端第三方合同为 `MessageCompiler(message, context) => MessageFunction`。

## 前端 / 服务端责任边界

完全由前端启动配置所有；不修改 Rust 命令、ACL 或服务端数据。

## 上下文和依赖来源

`tsconfig.json`、`main.ts`、`i18n-runtime.ts`、两份 locale JSON、`vue-i18n 9.14.5` 类型声明、`tauri.conf.json` CSP、CDP 异常证据。

## 边界情况

- 字面消息不含占位符。
- 一条消息含多个命名占位符。
- 命名值为数字或空值时使用 JavaScript 字符串化结果，与当前 `vue-i18n` 上下文契约一致。
- 非字符串资源显式进入错误回退。

## 验收标准

1. 新增测试先证明默认配置未注入 CSP-safe compiler，修复后通过。
2. 编译器正确处理字面文本、`{giftName}` / `{count}` 和非字符串输入。
3. 两份 locale 资源不包含当前编译器未批准的语法。
4. `type-check`、`lint:check`、相关 Vitest 和 `build:web:free` 通过。
5. 在严格 CSP 的安装版首次加载中，不再捕获 `new Function` CSP `EvalError`。
6. DOM 中存在 `header`、导航菜单项、主路由内容与 `footer`，主内容容器占据中间网格行。

## 人工评审与交接

评审时将每个改动 hunk 对应到编译器、注入或测试；附上失败/通过测试、构建和 CDP/DOM 运行时证据。

## 风险

主要风险是未来 locale 使用超出字面与命名占位符的语法。通过资源语法回归测试使其在 CI 中失败，而不是到安装版运行时静默破坏。

