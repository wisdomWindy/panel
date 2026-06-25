# 官网首页文档 Tailwind 化执行计划

## 任务拆解

### T1：补齐 VitePress 子项目的 Tailwind 样式入口

**关联规格**：spec/spec.md - 4.1 / 5.4 / 7.1 / 7.2 / AC-01 / AC-02 / AC-08

**执行内容**：
- 为 `packages/bilibili-live-tool-homepage` 新增 `.vitepress/theme/index.ts`
- 新增 `.vitepress/theme/style.css`，采用与仓库现有 Tailwind v4 兼容的接入方式
- 根据实际需要补 `postcss.config.*` 或子项目配置，使 VitePress 构建能处理 Tailwind
- 确保接线只作用于该官网子项目，不影响桌面端渲染工程

**测试策略**：
- 先以最小配置接入 Tailwind
- 完成后使用 `pnpm homepage:build` 验证样式链路工作

**执行模式**：串行

---

### T2：将首页下载区块从 `<style>` 迁移为 Tailwind utility class

**关联规格**：spec/spec.md - 4.2 / 5.1 / 5.3 / 5.5 / AC-03 / AC-05

**执行内容**：
- 改造 `packages/bilibili-live-tool-homepage/index.md`
- 删除页面底部 `<style>` 块
- 保留现有下载区块结构与平台按钮信息
- 用 Tailwind class 表达：
  - 居中布局
  - 按钮组换行
  - 按钮圆角
  - 平台渐变
  - hover 上浮与阴影
  - 副文案字级和对比度

**测试策略**：
- 通过代码检查确认无 `<style>` 残留
- 通过构建和人工预览确认视觉不明显回退

**执行模式**：串行（依赖 T1）

---

### T3：将下载页平台卡片区块从 `<style>` 迁移为 Tailwind utility class

**关联规格**：spec/spec.md - 4.3 / 5.1 / 5.3 / 5.5 / AC-04 / AC-06

**执行内容**：
- 改造 `packages/bilibili-live-tool-homepage/download/index.md`
- 删除页面底部 `<style>` 块
- 保留平台卡片结构与文案
- 用 Tailwind class 表达：
  - 响应式网格
  - 卡片圆角、边框、soft 背景
  - hover 上浮与阴影
  - 图标尺寸、按钮样式、文件信息样式

**测试策略**：
- 通过代码检查确认无 `<style>` 残留
- 通过构建和人工预览确认卡片层级与交互反馈保持

**执行模式**：串行（依赖 T1）

---

### T4：做源码层边界校验与构建验证

**关联规格**：spec/spec.md - 2.2 / 8 / 10 / 11 / AC-07 / AC-08

**执行内容**：
- 校验本次改动未触碰 `.vitepress/dist`
- 校验未改动 `public/logo.svg`
- 搜索确认首页与下载页已无 `<style>` 残留
- 运行 `pnpm homepage:build`
- 为后续 verification / review 准备证据

**测试策略**：
- `rg` 搜索
- `git diff --stat`
- `pnpm homepage:build`

**执行模式**：串行（依赖 T2 / T3）

## 依赖关系

```text
T1 Tailwind 入口接线
  ↓
T2 首页区块迁移
  ↓
T3 下载页区块迁移
  ↓
T4 边界校验与构建验证
```

说明：
- T2 与 T3 理论上可并行，但当前页面规模小、共享同一 Tailwind 接线与视觉约束，串行更利于控制一致性。

## 整洁性与复杂度控制

1. 不为平台按钮和卡片样式复用引入新 Vue 组件或 markdown component
2. `.vitepress/theme/style.css` 只保留 Tailwind 入口，不写页面业务 CSS
3. 首页和下载页允许出现重复的 Tailwind 类串，但应通过合理换行保持可读性
4. 仅在渐变、阴影等确有需要时使用任意值类，避免把所有尺寸都写成任意值

## 模式决策与替代方案

- **采用**：VitePress theme 扩展入口 + Tailwind utility class 直写
- **解决问题**：
  - 统一子项目样式供应链
  - 移除 Markdown 页面内嵌 `<style>`
  - 保持现有效果，不引入额外抽象成本
- **拒绝方案**：
  1. 不把 `<style>` 挪到全局 CSS 文件，因为这没有完成 Tailwind 化目标
  2. 不引入按钮组件/卡片组件，因为当前只是两页静态内容迁移
  3. 不处理 `.vitepress/dist`，避免把构建产物误纳入源码改造

## 并行执行建议

不启用 workflow-style 并行。

原因：
1. 任务量小；
2. T2 / T3 都依赖同一 Tailwind 接线；
3. 页面视觉一致性要求高，串行更稳。

## 触发与上下文准备

- 启动前读取：
  - `packages/bilibili-live-tool-homepage/index.md`
  - `packages/bilibili-live-tool-homepage/download/index.md`
  - `packages/bilibili-live-tool-homepage/.vitepress/config.ts`
  - 仓库现有 Tailwind v4 接入样例：`packages/bilibili-live-tool/src/renderer/styles/tailwind.css`
- 触发顺序：
  - 先接样式入口，再改首页，再改下载页，最后统一验证

## 受影响文件或模块

| 文件/模块 | 变更类型 | 关联任务 |
|---|---|---|
| `packages/bilibili-live-tool-homepage/.vitepress/theme/index.ts` | 新增 | T1 |
| `packages/bilibili-live-tool-homepage/.vitepress/theme/style.css` | 新增 | T1 |
| `packages/bilibili-live-tool-homepage/.vitepress/config.ts` | 可能修改 | T1 |
| `packages/bilibili-live-tool-homepage/postcss.config.js` 或等价文件 | 可能新增 | T1 |
| `packages/bilibili-live-tool-homepage/index.md` | 修改 | T2 |
| `packages/bilibili-live-tool-homepage/download/index.md` | 修改 | T3 |
| `packages/bilibili-live-tool-homepage/package.json` | 可能修改 | T1 |

## 测试策略

1. 结构验证
   - 搜索确认 `index.md` 与 `download/index.md` 不再含 `<style>`
2. 构建验证
   - 运行 `pnpm homepage:build`
3. 边界验证
   - 确认 `.vitepress/dist` 未被纳入源码提交
   - 确认 `public/logo.svg` 未改动
4. 人工验证
   - 核对首页下载按钮的渐变、圆角、hover 上浮
   - 核对下载页卡片的网格、边框、hover 阴影

## 观察与人工介入点

1. `plan` 审批：确认只做两页迁移，不扩展到整站
2. 执行后人工复核：首页和下载页视觉是否明显回退
3. 若接入 Tailwind 后 VitePress 构建出现兼容性问题，需要决定是补子项目 PostCSS 配置还是改 theme 样式入口声明

## 回滚说明

1. 若 Tailwind 接入导致 `homepage:build` 失败，可优先回滚 `.vitepress/theme/*` 与子项目 PostCSS 配置
2. 若页面视觉回退明显，可单独回滚 `index.md` 或 `download/index.md` 的迁移，不影响另一页
3. 不回滚 `.vitepress/dist`，因为其不属于本次源码改造范围
