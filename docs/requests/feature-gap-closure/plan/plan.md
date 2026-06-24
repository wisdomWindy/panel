# 功能缺口收口 - 执行计划

## 任务拆解

### T1：补齐插件系统底层运行时与文件系统 IPC

**关联规格**：spec/spec.md - 插件系统 / API 与数据契约 / AC-01 ~ AC-04

**执行内容**：
- 为主进程新增 `fs:*` IPC 处理器
- 为渲染端新增插件管理器单例运行时入口
- 修正插件安装流程中“路径 / ID 语义混用”问题
- 保证插件管理器初始化、读取、安装、卸载可在当前 Electron 链路中调用

**测试策略**：
- 先补纯逻辑测试，覆盖插件安装路径语义与 manager 行为
- 再实现主进程/运行时接线

**执行模式**：串行（TDD 优先）

---

### T2：接通插件管理页面真实交互链路

**关联规格**：spec/spec.md - 插件系统页面职责 / AC-01 ~ AC-03

**执行内容**：
- 页面接入插件运行时单例
- 替换页面中的 `TODO` 假逻辑
- 让刷新、启用、禁用、卸载、配置查看走真实链路
- 保持页面仅负责交互编排与反馈

**测试策略**：
- 以运行时/manager 行为测试为主
- 页面层通过类型检查与构建验证保证链路完整

**执行模式**：串行（依赖 T1）

---

### T3：收口设置页配置导入导出

**关联规格**：spec/spec.md - 配置导入导出 / AC-05 ~ AC-06

**执行内容**：
- 设计统一导出对象结构
- 从 settings 各配置域聚合导出
- 实现导入解析与显式分发写回
- 同步必要的运行态 side effect（如最小发送间隔、语言配置）

**测试策略**：
- 先写配置聚合/映射函数测试
- 再接入设置页面

**执行模式**：串行（TDD 优先）

---

### T4：收口录制上下文与自动录制

**关联规格**：spec/spec.md - 录制闭环 / AC-07 ~ AC-08

**执行内容**：
- 抽出录制启动上下文构造逻辑
- 在录制页面使用当前房间上下文发起录制
- 在 `danmaku-service` 的自动录制入口接入真实录制启动逻辑
- 缺少房间上下文时给出明确错误

**测试策略**：
- 先为录制上下文构造与自动录制触发条件写测试
- 再补 service / page 接线

**执行模式**：串行（TDD 优先）

---

### T5：收口多语言与 Element Plus locale 同步

**关联规格**：spec/spec.md - 多语言闭环 / AC-09 ~ AC-10

**执行内容**：
- 建立响应式 locale 来源
- 让 Element Plus locale 随应用语言同步
- 改造 `App.vue`、`dashboard/index.vue`、`settings/index.vue`、`LanguageSettings.vue` 目标硬编码文案
- 补充目标 i18n key

**测试策略**：
- 纯逻辑部分使用 locale 适配函数测试
- UI 文案覆盖通过代码检查、type-check、build 与人工验证

**执行模式**：串行（依赖 T3 的语言配置写回）

---

### T6：统一验证与工件回填

**关联规格**：spec/spec.md - 全部 AC

**执行内容**：
- 运行新增/既有测试
- 运行 `type-check` 与构建验证
- 记录 verification / review 工件

**执行模式**：串行（依赖 T1-T5）

## 依赖关系

```text
T1 插件底层运行时
  ↓
T2 插件管理页面真实接线
  ↓
T3 配置导入导出
  ↓
T4 录制闭环
  ↓
T5 多语言闭环
  ↓
T6 验证与工件回填
```

说明：
- 用户要求“按顺序完成”，因此即便某些任务局部可拆，也按串行推进
- T5 依赖 T3，是因为语言配置写回与当前配置收口共用设置页导入导出上下文

## 整洁性与复杂度控制

1. 为插件系统新增轻量单例运行时入口，不在多个页面重复构造 manager
2. 为设置页导入导出抽离显式聚合/映射函数，避免在页面方法中堆叠大量字段分支
3. 为录制上下文抽离小型 helper，避免页面和 service 各自拼接 `roomId`
4. 多语言改造只覆盖当前目标区域，避免一次性改动全项目导致 review 面失控

## 模式决策与替代方案

- **采用**：单例运行时入口、显式 adapter / mapper
- **解决问题**：
  - 单例入口解决插件管理器创建分散、依赖注入不一致
  - adapter / mapper 解决配置导入导出与录制上下文构造的规则归并
- **拒绝方案**：
  - 不引入新的全局事件总线
  - 不引入复杂 schema 驱动表单系统
  - 不引入额外多语言状态管理库

## 并行执行建议

不启用 workflow-style 并行。

原因：
1. 用户明确要求按顺序完成
2. 四类缺口在设置、房间上下文和应用根部依赖上存在耦合
3. 当前请求规模适中，串行更利于控制 artifact 与验证证据

## 触发与上下文准备

- 启动前读取：
  - 当前请求工件
  - 历史插件系统 / 录制 / 多语言请求工件
  - 目标代码文件
- 触发顺序：
  - 先补运行时缺口，再补页面交互，再补配置与录制，再补多语言

## 受影响文件或模块

| 文件/模块 | 变更类型 | 关联任务 |
|-----------|----------|----------|
| `src/main/index.ts` | 修改 | T1 |
| `src/main/ipc-handlers.ts` 或新增主进程 fs 模块 | 修改 | T1 |
| `src/preload/index.ts` | 可能补类型或接口一致性 | T1 |
| `src/renderer/plugins/plugin-manager.ts` | 修改 | T1 |
| `src/renderer/plugins/plugin-loader.ts` | 可能调整加载入口 | T1 |
| `src/renderer/plugins/index.ts` / 新增 runtime 文件 | 修改/新增 | T1 |
| `src/renderer/views/plugin-manager/index.vue` | 修改 | T2 |
| `src/renderer/views/settings/index.vue` | 修改 | T3 / T5 |
| `src/renderer/stores/config.ts` | 修改 | T3 |
| `src/renderer/services/danmaku-service.ts` | 修改 | T4 |
| `src/renderer/views/recording/index.vue` | 修改 | T4 |
| `src/renderer/main.ts` | 修改 | T5 |
| `src/renderer/App.vue` | 修改 | T5 |
| `src/renderer/views/dashboard/index.vue` | 修改 | T5 |
| `src/renderer/views/settings/components/LanguageSettings.vue` | 修改 | T5 |
| `src/renderer/locales/zh-CN.json` | 修改 | T5 |
| `src/renderer/locales/en-US.json` | 修改 | T5 |
| `tests/*` | 新增/修改 | T1 / T3 / T4 / T5 |

## 测试策略

1. 插件系统
   - 新增针对插件安装路径语义、manager 行为的测试
   - 验证 `fs:*` IPC 注册存在
2. 配置导入导出
   - 新增聚合与映射函数测试
3. 录制闭环
   - 新增录制上下文构造与自动录制触发条件测试
4. 多语言
   - 对可抽离 locale 适配逻辑做测试
   - UI 文案与 Element Plus locale 以 type-check / build / 人工验证补证
5. 全量验证
   - 运行相关测试文件
   - 运行 `rtk npm run type-check`
   - 运行 `rtk npm run build:vite`

## 观察与人工介入点

1. `spec` 审批：确认本次不扩展到插件配置编辑器和全项目国际化
2. `plan` 审批：确认四段顺序与测试策略
3. 若实现中发现插件系统需要额外主进程目录策略或安全边界，回写 execution/changelog 并继续按已批准范围处理

## 回滚说明

1. 若插件系统真实接线导致构建或 preload 契约异常，优先回滚到单任务前的局部文件修改，不触碰无关模块
2. 若多语言响应式注入影响 Element Plus 现有行为，保留 i18n key 改造并局部回退注入方式
3. 若录制自动触发引起运行态副作用异常，可先关闭自动录制入口改动，再保留手动录制上下文修复
