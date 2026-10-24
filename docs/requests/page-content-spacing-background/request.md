# Request: page-content-spacing-background

## request identifier

`page-content-spacing-background`

## source link or direct user instruction reference

- 直接用户缺陷反馈：“弹幕面板、数据看板、排行榜、敏感词、黑名单页面的样式是不是有点问题，最后面的背景是灰色的，然后红框中的部分没有边距，其他页面同步修改”。
- 截图：`C:/Users/YANGJI~1/AppData/Local/Temp/codex-clipboard-dea82ffb-1920-4406-ac0e-e3ac8c981edf.png`

## business summary

浅色主题下的共享内容区背景与页面白色 surface 不一致，导致页面间距显示成灰色边缘或底部灰色露出。

## goal statement

统一应用壳层内容区的浅色背景与响应式页面边距，使用户点名页面及其他同壳层页面不再露出灰色底层，且内容与导航/容器边缘保持现有 `p-3 sm:p-5` 间距。

## initial done signal

- 浅色模式的共享壳层和 `<main>` 内容区背景为白色。
- 内容区保留 `p-3 sm:p-5` 响应式边距。
- 弹幕面板、数据看板、排行榜、敏感词、黑名单以及其他路由共享同一修复。
- 深色主题和页面业务逻辑不变。

## trigger condition

用户在浅色主题下打开任一主应用路由页面。

## initial context sources

`App.vue`、五个点名页面的 root surface、路由表、应用壳层合同测试、用户截图。

## human handoff point

交付共享壳层修复、回归测试与桌面视口截图。

## affected area

`packages/bilibili-live-tool/src/renderer/App.vue` 及主应用所有 router-view 页面的共享内容背景。

## participating modules

- application shell
- renderer route pages
- shell contract tests

