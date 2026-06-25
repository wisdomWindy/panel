# 首页文档 Tailwind 化请求

## 请求 ID
`homepage-tailwind-migration`

## 来源
- 本地需求指令
- 目标路径：`packages/bilibili-live-tool-homepage`

## 业务摘要
当前 `packages/bilibili-live-tool-homepage` 是一个基于 VitePress 的官网文档子项目。源码层仍残留 Markdown 内嵌 `<style>` 块，首页与下载页通过自定义 CSS 控制下载按钮、下载卡片、平台渐变和 hover 效果。用户要求在尽量保持现有视觉效果的前提下，将这些源码层样式表达迁移为 Tailwind CSS 风格，并限定本次范围仅包含 Markdown / `.vitepress` / 必要配置，不处理构建产物或静态资源中的内联样式。

## 目标陈述
为 `packages/bilibili-live-tool-homepage` 接入本地 Tailwind 样式链路，并将源码层现有 `<style>` / 内联样式依赖改写为 Tailwind utility class，同时保持首页与下载页当前视觉效果和信息结构基本不变。

## 初始完成信号
当以下条件同时成立时，本请求可视为完成：
1. `packages/bilibili-live-tool-homepage` 已有可工作的 Tailwind 源码入口；
2. 首页与下载页源码中的 `<style>` 块被移除；
3. 页面样式通过 Tailwind utility class 表达，视觉结果不明显回退；
4. `homepage:build` 能通过，且 review / verification 工件明确记录约束合规。

## 触发条件
当团队继续维护官网文档页面，且需要将零散样式表达统一收敛到 Tailwind 风格时，启动本次迁移请求。

## 初始上下文来源
- `packages/bilibili-live-tool-homepage/index.md`
- `packages/bilibili-live-tool-homepage/download/index.md`
- `packages/bilibili-live-tool-homepage/.vitepress/config.ts`
- 根仓库现有 Tailwind 依赖与 `packages/bilibili-live-tool/src/renderer/styles/tailwind.css`
- 用户确认的边界：尽量保持当前视觉效果；仅改源码层，不处理 `public/logo.svg` 和 `.vitepress/dist`

## 人工交接点
- Spec 审批：确认“保持现有效果，仅做 Tailwind 化迁移”的范围
- Plan 审批：确认实现顺序、验证方式与不触碰构建产物的边界
- 交付后人工复核：首页与下载页视觉效果是否与迁移前一致

## 影响范围
- `packages/bilibili-live-tool-homepage/index.md`
- `packages/bilibili-live-tool-homepage/download/index.md`
- `packages/bilibili-live-tool-homepage/.vitepress/**`
- `packages/bilibili-live-tool-homepage/package.json`（如需补依赖）
- 必要的 PostCSS / Tailwind 配置文件

## 参与模块
- VitePress 站点配置
- Markdown 页面内嵌 HTML 区块
- Tailwind / PostCSS 构建链路
