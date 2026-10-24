# Task Board: page-content-spacing-background

| ID | 任务 | 状态 | Ready | 模式 | 前置 | 文件/符号 | 约束与来源 | 测试切入点 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| T1 | 壳层回归契约（RED） | done | ready | 串行 | approved spec | `shell-components-contract.test.ts` | BF1-BF3；U1-U5；product-content delta: none；只加局部静态断言 | focused Vitest 按预期失败 |
| T2 | 共享 shell 修复（GREEN） | done | ready | 串行 | T1 RED | `App.vue` root/main class | 两个 token；Tailwind only；Level 0 direct code；无页面/状态/API 改动 | focused Vitest 4 passed + class 检索通过 |
| T3 | 验证与审查 | done | ready | 串行 | T2 GREEN | tests/build/runtime/diff | 多路由、深色保护、dirty worktree 隔离；product-content delta: none | tests/type/lint/build/1280x800/diff |

## Execute readiness

- 业务功能点/规则：BF1-BF3 已完整定义；无字段、权限、校验、API 或状态语义缺口。
- 页面/模块范围：全部主路由共享 `App.vue` shell；不得改 route SFC。
- 关键交互与状态：原样保留；无新增 loading/error/empty 分支。
- 上下文预算：S1 scoped，只读 App、shell test、相关回归与 scripts。
- 已批准技术假设：静态 source contract 适合锁定 Tailwind shell token；可逆、非产品语义。
- Blockers：none。
