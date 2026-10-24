# Task Board

| ID | Task | Status | Evidence target |
| --- | --- | --- | --- |
| T1 | 删除入口、页面和插件文案 | completed | route/navigation/locale residual search |
| T2 | 删除前端状态、适配器和桌面契约 | completed | focused Vitest + typecheck |
| T3 | 删除 Rust 领域/应用/provider/IPC/权限命令链 | completed | source residual scan; cargo blocked before crate by whisper bindings |
| T4 | 删除配置分支并新增 0007 数据库迁移 | completed | migration/schema assertions updated |
| T5 | 迁移更新测试并适配所有受影响测试 | completed | 7 files / 28 focused tests passed |
| T6 | 删除专属文档并完成 residual/diff review | completed | classified rg + diff audit |

## Review Contract

- 每个 changed hunk 必须映射到 T1-T6。
- 共享文件中只允许插件相关删除及其直接测试适配。
- 不保留仅由测试引用的生产插件代码。
- 不得出现 debug code、dead code、stale comment、temporary name、unused import/export 或 stale mock。
- 交付时列出命令、结果、未运行项原因和剩余风险。

