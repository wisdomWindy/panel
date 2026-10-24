# 缺陷来源

## 来源系统与范围

当前 Codex 会话中的用户直接反馈；项目 `bilibili-live-tool`。

## 缺陷标识与标题

`theme-switch-bilibili-accent`：主题切换失效、黑白混用及品牌主色不一致。

## Observed Behavior

1. “界面的深色、浅色主题切换没用”。
2. “目前界面一块黑、一块白的主题配置是不是没生效还是冲突了”。
3. 主要颜色尚未统一为 bilibili 主题色。

## Expected Behavior

用户确认：按 `.dark` 单一主题源统一 Tailwind 与 Element Plus；系统模式响应系统变化；默认主色使用 `#FB7299`，旧默认 `#FF6699` 自动迁移，其他自定义色保留。

## 复现与影响页面

在设置页切换浅色/深色/系统，观察外壳、设置内容和 Element Plus 控件颜色。当前生产 CSS 的 `dark:` 规则位于 `prefers-color-scheme: dark` 媒体查询，而运行时切换根节点 `.dark`。

## 附件、日志与证据

已有安装版运行截图；构建 CSS 明确显示 `@media (prefers-color-scheme:dark)`，同时 `projection.ts` 使用 `root.classList.toggle('dark', isDark)`。

## Verbatim Defect Evidence Index

- U1：“界面的深色、浅色主题切换没用”。
- U2：“目前界面一块黑、一块白的主题配置是不是没生效还是冲突了”。
- U3：“把主要颜色改为bilibili主题色”。
- U4：用户回复“确认”，批准 `#FB7299`、系统监听、完整色阶、旧默认迁移与保留自定义色方案。

## Closed-World Fix Scope Allowlist

- 修复三种主题模式的视觉投影与系统变化监听。
- 消除 Tailwind/Element Plus 黑白主题冲突。
- 默认主色改为 `#FB7299`，补齐 Element Plus 派生色。
- 仅将精确旧默认 `#FF6699` 迁移为新默认；其他自定义色保持。
- 不改布局、文案、导航、权限、API 业务语义或其他效果配置颜色。

## 未决问题

none。

