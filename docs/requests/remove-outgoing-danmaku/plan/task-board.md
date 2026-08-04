# Task Board: remove-outgoing-danmaku

| ID | 任务 | 状态 | 依赖 | Spec / 来源 | Product-content delta | 完成证据 |
| --- | --- | --- | --- | --- | --- | --- |
| T-1 | 删除核心发送运行时、状态与持久化闭包 | completed | spec + architecture approved | S-1/S-3/S-4/S-5；SRC-1 + code facts | none | sender/发送 API/pool/config/IPC 残留检索为零；app build 通过 |
| T-2 | 删除 UI、插件、导航与配置闭包 | completed | T-1 | S-1 至 S-5；SRC-1 + code facts | 仅删除 allowlist 内容 | 页面/路由/插件/locale 无发送入口，`onDanmaku` 保留 |
| T-3 | 适配测试与当前文档，完成验证和自审 | completed | T-1、T-2 | S-2/S-4/S-5/S-6；SRC-1 + code facts | 删除当前能力宣称 | tests/build/residual/diff 通过；iteration 12/15 补齐 AI 自动触发、插件/架构/实施文档与失效注释闭包；type/lint 基线限制已记录 |

## 执行规则

- 串行顺序：T-1 → T-2 → T-3。
- execute 不得新增替代发送机制、兼容壳、重定向、feature flag、产品文案或视觉重设计。
- 测试引用是适配面，不是保留待删生产符号的调用方。
- 失败按 `plan.md` 自愈闭环回流；未决产品信息不得在 execute 猜测。
