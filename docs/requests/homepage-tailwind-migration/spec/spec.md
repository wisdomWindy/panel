# 官网首页文档 Tailwind 化规格说明

## 1. 背景与目标

`packages/bilibili-live-tool-homepage` 是当前仓库中的 VitePress 官网子项目。源码层实际只在两个页面存在自定义样式表达：

- `packages/bilibili-live-tool-homepage/index.md`
- `packages/bilibili-live-tool-homepage/download/index.md`

这两个页面通过 Markdown 内嵌 HTML + `<style>` 块实现下载按钮和下载卡片的视觉效果。当前问题不是视觉设计不足，而是样式表达方式与仓库既有 Tailwind 方向不一致，后续维护成本偏高。

本次目标是在尽量保持现有视觉效果的前提下，为该子项目补齐本地 Tailwind 样式供应链，并把源码层的 `<style>` 迁移为 Tailwind utility class。

## 2. 范围

### 2.1 本次包含

1. 为 `packages/bilibili-live-tool-homepage` 接入本地 Tailwind 样式入口
2. 在 `.vitepress` 目录下新增 theme 入口以加载 Tailwind 样式
3. 改造 `index.md` 首页下载区块的样式表达
4. 改造 `download/index.md` 下载页卡片区块的样式表达
5. 删除上述源码页面中的 `<style>` 块
6. 补充必要配置，使 `homepage:build` 能输出 Tailwind 样式

### 2.2 本次不包含

1. 不重做首页或下载页视觉设计
2. 不改动 `.vitepress/dist` 构建产物
3. 不改动 `public/logo.svg` 中的内联 `style`
4. 不扩散修改其他未使用 `<style>` 的 Markdown 页面
5. 不增加新的交互逻辑、动画库或组件系统

## 3. 用户流程

### 流程 1：首页访问

1. 用户进入官网首页
2. 继续看到 VitePress hero、功能卡片、版本提示和下载区块
3. 三平台下载按钮继续具备平台色渐变、圆角、hover 上浮和副文案
4. 点击按钮后跳转到 GitHub Release 安装包

### 流程 2：下载页访问

1. 用户进入下载页
2. 继续看到三平台下载卡片网格
3. 每张卡片内保留平台图标、平台名、支持版本、下载按钮和文件大小
4. 卡片 hover 继续保留上浮和阴影

## 4. 页面与模块设计

### 4.1 站点样式入口

- 新增 `packages/bilibili-live-tool-homepage/.vitepress/theme/index.ts`
  - 负责扩展默认 VitePress theme
  - 统一加载 Tailwind 样式入口

- 新增 `packages/bilibili-live-tool-homepage/.vitepress/theme/style.css`
  - 仅负责引入 Tailwind
  - 不承载首页或下载页业务选择器

### 4.2 首页 `index.md`

- 保持 frontmatter `layout: home`、`hero`、`features` 结构不变
- “下载安装” 区块继续使用内嵌 HTML，但只保留结构标签与 Tailwind class
- 删除页面底部 `<style>` 块
- 平台按钮仍区分三类色彩：
  - Windows：蓝色渐变
  - macOS：深色渐变
  - Linux：橙色渐变

### 4.3 下载页 `download/index.md`

- 保持文档结构、安装说明和系统要求不变
- 仅改造平台下载卡片区块的样式表达
- 下载卡片继续使用响应式网格 + 卡片 hover 反馈
- 删除页面底部 `<style>` 块

## 5. 责任边界与约束

### 5.1 责任与边界约束

1. `.vitepress/theme` 只负责 Tailwind 样式入口接线，不写页面业务逻辑
2. Markdown 页面自身只保留内容结构和 Tailwind class，不再定义 `<style>` 选择器
3. 视觉保持约束必须在页面结构层实现，不通过额外全局 CSS 覆盖来“补回效果”

### 5.2 命名与领域约束

1. 若需要保留少量语义容器 class，其职责仅限结构定位，不承载 CSS 规则
2. 新增文件名应与 VitePress theme 约定一致，避免自造样式管理入口

### 5.3 重复规则约束

1. 首页与下载页的重复平台按钮样式允许通过重复 Tailwind class 先落地，不强行引入组件抽象
2. 若出现完全一致的长类串，仅允许用非常轻量的局部结构优化，不引入新的运行时组件层

### 5.4 副作用与依赖约束

1. Tailwind 仅在 `packages/bilibili-live-tool-homepage` 子项目内接线，不影响桌面端渲染入口
2. 优先复用仓库已存在的 Tailwind / PostCSS 依赖，不新增不必要第三方库
3. 构建验证以 `pnpm homepage:build` 为准

### 5.5 复杂度与可读性约束

1. 不为复用几组按钮样式而引入 Vue 组件或自定义 markdown component
2. Tailwind 任意值类允许使用，但应限制在平台渐变、阴影等无法用现成 utility 精准表达的场景
3. 不引入额外 design token 层或复杂主题系统

## 6. 变化轴与模式决策

### 6.1 变化轴

1. 样式供应链：从页面内嵌 `<style>` 转为 theme 层统一加载 Tailwind
2. 页面表达：从“结构 + 选择器”转为“结构 + utility class”

### 6.2 模式决策

- **明确不引入命名模式层**
  - 不新增组件工厂、样式注册器、按钮抽象层
- **采用的最轻结构**
  - VitePress theme 扩展入口
  - 单个 Tailwind 样式入口文件
- **原因**
  - 这次问题是样式表达和供应链统一，不是交互逻辑复用或多变体组件抽象
  - 直接使用 Tailwind utility class 比额外封装更符合“尽量保持现有效果”的要求

## 7. 技术契约

### 7.1 站点主题入口契约

```ts
// .vitepress/theme/index.ts
import DefaultTheme from 'vitepress/theme'
import './style.css'

export default {
  extends: DefaultTheme
}
```

说明：
- 可以有等价写法，但必须由 VitePress theme 入口加载 Tailwind 样式；
- 不允许继续依赖 Markdown 页面内 `<style>` 提供业务样式。

### 7.2 Tailwind 样式入口契约

样式入口需与仓库现有 Tailwind v4 接入方式兼容，至少满足：

```css
@import "tailwindcss";
```

若内容扫描需要显式声明，可补相应配置文件，但不得恢复为 CDN 或页面内 `<style>` 方案。

## 8. 边界情况

1. 如果首页或下载页中的长 Tailwind 类导致 Markdown 可读性显著下降：
   - 允许做少量结构换行和语义包装，但不允许回退为 `<style>`。
2. 如果 Tailwind 未覆盖 Markdown 中的任意值类：
   - 优先修正 Tailwind / PostCSS / theme 接入方式，不回退到自定义 CSS。
3. 如果某些效果无法 1:1 还原：
   - 应优先保持视觉层级与交互反馈一致，而不是新增复杂样式补丁。

## 9. 风险

1. VitePress 子项目此前未显式声明 Tailwind 入口，接线错误会直接导致构建无样式。
2. Markdown 中的复杂渐变与阴影改写后，类串可能偏长，存在可维护性压力。
3. 如果根仓库依赖已存在但子项目缺少必要配置，首次构建可能暴露 PostCSS 入口问题。

## 10. 验收标准

| 编号 | 验收条件 | 验证方式 |
|---|---|---|
| AC-01 | `packages/bilibili-live-tool-homepage` 存在可工作的 Tailwind 源码入口 | 代码检查 / 构建 |
| AC-02 | `.vitepress` 通过 theme 入口加载 Tailwind 样式，而不是在 Markdown 内用 `<style>` | 代码检查 |
| AC-03 | `index.md` 首页源码中的 `<style>` 块被移除 | 代码检查 |
| AC-04 | `download/index.md` 源码中的 `<style>` 块被移除 | 代码检查 |
| AC-05 | 首页下载按钮区块改为 Tailwind utility class，且视觉层级不明显回退 | 代码检查 / 人工验证 |
| AC-06 | 下载页平台卡片区块改为 Tailwind utility class，且视觉层级不明显回退 | 代码检查 / 人工验证 |
| AC-07 | 不改动 `.vitepress/dist` 和 `public/logo.svg` | 代码检查 |
| AC-08 | `pnpm homepage:build` 能通过 | 命令验证 |

## 11. 验证与交接

- 技术验证：
  - `pnpm homepage:build`
- 人工复核重点：
  1. 首页下载按钮的渐变色、圆角、hover 上浮是否保留
  2. 下载页卡片网格、边框、hover 阴影是否保留
  3. 页面在窄屏下是否仍能合理换行

## 12. 相关文件

- `packages/bilibili-live-tool-homepage/index.md`
- `packages/bilibili-live-tool-homepage/download/index.md`
- `packages/bilibili-live-tool-homepage/.vitepress/config.ts`
- `packages/bilibili-live-tool-homepage/.vitepress/theme/index.ts`
- `packages/bilibili-live-tool-homepage/.vitepress/theme/style.css`
- `packages/bilibili-live-tool-homepage/package.json`
