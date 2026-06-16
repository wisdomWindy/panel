# 直播录制功能 - 任务看板

## 任务列表

| 任务 ID | 任务名称 | 状态 | 执行模式 | 执行说明 | 关联规格区域 |
|---------|----------|------|----------|----------|--------------|
| T1 | 数据库表结构与类型定义 | ✅ 已完成 | 串行 | 创建 types.ts，扩展 database.ts | spec/spec.md - 数据库表结构 |
| T2 | FFmpeg 管理器 | ✅ 已完成 | 串行 (TDD) | 创建 ffmpeg-manager.ts | spec/spec.md - 主进程录制模块 |
| T3 | 录制引擎 | ✅ 已完成 | 串行 (TDD) | 创建 recording-engine.ts | spec/spec.md - 录制引擎设计 |
| T4 | 高光检测器 | ✅ 已完成 | 串行 (TDD) | 创建 highlight-detector.ts | spec/spec.md - 高光检测器设计 |
| T5 | 文件管理器 | ✅ 已完成 | 串行 (TDD) | 创建 file-manager.ts | spec/spec.md - 文件管理器设计 |
| T6 | 主进程模块入口与 IPC 注册 | ✅ 已完成 | 串行 | 创建 index.ts，修改 ipc-handlers.ts | spec/spec.md - IPC 通道设计 |
| T7 | 录制状态管理 | ✅ 已完成 | 串行 | 创建 stores/recording.ts | spec/spec.md - stores/recording.ts |
| T8 | 录制服务 | ✅ 已完成 | 串行 | 创建 recording-service.ts | spec/spec.md - recording-service.ts |
| T9 | 录制状态栏组件 | ✅ 已完成 | 串行 (TDD) | 创建 RecordingStatusBar.vue | spec/spec.md - RecordingStatusBar.vue |
| T10 | 录制文件列表组件 | ✅ 已完成 | 串行 (TDD) | 创建 RecordingList.vue | spec/spec.md - RecordingList.vue |
| T11 | 高光时刻列表组件 | ✅ 已完成 | 串行 (TDD) | 创建 HighlightList.vue | spec/spec.md - HighlightList.vue |
| T12 | 录制管理页面 | ✅ 已完成 | 串行 | 创建 Recording.vue | spec/spec.md - Recording.vue |
| T13 | 设置页面集成录制配置 | ✅ 已完成 | 串行 | 修改 Settings.vue | spec/spec.md - 页面与模块设计 |
| T14 | 路由配置与菜单入口 | ✅ 已完成 | 串行 | 修改 router/index.ts, App.vue | spec/spec.md - 页面与模块设计 |
| T15 | 快捷键注册与全局监听 | ✅ 已完成 | 串行 | 修改 main/index.ts | spec/spec.md - 快捷键配置 |
| T16 | 集成录制模块到启动流程 | ✅ 已完成 | 串行 | 修改 main/index.ts, App.vue | spec/spec.md - 用户流程 |
| T17 | 端到端测试与验证 | ✅ 已完成 | 串行 | 验证所有验收标准 | spec/spec.md - 验收标准 |

## 依赖图

```
T1 → T2 → T3 → T6 → T7 → T9 → T12 → T13 → T17
 ↓         ↓    ↑         ↓    ↑
 T4 ──────→ T6  T5 ──→ T6   T10 → T12
                                    ↓
                                   T11 → T12
                                   
T6 → T8 → T7
T6 → T15
T6, T9 → T16 → T17
T12 → T14 → T17
```

## 执行进度

- 总任务数: 17
- 已完成: 17
- 进行中: 0
- 待开始: 0

## 备注

- 所有任务按串行顺序执行
- T2-T5、T9-T11 采用 TDD 方式（先写测试再实现）
- 每个任务完成后应提交代码
- 实现需符合 spec 和 plan 的要求
