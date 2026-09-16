# Module: Extensions Release And Resilience

## Boundary

PRD 功能 17、18，完成 16 配置操作，并收口 Sidecar 合规、数据保留/迁移、灾备、隐私、双通道发布与 Electron 删除。

## Raw Source Snapshot

- 9.16：配置 SQLite/Tauri Store、敏感字段加密、事务导入、二次确认重置。
- 9.17：独立更新通道、灰度、黑名单、强制更新、签名、失败回滚、离线静默。
- 9.18：WASM 独立实例、签名与权限、脱敏数据、文件/DB/密钥禁止、网络白名单。
- 11：FFmpeg LGPL 与 whisper 分发/许可/哈希。
- 13：日志/数据保留、启动清理、迁移备份/事务/3 备份。
- 14-15：15 类故障降级、本地存储和最小上传。

## Markdown Normalization

不适用：来源已为 Markdown 表格/列表。

## Source Grounding And Allowlist

- 允许页面：update modal/log、plugin center/config、安全提示、配置管理、关于/许可。
- 允许发布/灾备行为仅按 9.17、11、13-15。
- external-interface-pending：update manifest/channel、plugin/member signatures、第三方 binaries。
- out-of-scope：插件访问原始秘密/任意文件、默认崩溃上传、无签名动态 JS。

## Responsibilities

- Vue：更新/插件/配置/关于 UI 和错误状态。
- Rust：updater policy、WASM host、signature/capability、retention/migration/audit、privacy guard。
- External：signed update/plugin/artifact distribution。

## Acceptance

- 插件越权被拒、崩溃隔离；更新签名/灰度/黑名单 contract 可测；迁移回滚；数据保留；Electron production code/deps 全部移除；free/member build 完整验证。

## Downstream Obligations

- page_design_required: `true`
- architecture_design_required: `true`
- entry_stage: `page-design`
- execution_order: `6`
