# 执行计划：修复严格 CSP 下的应用壳层渲染

## 范围与来源

本计划只覆盖 `spec/spec.md` 批准的 i18n 编译边界、启动注入、回归测试和生产运行时验证。所有任务标签为 `code-fact-backed`，`product-content delta: none`。

## 实施顺序

### T1：建立失败的消息编译契约测试

- 目标文件：`packages/bilibili-live-tool/src/renderer/i18n-runtime.test.ts`。
- 目标行为：直接调用真实 `cspSafeMessageCompiler`，断言字面文本、多命名占位符和非字符串回退；扫描两份真实 locale 树，断言所有叶子字符串只使用已批准语法。
- RED 标准：测试因 `cspSafeMessageCompiler` 尚未导出而失败；不允许因路径、配置或 fixture 错误失败。
- 验证命令：`pnpm --filter @monorepo/bilibili-live-tool exec vitest run src/renderer/i18n-runtime.test.ts`。

### T2：实现并注入无 eval 消息编译器

- 目标文件与符号：
  - `i18n-runtime.ts`：新增 `cspSafeMessageCompiler: MessageCompiler`。
  - `main.ts`：从既有 i18n runtime owner 导入并传入 `createI18n({ messageCompiler })`。
- 实现合同：使用纯函数与明确的命名占位符正则；通过 `context.named` 取值并字符串化；非字符串调用 `onError` 后返回 key。
- 类型合同：复用 `vue-i18n` 公开 `MessageCompiler`，无 `any`、无宽泛断言。
- 不得执行：不改 CSP、locale 文案、Vue 组件或布局；不引入新依赖、类、缓存或构建插件。
- GREEN 标准：T1 全部通过，再运行全量相关前端测试。

### T3：构建与严格 CSP 集成验证

- 命令证据：`pnpm --filter @monorepo/bilibili-live-tool type-check`、`lint:check`、相关 Vitest、`build:web:free`，随后构建 Tauri 安装包。
- 运行时证据：在启用 CDP 且 `pauseOnExceptions=all` 的新进程首次加载中，过滤并确认无 `Evaluating a string as JavaScript` / `new Function` CSP 异常。
- DOM 证据：`header` 与 `footer` 为真实元素，侧栏菜单项存在并记录当前实际数量，主容器在中间网格行且路由内容可见。
- 交付证据：更新安装程序并在实际安装目录重启验证。

## 变更链路审查

业务链路为 locale JSON -> `main.ts/createI18n` -> `useI18n/t` -> shell SFC -> DOM。本次 owner 是 `i18n-runtime.ts` 与 `main.ts`；壳层 SFC、Pinia store、router 和 Rust 命令只是验证消费面。改动后用 import/export 搜索、类型检查、测试和 DOM 复查确认无重复编译路径、孤立符号或邻近行为变更。

## 模式与函数式约束

执行 spec 的 Level 1 轻量 Adapter 决策：一个 TypeScript 纯函数，不另建 pattern 层。输入不可变，不写全局状态，唯一可见副作用是向上游 `onError` 报告不支持的 AST。

## 代码评审预检合同

- 健壮性：非字符串输入显式失败回退；命名值不隐式假定类型。
- 可维护性：编译器保持小函数，正则常量由 `i18n-runtime.ts` 最窄 owner 管理。
- 性能与内存：消息在 `vue-i18n` 既有编译缓存边界执行；不另建缓存、watcher、订阅或 cleanup 责任。
- 项目一致性：保持 Vue 3 + TypeScript + `vue-i18n` + Vitest 技术栈与当前目录/import 惯例。
- 界面语义：不改字段格式、精度、文案、默认态或交互态。

## 人工评审可通过性

执行结束前将 diff hunks 映射到 T1-T3，删除调试代码和临时文件，确认无死代码、未用 import/export、过期注释、无关格式化或未适配测试。

## 回退条件

如测试证明 locale 需要复数、链接、列表占位符或 ICU 语法，停止 execute 并回到 spec，重新评估构建期预编译；不在 execute 中扩大自定义语法。
