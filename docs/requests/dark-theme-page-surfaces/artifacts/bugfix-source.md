# Bugfix Source

## Source System

Codex 当前会话中的用户直接反馈与附件截图。

## Source Scope And Identifier

- Scope：桌面应用深色主题视觉一致性。
- ID：`dark-theme-page-surfaces`。
- Title：深色主题下四个页面白黑表面混用。

## Observed Behavior

切换深色主题之后，弹幕面板页面、数据看板页面、敏感词页面和黑名单页面仍然“一块白、一块黑”。截图显示应用壳已为深色，但指定页面存在固定白色根表面、页头或页脚；敏感词和黑名单的表格主体已是深色，进一步暴露白黑白分层。

## Expected Behavior

四个指定页面在深色主题下使用连续、可读的深色表面层级，不再出现固定白色大块；浅色主题及现有业务行为保持不变。

## Reproduction Clues

1. 启动应用并切换深色主题。
2. 依次进入弹幕面板、数据看板、敏感词、黑名单。
3. 观察页面根表面、页头、内容区、指标区和页脚。

## Affected Pages

- `/danmaku-list`
- `/dashboard`
- `/sensitive-words`
- `/blacklist`

## Available Evidence

- U1：用户原文“切换深色主题之后弹幕面板页面、数据看板页面、敏感词、黑名单页面还是一块白、一块黑的”。
- U2：`codex-clipboard-745d0c14-deb6-43db-895e-4204bfa08d4b.png`，弹幕面板固定浅色内容区。
- U3：`codex-clipboard-bceb463d-aa99-4686-b6e7-69aa774d2e88.png`，数据看板固定浅色指标和图表区。
- U4：`codex-clipboard-34231c91-d84c-4ef2-83d4-4ccd6fb39ac2.png`，敏感词页白色页头/页脚与深色表格割裂。
- U5：`codex-clipboard-c4542dc7-5cfc-41ba-8e4d-ea95940c38bd.png`，黑名单页白色页头/页脚与深色表格割裂。

## Verbatim Defect Evidence Index

| Evidence | Verbatim / stable anchor | Meaning |
| --- | --- | --- |
| U1 | “切换深色主题之后弹幕面板页面、数据看板页面、敏感词、黑名单页面还是一块白、一块黑的” | 四个指定页面必须消除深色模式的黑白混用 |
| U2-U5 | 当前会话四张附件截图 | 锁定受影响页面和可见表面范围 |

## Closed-World Fix Scope Allowlist

- 为四个指定页面现有表面、标题、分隔线和辅助文字补齐深色主题状态。
- 为数据看板现有指标块、指标文字、图表容器/空状态补齐深色主题状态。
- 增加只验证上述主题契约的聚焦测试。
- 允许为保持现有视觉层级而复用项目既有 `zinc` 深色 utility 和 Element Plus 暗色变量。

## Out Of Scope

布局重构、文案变化、路由变化、字段/交互增删、数据查询、store/IPC/Rust、主题选择逻辑、品牌色、其他页面主题审计。

## Open Questions And Missing Context

无阻塞项。用户期望和受影响范围由原文及截图足够明确；具体 utility 由既有排行榜深色页面合同约束，不引入新的产品语义。

