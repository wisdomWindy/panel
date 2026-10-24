# 删除插件中心

## 请求标识

- request-id: `remove-plugin-center`
- 类型: 直接变更 / 功能下线
- 来源: 当前会话中的用户直接指令

## 业务摘要

从桌面应用及其配套资料中下线“插件中心”，删除该功能的入口、页面、前后端契约、运行逻辑、持久化接入、专属测试和专属文档；Tauri 框架自身使用的 updater、dialog、notification 等官方插件不属于该业务功能。

## 目标声明

应用不再向用户提供插件中心，也不再编译、注册或暴露插件中心专属代码和接口，同时不影响更新、关于、配置导入导出等相邻能力。

## 初始完成信号

- 导航和路由中不再存在插件中心入口或 `/plugins` 页面。
- 前端不再包含插件页面、插件状态、插件桌面契约和插件事件监听。
- Rust 端不再包含插件中心专属领域模型、服务、Provider、IPC 命令、权限声明和路径选择用途。
- 插件中心专属数据库/配置导入导出接入、测试、开发文档和产品宣传入口已清理。
- 搜索、类型检查、前端测试、Rust 格式/测试和构建相关检查无插件中心残留或回归。

## 触发条件

用户明确要求：“把插件中心功能及其代码删除”。

## 初始上下文来源

- 用户直接指令。
- `packages/bilibili-live-tool/src/renderer/` 中的路由、导航、release store、桌面契约、国际化和插件页面。
- `packages/bilibili-live-tool/src-tauri/` 中的 release 领域/应用/基础设施/IPC、权限、路径、数据库和测试。
- `README.md`、产品 PRD、主页功能说明和插件开发文档中的插件中心声明。

## 人工交接点

完成后交付变更摘要、删除边界和验证结果；当前不存在必须由用户补充的产品决策。

## 影响范围

- 桌面端 Vue 导航、路由、页面、状态、契约、适配器、国际化和测试。
- Tauri/Rust 插件中心专属模型、服务、端口、实现、IPC、权限、路径用途、数据库接入和测试。
- 插件中心专属文档与产品功能声明。

## 参与模块

- application shell
- release / desktop contract
- Tauri release application and IPC
- persistence and configuration import/export
- documentation

## 工作流分类

- 直接变更请求，不进入 PRD-only `requirement-analysis` / `requirement-splitting`。
- 页面设计不是本次主体，不进入 `page-design`。
- 涉及跨前后端模块边界和依赖闭包，`architecture_design_required=true`，在 spec 通过后进入 architecture-design。

