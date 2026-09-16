# Module: Interaction Media

## Boundary

PRD 功能 6、7、8、12、13：舰长特效、礼物提醒与音效、TTS、FFmpeg 录制与高光。

## Raw Source Snapshot

- 9.6：透明/无边框/置顶 effect window，5 秒淡出，多事件排队，DPI/多屏。
- 9.7：高价值礼物触发通知/弹窗/音效，最近 20 条与本场统计。
- 9.8：MP3/WAV、独立/全局音量、预览、批量配置、队列 >=10/s。
- 9.12：TTS 优先级“舰长＞礼物＞房管＞普通”，系统/云端降级。
- 9.13：FFmpeg LGPL、MP4/FLV、F9、高光前后 30 秒、10GB 配额、5GB 水位。

## Markdown Normalization

不适用：来源已是 Markdown 表格、ASCII wireframe 与步骤。

## Source Grounding And Allowlist

- 允许页面：提醒配置、礼物音效、语音播报、录制管理、录制设置、effect window。
- 允许状态/控件仅按 9.6-9.8/9.12-9.13。
- forbidden：特效/感谢文案对直播间发送；前端启动进程或直接访问文件。

## Responsibilities

- Vue：配置、队列/状态/文件列表、受限预览、多窗口渲染。
- Rust：audio/TTS queue、notification、Sidecar、file/quota/highlight、F9、cleanup。
- External：FFmpeg LGPL shared、系统/云 TTS；缺失时明确 unavailable/degraded。

## Acceptance

- 特效 <1s/30FPS、礼物音效 <500ms、TTS <2s、录制启动 <3s、CPU 增量 <15%、水位保护生效。

## Downstream Obligations

- page_design_required: `true`
- architecture_design_required: `true`
- entry_stage: `page-design`
- execution_order: `3`
