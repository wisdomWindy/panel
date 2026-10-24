# 请求：page-inner-padding-consistency

## 请求标识

`page-inner-padding-consistency`

## 来源

用户直接缺陷反馈 U1，并附带截图 `codex-clipboard-70c310db-bd57-4556-b43e-36c808bfae2f.png`。

## 业务摘要

上舰特效、礼物提醒、礼物音效、插件管理页面在共享主内容区边距之外又增加了页面级 padding，导致内容与侧栏之间的距离大于其他页面。

## 目标与初始完成信号

四个点名页面只使用应用壳层已有的 `p-3 sm:p-5` 外边距，不再叠加页面根节点 padding；内部表单、表格、按钮、分栏、状态和交互保持不变。运行时在相同视口下，四页标题左边界与语音播报、直播录制等正常页面一致。

## 触发条件与上下文

- 触发：用户从导航进入四个点名页面之一。
- 上下文：`App.vue` 共享 `<main>`、四个 route SFC、shell contract 测试、既有 `page-content-spacing-background` 规范。
- 人工交接：代码、测试和桌面/窄屏运行时验收完成后交付。

## 影响区域与参与模块

- `views/captain-effect/index.vue`
- `views/gift-reminder/index.vue`
- `views/gift-sound/index.vue`
- `views/plugin-manager/index.vue`
- `components/application-shell/shell-components-contract.test.ts`

非 PRD 缺陷，跳过 requirement-analysis 与 requirement-splitting；需要 page-design，因为缺陷直接涉及跨页面布局间距。
