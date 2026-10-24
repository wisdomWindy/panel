# 任务看板

| ID | 任务 | 状态 | Execute-ready | 模式 | 范围 | 测试切入点 | 来源 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| T1 | 建立预设目录与选择/渲染的失败测试 | completed | ready | 串行 | Rust DTO/contract test，TS contract/adapter/UI contract tests | Vitest + Cargo focused tests | spec AC1/AC2/AC4 |
| T2 | 实现 Rust 目录、TS adapter 与页面选择配色 | completed | ready | 串行 | IPC snapshot -> adapter -> page draft | T1 tests green | spec AC1-AC3，ADR-01 |
| T3 | 实现 Canvas 多风格映射与绘制 | completed | ready | 串行 | `CaptainEffectSurface.vue` 局部策略 | UI contract + type-check | spec AC4/AC5，ADR-02 |
| T4 | 变更链清理、验证和审阅预检 | completed | ready | 串行 | scoped diff 及相关检查 | type/lint/Vitest/Cargo/diff | spec AC1-AC6 |

## 通用执行约束

- speed profile：`S2 scoped`，只读取和验证特效链相关文件。
- pattern：Level 1 typed record / adapter 轻量结构，禁止新 manager/factory/plugin 层。
- 函数式边界：目录/适配/风格选择为纯转换；草稿写入只在页面 handler；Canvas 副作用只在 draw 生命周期。
- 变更链：每个修改都必须在 `Rust catalog -> DTO -> adapter -> page/config -> payload -> canvas` 中有明确位置，测试只是适配面，不是生产代码保留理由。
- 人工审阅：只检查本请求 scoped diff，不回退工作树中既有用户改动。
