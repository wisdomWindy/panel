# 功能缺口收口 - 功能规格

## 背景与目标

当前项目的历史请求与产品 PRD 将插件系统、配置导入导出、录制能力和多语言能力标记为已完成，但实际代码仍存在未接线、伪成功、局部覆盖的缺口。本请求目标不是扩展新需求，而是将这些已承诺能力收口到真实可用状态，并形成可验证证据。

目标按顺序分为四段：

1. 插件系统运行链路收口
2. 配置导入导出收口
3. 录制上下文与自动录制收口
4. 多语言与组件库语言同步收口

## 范围内

1. 为插件系统补齐主进程 `fs:*` IPC 处理器与渲染端运行时接线
2. 为插件管理页面接入真实插件管理器单例，支持加载、安装、启用、禁用、卸载、查看配置
3. 修正插件安装路径语义，避免把外部插件路径误当成已安装插件 ID
4. 调整设置页导出范围，覆盖当前支持的核心配置域
5. 实现设置页导入配置真实写回，并同步刷新相关运行状态
6. 实现自动录制触发逻辑，使用录制 store / service 和当前房间上下文启动录制
7. 修正录制页面手动启动时的房间上下文来源
8. 为目标页面与导航移除硬编码中文，改用 i18n key
9. 让 Element Plus locale 随运行时语言切换同步更新
10. 为新增或修正的可测试行为补充自动化测试

## 范围外

1. 插件市场、插件自动更新、插件签名验证、多进程隔离
2. 设置页的全量重构或重新设计
3. 录制模块的性能优化、FFmpeg 能力扩展或高光算法调整
4. 全项目所有页面的国际化彻底补完
5. 新增第三种及以上语言
6. 与本次缺口无关的 UI 样式调整

## 触发与启动条件

- 用户要求使用 `frontend-agent-framework` 按顺序补齐当前审查出的功能缺口
- 项目当前代码可读，历史请求文档可作为约束参考
- 无需外部 PRD 变更，直接以本地 PRD 与当前代码审查结论作为输入

## 用户流程

### 流程 1：插件管理真实可用

```
用户进入 /plugins
  → 页面初始化插件管理器单例
  → 从插件目录加载已安装插件
  → 列表展示真实插件状态
  → 用户执行启用 / 禁用 / 卸载 / 刷新
  → 页面调用真实插件管理器并回写 store
  → 用户查看插件配置时看到真实配置快照
```

### 流程 2：导出并导入配置

```
用户打开设置页
  → 点击导出配置
  → 系统导出当前支持的核心配置域
  → 用户粘贴或导入配置 JSON
  → 系统解析并归一配置
  → 写回 config / AI / ASR / TTS / moderation / recording 等状态
  → 页面展示导入成功且对应状态真实生效
```

### 流程 3：自动录制与手动录制使用真实房间上下文

```
用户连接直播间且启用自动录制
  → danmaku service 收到 autoRecordStart
  → 读取 roomStore 当前房间信息与 recording 配置
  → 调用 recording store / service 开始录制

用户进入录制页并点击开始录制
  → 页面读取当前 roomStore 房间上下文
  → 使用真实 roomId / roomName 发起录制
```

### 流程 4：运行时切换语言

```
用户进入语言设置
  → 选择 zh-CN / en-US
  → 应用 i18n locale 立即更新
  → Element Plus locale 同步更新
  → 导航、Dashboard、Settings 目标区域使用对应语言重渲染
  → 语言配置持久化
```

## 页面与模块设计

### 1. 插件系统

#### 页面职责

- `src/renderer/views/plugin-manager/index.vue`
  - 只负责页面交互、状态展示和错误反馈
  - 不自行持有插件生命周期逻辑

#### 运行时职责

- `src/renderer/plugins/plugin-runtime.ts` 或同级单例模块
  - 提供 `getPluginManager()` 单例入口
  - 负责插件目录路径、应用版本和依赖注入
  - 对页面隐藏初始化细节

- `src/renderer/plugins/plugin-manager.ts`
  - 继续负责安装、启用、禁用、卸载、状态读取
  - 修正“外部路径安装”与“已安装目录读取”语义混用问题

- `src/main/*`
  - 提供 `fs:checkDirectoryExists`、`fs:checkFileExists`、`fs:readFile`、`fs:getDirectories`、`fs:copyDirectory`、`fs:deleteDirectory`
  - 保证 preload 暴露接口有真实落点

### 2. 配置导入导出

- `src/renderer/views/settings/index.vue`
  - 导出：从各子配置域收集当前值，形成一个统一配置对象
  - 导入：解析统一配置对象，分别写回 `configStore`、`recordingStore`、`ttsStore`、`asrStore`、`aiStore`、`moderationStore`
  - 仅负责编排，不在模板层做兜底修正

- `src/renderer/stores/config.ts`
  - 保持扁平核心配置真值来源
  - 与其他专用 store 通过显式导入映射同步

### 3. 录制闭环

- `src/renderer/services/danmaku-service.ts`
  - `handleAutoRecordStart()` 负责触发自动录制
  - 只做条件判断与调用编排，不承载录制业务细节

- `src/renderer/views/recording/index.vue`
  - 手动开始录制时从 `roomStore` 读取真实房间上下文
  - 未连接或无房间信息时给出清晰提示，不允许继续使用写死值

### 4. 多语言闭环

- `src/renderer/main.ts`
  - 应将当前 locale 作为响应式来源，为 Element Plus 提供随语言变更同步的 locale

- `src/renderer/App.vue`
  - 导航中的目标硬编码文案改为 i18n key

- `src/renderer/views/dashboard/index.vue`
  - 统计标题、图表标题、状态文案、空态文案改为 i18n key

- `src/renderer/views/settings/index.vue`
  - 菜单标题、tab 标题、导入导出相关文案改为 i18n key

- `src/renderer/views/settings/components/LanguageSettings.vue`
  - 维持语言切换入口，但不再只更新 `vue-i18n`，需要触发组件库语言同步

## 设计约束

### 责任与边界约束

1. 页面组件只负责交互编排和展示，不直接持有底层文件系统或插件生命周期细节
2. 插件管理器的单例构建与依赖注入必须集中在一个运行时入口，不允许在多个页面或组件里散落创建逻辑
3. 设置导入导出中的“配置归一与写回”必须集中在导入编排层，不允许在子组件或模板中做导入后兜底修正
4. 自动录制触发属于 service 层编排责任，不放入页面层

### 命名与领域约束

1. “pluginPath” 表示外部待安装目录路径
2. “pluginId” 只表示插件安装后的目录 ID / manifest id
3. 录制上下文统一使用 `roomId`、`roomName`，不得继续使用 `default` 这类伪值

### 重复规则约束

1. 插件管理页面的启停、安装、卸载状态更新只保留一套真实调用链
2. 配置导入导出结构只保留一套统一聚合格式，避免页面导出与 store 导入各自维护不同字段集合

### 副作用与依赖约束

1. 文件系统副作用只能通过主进程 IPC 暴露
2. 插件目录读取、复制、删除必须经过 preload 暴露 API
3. 语言切换时的 Element Plus locale 更新必须通过单一响应式来源驱动

### 复杂度与可读性约束

1. 不引入新的重型模式层；本次只允许增加轻量单例运行时封装或适配函数
2. 若页面需要多个 store 协同更新，优先使用显式映射函数，避免在一个大函数中散落条件分支

## 变化轴与模式决策

### 变化轴

1. 插件系统有“页面交互”和“底层运行时”两个变化面
2. 配置导入导出有“聚合结构”与“分发写回”两个变化面
3. 多语言有“应用 locale”与“组件库 locale”同步变化面

### 模式决策

- **本次采用的最轻结构**：单例运行时入口 + 显式适配/映射函数
- **原因**：
  - 插件管理器存在明确的单实例与依赖注入问题，适合使用轻量单例入口
  - 配置导入导出存在字段归并与写回映射问题，适合用显式 adapter / mapper，而不是把逻辑散到组件里
- **拒绝的更重方案**：
  - 不引入新的 Manager-of-Manager、事件总线或复杂工厂层
  - 不为本次语言切换再新增全局状态框架，仅在现有 i18n / App 根部位补响应式同步

## API 与数据契约

### 插件系统

#### 主进程 IPC 契约

```typescript
ipcMain.handle('fs:checkDirectoryExists', (_event, path: string) => Promise<{ success: boolean; exists: boolean }>)
ipcMain.handle('fs:checkFileExists', (_event, path: string) => Promise<{ success: boolean; exists: boolean }>)
ipcMain.handle('fs:readFile', (_event, path: string) => Promise<{ success: boolean; content: string }>)
ipcMain.handle('fs:getDirectories', (_event, path: string) => Promise<{ success: boolean; directories: string[] }>)
ipcMain.handle('fs:copyDirectory', (_event, source: string, target: string) => Promise<{ success: boolean }>)
ipcMain.handle('fs:deleteDirectory', (_event, path: string) => Promise<{ success: boolean }>)
```

预加载与渲染端可以继续使用现有简化签名，但底层必须有真实响应。

#### 插件安装契约

```typescript
installPlugin(pluginPath: string): Promise<void>
```

- `pluginPath` 为外部待安装插件目录路径
- 插件安装时先读取该路径下 `manifest.json`
- 校验通过后复制到 `pluginDir/<manifest.id>`
- 再按 `manifest.id` 注册插件

### 配置导入导出

#### 导出对象结构

```typescript
interface AppExportConfig {
  core: Partial<AppConfig>;
  recording: Partial<RecordingConfig>;
  tts: Record<string, unknown>;
  asr: Record<string, unknown>;
  ai: Record<string, unknown>;
  moderation: Record<string, unknown>;
}
```

- 可在实现时精简字段，但必须覆盖当前页面支持编辑的核心配置域
- 导入时缺失字段按“局部更新”处理，不覆盖其他未提供字段

### 录制上下文

```typescript
interface RecordingStartContext {
  roomId: string;
  roomName?: string;
}
```

- 手动与自动录制都必须从当前房间上下文生成该结构
- 若当前未连接或无房间信息，则拒绝启动并给出错误提示

### 多语言

- `locale` 继续使用 `'zh-CN' | 'en-US'`
- 目标页面的现有硬编码文案必须有对应 i18n key
- Element Plus locale 的选择必须由当前应用 locale 派生

## 上下文与依赖来源

- 本地 PRD 与历史请求工件
- 现有 Pinia stores
- 现有 `window.electronAPI`
- Element Plus locale 包
- 现有轻量脚本式测试文件

## 边界情况

1. 插件目录不存在：插件页面展示空列表，不抛出未捕获异常
2. 插件 manifest 缺失或入口文件缺失：安装失败并提示明确原因
3. 导入 JSON 缺少部分配置域：只更新已有字段，不重置其他配置
4. 导入 JSON 结构非法：阻止导入并提示格式错误
5. 未连接直播间时点击开始录制：拒绝录制并提示先连接直播间
6. 自动录制触发时没有有效房间信息：跳过自动录制并记录错误
7. 某些页面仍没有对应翻译 key：本次范围内目标页面不得保留硬编码中文
8. 运行时切换语言后，若某组件依赖启动期静态 locale：必须改为响应式同步方案

## 验收标准

| 编号 | 验收条件 | 可测试性 |
|------|----------|----------|
| AC-01 | 插件页面初次进入时能加载真实插件列表，不再依赖 `TODO` 假逻辑 | 手动验证 / 代码检查 |
| AC-02 | 插件启用、禁用、卸载、刷新调用真实插件管理器，并更新页面状态 | 手动验证 / 测试 |
| AC-03 | 插件安装按“外部路径 -> 复制到插件目录 -> 注册”流程执行 | 测试 / 代码检查 |
| AC-04 | 预加载暴露的 `fs:*` 能在主进程找到真实 IPC 处理器 | 代码检查 / 测试 |
| AC-05 | 设置页导出覆盖当前核心配置域，而不是仅旧五项配置 | 手动验证 / 测试 |
| AC-06 | 设置页导入会真实写回配置状态，不再仅 `JSON.parse` 后提示成功 | 手动验证 / 测试 |
| AC-07 | 自动录制触发时会真正调用录制启动逻辑 | 测试 / 手动验证 |
| AC-08 | 手动录制使用当前直播间 `roomId`，不存在写死 `default` | 代码检查 / 测试 |
| AC-09 | 语言切换后，App 导航、Dashboard、Settings 目标区域切换为对应语言 | 手动验证 / 代码检查 |
| AC-10 | Element Plus locale 在运行时语言切换后同步更新 | 手动验证 |
| AC-11 | 本次新增或修复的可测试行为有自动化测试覆盖，且先经历 fail 再 pass | 测试记录 |

## 人工复核与交接

- 人工需要重点复核：
  1. 插件系统是否达到“真实可用”而非“页面可点”
  2. 导入导出结构是否与当前设置页支持范围一致
  3. 多语言闭环是否覆盖本次约定的目标区域

- 交接产物：
  - `verification/verification.md`
  - `review/review.md`
  - 必要的测试命令与结果摘要

## 风险

1. 插件系统底层文件能力缺失会扩大第一阶段范围，需要补主进程 IPC
2. 设置页存在多个独立 store，导入映射若处理不全会造成“部分成功”
3. 多语言切换涉及根部依赖注入，若处理不当可能影响现有 Element Plus 使用方式
4. 录制能力依赖 Electron / FFmpeg 运行环境，自动化验证可能需要用 mock 或静态契约替代真实录制
