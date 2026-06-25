# 官网首页文档 Tailwind 化执行记录

## 执行概览

- 当前阶段：`execute`
- 执行顺序：`T1 -> T2 -> T3 -> T4`
- TDD 说明：
  - 本次改动主要是 VitePress 样式供应链接线与 Markdown 页面样式表达迁移，缺少合适的细粒度自动化单元测试载体。
  - 按规格约定采用构建验证 + 代码检查 + 人工视觉复核作为替代验证方式。

## T1：补齐 VitePress 子项目的 Tailwind 样式入口

- 状态：已完成
- 计划动作：
  - 新增 `.vitepress/theme/index.ts`
  - 新增 `.vitepress/theme/style.css`
  - 视需要补充子项目 `postcss.config.js` 与 `tailwind.config.js`
- 执行记录：
  1. 首次完成 theme / style / postcss / tailwind config 接线后，执行 `rtk npm run homepage:build` 失败
  2. 根因定位：`packages/bilibili-live-tool-homepage/node_modules` 中仅有 `vitepress`，缺少 `tailwindcss`、`@tailwindcss/postcss`、`postcss`、`autoprefixer` 的子项目依赖链接，导致 `@import "tailwindcss"` 无法解析
  3. 修复动作：为 homepage 子项目补齐上述 devDependencies，并尝试通过离线安装恢复 workspace 依赖链接
  4. 离线安装因本地 store 缺少 `@tailwindcss/postcss` tarball 失败，随后执行 `pnpm install --filter @monorepo/bilibili-live-tool-homepage` 成功补齐依赖链接

## T2：迁移首页下载区块样式

- 状态：已完成
- 已完成项：
  - 删除 `packages/bilibili-live-tool-homepage/index.md` 中的 `<style>` 块
  - 将下载按钮区块改为 Tailwind utility class
  - 保留三平台渐变、圆角、hover 上浮和副文案层级

## T3：迁移下载页平台卡片区块样式

- 状态：已完成
- 已完成项：
  - 删除 `packages/bilibili-live-tool-homepage/download/index.md` 中的 `<style>` 块
  - 将卡片网格、卡片 hover、平台按钮和文件信息改为 Tailwind utility class

## T4：源码层边界校验与构建验证

- 状态：已完成
- 验证记录：
  1. `rtk rg -n '<style|style=' packages/bilibili-live-tool-homepage/index.md packages/bilibili-live-tool-homepage/download/index.md packages/bilibili-live-tool-homepage/.vitepress` 未命中源码残留样式
  2. `rtk git diff --stat -- packages/bilibili-live-tool-homepage/public/logo.svg packages/bilibili-live-tool-homepage/.vitepress/dist` 无输出，说明未改动边界外文件
  3. `rtk npm run homepage:build` 通过
