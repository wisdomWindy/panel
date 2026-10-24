# Bugfix Source

## source system

当前对话中的直接用户缺陷反馈与附件截图。

## project key or source scope

`bilibili-live-tool` 主应用壳层下的路由页面。

## defect or work item id

`page-content-spacing-background`（本地归一化 ID）。

## defect title

浅色页面内容区露出灰色底层且边距视觉不一致。

## observed behavior

- 弹幕面板、数据看板、排行榜、敏感词、黑名单的白色页面 surface 后方可见灰色内容区。
- 截图红框所示区域的白色 surface 边界使内容间距看起来不连续。

## expected behavior

- 浅色主题下内容区为连续白色背景，不在页面四周或底部露出灰色底层。
- 内容与导航/视口边缘保持统一响应式边距。
- 其他同壳层页面同步生效。

## reproduction clues

1. 以浅色主题启动桌面应用。
2. 打开弹幕面板或其他点名页面。
3. 在 1280x800 附近视口观察内容区四周与底部。

## affected module or page

共享 `App.vue` 壳层；用户点名的弹幕面板、数据看板、排行榜、敏感词、黑名单；同一 `<router-view>` 下的其他页面。

## available screenshots, logs, or comments

- 附件截图：`codex-clipboard-dea82ffb-1920-4406-ac0e-e3ac8c981edf.png`
- 无运行时错误日志；属于视觉布局缺陷。

## verbatim defect evidence index

- U1 observed/pages：“弹幕面板、数据看板、排行榜、敏感词、黑名单页面的样式是不是有点问题”。
- U2 observed：“最后面的背景是灰色的”。
- U3 expected：“红框中的部分没有边距”。
- U4 scope：“其他页面同步修改”。
- U5 screenshot：红框标出主内容 surface，其周边可见灰色壳层背景。

## closed-world fix scope allowlist

- 将主壳层浅色背景与内容区统一为白色。
- 保留并验证共享内容区的 `p-3 sm:p-5` 响应式边距。
- 通过共享 shell owner 让全部路由页面同步生效。
- 深色主题现有背景层级保持不变。
- 不新增字段、文案、控件、交互、状态、导航或 API 语义。

## open questions and missing context

无阻塞问题。“其他页面”按字面与代码事实解释为同一 `App.vue` router-view 的所有路由，不扩展为逐页重设计。

