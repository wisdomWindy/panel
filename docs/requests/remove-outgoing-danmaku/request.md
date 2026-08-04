# 移除主动发送弹幕能力

## 请求标识

- request_id: `remove-outgoing-danmaku`
- 类型: 直接变更请求
- 来源: 用户指令“移除 以主播账号发送弹幕 互动面板弹幕指令 功能”

## 业务摘要

移除产品中所有依赖主播账号向 Bilibili 直播间主动发送弹幕的能力，并清理未实现的互动面板弹幕指令宣称；保留弹幕接收、展示、统计、TTS 和其他只读消费链路。

## 目标

生产代码、配置、路由、插件能力和产品文档不再提供或宣称主动发送弹幕能力，且发送链路的依赖闭包完整清除。

## 初始完成信号

- 不存在手动发送、定时/随机发送、触发词回复、礼物感谢回复或插件发送入口。
- 不存在对 `api.live.bilibili.com/msg/send` 的生产调用。
- 弹幕接收、展示、统计和 TTS 仍可通过类型检查与现有测试。
- 产品文档不再把主动发送或互动面板弹幕指令列为产品能力。

## 触发条件

用户在现有 API 可行性审查后明确要求移除相关功能。

## 初始上下文来源

- 用户直接指令。
- `packages/bilibili-live-tool/src/renderer/modules/danmaku/`。
- `packages/bilibili-live-tool/src/renderer/services/danmaku-service.ts`。
- 弹幕页面、设置、路由、插件 API、数据库配置和产品文档中的引用闭包。

## 人工交接点

代码、文档、类型检查和测试完成后交付变更摘要；没有需要前置确认的产品决策。

## 影响范围

- 主播账号主动发送弹幕链路。
- 弹幕池及自动回复配置与页面。
- 插件 `sendDanmaku` 能力。
- 产品 README、插件文档、架构/实现/需求文档中的现状宣称。

## 参与模块

- renderer danmaku manager/service/store/views/router/settings/plugins/locales
- main database and IPC danmaku-pool persistence
- README、产品与开发文档

## 生命周期路由

- 直接变更请求，不进入 PRD-only requirement-analysis / requirement-splitting。
- 页面仅删除入口，不重新设计布局，因此不进入 page-design。
- 删除跨模块 API、状态和文件所有权，`architecture_design_required=true`，在 spec 后进入 architecture-design。

