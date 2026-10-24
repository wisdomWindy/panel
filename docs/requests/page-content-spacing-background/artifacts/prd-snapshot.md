# Normalized Bugfix Snapshot

## verbatim source evidence index

- U1-U4：见 `artifacts/bugfix-source.md` 的原文证据索引。
- U5：用户附件截图，标记灰色壳层背景与红框内容区。

## source summary

修复浅色主题下共享内容区的灰色底层露出和边距视觉不连续，并同步所有同壳层页面。

## key business goals

- 页面背景连续、干净，不再有意外灰色边缘/底部。
- 内容保持统一壳层边距。
- 同一处共享修复覆盖所有路由页面。

## greenfield scaffold sensitivity

不适用：not a project/app/package/frontend-surface creation request。

## explicit behavior constraints

- 只修改布局背景与视觉间距。
- 现有页面的表格、筛选、按钮、空状态、loading、错误和对话框行为不变。
- 保留深色主题 `zinc-950` 壳层背景。

## closed-world product scope allowlist

- 浅色共享壳层和主内容区背景改为白色。
- 共享主内容区保留 `p-3 sm:p-5`。
- 全部主应用路由共享上述外层行为。

## forms, tables, displays, and interactions extracted from source

无新增或修改；所有现有内容与交互保持不变。

## workflow and state rules extracted from source

浅色主题进入任意路由 -> 共享白色内容背景与响应式边距 -> 页面内部状态照常渲染。

## relevant modules or pages

`App.vue` 共享 shell；弹幕面板、数据看板、排行榜、敏感词、黑名单及其他主 router-view 页面。

## notable open questions from the upstream PRD or direct instruction

无。修复不改动页面内部间距、业务内容或深色主题层级。

