# 代码上下文

## Context Requirement

bugfix-intake 需要追踪设置值到 DOM/CSS 的完整链；结构范围局部，code graph preferred but not required。

## Graph Availability / Bootstrap / Fallback

- graph status: missing；检测方式为仓库工具清单与当前可用命令。
- bootstrap attempted: no；S1 单链路用直接引用搜索已足够，安装图工具收益不足。
- fallback: yes；使用 `rg`、源码、构建 CSS 与实时 DOM。残余盲点由生产构建/安装版切换验证覆盖。

## Relevant Entrypoints

设置页 `InterfaceSettingsPanel.vue` -> store preview/save -> backend -> `previewUi` watch -> `applyUiProjection` -> root class/CSS variables -> Tailwind/Element Plus。

## Functional Chain Under Change

变更 owner 为 `projection.ts`、`useApplicationShellRuntime.ts`、Tailwind 入口和 UI 默认值。邻近路由、页面布局、业务 store、媒体效果配色不变。测试是适配面，不是生产 owner。

## Key Symbols And Boundaries

- `applyUiProjection`: DOM theme/accent projection owner。
- `createApplicationShellRuntime`: watch 与 listener lifecycle owner。
- `UiPreferences::default`: 持久层默认 owner。
- `adaptUiPreferences`: renderer 边界归一 owner。
- `tailwind.css`: Tailwind 4 dark variant owner。

## Impact Scope

主题投影、系统媒体查询生命周期、默认/旧默认主色、Element Plus 色阶、CSS 构建输出。回归敏感区为 shell、设置页和所有 `dark:` utilities。

## Open Follow-Up Checks

构建后确认 dark selector 不再由媒体查询控制；安装版分别检查 light/dark/system 和控件颜色。

