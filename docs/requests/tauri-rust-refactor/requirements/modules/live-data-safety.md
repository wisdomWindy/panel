# Module: Live Data And Safety

## Boundary

PRD 功能 1、2、3、5、9、10、11：只读连接/事件、身份、敏感词、黑名单、看板、榜单、CSV。

## Raw Source Snapshot

- 9.1：“仅做只读接收，无任何主动发送、操作直播间权限”；5 秒重连，最多 10 次。
- 9.2：guard/admin/vip/medal 归一为 IdentityTag；缺失降普通。
- 9.3：Aho-Corasick 精确、regex 模糊；屏蔽展示/TTS/AI，过滤日志保留。
- 9.5：黑名单用户“不展示、不播报、不统计”。
- 9.9：每 5 秒 `stat_updated`；五指标、三趋势；两个安装包共有。
- 9.10：每 10 秒 `rank_updated`；三榜、TOP100、变动高亮 3 秒。
- 9.11：系统对话框授权路径，Rust 输出 CSV。

## Markdown Normalization

不适用：来源已是 Markdown 表格和文本流程。

## Source Grounding And Allowlist

- 允许页面：弹幕面板、敏感词、黑名单、数据看板、排行榜、导出弹窗、用户详情卡。
- 允许控件/字段/状态完全按 PRD 9.1/9.2/9.3/9.5/9.9/9.10/9.11。
- forbidden：发送输入框/发送按钮、前端 WebSocket、前端 SQL、任意导出路径。

## Responsibilities

- Vue：视图筛选/搜索/暂停、图表/列表、loading/empty/error、调用 gateway。
- Rust：房间初始化、WS/reconnect、parse/filter/log/stat/rank/export。
- External：B 站只读 HTTP/WS；正式凭证由配置提供。

## State And Acceptance

- 状态：disconnected/connecting/connected/reconnecting/error；高并发时 UI 批量渲染但日志/统计不丢失。
- 目标：弹幕 <100ms、1000/s、连接 >99%、重连 >=95%、统计与榜单准确。

## Downstream Obligations

- page_design_required: `true`
- architecture_design_required: `true`
- entry_stage: `page-design`
- execution_order: `2`
