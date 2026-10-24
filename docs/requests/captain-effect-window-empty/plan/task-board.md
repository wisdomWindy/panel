# Task Board

| ID | 名称 | 状态 | Readiness | 模式 | 目标文件/符号 | 规格/来源 | Product delta | 测试切入点 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| T1 | Rust active payload | completed | ready | 串行 | `effects.rs`, `media.rs`, `lib.rs`, `permissions/foundation.toml`, `capabilities/effect.json`, Rust tests | AC1-3 / U1 + code facts | none | active preview/live/clear/capability RED |
| T2 | Renderer 首帧恢复 | completed | ready | 串行，依赖 T1 | media contract/adapter, `EffectWindowRoot.vue`, tests | AC4 / U1 + ADR-2 | none | nullable + event-wins RED |
| T3 | 回归与审查 | completed | ready | 串行，依赖 T2 | verification/review artifacts | AC5 | none | full tests/type/lint/build |
| T4 | Preview permission and startup isolation | completed | ready | serial | `capabilities/effect.json`, `EffectWindowRoot.vue`, contract tests | AC6-8 / reopen evidence | none | capability + listener failure isolation RED |
| T5 | Remove captain/entry semantic contradiction | completed | ready | serial | `CaptainEffectSurface.vue`, zh/en locales, media contract test | AC9-12 / U2 | remove entry sentence only | source-contract RED then focused/full Vitest |

## 执行约束

- speed/context：S2 scoped，仅 captain-effect 调用链。
- API：`captain_effect_current -> CaptainEffectPayload | null`；adapter 是语义边界。
- 副作用：Rust worker/state、Tauri command、Vue lifecycle；转换保持纯函数。
- 架构：active 单 owner，event 新数据优先，listener/unmount cleanup 必须验证。
- 测试：严格 RED -> GREEN -> REFACTOR；每个 RED 必须记录预期失败。
- 变更链：修改前后搜索命令、调用方、event、active 和 tests，禁止孤立路径。
- 整洁性：无新 helper 文件/store/component、无无关格式化、无 debug/dead/stale/unused。
- 禁止扩展：不改模板视觉、窗口几何、音频、其他 media 能力。
- 待确认项：无。
