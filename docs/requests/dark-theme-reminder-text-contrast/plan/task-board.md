# 任务板：dark-theme-reminder-text-contrast

| ID | 任务 | 状态 | Ready | 模式 | 前置 | 文件/所有者 | 来源与约束 | 测试切入点 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| T1 | 深色文字/分隔线契约 RED | done | ready | 串行 | approved spec | `media-pages-contract.test.ts` | U1-U3；spec AC1-2；product-content delta: none | focused Vitest 预期失败 |
| T2 | 四个模板补齐 dark utilities | done | ready | 串行 | T1 RED | 三页 + `GiftSoundTable.vue` | spec 层级；Tailwind only；Level 0 direct code | focused Vitest GREEN + `rg` |
| T3 | 验证与审查 | done | ready | 串行 | T2 GREEN | tests/type/lint/build/runtime/diff | spec AC3-6；dirty worktree 隔离 | 三路由 + 工程命令 |

## Execute readiness

- 功能点：主/次/错误文字与分隔线的深色层级已精确定义。
- 范围：四个 Vue 模板、一个测试和本请求文档。
- 交互/状态：原样保留，无新分支。
- 上下文：`S1 local` / scoped / compact。
- blocker：none。
