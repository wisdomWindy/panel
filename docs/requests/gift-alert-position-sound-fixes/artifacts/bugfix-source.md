# 缺陷来源

## 来源系统与标识

- source: 当前 Codex 会话
- defect id: `gift-alert-position-sound-fixes`
- affected: 礼物提醒页面、主窗口本地弹窗、Rust 礼物媒体 worker

## Verbatim Defect Evidence Index

- U1：“礼物提醒页面的弹窗位置选择居中之后，实际弹窗位置还是在右下方”
- U2：“礼物提醒页面的提醒方式中选择了声音、本地弹窗、系统通知，结果没声音”

## Observed / Expected

- observed 1：选择居中后，只看到右下方提醒。
- expected 1：本地弹窗按选择的位置显示；系统通知位置不受应用控制。
- observed 2：同时选择声音、本地弹窗、系统通知后没有声音。
- expected 2：声音方式开启时有可听提醒；专属音效存在时保持优先。

## Reproduction Scope

礼物提醒页面保存或使用当前草稿点击“测试提醒”；真实高价值礼物沿用同一提醒配置。

## Closed-World Fix Allowlist

- source-backed：本地弹窗遵循 popupPosition；声音开关产生声音。
- code-fact-backed：渲染 `latestAlert`、测试命令发出 `gift_alert_trigger`、内置提示音作为无绑定 fallback、timer cleanup、测试适配。
- out-of-scope：移动 Windows toast、改变礼物绑定选择、视觉重设计、其他媒体功能。

## PRD Anchor

PRD 9.7.2 高价值礼物触发系统通知 + 本地弹窗 + 音效；9.7.5 `gift_notify_test` 触发本地弹窗 + 音效；9.7.6 系统通知权限失败降级本地弹窗、音效失败静默降级并记录日志。

## Ambiguity Resolution

“右下方”与代码事实一致地指向 Windows 系统通知：当前应用没有本地弹窗 renderer。位置配置只约束本地弹窗，不能移动系统通知。声音的最低可接受行为是专属绑定优先、无绑定时使用应用内置提示音，避免勾选后静默无效果。
