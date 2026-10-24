# Clarifications

## Confirmed Decisions

- 当前视觉、模板、时长、窗口位置与关闭行为不变。
- `null` 仅表示当前无活动特效；查询失败不得伪造内容。
- 瞬时 event 继续用于实时更新，只读 query 用于恢复窗口首帧。
- 2026-09-23 复发确认：预览首次打开必须显示现有配置对应的演示；监听权限失败不得连带阻断 current query。

## Framework Assumptions

- 使用现有 typed desktop command/adapter 模式是可逆技术选择，不改变产品语义。

## Missing Source

无。

## Open Questions

无阻塞项。

## 2026-09-23 Semantic Reopen

- Confirmed by code facts: “上舰特效” consumes captain purchase/renewal events, not room-entry events.
- Confirmed by U2: the room-entry sentence is contradictory and cannot remain under the purchase/renewal interpretation.
- Decision: remove the sentence without replacement copy, avoiding an unconfirmed product wording choice.
- Missing source: none.
