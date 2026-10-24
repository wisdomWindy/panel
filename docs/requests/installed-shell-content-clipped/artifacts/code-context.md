# 代码上下文

## 涉及模块

- `packages/bilibili-live-tool/src/renderer/App.vue`：应用壳层网格和三个壳层组件的组装点。
- `packages/bilibili-live-tool/src/renderer/components/app-shell/AppShellToolbar.vue`：顶部工具栏。
- `packages/bilibili-live-tool/src/renderer/components/app-shell/AppShellNavigation.vue`：左侧导航。
- `packages/bilibili-live-tool/src/renderer/components/app-shell/AppShellStatusBar.vue`：底部状态栏。
- `packages/bilibili-live-tool/src/renderer/core/navigation.ts`：导航树来源。
- `packages/bilibili-live-tool/src/renderer/membership/channel.free.ts`：免费版导航过滤器。

## 运行时观察

- Vue 应用已挂载，路由内容的文本已进入 DOM。
- 未观察到 JavaScript 异常或未处理 Promise 拒绝。
- `AppShellToolbar` 和 `AppShellStatusBar` 对应位置为注释节点。
- `AppShellNavigation` 外层已渲染，但导航数组在运行时表现为空。

## 工具降级说明

当前工作区没有可用的 code graph 查询命令，也未声明对应安装或引导流程。本次按规则降级为定向文本搜索、Git 历史和 WebView 运行时 DOM/CDP 证据。

## 当前根因状态

已通过 CDP `Debugger.setPauseOnExceptions` 捕获到根因：`vue-i18n 9.14.5` 的默认消息编译器在生产包首次翻译时执行 `new Function`，被 Tauri 的严格 CSP 拒绝，抛出 `EvalError`。依赖 `t(...)` 的壳层组件因首次渲染失败而被 Vue 替换为注释节点，进而造成网格子项错位和主内容裁剪。

## TypeScript 与第三方契约

- 目标文件由 `packages/bilibili-live-tool/tsconfig.json` 管理：`strict=true`、`moduleResolution=bundler`、`resolveJsonModule=true`。
- `vue-i18n` 公开导出 `MessageCompiler`，`createI18n` 支持通过 `messageCompiler` 注入自定义消息函数编译器。
- 当前两份 locale JSON 只使用字面文本与命名占位符；唯一的参数化文案为 `dashboard.giftMessage`，格式为 `{giftName}` 和 `{count}`。没有复数、链接消息或列表占位符。

