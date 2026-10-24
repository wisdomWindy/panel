# 架构设计

## Scope

仅修复 captain-effect 窗口首帧 payload 交付；保持 framework-approved spec 的视觉、字段、事件和验收语义不变。

## Business Capability Boundary

- `EffectRuntime`：上舰特效队列、当前活动项和生命周期 owner。
- media IPC：application read model 到 `CaptainEffectPayloadDto` 的唯一转换 owner。
- desktop contract/adapter：unknown payload 到可信 renderer 类型的边界。
- `EffectWindowRoot`：窗口订阅、初始查询和竞态协调 owner。
- `CaptainEffectSurface`：纯展示消费者，不查询 runtime。

## Ownership Map

| Concern | Owner | Writable/derived |
| --- | --- | --- |
| pending/current active | Rust `EffectQueueState` | writable single owner |
| effective config/isPreview | active read model | worker 写入，查询只读 |
| DTO normalization | `ipc/media.rs` | pure derived |
| window payload | `EffectWindowRoot` | lifecycle-local writable |
| canvas frame | `CaptainEffectSurface` | payload-derived |

## File Structure And Locality

- 修改 `application/media/effects.rs`：active read model 和查询方法，就近于 runtime owner。
- 修改 `ipc/media.rs`：current command 与 payload builder，就近复用既有 DTO mapper。
- 修改 `lib.rs`：命令注册。
- 修改 renderer media contract/adapter：nullable command response。
- 修改 `EffectWindowRoot.vue`：监听后查询及事件优先保护，保持单调用方逻辑本地。
- 修改相邻 tests。无新生产文件、hook、store 或 component。

## Dependency Rule Matrix

| From | May depend on | Must not depend on |
| --- | --- | --- |
| EffectRuntime | domain media/live-data types, ports | IPC DTO, Vue/Tauri API |
| IPC | application read model, domain mapper | renderer implementation |
| desktop adapter | desktop raw contract | Vue component state |
| EffectWindowRoot | typed command/listener, adapter, surface | Rust/domain internals |
| surface | typed payload | invoke/listen/runtime state |

公开 API：`EffectRuntime::active_effect`、Tauri `captain_effect_current`、typed command response。私有：DTO builder、Vue revision guard。无循环风险。

## State Topology

`EffectQueueState.active` 是当前活动项的唯一 Rust owner，包含 `EffectItem + effective config + isPreview`。worker 在 `snapshot.current` 同一临界区写 active，在效果结束同一临界区清理。Vue payload 是窗口展示副本；query/event 两条输入按事件版本决策，不形成第二个业务 owner。

## Data Lifecycle

- live：`LiveEvent::Captain -> EffectItem(with audience) -> ActiveCaptainEffect -> CaptainEffectPayloadDto -> adapter -> window payload -> surface`。
- preview：preview synthetic audience metadata + config override -> 同一 active/DTO/adapter 链。
- initial recovery：listener ready -> current command -> nullable adapter -> payload。
- realtime：event -> existing adapter -> payload；event revision 阻止早发 query 的旧结果覆盖。

## Data And Type Strategy

- `EffectItem` 保留绘制与 DTO 所需 audience 信息；preview 使用明确的 synthetic audience，而非空字符串。
- 新增 `ActiveCaptainEffect` 作为 application read model，derive `Clone/Debug/PartialEq`，不要求 `Eq`。
- command response 为 `Option<CaptainEffectPayloadDto>` / `CaptainEffectPayload | null`。
- `null` 是无 active；异常仍通过 rejected command 表达。

## Pure Functions And Side Effects

- pure：active/event 到 DTO、unknown nullable response 到 typed nullable payload。
- side effects：worker state mutation、Tauri emit/invoke、Vue lifecycle assignment。
- 不在 computed/watch/template 进行 DTO 语义兜底。

## Reuse Candidates And Anti-DRY

| Candidate | Semantics/layer | Sites | Stability/axes | Decision |
| --- | --- | --- | --- | --- |
| Captain payload DTO construction | 同一 media IPC 语义 | event + query | stable；source/live-preview 两轴 | 抽为 `ipc/media.rs` 私有 builder |
| renderer payload validation | 同一 adapter 语义 | event + query | stable；nullable 只在 command 外层 | 复用 `adaptCaptainEffectPayload`，加窄 nullable adapter |
| lifecycle race guard | 特效窗口本地 | 1 | 单一窗口生命周期 | 留在 SFC，不抽文件 |
| gift alert recovery | 不同能力/窗口生命周期 | adjacent only | 未在缺陷范围 | 保持分离 |

共性分类：DTO builder 为 domain/media 边界；nullable adapter 为技术 contract 边界；无 shared promotion，因此无需公共 JSDoc。

## Pattern-Fit Decision

Level 2 reuse existing typed command + adapter + lifecycle pattern。查询/事件是 CQRS 风格的既有 query/notification 协作，但不新增 formal bus。拒绝：定时重发、全局 store、observer manager、factory/registry、通用 durable event cache；它们增加 owner 或时序复杂度而未解决更多已批准行为。

## Architecture Decision Records

### ADR-1 Active Read Model

- problem：瞬时事件不能作为新窗口首帧状态源。
- choice：runtime 队列状态增加 active read model，并提供只读查询。
- alternative：延时重发/预创建窗口；均仍依赖时序。
- dependency：application 不依赖 IPC；IPC 单向读取 application。
- state/data/side effect：worker 写，command 读，DTO mapper 转换。
- migration：补齐 EffectItem audience，更新测试；无兼容双路径。
- proof：preview/live/null/clear Rust tests。

### ADR-2 Listener-Then-Query

- problem：query 与 event 并发时可能覆盖新数据。
- choice：先注册 listener，再 query；记录事件 revision，只有查询期间无新事件时应用 query。
- alternative：query 后 listen 会留下窗口期；Promise race 无法定义新旧。
- owner：EffectWindowRoot lifecycle。
- proof：组件/契约测试验证顺序和 stale protection。

## Extension Seams And Rejected Future-Proofing

唯一 seam 是 typed command map；不推广成通用窗口状态协议。未来若其他窗口出现同类缺陷，应分别验证生命周期后再设计，不在本次预建 abstraction。

## Complexity Budget

- 新生产文件：0。
- 新公开 command：1；新 application read method：1。
- 最大间接层：runtime -> IPC mapper -> renderer adapter -> root。
- reactive fan-out：单一 ref；每次窗口挂载一次查询。
- 迁移范围：captain-effect 闭包内约 8 个文件含测试。

## Compatibility, Cleanup And Rollback

保留原 event contract 作为实时通道，无 feature flag、fallback 或临时兼容路径。回滚面为 current command/active read model/root query；event 原行为不变。无需后续 cleanup trigger。

## Verification Boundary

- Rust：active 在 preview/live 中内容正确，结束清理。
- contract：命令注册及 nullable adapter。
- Vue：listen-before-query、event-wins、unmount guard。
- integration：type-check、Vitest、Cargo tests、desktop build。

## Risks And Open Questions

风险：active 与 snapshot 清理不一致、query 迟到覆盖 event。设计通过同临界区写清理和 revision guard约束。无开放问题。
