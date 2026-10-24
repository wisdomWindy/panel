# 验证报告

## 交付单元标识

`installed-shell-content-clipped`

## 工作流效率证据

- `speed_profile.level`: `S1-local`。
- 验证范围：i18n 编译边界、主启动注入、完整前端测试、类型、lint、生产 Web 构建、安装包一致性和安装版首屏。
- 已运行：focused Vitest、full Vitest (`--mode free`)、`type-check`、`lint:check`、`build:web:free`、`git diff --check`、SHA-256、CDP reload/exception/DOM 检查。
- 未重复运行完整 Rust/NSIS 构建：execute 已在同一修复后产出并安装 NSIS；本轮以新 Web 构建、两个 NSIS 文件相同哈希和实际安装进程复查作为替代证据。
- profile upgrade: none。

## 自愈循环符合性

- result: pass。
- `state.json.loop.iteration`: 4，交付单元迭代数 4，符合 `<= 7`。
- defect inventory: requirement-mismatch none；architecture-gap none；logic-error none；syntax-type-error none；runtime-error 已修复（默认 message compiler 触发 CSP `EvalError`）；code-organization-error none；maintainability-error none；technical-debt-blocker none；plan-gap 已修复（删除无来源的固定菜单数量，改为记录实际数量）。
- blocker attempts: CSP runtime blocker same-stage 1、total 1；无剩余 blocker，未超过同阶段 `<= 2`、总计 `<= 3`。
- automatic re-entry target: none。
- current-pass progress: 新增全量自动化结果、新生产构建、安装版 reload 异常统计和 DOM 尺寸证据；修正计划中的脆弱数量假设。
- evidence rechecked: 6/6 focused、107/107 full、类型通过、lint 0 error、build exit 0、CSP pause 0、四区 DOM 完整。
- remaining blockers: none。
- self-healing loop compliance: pass。
- completion allowed: yes，进入 review 后再作最终判断。

## 验收覆盖

| acceptance item id | verification method | result | evidence reference | follow-up if failed | handoff status |
| --- | --- | --- | --- | --- | --- |
| AC1 TDD 证明注入边界 | 检查 RED/GREEN 记录与真实集成测试 | pass | `execution/changelog.md` TDD 证据；focused 6/6 | none | ready |
| AC2 字面、命名占位符、非字符串 | 直接单测编译器 | pass | `i18n-runtime.test.ts`；focused 6/6 | none | ready |
| AC3 locale 语法封闭 | 递归扫描 zh-CN/en-US 叶子 | pass | locale grammar test | none | ready |
| AC4 类型、lint、测试、Web 构建 | 运行批准命令 | pass | type-check exit 0；lint 0 error/2 existing warnings；107/107；build exit 0 | none | ready |
| AC5 严格 CSP 首屏无 EvalError | CDP `pauseOnExceptions=all` 后 reload | pass | 15 pauses 中 CSP/EvalError 计数 0 | none | ready |
| AC6 壳层四区与中间网格行 | CDP `Runtime.evaluate` 读取 DOM/rect | pass | header `1280x56`；aside `208x712`/13 items；main `1072x712` at y=56；footer `1280x32`；rows `56px 712px 32px` | none | ready |

## 用户意图符合性

- result: pass；literal compliance: pass；practical-goal compliance: pass。
- 禁止解读检查：未隐藏组件、删除翻译或加入 `unsafe-eval`。
- 风险未转移：对实际安装版首屏而非仅单测进行了 reload、异常和布局检查。

## 变更链路完整性

- result: pass。
- chain: locale JSON -> `cspSafeMessageCompiler` -> `createI18n` -> `useI18n/t` -> shell SFC -> DOM。
- 前后链路证据：故障前 CDP 定位默认 compiler；修复后 import/export、类型、测试与安装版 DOM 均通过。
- 已检查 entrypoint、owner、imports/exports、真实调用方、消费者、测试；无 store/router/mock/side-effect 变更，无缺失或重复路径，无邻近产品行为扩张。

## 删除清理符合性

不适用：未删除调用、字段、控件、分支或副作用。

## 规格约束符合性

- result: pass；spec constraint compliance: pass。
- 保持 CSP、locale、Vue SFC、布局、路由和 API 不变；编译器位于既有 owner `i18n-runtime.ts`，未创建单调用方 utility 文件。
- spec-plan granularity alignment: pass；修复了计划中不属于验收合同的固定菜单数量假设。

## 来源锚定符合性

- result: pass；source grounding compliance: pass。
- reverse diff inventory：新增 compiler 分支、启动注入和六项回归行为，均映射 T1-T3；全部为 `product-content delta: none`。
- 未新增字段、文案、控件、交互、状态、默认值、权限、导航、API/payload、状态映射或埋点；未以相邻模块或惯例制造产品语义。

## 设计模式符合性

- result: pass；design-pattern compliance: pass。
- 已批准决策：Level 1 `adapt lightweight pattern`，以类型化纯函数适配第三方 `MessageCompiler`。
- 实现未升级为 class、manager、factory、handler 或共享层；放宽 CSP 和新增构建插件仍被拒绝。

## 专业前端工程符合性

- result: pass；expert frontend engineering compliance: pass。
- 启动到首屏的用户旅程恢复；状态 owner、异步请求、路由、交互、可访问性结构和组件组合均未改动。
- 性能边界：无新 watcher/cache/reactive fan-out；生产构建仅有既有 chunk-size warning。

## 字段显示语义符合性

不适用：未修改展示字段、格式、精度、链接、状态或视觉强调。

## 前端架构质量符合性

不适用：无 architecture-design，未改变模块边界、状态拓扑、依赖方向、公开 API 或迁移路径。

## 架构复用符合性

- result: pass；architecture reuse compliance: pass。
- 复用既有 `i18n-runtime.ts` owner；真实生产调用方为 `main.ts`。单一局部适配无共享晋升信号，无重复旧路径、God utils 或环境副作用。

## 生产代码质量符合性

- result: pass；production code quality compliance: pass。
- 复用 `CompileError`/`MessageCompiler`/`MessageContext` 精确类型；非字符串显式 `onError` 并回退 key；无 `any`、隐藏 mutation、缓存、类或魔法依赖。
- UI 空/loading/error 不适用：未改变列表、异步操作或表单。

## 人工评审就绪符合性

- result: pass；human review readiness compliance: pass。
- 生产与测试 hunks 分别映射 T1/T2/T3；无无关格式化、debug code、dead code、临时命名、未用 import/export 或 stale mock。
- `git diff --check` 通过；lint 的两条 `logger.ts` console warning 为本次未改既有项。

## 函数式编程符合性

- result: pass；functional-programming compliance: pass。
- 编译与占位符替换为确定性纯转换，不修改参数或全局状态；唯一副作用是契约边界 `onError`；组合保持可读。

## 前端样式符合性

不适用：未新增或修改样式。

## API 合同符合性

不适用：无后端 API、request/response 或 payload 变更。

## TypeScript 上下文符合性

- result: pass；TypeScript context compliance: pass。
- 已核对 package `tsconfig.json`/`tsconfig.eslint.json` 的 strict、bundler resolution、JSON module 设置和 `vue-i18n 9.14.5` 公开声明；type-check 通过，未猜测 alias/global/generated type。

## 汇总

所有六条验收项通过。NSIS 构建产物与发布目录安装包 SHA-256 均为 `0d2754f15b444008564691feb97b3d8c3dbc078cc644ce2ac46b83ec6abf7951`。安装版进程 PID 52460 正在运行，严格 CSP reload 后界面四区完整。另有既存 Tauri ACL promise rejection 和 Element Plus 捕获异常，不是本缺陷根因，按批准范围未修改。
