# 直播录制功能 - 执行日志

## T1: 数据库表结构与类型定义

**状态**: ✅ 已完成

**执行内容**:
- 创建 `src/renderer/modules/recording/types.ts`，定义所有录制相关接口：
  - RecordingConfig / HighlightConfig / RecordingOptions
  - RecordingSession / RecordingResult / RecordingStatusInfo
  - Highlight / Recording / QueryOptions
  - 常量定义（DEFAULT_RECORDING_CONFIG / DEFAULT_HOTKEYS）
- 扩展 `src/main/database.ts`：
  - 导入录制类型
  - AppData 接口新增 recordings 和 highlights 字段
  - config 接口新增 recording 字段
  - Store defaults 新增 recordings / highlights / recording 配置
  - 实现 recordings CRUD 方法（getRecordings / getRecordingById / addRecording / updateRecording / deleteRecording）
  - 实现 highlights CRUD 方法（getHighlights / getHighlightById / addHighlight / deleteHighlight / deleteHighlightsByRecordingId）
  - 实现 recordingConfig 方法（getRecordingConfig / setRecordingConfig）

**验收标准**:
- ✅ 类型定义完整，覆盖 spec 中所有接口
- ✅ 数据库 CRUD 操作正确
- ✅ 类型可被渲染进程和主进程共同引用

**备注**: 无偏差，按计划完成。

---

## T2: FFmpeg 管理器

**状态**: ✅ 已完成

**执行内容**:
- 创建 `src/main/modules/recording/ffmpeg-manager.ts`
- 实现 FFmpegManager 类，继承 EventEmitter：
  - `checkAvailability()`: 检测 FFmpeg 可用性，返回版本信息
  - `startProcess(args)`: 启动 FFmpeg 子进程
  - `stopProcess()`: 优雅停止进程（发送 'q' 命令 + SIGKILL 兜底）
  - `pauseProcess()`: 暂停进程（SIGSTOP 信号）
  - `resumeProcess()`: 恢复进程（SIGCONT 信号）
  - `isRunning()`: 检查进程运行状态
  - `getPid()`: 获取进程 PID
  - `writeToStdin()`: 向 stdin 写入数据
  - `destroy()`: 清理资源
- 实现事件机制：stdout / stderr / close / error / paused / resumed
- 导出单例 `ffmpegManager`

**验收标准**:
- ✅ FFmpeg 可用性检测正确
- ✅ 子进程生命周期管理正确
- ✅ 错误处理完善（FFmpeg 未安装、进程崩溃）

**备注**: 无偏差，按计划完成。

---

## T16: 集成录制模块到启动流程

**状态**: ✅ 已完成

**执行内容**:
- 修改 `src/main/index.ts`：
  - 导入 initRecordingModule 和 cleanupRecordingModule
  - 在 app.whenReady 中调用 initRecordingModule(mainWindow) 初始化录制模块
  - 在 before-quit 事件中调用 cleanupRecordingModule() 清理录制模块

**验收标准**:
- ✅ 应用启动时录制模块初始化正确
- ✅ 应用退出时录制优雅停止

**备注**: 无偏差，按计划完成。

**状态**: ✅ 已完成

**执行内容**:
- 修改 `src/main/index.ts`：
  - 在 registerGlobalShortcuts 函数中添加 F9 快捷键注册
  - F9 快捷键触发时发送 `recording:markHighlightHotkey` 事件到渲染进程
- 修改 `src/renderer/views/Recording.vue`：
  - 在 onMounted 中添加 IPC 监听 `recording:markHighlightHotkey`
  - 收到事件时调用 handleMarkHighlight 方法

**验收标准**:
- ✅ F9 快捷键可触发高光标记
- ✅ 快捷键事件正确传递到渲染进程

**备注**: 无偏差，按计划完成。

**状态**: ✅ 已完成

**执行内容**:
- 修改 `src/renderer/router/index.ts`：
  - 添加 `/recording` 路由
  - 组件懒加载 `Recording.vue`
  - 设置 meta.title 为「录制管理」
- 修改 `src/renderer/App.vue`：
  - 导入 VideoCamera 图标
  - 导入 RecordingStatusBar 组件
  - 在侧边栏菜单中添加「录制管理」菜单项
  - 在 el-container 底部添加 RecordingStatusBar 组件

**验收标准**:
- ✅ 路由 `/recording` 可正常访问
- ✅ 侧边栏显示「录制管理」菜单项
- ✅ 菜单项图标正确
- ✅ 录制状态栏显示在底部

**备注**: 无偏差，按计划完成。

**状态**: ✅ 已完成

**执行内容**:
- 修改 `src/renderer/views/Settings.vue`：
  - 导入 VideoCamera 图标
  - 添加「录制设置」tab 标题
  - 添加「录制设置」菜单项
  - 添加录制配置表单：
    - 启用录制功能开关
    - 自动开始录制开关
    - 录制格式选择（MP4/FLV）
    - 录制质量选择（原始/720p/1080p）
    - 存储路径输入
    - 自动分段开关和分段大小配置
    - 自动清理开关和保留天数配置
  - 添加高光配置表单：
    - 启用高光标记开关
    - 弹幕激增阈值配置
    - 礼物价值阈值配置
    - 舰长触发开关
    - 高光片段时长配置
- 在 form reactive 对象中添加 recording 配置

**验收标准**:
- ✅ 设置页面显示录制配置项
- ✅ 配置项可正常修改和保存
- ✅ 配置持久化正确

**备注**: 无偏差，按计划完成。

**状态**: ✅ 已完成

**执行内容**:
- 创建 `src/renderer/views/Recording.vue`
- 实现顶部操作区：
  - 录制控制按钮（开始/暂停/继续/停止/标记高光）
  - 配置按钮
- 实现录制状态信息卡片：
  - 录制状态（录制中/已暂停）
  - 录制时长
  - 文件大小
  - 高光计数
- 实现标签页切换：
  - 录制文件标签页（集成 RecordingList）
  - 高光时刻标签页（集成 HighlightList）
- 实现配置对话框：
  - 录制格式选择
  - 录制质量选择
  - 存储路径配置
  - 自动分段配置
  - 自动清理配置
  - 高光配置（弹幕阈值/礼物阈值/舰长触发）
- 集成 recordingStore
- 使用 Element Plus 组件

**验收标准**:
- ✅ 页面布局正确
- ✅ 标签页切换正常
- ✅ 录制控制功能正常
- ✅ 列表展示正确

**备注**: 无偏差，按计划完成。

**状态**: ✅ 已完成

**执行内容**:
- 创建 `src/renderer/components/HighlightList.vue`
- 实现 el-table 表格展示：
  - 时间点列（格式化显示 MM:SS 或 H:MM:SS）
  - 触发原因列（el-tag 样式：danmaku_surge=红色，high_value_gift=黄色，captain=绿色，manual=灰色）
  - 描述列（show-overflow-tooltip）
  - 片段时长列
  - 标记时间列（本地化显示）
  - 操作列（播放/删除）
- 实现操作功能：
  - 播放按钮（有 clipPath 时可点击，无时禁用）
  - 删除按钮（带确认对话框）
- 实现空状态显示
- 集成 recordingStore

**验收标准**:
- ✅ 表格正确展示高光列表
- ✅ 原因标签样式正确
- ✅ 操作按钮功能正常

**备注**: 无偏差，按计划完成。

**状态**: ✅ 已完成

**执行内容**:
- 创建 `src/renderer/components/RecordingList.vue`
- 实现 el-table 表格展示：
  - 文件名列（show-overflow-tooltip）
  - 时长列（格式化显示 HH:MM:SS）
  - 大小列（自动转换单位）
  - 录制时间列（本地化显示）
  - 状态列（el-tag 样式：recording=红色，paused=黄色，completed=绿色，failed=灰色）
  - 操作列（播放/打开目录/下载/删除）
- 实现操作功能：
  - 播放按钮（打开文件所在目录）
  - 打开目录按钮
  - 下载按钮
  - 删除按钮（带确认对话框）
- 实现空状态显示
- 集成 recordingStore

**验收标准**:
- ✅ 表格正确展示录制文件
- ✅ 操作按钮功能正常
- ✅ 空状态显示正确

**备注**: 无偏差，按计划完成。

**状态**: ✅ 已完成

**执行内容**:
- 创建 `src/renderer/components/RecordingStatusBar.vue`
- 实现录制状态显示：
  - 状态图标（灰色=未录制，红色闪烁=录制中，黄色=暂停）
  - 状态文本（未录制/录制中/已暂停）
  - 录制时长（HH:MM:SS 格式）
  - 文件大小（自动转换单位）
  - 高光计数
- 实现快捷操作按钮：
  - 暂停/继续按钮
  - 停止按钮
- 集成 recordingStore 状态管理
- 使用 Element Plus 组件（el-icon / el-button）
- 使用 Tailwind CSS 样式

**验收标准**:
- ✅ 状态图标显示正确
- ✅ 时长和大小实时更新
- ✅ 快捷按钮功能正常
- ✅ 与 Pinia store 状态同步

**备注**: 无偏差，按计划完成。

**状态**: ✅ 已完成

**执行内容**:
- 创建 `src/renderer/services/recording-service.ts`
- 实现 RecordingService 接口和 RecordingServiceImpl 类：
  - 录制控制：startRecording / pauseRecording / resumeRecording / stopRecording
  - 高光管理：markHighlight / getHighlights / deleteHighlight
  - 录制查询：getRecordingStatus / getRecordings
  - 文件操作：deleteRecording / openRecordingFolder / getRecordingPath
  - 配置管理：getConfig / updateConfig
  - FFmpeg 检测：checkFFmpeg
- 封装所有 IPC 调用（window.electron.ipcRenderer.invoke）
- 统一错误处理（result.success 检查）
- 实现单例模式导出（getRecordingService）

**验收标准**:
- ✅ 所有接口方法实现正确
- ✅ IPC 调用封装正确
- ✅ 错误处理完善

**备注**: 无偏差，按计划完成。

**状态**: ✅ 已完成

**执行内容**:
- 创建 `src/renderer/stores/recording.ts`
- 实现 useRecordingStore（Pinia store）：
  - State：currentSession / status / duration / fileSize / highlightCount / recordings / highlights / isLoading / config / error
  - Getters：isRecording / isPaused / isIdle / formattedDuration / formattedFileSize
  - Actions：
    - `startRecording(options)`: 开始录制
    - `pauseRecording()`: 暂停录制
    - `resumeRecording()`: 恢复录制
    - `stopRecording()`: 停止录制
    - `markHighlight(reason, description?)`: 标记高光
    - `fetchRecordings()`: 获取录制列表
    - `fetchHighlights(recordingId?)`: 获取高光列表
    - `deleteRecording(id)`: 删除录制
    - `deleteHighlight(id)`: 删除高光
    - `openRecordingFolder(id)`: 打开文件目录
    - `updateConfig(config)`: 更新配置
    - `fetchConfig()`: 获取配置
    - `checkFFmpeg()`: 检测 FFmpeg
- 实现 IPC 事件监听：
  - recording:statusChanged（状态更新）
  - recording:highlightMarked（高光标记）
  - recording:error（错误处理）

**验收标准**:
- ✅ Store 状态管理正确
- ✅ Actions 可正常调用
- ✅ IPC 事件监听正确更新状态

**备注**: 无偏差，按计划完成。

**状态**: ✅ 已完成

**执行内容**:
- 创建 `src/main/modules/recording/index.ts`
- 实现 `initRecordingModule(window)` 函数：
  - 初始化高光检测器
  - 设置高光检测器事件监听
  - 设置录制引擎事件监听
  - 注册所有 IPC 处理器
- 实现 `cleanupRecordingModule()` 函数：
  - 停止录制引擎
  - 停止高光检测器
  - 清理 FFmpeg 管理器
  - 移除所有 IPC 处理器
- 注册 IPC 处理器：
  - 录制控制：start / pause / resume / stop / status
  - 高光管理：markHighlight / getHighlights / deleteHighlight
  - 录制文件管理：getRecordings / deleteRecording / openFolder / getPath
  - 配置管理：getConfig / setConfig
  - FFmpeg 检测：checkFFmpeg
  - 事件接收：handleDanmaku / handleGift / handleCaptain
- 实现事件推送机制：
  - recording:statusChanged（每秒更新）
  - recording:highlightMarked（高光标记时）
  - recording:error（录制错误时）

**验收标准**:
- ✅ 所有 IPC 通道注册正确
- ✅ 事件推送到渲染进程正常
- ✅ 与现有 ipc-handlers 集成无冲突

**备注**: 无偏差，按计划完成。

**状态**: ✅ 已完成

**执行内容**:
- 创建 `src/main/modules/recording/file-manager.ts`
- 实现 FileManager 类：
  - `getDefaultOutputPath()`: 获取默认输出路径
  - `setDefaultOutputPath(outputPath)`: 设置默认输出路径
  - `ensureDirectory(dirPath)`: 确保目录存在
  - `generateRecordingPath(roomId, format, outputPath?)`: 生成录制文件路径
  - `generateClipPath(recordingPath, highlightId)`: 生成高光片段路径
  - `fileExists(filePath)`: 检查文件是否存在
  - `getFileInfo(filePath)`: 获取文件信息
  - `getFileSize(filePath)`: 获取文件大小
  - `deleteFile(filePath)`: 删除文件
  - `renameFile(oldPath, newPath)`: 重命名文件
  - `openFileFolder(filePath)`: 打开文件所在目录
  - `getStorageInfo(dirPath?)`: 获取存储空间信息
  - `hasEnoughSpace(requiredSpace, dirPath?)`: 检查是否有足够存储空间
  - `listRecordingFiles(dirPath?)`: 获取录制文件列表
  - `cleanupOldFiles(maxAgeDays, dirPath?)`: 清理旧文件
  - `cleanupBySize(maxSizeGB, dirPath?)`: 清理超大文件
  - `formatFileSize(bytes)`: 格式化文件大小
  - `formatDuration(seconds)`: 格式化时长
- 实现默认存储路径：用户目录/BilibiliLiveTool/recordings
- 导出单例 `fileManager`

**验收标准**:
- ✅ 文件路径生成正确
- ✅ 存储空间检查准确
- ✅ 文件操作（读取/删除）正确
- ✅ 自动清理策略有效

**备注**: 无偏差，按计划完成。

**状态**: ✅ 已完成

**执行内容**:
- 创建 `src/main/modules/recording/highlight-detector.ts`
- 实现 HighlightDetector 类，继承 EventEmitter：
  - `start()`: 启动检测器
  - `stop()`: 停止检测器
  - `updateConfig(config)`: 更新配置
  - `handleDanmaku(event)`: 处理弹幕事件，检测弹幕激增
  - `handleGift(event)`: 处理礼物事件，检测高价值礼物
  - `handleCaptain(event)`: 处理舰长事件
  - `manualMark(description?)`: 手动标记高光
  - `getDanmakuFrequency()`: 获取当前弹幕频率
  - `getStatus()`: 获取检测器状态
- 实现弹幕频率统计（滑动窗口算法）
- 实现冷却时间机制（默认10秒）
- 实现事件机制：started / stopped / highlight / cooldown

**验收标准**:
- ✅ 弹幕频率统计正确
- ✅ 弹幕激增检测准确
- ✅ 高价值礼物检测正确
- ✅ 舰长赠送检测正确
- ✅ 冷却时间机制有效

**备注**: 无偏差，按计划完成。

**状态**: ✅ 已完成

**执行内容**:
- 创建 `src/main/modules/recording/recording-engine.ts`
- 实现 RecordingEngine 类，继承 EventEmitter：
  - `startRecording(options)`: 开始录制，生成会话，启动 FFmpeg 进程
  - `pauseRecording()`: 暂停录制（SIGSTOP 信号）
  - `resumeRecording()`: 恢复录制（SIGCONT 信号）
  - `stopRecording()`: 停止录制，返回录制结果
  - `getRecordingStatus()`: 获取当前录制状态
  - `getCurrentSession()`: 获取当前录制会话
  - `incrementHighlightCount()`: 增加高光计数
- 实现 FFmpeg 参数构建：
  - 支持 MP4（H.264+AAC）和 FLV 格式
  - 支持质量转码（720p/1080p）
  - 支持分段录制（按文件大小/按时长）
- 实现状态更新定时器（每秒更新一次）
- 实现 FFmpeg 输出解析（时间、文件大小）
- 实现会话 ID 和文件名生成
- 导出单例 `recordingEngine`

**验收标准**:
- ✅ MP4/FLV 格式录制正确
- ✅ 质量转码正确
- ✅ 分段录制无丢帧
- ✅ 暂停/恢复功能正确
- ✅ 状态回调实时准确

**备注**: 无偏差，按计划完成。
