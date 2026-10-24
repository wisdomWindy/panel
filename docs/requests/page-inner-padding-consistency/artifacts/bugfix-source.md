# 缺陷来源

## 来源系统与范围

- 来源：当前会话用户反馈。
- 项目范围：biliLiveHelper 主应用路由页面。
- 缺陷 ID：无外部编号，内部标识 `page-inner-padding-consistency`。
- 标题：四个页面重复内边距导致内容起始位置不一致。

## Observed Behavior

上舰特效、礼物提醒、礼物音效、插件管理页面的内容与侧栏之间距离比其他页面大。截图红框显示上舰特效页面在共享内容区边距之外仍有一段额外空白。

## Expected Behavior

四个页面与其他页面使用一致的内容区外边距；共享壳层负责页面外侧间距，route 页面不再重复增加根级 padding。

## 复现线索与受影响页面

在桌面宽度进入 `/captain-effect`、`/gift-reminder`、`/gift-sound`、`/plugins`，对比 `/tts` 或 `/recording` 的标题左边界。截图尺寸约 1280x800。

## 截图、日志与评论

- 附件：`C:/Users/YANGJI~1/AppData/Local/Temp/codex-clipboard-70c310db-bd57-4556-b43e-36c808bfae2f.png`
- 无错误日志；纯布局缺陷。

## Verbatim Defect Evidence Index

- U1："上舰特效、礼物提醒、礼物音效、插件管理页面的边距与其他页面不一致，例如：图片中红框中部分的距离"

## Closed-World Fix Scope Allowlist

- 删除四个点名页面根节点重复的水平/垂直 padding utility。
- 保留 `App.vue` `<main>` 的 `p-3 sm:p-5` 作为唯一页面外侧间距 owner。
- 增加静态回归测试，防止四个页面再次叠加根级 padding。
- 不改变文案、控件、内部间距、业务逻辑、状态、路由、API、主题颜色或其他页面。

## 开放问题与缺失上下文

无。用户点名范围、截图与既有 shell 间距合同足以确定预期。
