# 架构设计

## Owners

- `GiftMediaRuntime`：gift alert 执行策略与 custom/default sound 选择。
- `AudioOutputPort` / `RodioAudioOutput`：文件音频与内置短提示音的设备播放。
- media IPC：preview GiftRecord 到既有 GiftAlert DTO。
- media gifts store：typed latest alert state；real/test 两条输入汇合。
- `GiftAlertOverlay`：本地弹窗 position/render/timer lifecycle；`App.vue` 仅组合。

## Data Flow

real: LiveEvent -> runtime + event sink -> adapter -> store -> overlay。

test: config -> runtime preview/audio -> command GiftAlertPayload -> adapter -> store -> overlay。

sound: exact enabled binding -> file play；missing/invalid/play failure -> `play_alert(globalVolume)`；disabled method -> none。

## Dependency Rules

domain/application 不依赖 Vue/Tauri DTO；IPC 读取 application record；store 依赖 desktop contract/adapter；overlay 依赖 typed payload与 formatter，不依赖 store/transport。Rodio 实现 port，application 不依赖 rodio。

## ADR-1 Local Overlay

复用现有 `latestAlert` state，新增一个独立生命周期组件。单 caller 例外由 timer cleanup、五位置状态映射和 transient rendering 证明；若内联 App 会混入业务展示/计时职责。无全局 manager。

## ADR-2 Test Payload

`gift_notify_test` 返回现有 GiftAlertPayload，不另建事件缓存或直接让 IPC操作 Pinia。Store 复用 adapter并赋值。真实 event不变。

## ADR-3 Built-In Tone

在 `AudioOutputPort` 增加 `play_alert`，Rodio 用生成式 sine tone，避免文件部署/路径/grant。custom binding 优先；fallback 只在无可播放 custom 时触发。音频 playback 继续由现有 RAII/stop worker 管理。

## State / Failure / Cleanup

`latestAlert` 单 owner；overlay timer 是派生生命周期，不复制业务 state。timer 在 payload变化和 unmount清理。音频 output error不阻断 popup/notification；custom error记录后尝试 tone，tone error记录后结束。

## Reuse / Anti-DRY

复用 GiftAlertPayload、adapter、store、formatter、RodioPlayback。位置映射只在 overlay 一处，保持本地。文件播放与 tone setup共享 `RodioPlayback`，不抽通用 manager。无 shared API promotion。

## Complexity Budget

新增生产文件 1 个 Vue SFC；新增 port method 1；修改 command result 1；无依赖、store、route、配置字段或 migration。

## Verification

Rust worker custom/fallback/off tests；TS store test result -> latestAlert；static component contract验证 App接线、五位置、timer cleanup；全量构建。
