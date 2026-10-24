# 直接变更来源快照

## Verbatim Source Evidence Index

- `SRC-1`（source-backed，当前会话用户原话）：“把插件中心功能及其代码删除”

## 来源摘要

唯一产品授权是完整下线插件中心及其代码。仓库事实仅用于确认该功能的调用链和清理闭包，不授权相邻功能重构。

## 关键业务目标

- `SRC-1`：用户不可再访问或使用插件中心。
- `SRC-1`：插件中心专属实现代码应从编译和运行路径中移除。
- `code-fact-backed`：删除共享模块中的插件分支时，必须保持更新、关于、韧性状态、配置导入导出等相邻功能正常。

## Greenfield Scaffold Sensitivity

不适用：本请求是现有功能删除，不创建项目、应用、包或新的前端业务面。

## 明确行为约束

- 删除目标：插件中心功能及其代码。
- 保留边界：名称中包含 `tauri_plugin_*` 的第三方框架依赖，只要服务于更新、文件对话框、通知、快捷键等非插件中心能力，就不属于删除目标。
- 删除闭包：入口、路由、页面、状态、事件、IPC、领域/应用/基础设施实现、权限、持久化接入、测试和专属文档均属于插件中心依赖闭包。
- 禁止解释：不得借此重构 release、application shell、配置系统或其他相邻功能。

## Closed-World Product Scope Allowlist

- 删除“插件中心”导航项及 `/plugins` 页面。
- 删除插件安装、启停、卸载、配置、状态和崩溃事件等插件中心能力。
- 删除插件中心专属显示文案、类型、命令、权限、数据接入和文档。
- 对共享文件仅删除插件中心分支，保留其余既有行为。

## 表单、表格、展示与交互

- 删除插件列表、安装对话框、安装/启停/配置/卸载等交互。
- 不新增替代页面、提示、跳转或占位内容。

## 工作流与状态规则

- 删除前链路：导航 `/plugins` -> 插件页面 -> release store -> desktop command/event -> Rust IPC -> release service/provider/persistence。
- 删除后状态：上述链路不存在；访问 `/plugins` 不再匹配命名路由；应用其余 release 能力保持原有路径。

## 相关模块与代码事实

- `code-fact-backed`：前端插件能力分布于 router、application-shell navigation/adapters、release store/adapters/contracts/runtime/locales 和 `views/plugin-manager`。
- `code-fact-backed`：Rust 插件能力分布于 release domain/application/ports/infrastructure/IPC、path purpose、permissions、database/config import-export 及测试。
- `code-fact-backed`：旧迁移已创建 `t_plugin`，删除策略需区分新建数据库定义与既有安装升级清理。

## 开放问题

无。用户的删除指令足以确定目标；迁移与共享文件处理属于依赖闭包的技术实现选择，不新增产品语义。

## Out Of Scope

- 删除或替换 Tauri 官方框架插件依赖。
- 调整自动更新、关于信息、崩溃恢复或配置系统的非插件行为。
- 新增插件中心替代功能。

