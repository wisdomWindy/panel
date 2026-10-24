# 实施计划

## Delivery Unit

`gift-alert-position-sound-fixes`，S2 scoped，串行 TDD。

## T1 Renderer Local Popup

- RED：扩展 media page contract，要求 App 接入 `GiftAlertOverlay`、overlay 五位置/center/timeout cleanup；扩展 gifts store test，要求 test command payload 进入 `latestAlert`。
- GREEN：新增生命周期组件；App组合 store payload；store `testNotify` 适配 result并接收。
- 文件：`App.vue`、`components/media/GiftAlertOverlay.vue`、`stores/media/gifts.ts`、tests。
- 约束：Tailwind utility、无 scoped/inline style；timer cleanup；无新文案/manager。

## T2 Test Payload Contract

- RED：TS command result与 Rust contract test要求 `GiftAlertPayload`/`GiftAlertPayloadDto`。
- GREEN：`GiftMediaRuntime::preview -> GiftRecord`；IPC纯 mapper构造 preview DTO；TS result map更新。
- 文件：`gifts.rs`、`ipc/media.rs`、desktop contract、contract tests。
- 约束：real event不变；mapping复用现有 field semantics。

## T3 Sound Fallback

- RED：Rust tests覆盖 sound off、custom priority、no-binding fallback；RecordingAudio记录 file/tone calls。
- GREEN：`AudioOutputPort::play_alert`；worker custom-first fallback；Rodio generated short tone与 cleanup。
- 文件：`ports.rs`、`gifts.rs`、`audio.rs`、application/infrastructure tests。
- 约束：全局音量有效；不新增 asset/dependency；错误 log但不阻断其他提醒。

## Review Checklist Contract

- robustness：malformed DTO走adapter；timer stale/unmount；音量校验；audio errors fallback/log。
- maintainability：component仅 transient UI；worker helper仅声音决策；无>150行新增混责函数或魔法路径。
- performance/memory：每条提醒仅一个 timer；旧 timer清理；tone worker与 playback drop可终止；无 cache/debounce。
- conventions：Vue setup/Pinia/typed desktop/Tailwind/Rust port+tokio沿用现有模式。
- field semantics：位置仅本地弹窗；OS toast不声称可定位；已有礼物字段格式不变。

## Verification

focused Vitest与Cargo RED/GREEN；Cargo fmt/test；Vitest full；type-check；lint；web build；完整 `build:free`/NSIS。符号闭包与 diff check。

## Pattern / Locality

Level 2 existing adapter/store/event/port，overlay component为获批生命周期边界；position helper组件内；无通用抽取。

## Rollback

移除 overlay组合、test payload返回和 port tone method即可恢复；无数据迁移。
