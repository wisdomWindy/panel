# 代码上下文

## Graph / Fallback

code graph 在相邻已完成请求中已确认不可用；本轮使用定向 `rg`、完整 owner 文件阅读与测试闭包。

## Root Cause 1: Popup Position

`TauriMediaEventSink::publish_gift` 已把 `popupPosition` 放入 `gift_alert_trigger`；`useMediaRuntime` 也把事件写入 `useMediaGiftsStore.latestAlert`。但全 renderer 没有 Vue 组件消费 `latestAlert`，因此不存在本地弹窗。用户看到的右下角是 `TauriNotificationPort` 发出的系统通知，位置由 Windows 控制。

## Root Cause 2: Sound

`GiftWorker` 仅在 `bindings[gift_id]` 存在且启用时调用音频端口。`gift_notify_test` 生成固定 gift ID `preview`，不会命中任何真实绑定；真实高价值礼物无专属绑定时也静默。`methods.sound=true` 没有默认声音路径。

## Broken Test Path

`gift_notify_test` 仅将 preview 放入 worker，没有调用 event sink 或返回 alert payload，所以测试按钮不能触发 renderer 本地弹窗；当前可见的只有系统通知。

## Working References

- `CaptainEffectSurface`/effect window 已有 typed payload consumer。
- `GiftAlertPayload`、adapter、store、runtime listener 已完整存在，可复用。
- `RodioAudioOutput` 是统一音频 owner，可在同一端口提供生成式短提示音，避免新增二进制资产。

## Impact Boundary

主窗口 App/礼物弹窗组件、gifts store、gift test IPC、GiftMediaRuntime sound fallback、audio port/rodio 实现及相邻 tests。专属 gift sound binding 仍优先，其他 media 路径不改。
