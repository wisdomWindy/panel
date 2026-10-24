# Clarifications: page-content-spacing-background

## Question

是否需要逐页重做内部 padding 或新增统一页面 wrapper？

## Answer

不需要。代码事实显示全部目标页共享 `App.vue` 的 `<main>`，且该节点已经提供 `p-3 sm:p-5`；截图中的灰色边缘来自浅色 shell 背景差异。

## Final decision

只在共享 shell owner 替换浅色背景 token，保留页面内部布局和深色主题。

## Affected spec area

In scope、UI / UX design contract、function-complete behavior、acceptance criteria。

## Grounding status and required source

`source-backed`（U1-U5）+ `code-fact-backed`（App.vue/router/target page roots）。无 `missing-source`，无需用户补充确认。
