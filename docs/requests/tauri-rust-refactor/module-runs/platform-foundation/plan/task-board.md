# Platform Foundation Task Board

| ID | 任务 | 状态 | Readiness | 模式 | 前置 | Spec / 规则 | 范围与目标 | 质量、清理与架构约束 | Source / delta | 验证切入点 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| PF-01 | 工具链、基线、Tauri scaffold | done | ready | 串行 | plan approved | Bootstrap; ADR-001 | manifests/Vite/generated `src-tauri`; existing Vue retained | official v2 init; remove demo; no second frontend; record baseline | source/code-fact; none | versions, config parse, Vite build |
| PF-02 | Typed desktop gateway | done | ready | 串行 TDD | PF-01 | PF-BR-02; ADR-001/006 | `platform/desktop` contracts/transport/index/tests | types first, injected transport, pure error mapping, caller-owned unsubscribe; preserve legacy wrapper until closure | source-backed; none | red/green tests, type-check, import search |
| PF-03 | Rust validation/state/commands | done | ready | 串行 TDD | PF-02 | PF-BR-01..03; ADR-002/004/006 | Rust domain/application/ipc/build channel/state/tests | thin commands, no Tauri/SQLx in domain, no panic/any-equivalent, dual-feature compile | source-backed; none | fmt, clippy, unit tests both features |
| PF-04 | SQLite/config/migration | done | ready | 串行 TDD | PF-03 | PF-BR-04; ADR-003 | migration SQL, database/config infra/app/tests | exact 21/7, WAL, idempotent transaction/rollback, no generic repositories/SQL over IPC | source-backed; none | temp DB schema/rollback/config tests |
| PF-05 | Path grants/capabilities | done | ready | 串行 TDD | PF-04 | PF-BR-05; ADR-005 | path app/infra, dialog wiring, permission/capability/tests | opaque purpose grant, canonical checks, zero-permission effect, no broad fs | source-backed; none | invalid/revoke/escape tests, ACL inspection |
| PF-06 | Free/member builds and integration | done | ready | 串行 | PF-05 | PF-BR-06; ADR-004 | Cargo/Vite/Tauri overlays/scripts/locks/registry tests | matching compile channel, shared identifier, six commands absent in free, no runtime-only hiding | source-backed; none | both Rust/frontend builds, bundle/registry search |
| PF-07 | Cleanup, self-review, acceptance | done | ready | 串行 | PF-06 | all acceptance | caller closure, task-owned obsolete symbols, changelog/evidence | no unrelated cleanup; map every hunk; preserve later-module Electron code; no send path | source/code-fact; none | full scoped suite, smoke, diff/graph/search |

## 通用执行合同

- 执行人：当前 coding agent 按 PF-01 -> PF-07 顺序实施；每完成一项立即更新状态和 changelog。
- Context budget：`S3 broad`，但每项只读目标、direct callers/tests/contracts；业务 UI 不在闭包。
- API/type：Rust serde canonical DTO + TS mapped mirror；nullable/invalid semantics按 spec；external service only unavailable。
- 函数式/副作用：validator/mapper/derivation pure；Tauri/SQL/dialog/path/secure-store at explicit boundaries。
- Review：禁止技术栈漂移、宽格式化、debug/dead/stale code、broad typing、hidden side effect、unowned subscription。
- 测试引用仅是适配面；删除生产符号前必须证明无真实生产 caller。
- 待确认项：none。已批准技术假设仅为 compatible patch versions and generated v2 config syntax，不改变产品语义。
