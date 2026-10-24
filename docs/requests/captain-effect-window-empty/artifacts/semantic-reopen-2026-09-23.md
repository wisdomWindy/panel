# Semantic Reopen Source

## Source System

Direct user feedback in the current Codex thread.

## Defect Title

Captain purchase effect contains an unrelated live-room-entry message.

## Verbatim Defect Evidence Index

- U2: “上舰特效指的是开通舰长时的特效还是舰长进入直播间的特效，如果是开通舰长时的特效为什么特效窗口中的文本存在欢迎进入直播间，如果是进入直播间的特效，就不能把页面和菜单名称设置为上舰特效”

## Observed Behavior

The page/menu is named “上舰特效”, while the effect Canvas also renders “欢迎加入直播间”, mixing captain purchase semantics with audience-entry semantics.

## Expected Behavior

The feature must have one coherent business meaning. Repository event facts establish that this feature is triggered by captain purchase/renewal events, so the unrelated live-room-entry message must not be rendered.

## Reproduction Clues

Open the 上舰特效 page, click Preview, and inspect the text drawn in the effect window.

## Affected Module Or Page

`CaptainEffectSurface.vue` and the now-unused `captainEffect.welcome` locale entries.

## Code Facts

- `TauriMediaEventSink::publish_captain_effect` accepts only `LiveEvent::Captain`.
- The payload action contract accepts only `purchase` and `renewal`.
- `LiveEvent::Enter` is a separate event path and does not trigger this effect window.

## Closed-World Fix Scope Allowlist

- Remove the live-room-entry sentence from the captain purchase/renewal effect Canvas.
- Remove the locale keys that become unused solely because of that removal.
- Add/update regression coverage proving the surface no longer references entry semantics.
- Preserve page/menu naming, action/level/username display, event contracts, layout, templates, and configuration behavior.

## Open Questions And Missing Context

None. Removing the contradicted sentence avoids inventing replacement product copy.
