# 工程规格

## Delivery Unit

`gift-alert-position-sound-fixes`，S2 scoped bugfix。

## Goal / Scope

恢复 PRD 9.7 已定义的三种提醒方式：系统通知、本地弹窗、声音。系统通知仍由 OS 定位；应用内本地弹窗遵循五种 `popupPosition` 并在 `popupDurationSeconds` 后消失。声音开启时专属礼物绑定优先，无可用绑定或绑定播放失败时播放短内置提示音。

## Source Grounding

| Behavior | Label | Source |
| --- | --- | --- |
| 本地弹窗遵循位置与时长 | source-backed | U1，PRD 9.7.3-9.7.5 |
| 声音方式实际可听 | source-backed | U2，PRD 9.7.1-9.7.2 |
| 测试按钮触发本地弹窗与声音 | source-backed | PRD 9.7.5 |
| 系统通知位置不由应用控制 | code-fact-backed | Tauri/Windows notification boundary |
| 专属绑定优先、内置提示音 fallback | source-derived | U2 + 现有 gift-sound contract；不改变绑定语义 |

闭集产品内容：只恢复现有 username、giftName、quantity、value 与三种提醒方式；不新增配置项、导航、权限、统计或远端 API。

## User Intent Contract

- literal：居中要真正居中；勾选声音要有声音。
- practical：测试提醒和真实高价值礼物都能明确验证所选提醒方式。
- forbidden：把 Windows toast 当本地弹窗、只改下拉显示、要求用户必须先绑定一个名为 preview 的音效、静默吞掉无绑定声音。
- success：五种位置映射、自动消失、测试 payload、custom-first/fallback sound 均有回归证据。

## Functional Behavior

1. `gift_alert_trigger` 或测试命令结果进入同一个 `GiftAlertPayload` adapter/store。
2. 主窗口显示一条 transient local popup；新 payload 重置计时器，旧计时器不得清除新提醒。
3. 位置：topLeft/topRight/bottomLeft/bottomRight/center；中心使用视口水平与垂直中心。
4. 弹窗展示用户名、礼物名、数量和已有金额 formatter；无点击行为，pointer events 不拦截主界面。
5. 测试命令返回 `GiftAlertPayload`，`isPreview=true`；真实 event contract 不变。
6. 声音开启时：可用且启用的 exact gift binding -> custom file；否则或 custom play error -> built-in alert tone；声音关闭时不调用任何音频。
7. built-in tone 使用系统默认输出设备、现有全局音量，短时播放并支持 stop/drop cleanup。

## Type / API Contract

- `gift_notify_test` result 从 `{ triggered: boolean }` 收窄为现有 `GiftAlertPayload`；request 不变。
- Rust preview 返回 queued `GiftRecord` 供 IPC 纯 mapper 构造 DTO。
- `AudioOutputPort::play_alert(volume)` 是窄基础设施能力；file playback API 不变。
- TS governing config 为 package `tsconfig`/Vue strict + bundler + `@/` alias；复用现有 desktop contract、GiftAlertPayload、GiftPopupPosition。

## State / Lifecycle / Failure

- store `latestAlert` 是 renderer 唯一 writable alert state；overlay 只接收 prop 并 emit dismiss。
- timer owner 在 overlay，watch payload identity/eventId 后重新计时，unmount 清理。
- malformed payload 继续由 adapter 拒绝；test invoke failure沿用 store error。
- notification permission failure仍由 Rust worker降级为 local popup。
- custom audio failure尝试默认 tone；default tone failure按 PRD静默降级，但 Rust 记录 warning。

## UI / Field Semantics

本地弹窗是主窗口内 transient overlay，不是 OS toast。username 表示送礼用户，giftName/quantity 表示礼物与数量，valueMinor 使用现有金额 formatter。默认可见、无 hover/click/visited 语义；不获得焦点；颜色沿用当前中性/粉色产品配色，不改变页面布局。

## Quality Constraints

- Vue component extraction：新增 `GiftAlertOverlay.vue`，真实生产 caller 为 1，但具备获批单调用方例外：独立 transient timer/position/render lifecycle 与可测试边界；props `payload`，emit `dismiss`，无 store/adapter/request 依赖。
- Tailwind utilities only；基础 class 保持模板内，位置 class binding 仅承载小型状态切换，不用 scoped CSS/inline style。
- pure：position mapping、GiftRecord -> DTO；side effects：store assignment、timer、audio output。
- no cache/debounce/global manager/event bus；无新依赖。
- human review：仅 App/overlay/store contract、gift runtime/audio port/rodio/IPC 与相邻 tests；不格式化其他脏文件。

## Pattern Fit

Level 2 reuse existing event/store/adapter pattern，加 Level 1 typed position map 与 port method。Observer 已由 `useMediaRuntime` 存在；不新增 bus。Component boundary 因独立生命周期成立；声音使用既有 Port/Adapter dependency boundary。拒绝 OS notification reposition、全局 toast manager、二进制音频资产和新音频库。

## Architecture Routing

`architecture-design_required: true`：跨 renderer lifecycle、IPC result 和 audio port，需固定 owner、依赖方向、fallback 与 cleanup；不得改变上述业务语义。

## Acceptance Criteria

- AC1：App 实际消费 `latestAlert`，五种位置映射包含真正 center，时长到期清理且新提醒不被旧 timer 清除。
- AC2：`gift_notify_test` 返回并应用 `isPreview=true` payload，本地弹窗可见。
- AC3：sound off 不播放；custom binding 可用时仅播放 custom；无绑定或 custom 失败时播放 default alert。
- AC4：系统通知、真实 event、gift sound binding/save/preview、统计行为保持。
- AC5：focused RED/GREEN、全量 Rust/Vitest/type/lint/build 通过。

## Risks

主要风险为 stale timer、重复/双重声音和 trait 扩散；通过 eventId timer guard、custom-first 单路径与 port contract tests 控制。
