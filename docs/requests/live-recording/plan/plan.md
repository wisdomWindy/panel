# 直播录制功能 - 执行计划

## 任务拆解

### T1: 数据库表结构与类型定义

**描述**: 创建 recordings 和 highlights 数据表的类型定义，扩展 electron-store 数据结构。

**关联规格**: spec/spec.md - 数据库表结构、API 与数据契约

**执行内容**:
- 创建 `src/renderer/modules/recording/types.ts`，定义 Recording、Highlight、RecordingConfig、RecordingSession 等接口
- 扩展 `src/main/database.ts` 的 AppData 接口，新增 recordings 和 highlights 字段
- 实现 recordings 和 highlights 的 CRUD 操作方法

**验收标准**:
- 类型定义完整，覆盖 spec 中所有接口
- 数据库 CRUD 操作正确
- 类型可被渲染进程和主进程共同引用

**执行模式**: 串行

---

### T2: FFmpeg 管理器 (ffmpeg-manager.ts)

**描述**: 实现 FFmpeg 进程管理器，负责检测 FFmpeg 可用性、管理子进程生命周期。

**关联规格**: spec/spec.md - 主进程录制模块

**执行内容**:
- 创建 `src/main/modules/recording/ffmpeg-manager.ts`
- 实现 FFmpeg 可用性检测（`which ffmpeg` 或 `ffmpeg -version`）
- 实现 FFmpeg 子进程启动、暂停、恢复、停止
- 实现 FFmpeg 输出和错误流处理
- 导出 FFmpegManager 类

**验收标准**:
- FFmpeg 可用性检测正确
- 子进程生命周期管理正确
- 错误处理完善（FFmpeg 未安装、进程崩溃）

**执行模式**: 串行（TDD：先写测试再实现）

---

### T3: 录制引擎 (recording-engine.ts)

**描述**: 实现录制引擎，封装 FFmpeg 调用，支持全程录制和分段录制。

**关联规格**: spec/spec.md - 录制引擎设计

**执行内容**:
- 创建 `src/main/modules/recording/recording-engine.ts`
- 实现 MP4（H.264+AAC）格式录制
- 实现 FLV 格式录制
- 实现质量转码（720p/1080p）
- 实现分段录制（按文件大小/时长）
- 实现暂停/恢复录制
- 实现录制状态实时回调（时长、文件大小）
- 导出 RecordingEngine 类

**验收标准**:
- MP4/FLV 格式录制正确
- 质量转码正确
- 分段录制无丢帧
- 暂停/恢复功能正确
- 状态回调实时准确

**执行模式**: 串行（依赖 T2，TDD：先写测试再实现）

---

### T4: 高光检测器 (highlight-detector.ts)

**描述**: 实现高光检测器，监听弹幕/礼物/舰长事件，自动标记高光时刻。

**关联规格**: spec/spec.md - 高光检测器设计

**执行内容**:
- 创建 `src/main/modules/recording/highlight-detector.ts`
- 实现弹幕频率统计（滑动窗口算法）
- 实现弹幕激增检测（30条/10秒，可配置）
- 实现高价值礼物检测（>= 100 舰长数，可配置）
- 实现舰长赠送检测
- 实现冷却时间机制（10秒）
- 实现高光标记触发回调
- 导出 HighlightDetector 类

**验收标准**:
- 弹幕频率统计正确
- 弹幕激增检测准确
- 高价值礼物检测正确
- 舰长赠送检测正确
- 冷却时间机制有效

**执行模式**: 串行（TDD：先写测试再实现）

---

### T5: 文件管理器 (file-manager.ts)

**描述**: 实现录制文件管理器，负责文件路径管理、元数据读取、存储空间检查。

**关联规格**: spec/spec.md - 文件管理器设计

**执行内容**:
- 创建 `src/main/modules/recording/file-manager.ts`
- 实现录制文件路径生成（按日期和直播间 ID）
- 实现存储空间检查（录制前检查可用空间）
- 实现文件元数据读取（大小、时长）
- 实现文件删除和清理
- 实现自动清理策略（按天数/总大小）
- 导出 FileManager 类

**验收标准**:
- 文件路径生成正确
- 存储空间检查准确
- 文件操作（读取/删除）正确
- 自动清理策略有效

**执行模式**: 串行（TDD：先写测试再实现）

---

### T6: 主进程录制模块入口与 IPC 注册

**描述**: 创建主进程录制模块入口，注册所有 IPC 处理器，实现事件推送机制。

**关联规格**: spec/spec.md - IPC 通道设计

**执行内容**:
- 创建 `src/main/modules/recording/index.ts`
- 注册录制控制 IPC 处理器（start/pause/resume/stop/status）
- 注册高光管理 IPC 处理器（markHighlight/getHighlights/deleteHighlight）
- 注册录制文件管理 IPC 处理器（getRecordings/deleteRecording/openFolder/getPath）
- 注册配置管理 IPC 处理器（getConfig/setConfig）
- 实现事件推送（statusChanged/highlightMarked/error）
- 修改 `src/main/ipc-handlers.ts` 导入录制模块

**验收标准**:
- 所有 IPC 通道注册正确
- 事件推送到渲染进程正常
- 与现有 ipc-handlers 集成无冲突

**执行模式**: 串行（依赖 T2-T5）

---

### T7: 录制状态管理 (stores/recording.ts)

**描述**: 创建 Pinia store 管理录制相关状态。

**关联规格**: spec/spec.md - stores/recording.ts 设计

**执行内容**:
- 创建 `src/renderer/stores/recording.ts`
- 定义 RecordingState 接口
- 实现 startRecording/pauseRecording/resumeRecording/stopRecording actions
- 实现 markHighlight action
- 实现 fetchRecordings/fetchHighlights actions
- 实现 deleteRecording/deleteHighlight actions
- 实现 updateConfig action
- 实现 IPC 事件监听（statusChanged/highlightMarked）

**验收标准**:
- Store 状态管理正确
- Actions 可正常调用
- IPC 事件监听正确更新状态

**执行模式**: 串行（依赖 T6）

---

### T8: 录制服务 (recording-service.ts)

**描述**: 创建渲染进程录制服务，封装 IPC 调用，提供统一的服务接口。

**关联规格**: spec/spec.md - recording-service.ts 接口定义

**执行内容**:
- 创建 `src/renderer/services/recording-service.ts`
- 实现 RecordingService 接口的所有方法
- 封装 IPC 调用（window.electron.ipcRenderer.invoke）
- 实现单例模式导出
- 处理错误和降级

**验收标准**:
- 所有接口方法实现正确
- IPC 调用封装正确
- 错误处理完善

**执行模式**: 串行（依赖 T6）

---

### T9: 录制状态栏组件 (RecordingStatusBar.vue)

**描述**: 创建录制状态栏组件，显示在 App.vue 底部状态栏。

**关联规格**: spec/spec.md - RecordingStatusBar.vue 设计

**执行内容**:
- 创建 `src/renderer/components/RecordingStatusBar.vue`
- 实现录制状态图标（灰色=未录制，红色闪烁=录制中，黄色=暂停）
- 实现录制时长显示（HH:MM:SS 格式）
- 实现文件大小显示（自动转换单位）
- 实现快捷操作按钮（开始/暂停/停止）
- 修改 `src/renderer/App.vue` 集成状态栏组件

**验收标准**:
- 状态图标显示正确
- 时长和大小实时更新
- 快捷按钮功能正常
- 与 Pinia store 状态同步

**执行模式**: 串行（依赖 T7，TDD：先写测试再实现）

---

### T10: 录制文件列表组件 (RecordingList.vue)

**描述**: 创建录制文件列表组件，展示所有录制文件。

**关联规格**: spec/spec.md - RecordingList.vue 设计

**执行内容**:
- 创建 `src/renderer/components/RecordingList.vue`
- 实现 el-table 表格展示
- 实现列定义（文件名、时长、大小、录制时间、状态、操作）
- 实现播放预览功能
- 实现重命名、删除、打开目录、导出操作
- 实现分页和排序

**验收标准**:
- 表格正确展示录制文件
- 操作按钮功能正常
- 分页和排序正确

**执行模式**: 串行（依赖 T7，TDD：先写测试再实现）

---

### T11: 高光时刻列表组件 (HighlightList.vue)

**描述**: 创建高光时刻列表组件，展示所有高光标记。

**关联规格**: spec/spec.md - HighlightList.vue 设计

**执行内容**:
- 创建 `src/renderer/components/HighlightList.vue`
- 实现 el-table 表格展示
- 实现列定义（时间戳、触发原因、描述、片段时长、操作）
- 实现播放预览功能
- 实现删除操作
- 实现原因标签样式（不同颜色区分）

**验收标准**:
- 表格正确展示高光列表
- 原因标签样式正确
- 操作按钮功能正常

**执行模式**: 串行（依赖 T7，TDD：先写测试再实现）

---

### T12: 录制管理页面 (Recording.vue)

**描述**: 创建录制管理页面，整合录制控制、高光列表和录制文件列表。

**关联规格**: spec/spec.md - Recording.vue 设计

**执行内容**:
- 创建 `src/renderer/views/Recording.vue`
- 实现顶部操作区（开始/暂停/停止录制按钮 + 配置入口）
- 实现标签页切换（高光列表 / 录制文件列表）
- 实现底部状态区（当前录制状态）
- 集成 RecordingList 和 HighlightList 组件
- 集成 recording store

**验收标准**:
- 页面布局正确
- 标签页切换正常
- 录制控制功能正常
- 列表展示正确

**执行模式**: 串行（依赖 T9-T11）

---

### T13: 设置页面集成录制配置

**描述**: 在设置页面中集成录制配置项。

**关联规格**: spec/spec.md - 页面与模块设计

**执行内容**:
- 修改 `src/renderer/views/Settings.vue` 添加「录制设置」tab
- 实现录制格式选择（MP4/FLV）
- 实现录制质量选择（原始/720p/1080p）
- 实现分段规则配置（按大小/按时长）
- 实现存储路径配置
- 实现最大存储空间配置
- 实现自动清理策略配置
- 实现高光功能开关和阈值配置
- 实现快捷键配置

**验收标准**:
- 设置页面显示录制配置项
- 配置项可正常修改和保存
- 配置持久化正确

**执行模式**: 串行（依赖 T12）

---

### T14: 路由配置与菜单入口

**描述**: 配置录制管理页面路由和侧边栏菜单入口。

**关联规格**: spec/spec.md - 页面与模块设计

**执行内容**:
- 修改 `src/renderer/router/index.ts` 添加 `/recording` 路由
- 修改 `src/renderer/App.vue` 添加侧边栏「录制管理」菜单项
- 实现录制状态指示图标

**验收标准**:
- 路由 `/recording` 可正常访问
- 侧边栏显示「录制管理」菜单项
- 菜单项图标正确

**执行模式**: 串行（依赖 T12）

---

### T15: 快捷键注册与全局监听

**描述**: 注册录制相关快捷键（F9 标记高光）。

**关联规格**: spec/spec.md - 快捷键配置

**执行内容**:
- 修改 `src/main/index.ts` 注册全局快捷键
- 实现 F9 快捷键触发高光标记
- 实现快捷键冲突处理
- 支持自定义快捷键配置

**验收标准**:
- F9 快捷键可触发高光标记
- 快捷键冲突时给出提示
- 自定义快捷键配置生效

**执行模式**: 串行（依赖 T6）

---

### T16: 集成录制模块到应用启动流程

**描述**: 在应用启动时初始化录制模块，监听直播间连接事件。

**关联规格**: spec/spec.md - 用户流程

**执行内容**:
- 修改 `src/main/index.ts` 初始化录制模块
- 实现直播间连接后自动获取直播流 URL
- 实现应用退出时优雅停止录制
- 实现录制状态栏在 App.vue 中的集成

**验收标准**:
- 应用启动时录制模块初始化正确
- 直播间连接后可获取直播流 URL
- 应用退出时录制优雅停止

**执行模式**: 串行（依赖 T6, T9）

---

### T17: 端到端测试与验证

**描述**: 执行端到端测试，验证所有验收标准。

**关联规格**: spec/spec.md - 验收标准

**执行内容**:
- 验证 AC-01 到 AC-27 所有验收条件
- 记录测试结果到 verification/verification.md

**验收标准**: 所有验收条件通过

**执行模式**: 串行（依赖 T1-T16）

---

## 依赖关系

```
T1 (数据库类型与表结构)
  ↓
T2 (FFmpeg 管理器) ← 依赖 T1
  ↓
T3 (录制引擎) ← 依赖 T2
  ↓
T4 (高光检测器) ← 依赖 T1
  ↓
T5 (文件管理器) ← 依赖 T1
  ↓
T6 (主进程模块入口与 IPC) ← 依赖 T2, T3, T4, T5
  ↓
T7 (录制状态管理) ← 依赖 T6
  ↓
T8 (录制服务) ← 依赖 T6
  ↓
T9 (录制状态栏) ← 依赖 T7
  ↓
T10 (录制文件列表) ← 依赖 T7
  ↓
T11 (高光列表) ← 依赖 T7
  ↓
T12 (录制管理页面) ← 依赖 T9, T10, T11
  ↓
T13 (设置页面集成) ← 依赖 T12
  ↓
T14 (路由配置) ← 依赖 T12
  ↓
T15 (快捷键注册) ← 依赖 T6
  ↓
T16 (启动流程集成) ← 依赖 T6, T9
  ↓
T17 (端到端验证) ← 依赖 T1-T16
```

## 并行执行建议

当前任务集规模较大（17 个任务），但任务间依赖关系紧密，不建议启用 workflow-style 并行执行。所有任务按串行顺序执行。

**推荐执行顺序**: T1 → T2 → T3 → T4 → T5 → T6 → T7 → T8 → T9 → T10 → T11 → T12 → T13 → T14 → T15 → T16 → T17

## 受影响文件或模块

| 文件/模块 | 变更类型 | 关联任务 |
|-----------|----------|----------|
| `src/renderer/modules/recording/types.ts` | 新增 | T1 |
| `src/main/database.ts` | 修改 | T1 |
| `src/main/modules/recording/ffmpeg-manager.ts` | 新增 | T2 |
| `src/main/modules/recording/recording-engine.ts` | 新增 | T3 |
| `src/main/modules/recording/highlight-detector.ts` | 新增 | T4 |
| `src/main/modules/recording/file-manager.ts` | 新增 | T5 |
| `src/main/modules/recording/index.ts` | 新增 | T6 |
| `src/main/ipc-handlers.ts` | 修改 | T6 |
| `src/renderer/stores/recording.ts` | 新增 | T7 |
| `src/renderer/services/recording-service.ts` | 新增 | T8 |
| `src/renderer/components/RecordingStatusBar.vue` | 新增 | T9 |
| `src/renderer/components/RecordingList.vue` | 新增 | T10 |
| `src/renderer/components/HighlightList.vue` | 新增 | T11 |
| `src/renderer/views/Recording.vue` | 新增 | T12 |
| `src/renderer/views/Settings.vue` | 修改 | T13 |
| `src/renderer/router/index.ts` | 修改 | T14 |
| `src/renderer/App.vue` | 修改 | T9, T14, T16 |
| `src/main/index.ts` | 修改 | T15, T16 |

## 测试策略

### 单元测试

| 测试目标 | 测试内容 | 测试框架 |
|----------|----------|----------|
| types.ts | 接口类型正确性 | TypeScript 类型检查 |
| ffmpeg-manager.ts | FFmpeg 检测、进程管理 | Vitest |
| recording-engine.ts | 录制启停、分段、格式转换 | Vitest |
| highlight-detector.ts | 弹幕频率统计、高光触发 | Vitest |
| file-manager.ts | 文件路径生成、存储检查、清理 | Vitest |
| stores/recording.ts | 状态管理、actions | Vitest |
| RecordingStatusBar.vue | 状态显示、时长格式化 | Vue Test Utils |
| RecordingList.vue | 列表渲染、操作按钮 | Vue Test Utils |
| HighlightList.vue | 列表渲染、原因标签 | Vue Test Utils |

### 集成测试

| 测试目标 | 测试内容 |
|----------|----------|
| 录制流程 | 开始 → 暂停 → 恢复 → 停止 → 文件生成 |
| 高光流程 | 弹幕激增 → 自动标记 → 片段提取 |
| IPC 通信 | 渲染进程 → 主进程 → 状态推送 |
| 配置流程 | 修改配置 → 持久化 → 生效 |

### 端到端测试

验证所有 AC-01 到 AC-27 验收条件。

## 回滚说明

如果实现过程中发现问题：

1. **T2-T5 失败**: 回退到 T1，检查类型定义和数据结构
2. **T6 失败**: 回退到 T2-T5，检查主进程模块实现
3. **T7-T8 失败**: 回退到 T6，检查 IPC 通道定义
4. **T9-T12 失败**: 回退到 T7，检查状态管理和服务实现
5. **T13-T16 失败**: 回退到 T12，检查页面组件实现
6. **T17 验证失败**: 回退到对应任务修复问题

每个任务完成后应提交代码，便于回滚。
