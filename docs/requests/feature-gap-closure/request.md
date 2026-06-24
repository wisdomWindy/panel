# 功能缺口收口

## 请求标识

- **request-id**: feature-gap-closure
- **创建时间**: 2026-06-24

## 来源

- `docs/requirements/Bilibili_Live_Tool_PRD.md`
- 用户指令：使用 `frontend-agent-framework` 审查后，将当前项目中已宣称完成但仍未闭环的功能按顺序补齐

## 业务摘要

当前项目的 PRD 与历史请求文档将插件系统、配置导入导出、录制能力和多语言支持标记为已完成，但静态审查发现这些能力仍存在实际缺口：插件管理页面未接入真实运行链路，设置页导入导出能力不完整且导入为伪成功，自动录制与录制上下文未闭环，多语言切换仅覆盖局部区域且 Element Plus 语言不会在运行时同步切换。本请求用于按顺序收口这些功能缺口，使“已完成”声明与实际可用状态对齐。

## 目标声明

按以下顺序补齐当前项目中仍未闭环的功能缺口，并形成可验证的交付结果：

1. 补齐插件系统的运行链路，使插件管理页的加载、安装、启用、禁用、卸载、配置展示可真实工作
2. 修复设置页配置导入导出，使导出覆盖当前受支持配置，导入真实写回各相关 store / 配置层
3. 补齐录制功能闭环，使自动录制开关与手动录制都使用真实直播间上下文
4. 补齐多语言闭环，使目标页面的界面文本与 Element Plus 语言可在运行时正确切换

## 初始完成信号

- [ ] 插件管理页面不再依赖 `TODO` 假实现，用户可完成列表加载、安装、启停、卸载与配置查看
- [ ] 主进程提供插件系统所需的文件系统 IPC 能力，预加载暴露的 `fs:*` 接口均有真实处理器
- [ ] 设置页导出配置覆盖当前支持的核心配置域，导入配置会真实更新状态而非仅校验 JSON
- [ ] 自动录制开关触发时可真正启动录制，手动录制使用当前房间上下文而不是写死值
- [ ] 语言切换后，目标页面与导航中的既有硬编码文案切换为对应语言
- [ ] Element Plus 组件语言与应用语言保持同步，不要求重启应用
- [ ] 针对可测试行为补充或更新自动化测试，并保留执行证据

## 触发条件

当项目审查发现“功能在 PRD / 请求工件中已完成，但在运行代码中仍存在未接线、伪成功或局部未覆盖”的情况时，触发本次缺口收口流程。

## 初始上下文来源

- `docs/requirements/Bilibili_Live_Tool_PRD.md`
- 历史请求目录：`docs/requests/plugin-system/`、`docs/requests/live-recording/`、`docs/requests/multi-language/`
- 运行代码：
  - `src/renderer/views/plugin-manager/index.vue`
  - `src/renderer/plugins/*`
  - `src/main/index.ts`
  - `src/preload/index.ts`
  - `src/renderer/views/settings/index.vue`
  - `src/renderer/services/danmaku-service.ts`
  - `src/renderer/views/recording/index.vue`
  - `src/renderer/main.ts`
  - `src/renderer/App.vue`
  - `src/renderer/views/dashboard/index.vue`

## 人工交接点

- `spec/spec.md` 完成后进行一次范围和实现约束审批
- `plan/plan.md` 完成后进行一次执行顺序与测试策略审批
- `verification/verification.md` 与 `review/review.md` 产出后，交付用户确认是否需要继续扩展到更多页面或继续推进提交/发布

## 影响范围

- `src/main/index.ts`
- `src/main/ipc-handlers.ts`
- `src/preload/index.ts`
- `src/renderer/plugins/*`
- `src/renderer/views/plugin-manager/index.vue`
- `src/renderer/views/settings/index.vue`
- `src/renderer/stores/config.ts`
- `src/renderer/services/danmaku-service.ts`
- `src/renderer/views/recording/index.vue`
- `src/renderer/stores/recording.ts`
- `src/renderer/main.ts`
- `src/renderer/App.vue`
- `src/renderer/views/dashboard/index.vue`
- `src/renderer/views/settings/components/LanguageSettings.vue`
- `src/renderer/locales/*.json`
- `tests/*`

## 参与模块

- 主进程 IPC 注册
- 预加载层 `electronAPI`
- 插件加载器 / 插件管理器 / 插件 API / 插件 store
- 设置页配置编排
- 录制服务与录制 store
- 房间上下文状态
- 国际化与 Element Plus locale 注入
- 目标页面文案渲染
