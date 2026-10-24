# 任务看板

| ID | 任务 | 状态 | execute readiness | 模式 | 前置 | 目标文件 / 符号 | 规格锚定 | 测试切入点 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| T1 | 新增 CSP-safe 消息编译失败测试 | completed | ready | 串行 | spec 已批准 | `i18n-runtime.test.ts`; `cspSafeMessageCompiler` | 功能分解 1-3；`code-fact-backed`；product delta none | 聚焦 Vitest 必须先因缺失生产导出而失败 |
| T2 | 实现纯函数编译器并注入启动配置 | completed | ready | 串行 | T1 RED | `i18n-runtime.ts/cspSafeMessageCompiler`; `main.ts/createI18n` | 功能分解 1-4；`code-fact-backed`；product delta none | T1 GREEN；类型检查 |
| T3 | 生产构建、安装和首屏运行时验证 | completed | ready | 串行 | T2 GREEN | 构建产物、安装版 WebView | 验收 4-6；`code-fact-backed`；product delta none | type/lint/test/build + CDP exception + DOM 布局 |

## 全局执行约束

- 执行人：当前编码代理，不启动子代理。
- 页面/模块范围：i18n 启动边界；壳层仅作集成验证面。
- 关键状态：不改任何可写业务状态，不增加 loading/error/empty 状态。
- API 与类型：无后端 API；复用 `vue-i18n.MessageCompiler`。
- 函数式边界：编译器为纯函数，无隐式模块状态，只在不支持输入时调用 `onError`。
- 模式约束：Level 1 轻量 Adapter，禁止新增 manager/factory/class 或独立通用 helper 层。
- 变更链审查：执行前后复查 locale -> i18n -> shell DOM 链路与 import/export 闭包。
- 专家约束：不转移风险到 CSP、组件或缓存；以首次加载异常和 DOM 作为集成证据。
- 评审预检：无 `any`、无静默异常、无新副作用、无新响应式 fan-out、无技术栈替换。
- speed/context：`S1-local` / scoped；待确认项为 none。
