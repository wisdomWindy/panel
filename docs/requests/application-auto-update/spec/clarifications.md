# 应用自动更新与检查更新 - 澄清记录

## Resolved By Source

1. “自动更新”按 PRD 9.18 执行为启动后自动检查，有更新时展示对话框，用户确认后才下载/安装；不是无提示静默安装。
2. 离线检查必须静默跳过，不得阻断应用启动。
3. 强制更新由原生契约中的 `isForced` 决定，不提供普通延后/跳过通道。

## Framework Assumptions

- 可逆技术选择：手动检查入口放在已显示应用版本和更新通道的设置 About 区域，不改变用户获得的业务能力。
- 可逆技术选择：自动检查由现有 release runtime 编排，全局对话框由已存在的主 `App.vue` 挂载。

## Open Questions

无阻塞性 `missing-source` 项。

## Out Of Scope Candidates

- 可配置自动检查开关/频率。
- 后台定时轮询或静默预下载。
- 新的发布后端、分析埋点或更新历史数据库。
