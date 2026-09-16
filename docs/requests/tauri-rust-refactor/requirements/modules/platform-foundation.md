# Module: Platform Foundation

## Boundary

Tauri v2 运行面、Rust 分层、typed Command/Event gateway、SQLite/配置底座、路径和 Capability guard、免费/会员编译特性。暂不实现具体业务页面。

## Raw Source Snapshot

- PRD 标题：“Tauri 框架 + Rust 核心 + 前端 WebView + Sidecar”。
- 2.2：“核心能力 Rust 化”；“前端轻量化：仅通过 invoke 和 listen 与 Rust 交互”。
- 3.3：UI/IPC/应用服务/领域/基础设施五层。
- 4.1：“两个安装包代码基础相同，构建配置不同”。
- 10.1：“所有 Tauri Command 在 Rust 侧做参数校验、类型校验、范围校验、路径白名单”。
- 12：“继续使用 SQLite，建议启用 WAL。前端不直接执行 SQL”。

## Markdown Normalization

不适用：来源已经是稳定 Markdown；上述内容保持原文摘录并用章节锚点关联。

## Source Grounding And Allowlist

- source-backed：Tauri/Rust/Vue、五层、SQLite WAL、21 表、路径白名单、effect capability、双 build。
- code-fact-backed：现有 package 是 Vue/Vite；main/preload 是 Electron；无 Cargo/Tauri。
- source-derived-safe：会员版内置最小会员中心壳所需基础 capability，具体页面归后续模块。
- out-of-scope：任何本模块新增产品页面、发送弹幕、远程服务实现。

## Responsibilities

- Vue：typed gateway 与错误/事件消费，不直接 I/O。
- Rust：command guard、app state、repository/migrations、config、capability/build registry。
- External：无；仅 scaffold/依赖来源。

## Explicit Constraints

- room/UID/username/word/volume/threshold/range/template/plan/module 校验规则必须可复用。
- 路径不能由业务 command 接受任意字符串；只用 dialog grant/config scope。
- free build 不注册 member commands。
- 迁移必须保留旧数据、失败回滚。

## Downstream Obligations

- page_design_required: `false`
- architecture_design_required: `true`
- entry_stage: `spec`
- execution_order: `1`
