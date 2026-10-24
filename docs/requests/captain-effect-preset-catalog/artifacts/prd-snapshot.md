# 规范化缺陷快照

## 原文证据

见 `artifacts/bugfix-source.md` 中 U1 与 P1-P3。

## 问题摘要

Rust IPC 将上舰特效 `presets` 固定为空数组，配置页只能用当前已保存 ID 构造兼容选项，因此用户只看到 `captain-default`，与 PRD 的 10+ 预设及“金色尊享”不一致。

## 业务目标

主播可从至少 10 个内置模板中选择上舰特效，能直接选到“金色尊享”，并通过预览确认选择后的配色和视觉效果。

## 显式约束

- 使用现有 `captain_effect_get/save/preview` 与 `captain_effect_trigger` 链路。
- 自定义配色仍是最终渲染颜色，模板颜色仅在用户切换模板时作为默认值。
- 未知或旧版 `presetId` 不导致配置丢失或预览失败。
- 不增加网络下载、外部素材、音效文件或新 IPC 命令。

## 影响链路

`ipc::media::captain_snapshot -> CaptainEffectPresetDto -> adaptCaptainEffectSnapshot -> effects store -> captain-effect page -> CaptainEffectConfig -> captain_effect_preview/trigger -> CaptainEffectSurface`

## 非范围

不改动上舰事件队列、音效选择、音量、连击策略、窗口尺寸/位置和直播连接。
