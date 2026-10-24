# 执行记录

## 交付单元标识

`installed-shell-content-clipped`

## 已批准规格与计划

`spec/spec.md`、`plan/plan.md`、`plan/task-board.md`。

## 执行摘要

为 `vue-i18n` 注入无动态代码执行的消息编译器，支持当前 locale 资源的字面文本和命名占位符。严格 CSP 未改动。

## 自愈执行记录

- iteration: 3，初次 execute。
- defect class: `runtime-error`；根因为 `vue-i18n` 默认编译器与 Tauri CSP 冲突。
- 新证据：TDD 失败/通过记录、新安装包、CDP/DOM/截图首屏证据。
- verify/review 重点：`CompileError` 适配、locale 语法封闭性、启动注入和严格 CSP 首屏。

## 任务看板更新

T1、T2、T3 全部 completed。

## 变更文件和所有符号

- `i18n-runtime.ts`：新增 `cspSafeMessageCompiler`，负责字面/命名占位符消息编译和非字符串错误回退。
- `main.ts`：主 `createI18n` 注入 `cspSafeMessageCompiler`。
- `i18n-runtime.test.ts`：6 个测试覆盖导出、字面、命名占位符、异常资源、禁用 `Function` 的真实 `vue-i18n` 翻译和 locale 语法。
- 本请求 docs 工件：记录来源、规格、计划、执行与验证。

## 引入内容来源台账

| hunk | 内容 | 任务 / 来源 | 结果 |
| --- | --- | --- | --- |
| `i18n-runtime.ts/cspSafeMessageCompiler` | CSP 技术适配 | T2；code-fact-backed；product-content delta none | authorized |
| `main.ts/createI18n` | 启动注入 | T2；code-fact-backed；product-content delta none | authorized |
| `i18n-runtime.test.ts` | 回归证据 | T1/T3；code-fact-backed；product-content delta none | authorized |

无新增或修改的产品文案、字段、控件、交互、状态、默认值、权限、导航、API/payload 语义或埋点。

## 实现决策

保留 CSP，拒绝 `'unsafe-eval'`；落地 Level 1 轻量 Adapter；未新建 helper 文件、class 或构建插件。语法严格限定为当前资源使用的字面与 `{identifier}`。

## 专业前端工程执行记录

用户旅程、组件边界、状态所有权、异步流、交互、路由和布局均未改动。直接观察首屏 CSP 异常和完整 DOM，未将风险转移到组件层。

## 前端架构质量执行记录

不适用：无 architecture-design，且未改变模块边界、状态拓扑或依赖方向。

## 生产代码质量执行记录

- type-first：复用 `CompileError`、`MessageCompiler`、`MessageContext` 公开类型。
- fail-fast/strict null：非字符串调用可选 `onError` 并返回 key。
- maintainability/performance：简单正则替换；无新缓存、watcher、debounce 或微优化。
- pure/immutable：编译与替换不修改输入或模块状态。
- naming/constants：符号名表达 CSP 和资源语法意图；错误码常量归 i18n runtime 所有。
- UI 边界状态：未触碰列表、异步操作或表单状态。

## 人工评审可读性记录

生产 hunk 映射 T2，测试 hunk 映射 T1/T3。已恢复构建工具对 `components.d.ts` 造成的无关生成差异。无 debug code、dead code、stale comment、临时命名、未用 import/export 或无关格式化。

## TDD 证据

1. 导出 RED：期望 `cspSafeMessageCompiler`，实际属性缺失。
2. 字面 RED：期望 `biliLiveHelper`，实际为 key。
3. 命名占位符 RED：期望 `Sent Super Chat x2`，实际未替换。
4. 非字符串 RED：期望 `onError` 一次，实际 0 次。
5. 每轮以最小实现转 GREEN；最终 focused 6/6、full free-mode 107/107 通过。

`main.ts` 启动注入没有隔离的真实 DOM 挂载缝，按计划用新安装版、严格 CSP、CDP pause-on-exceptions 与 DOM 尺寸作集成证据。

## 变更链路证据

- 变更前：locale JSON -> `main.ts/createI18n` -> `useI18n/t` -> shell SFC -> 渲染失败注释节点。
- 变更后：locale JSON -> `cspSafeMessageCompiler` -> `createI18n` -> shell SFC -> 完整 DOM。
- 真实生产调用方为 `main.ts`；未改 SFC、router、Pinia、Tauri CSP 或后端。

## 删除清理证据

不适用：未删除生产行为或符号。

## 函数式编程执行记录

`cspSafeMessageCompiler` 是确定性纯适配，参数不可变，副作用仅为契约指定的 `onError`。未引入函数工具库或复杂组合。

## 前端样式执行记录

不适用：无样式改动。

## API 合同与适配执行记录

无后端 API 改动。适配的权威类型来源是 `vue-i18n 9.14.5` 导出。

## TypeScript 上下文恢复记录

管理配置为 `tsconfig.json` 与 `tsconfig.eslint.json`；关键选项为 `strict`、`moduleResolution=bundler`、`resolveJsonModule`。仅读取 `vue-i18n` 及直接消息编译类型声明。

## 架构符合性或失效记录

不适用：无架构工件，未发现需回退的结构不匹配。

## 代码上下文更新

`artifacts/code-context.md` 已记录 CSP `EvalError`、第三方契约、locale 语法和 TypeScript 配置。

## 偏差、阻塞与回退触发器

- 无计划偏差或阻塞。
- 全量 Vitest 首次未带 `--mode free`，因现有 `VITE_BUILD_CHANNEL` 门禁失败；用 free mode 重跑后 107/107 通过。
- 运行时另见 `update_status`/`plugin_list`/`about_info`/`resilience_status` ACL 拒绝，与本次 CSP 渲染缺陷无因果关系，未纳入本修复。

## 验证交接记录

- focused Vitest: 6/6 passed。
- full free-mode Vitest: 38 files, 107/107 passed。
- type-check: passed。
- lint: 0 errors；2 条既有 `logger.ts` console warning。
- Tauri release/NSIS build: passed。
- 安装版首屏：CSP 相关 pause 0；header/footer 存在；13 个 menu item；main 为 `1072x712` 且 y=56。
- 截图：`C:/Users/yangjianlin/.codex/visualizations/2026/09/21/01a0c3bb-5b2d-73f0-8534-94eb609c64e3/biliLiveHelper-shell-fixed.png`。
- 发布安装包 SHA-256：`0d2754f15b444008564691feb97b3d8c3dbc078cc644ce2ac46b83ec6abf7951`。

