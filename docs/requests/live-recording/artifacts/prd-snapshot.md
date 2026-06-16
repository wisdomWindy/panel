# PRD 快照

## 来源摘要

来自 `docs/requirements/Bilibili_Live_Tool_PRD.md` 第 3.4 节「直播录制」。该功能为 Bilibili 直播助手的 Could Have 级别功能，提供直播流录制能力，支持全程录制和高光片段录制，满足主播内容存档、二次剪辑和内容复用需求。

## 关键业务目标

- 主播可全程录制直播内容，便于后期剪辑和存档
- 系统自动检测热度激增（弹幕刷屏、高价值礼物、舰长赠送）并标记高光时刻
- 主播可手动打点标记高光时刻
- 高光片段可独立保存，便于直接上传 B 站动态
- 录制过程对系统性能影响可控（CPU < 15%、内存 < 200MB）

## 相关模块/页面

- 新增 `src/renderer/views/Recording.vue`：录制管理页面
- 新增 `src/renderer/components/RecordingStatusBar.vue`：录制状态栏组件
- 新增 `src/renderer/components/RecordingList.vue`：录制文件列表组件
- 新增 `src/renderer/components/HighlightList.vue`：高光时刻列表组件
- 新增 `src/renderer/services/recording-service.ts`：录制服务
- 新增 `src/renderer/stores/recording.ts`：录制状态管理
- 新增 `src/main/modules/recording/`：主进程录制模块
- `src/renderer/views/Settings.vue`：设置页面新增录制配置项
- `src/renderer/App.vue`：主框架布局新增录制状态栏
- `src/main/database/`：新增 recordings/highlights 数据表

## 上游需确认问题

1. FFmpeg 集成方式：使用 fluent-ffmpeg 库调用系统 FFmpeg，还是打包 FFmpeg 二进制文件？
2. 直播流获取方式：从 Bilibili 直播间获取直播流 URL 的具体协议和接口
3. 高光自动触发的弹幕阈值默认值：弹幕数量达到多少时触发高光标记？
4. 高价值礼物的默认阈值：礼物价值达到多少舰长数时触发高光标记？
5. 分段录制在关键帧处分段的技术实现细节
6. 硬件加速支持范围：是否需要在 MVP 阶段支持 NVIDIA NVENC / Intel Quick Sync？
7. 录制文件的元数据存储方案：使用 SQLite 还是独立 JSON 文件？
