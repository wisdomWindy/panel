# 源需求快照

## source summary

- 用户明确要求使用 `frontend-agent-framework` 处理 `packages/bilibili-live-tool-homepage` 项目；
- 目标是把该子项目中的样式改成 Tailwind CSS 风格；
- 用户进一步限定：尽量保持当前视觉效果，只做 `style` / 内联样式到 Tailwind class 的迁移；
- 边界再次明确：只改源码层（Markdown / `.vitepress` / 必要配置），不处理 `public/logo.svg` 和 `.vitepress/dist`。

## key business goals

1. 统一官网源码层样式表达方式，移除 Markdown 页面中的 `<style>` 块；
2. 给 VitePress 子项目建立可维护的本地 Tailwind 样式供应链；
3. 在不重设计页面的前提下，保持首页和下载页现有视觉效果；
4. 为后续官网页面继续使用 Tailwind utility class 提供稳定入口。

## relevant modules or pages

- `packages/bilibili-live-tool-homepage/index.md`
- `packages/bilibili-live-tool-homepage/download/index.md`
- `packages/bilibili-live-tool-homepage/.vitepress/config.ts`
- 根仓库 Tailwind 依赖与既有接入方式

## notable open questions from the upstream PRD

1. 是否需要把站点内其他 Markdown 页面同步引入 Tailwind 风格约束
   - 当前答案：本次仅处理实际存在 `<style>` 的首页与下载页。
2. 是否需要连同静态资源（如 `public/logo.svg`）中的样式表达一起改
   - 当前答案：否，用户已明确排除。
3. 是否允许顺手重构视觉层级
   - 当前答案：否，需尽量保持当前视觉效果。
