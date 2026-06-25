# 官网首页文档 Tailwind 化任务看板

| 任务 ID | 任务名称 | 状态 | 执行模式 | 执行说明 | 触发/前置条件 | 关联规格区域 | 整洁性约束或注意点 | 模式约束或抽象边界 |
|---|---|---|---|---|---|---|---|---|
| T1 | 补齐 VitePress 子项目的 Tailwind 样式入口 | 已完成 | 串行 | 已新增 `.vitepress/theme` 入口并接通 Tailwind | spec 已批准 | spec 4.1 / 7.1 / 7.2 / AC-01 / AC-02 / AC-08 | `style.css` 只做 Tailwind 接线 | 不引入主题系统或组件层 |
| T2 | 迁移首页下载区块样式 | 已完成 | 串行 | 已删除 `index.md` 的 `<style>` 并改为 Tailwind class | T1 完成 | spec 4.2 / AC-03 / AC-05 | 保持现有视觉效果，不做重设计 | 不引入首页下载按钮组件 |
| T3 | 迁移下载页平台卡片区块样式 | 已完成 | 串行 | 已删除 `download/index.md` 的 `<style>` 并改为 Tailwind class | T1 完成 | spec 4.3 / AC-04 / AC-06 | 保持现有卡片层级和 hover 反馈 | 不引入下载卡片组件 |
| T4 | 源码层边界校验与构建验证 | 已完成 | 串行 | 已完成残留样式搜索、边界校验与 `homepage:build` | T2 / T3 完成 | spec 2.2 / 10 / 11 / AC-07 / AC-08 | 不触碰 `.vitepress/dist` 与 `public/logo.svg` | 仅验证，不扩 scope |
