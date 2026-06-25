# 官网首页文档 Tailwind 化验证记录

## acceptance coverage

| acceptance item id | verification method | result | evidence reference | follow-up if failed | handoff status |
|---|---|---|---|---|---|
| AC-01 | 代码检查 + 构建验证 | pass | `packages/bilibili-live-tool-homepage/.vitepress/theme/index.ts`、`packages/bilibili-live-tool-homepage/.vitepress/theme/style.css`、`packages/bilibili-live-tool-homepage/tailwind.config.js`、`packages/bilibili-live-tool-homepage/postcss.config.js`；`rtk npm run homepage:build` 通过 | 无 | ready |
| AC-02 | 代码检查 | pass | `.vitepress/theme/index.ts` 统一加载 `style.css`；首页与下载页已无页面级 `<style>` 作为业务样式来源 | 无 | ready |
| AC-03 | 代码检查 | pass | `packages/bilibili-live-tool-homepage/index.md` 已移除 `<style>` 块；`rtk rg -n '<style|style=' ...` 对首页未命中 | 无 | ready |
| AC-04 | 代码检查 | pass | `packages/bilibili-live-tool-homepage/download/index.md` 已移除 `<style>` 块；`rtk rg -n '<style|style=' ...` 对下载页未命中 | 无 | ready |
| AC-05 | 代码检查 + 构建验证 | pass | `packages/bilibili-live-tool-homepage/index.md` 中下载区块已改为 Tailwind utility class，保留三平台渐变、圆角、hover 上浮和副文案结构；`rtk npm run homepage:build` 通过 | 无 | ready |
| AC-06 | 代码检查 + 构建验证 | pass | `packages/bilibili-live-tool-homepage/download/index.md` 中卡片区块已改为 Tailwind utility class，保留网格、边框、圆角、hover 阴影与平台按钮 | 无 | ready |
| AC-07 | 代码检查 | pass | `rtk git diff --stat -- packages/bilibili-live-tool-homepage/public/logo.svg packages/bilibili-live-tool-homepage/.vitepress/dist` 无输出 | 无 | ready |
| AC-08 | 命令验证 | pass | `rtk npm run homepage:build` 通过 | 无 | ready |

## spec constraint compliance

- result: pass
- checked constraints:
  - 仅在 `packages/bilibili-live-tool-homepage` 子项目内接入 Tailwind，不影响桌面端渲染入口
  - `.vitepress/theme` 只负责样式入口接线，不承载页面业务逻辑
  - Markdown 页面只保留结构和 Tailwind class，不再定义 `<style>` 选择器
  - 未引入按钮组件、卡片组件或新的抽象层
  - 未触碰 `.vitepress/dist` 与 `public/logo.svg`
- evidence reference:
  - `packages/bilibili-live-tool-homepage/.vitepress/theme/index.ts`
  - `packages/bilibili-live-tool-homepage/.vitepress/theme/style.css`
  - `packages/bilibili-live-tool-homepage/index.md`
  - `packages/bilibili-live-tool-homepage/download/index.md`
  - `rtk npm run homepage:build`
  - `rtk git diff --stat -- packages/bilibili-live-tool-homepage/public/logo.svg packages/bilibili-live-tool-homepage/.vitepress/dist`
- follow-up if failed: 无

## summary

- 本次改动以构建验证和代码检查为主；未编写单元测试，原因是本次主要为静态样式供应链接线与 Markdown 页面样式迁移，缺少合适的行为级测试载体。
- 关键验证命令：
  - `rtk rg -n '<style|style=' packages/bilibili-live-tool-homepage/index.md packages/bilibili-live-tool-homepage/download/index.md packages/bilibili-live-tool-homepage/.vitepress`
  - `rtk git diff --stat -- packages/bilibili-live-tool-homepage/public/logo.svg packages/bilibili-live-tool-homepage/.vitepress/dist`
  - `rtk npm run homepage:build`
- 结果：全部通过。
