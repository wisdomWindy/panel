# 应用壳与直播间首屏重设计

## 请求类型

- 类型：direct-change
- 交付单元：`shell-connection-redesign`
- speed profile：`S2 scoped`
- 原因：改动覆盖应用壳、导航、工具栏、状态栏、直播间首屏与 Web 启动脚本，但不改变业务数据或后端合同。

## 初始目标

修复当前运行后近似空白或信息不可读的首屏，把既有直播间连接流程重设计为清晰、稳定、可操作的桌面工作台，同时保持导航、房间号输入、连接/断开、通知、设置、状态栏和直播间摘要功能正常。

## Done Signal

- `pnpm dev:web` 能以 free 通道直接启动，页面不再因缺失 `VITE_BUILD_CHANNEL` 而挂载失败。
- 1280x720 桌面视口下，品牌区、连接工具栏、侧边导航、直播间状态、三个房间摘要字段与底部状态栏均清晰可见。
- 深色与浅色偏好下均无低对比文字、内容重叠或空白壳层。
- 既有房间号校验、连接/断开、路由导航、通知与设置入口保持原有事件链路。
- 类型检查、相关 Vitest、lint、构建和浏览器人工验证通过。

## Trigger / Context / Handoff

- trigger：用户反馈运行结果如截图，并要求重新设计且功能正常。
- context：用户截图；`packages/bilibili-live-tool/src/renderer` 现有 Vue/Tailwind/Element Plus 实现；现有应用壳与直播间合同测试。
- steerability：以本目录中的设计、spec、plan、verification 和 review 工件为准。
- handoff：完成后保留本地开发服务器与可访问 URL，并在 verification 中记录截图级验证证据。

## 范围

- `packages/bilibili-live-tool/package.json`
- `src/renderer/App.vue`
- `src/renderer/components/application-shell/AppShellToolbar.vue`
- `src/renderer/components/application-shell/AppShellNavigation.vue`
- `src/renderer/components/application-shell/AppShellStatusBar.vue`
- `src/renderer/views/connection/index.vue`
- 与上述表面直接相关的合同测试

## 非范围

- 不新增直播业务字段、接口、权限、路由或会员功能。
- 不改变 Rust command、DTO、store、adapter、连接重试策略或生产 Bilibili 接入合同。
- 不重设计其他业务页面内部内容。

## 人工交接点

无阻塞性产品决策。用户要求的 Figma MCP 在当前会话工具清单中不存在，不能生成或引用 Figma 文件；本请求使用 framework 的 durable page-design artifact 完成等价的设计约束，并在最终交付中如实披露。

## 工作流路由

这是稳定的 direct-change，不需要 PRD-only requirement-analysis / requirement-splitting。页面布局、视觉层级与交互组织是主要工作，因此路由为 `page-design -> spec -> plan -> execute -> verify -> review`。现有组件 owner 与状态拓扑不变，不触发 architecture-design。
