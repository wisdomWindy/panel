# 实施计划：删除插件中心

## 交付单元

`remove-plugin-center`，依据已批准的 spec 与 architecture-design 执行。

## 执行约束

- source-grounding: 用户直接指令 `SRC-1`。
- practical goal: 产品中不再存在可达或 dormant 的业务插件系统。
- forbidden interpretations: 仅隐藏入口、保留后端/存储，或误删 Tauri 框架插件。
- product-content delta: 只删除既有插件中心内容，不新增替代文案、页面或行为。
- TypeScript 依据 `packages/bilibili-live-tool/tsconfig.json` 的 strict/noUnused/bundler/alias 约束。
- 所有共享脏文件采用窄 hunk；禁止无关格式化、重构、重命名和生成文件刷新。

## 任务依赖

`T1 -> T2 -> T3 -> T4 -> T5 -> T6`

前四项按功能链由消费者向所有者清理；T5 统一适配测试；T6 做文档和依赖闭包复查。

## T1 删除入口、页面与产品文案

- Grounding: source-backed, spec“Function-Complete Behavior Breakdown”。
- 文件/符号:
  - 从 `router/index.ts` 删除 `/plugins` 路由和页面 import。
  - 从 application-shell navigation、图标映射与可用配置 scope 中删除 `plugins`。
  - 删除 `views/plugin-manager/index.vue` 与 `PluginInstallDialog.vue`。
  - 从中英文 locale 删除插件导航、设置 scope、插件页面文案。
- 保留: 其他导航、路由、图标、配置 scope、更新页面和本地化。
- 清理闭包: imports、route names、component references、tests/mocks。
- 验证: `rg` 不再找到 `/plugins`、`plugin-manager` 或插件页面 locale key；shell contract 测试更新。

## T2 删除前端 release 状态、适配器和桌面契约

- Grounding: source-backed + code-fact-backed, spec“API And Data Contracts”。
- 文件/符号:
  - `stores/release.ts`: 删除 plugin snapshot state 和 list/install/enable/disable/configure/uninstall actions。
  - `useReleaseRuntime.ts`: 删除 plugin status/crash listener 与 cleanup，保留 update listener。
  - `modules/release/adapters.ts`: 删除 plugin mapper 和常量。
  - `platform/desktop/contracts/release.ts`: 删除所有 plugin DTO/request/result/command/event。
  - `contracts/base.ts` 删除 `pluginPackage`; `contracts/shell.ts` 删除 `plugins` scope。
- 数据/状态: 删除整条 PluginSnapshot 生命周期，不创建兼容类型或可选字段。
- 副作用: lifecycle 仅保留 update event；dispose 仍清理唯一 listener。
- 测试: 删除 plugin store/adapter/runtime cases，改写 listener 数量与失败边界，保留 update race/cleanup 覆盖。
- 验证: TypeScript 编译利用 `noUnused*` 揭示遗漏；运行相关 Vitest。

## T3 删除 Rust 插件命令链

- Grounding: source-backed + code-fact-backed, architecture ADR-1/ADR-3。
- 文件/符号:
  - 删除 domain plugin permission/resource/id validation 与测试。
  - 删除 `PluginProvider`、plugin snapshots/status/generations/service methods、plugin event sink methods。
  - 删除 infrastructure unavailable plugin provider 和 provider tuple 分支。
  - 删除 IPC plugin request DTO、handlers、validation、events 与 error mappings。
  - 从 `lib.rs` free/member state construction 和 invoke registration 中删除 plugin command。
  - 从 build-channel command names 与 foundation permissions 删除 plugin commands。
  - 删除 `PathPurpose::PluginPackage`。
- 保留: update/about/resilience commands, providers, events, errors, path purposes, Tauri framework plugins。
- 清理闭包: imports、trait impl methods、mock providers、tests、command contract expectations。
- 验证: Rust format/check/test 覆盖 free/member features和 release contract tests。

## T4 删除配置与持久化分支

- Grounding: code-fact-backed, architecture ADR-2。
- 文件/符号:
  - 从 `domain/shell.rs` 删除 `ShellConfigScope::Plugins`、scope key/available scope 与计数断言。
  - 从 `infrastructure/shell.rs` 删除 PluginEntry、PreparedSection::Plugins、export/import/apply/reset SQL。
  - 新增 `migrations/0007_remove_plugin_center.sql`: `DROP TABLE IF EXISTS t_plugin` 并写 schema version 7。
  - `infrastructure/database.rs` 断言 schema version 7 且表集合不含 `t_plugin`。
- 禁止: 修改 `0001_initial.sql` 或保留运行时 table fallback。
- 验证: database/shell Rust tests，迁移后表不存在。

## T5 适配并保留有效测试

- Grounding: spec edge cases, architecture ADR-4。
- 将 `views/plugin-manager/release-pages.test.ts` 中 update dialog 两个测试迁移到 `views/update-log/release-pages.test.ts`，删除 plugin page case。
- 更新 frontend release store/runtime/adapter/shell/component contracts；仅删除 plugin expectation，保留相邻行为断言。
- 更新 Rust release/domain/shell/database/IPC contract tests和 mocks。
- TDD 例外: 这是功能删除；先通过现有测试识别依赖，再同步删除 obsolete expectations，最终 green。
- 验证: focused Vitest and Cargo test suites; exact commands recorded in verification artifact。

## T6 删除专属文档并复查依赖闭包

- Grounding: source-backed, spec acceptance 5/6。
- 删除 `docs/plugin-development.md`、`docs/requests/plugin-system/` 和 homepage `features/plugins.md`。
- 从 README、PRD、主页首页/功能索引/设置指南、架构说明、changelog 中删除当前插件中心产品声明和链接；保留 Tauri/Vue tooling terminology。
- 使用 scoped `rg` 搜索 plugin business symbols, commands, events, table, route, scope, docs links。
- 检查 `git diff`，将每个 hunk 映射到 T1-T6；删除 unused imports/exports、stale comments/tests/mocks，确认无无关格式化。

## Verification Strategy

- Frontend focused tests: release adapters/store/runtime, shell contracts/components/settings, relocated update page tests。
- Frontend static/build: package TypeScript check and build for available channel(s)。
- Rust: `cargo fmt --check`; `cargo test --no-default-features --features free`; member variant or scoped equivalent; `cargo check` as needed。
- Persistence: database migration tests assert marker 7 and absence of `t_plugin`。
- Residual: classify every remaining case-insensitive `plugin` hit as framework/tooling or fix it。
- If a broad command fails due to unrelated dirty-worktree code, record the exact failure and run the narrowest authoritative focused checks; in-scope failures remain blockers。

## Execute Readiness

- Product behavior, API deletion set, storage migration, file owners, cleanup targets, and verification paths are explicit。
- No product clarification or new user-visible decision is delegated to execute。
- Pattern decision: reuse existing Level 2 layered boundaries; add no abstraction。
- Plan approval blockers: none。

